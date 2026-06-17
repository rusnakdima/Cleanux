/* helpers */
use crate::utils::common_paths::CommonPath;
use crate::utils::{
  data_empty_string, format_size, get_dir_size, remove_dir_contents, service_method_full,
  success_response,
};
/* models */
use crate::models::{Response, Status};
/* errors */
use crate::models::AppError;

use serde_json::Value;
use std::fs;

pub struct MediaCacheService;

#[derive(Debug, Clone)]
pub struct SteamInfo {
  pub game_count: u32,
  pub shader_cache_size: u64,
  pub download_cache_size: u64,
}

#[derive(Debug, Clone)]
pub struct MediaCacheSummary {
  pub steam_shader_size: u64,
  pub steam_download_size: u64,
  pub steam_game_count: u32,
  pub spotify_cache_size: u64,
  pub vlc_cache_size: u64,
  pub thumbnail_cache_size: u64,
  pub icon_cache_size: u64,
}

type MediaResult<T> = Result<T, AppError>;

#[allow(non_snake_case)]
impl MediaCacheService {
  pub fn get_steam_info(&self) -> SteamInfo {
    let steam_root = CommonPath::SteamRoot.path().unwrap_or_default();
    let game_count = if steam_root.exists() {
      fs::read_dir(&steam_root)
        .ok()
        .map(|entries| {
          entries
            .filter_map(|e| e.ok())
            .filter(|e| {
              e.path()
                .components()
                .next_back()
                .map(|c| {
                  let s = c.as_os_str().to_string_lossy();
                  s.starts_with("steamapps") || s.contains("appmanifest")
                })
                .unwrap_or(false)
            })
            .count() as u32
        })
        .unwrap_or(0)
    } else {
      0
    };

    let shader_path = CommonPath::SteamShaderCache.path().unwrap_or_default();
    let download_path = CommonPath::SteamDownloadCache.path().unwrap_or_default();

    SteamInfo {
      game_count,
      shader_cache_size: get_dir_size(&shader_path),
      download_cache_size: get_dir_size(&download_path),
    }
  }

  service_method_full!(clean_steam_shader_cache => clean_steam_shader_cache_inner);

  fn clean_steam_shader_cache_inner(&self) -> MediaResult<Response<Value>> {
    let shader_path = CommonPath::SteamShaderCache
      .path()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;

    if !shader_path.exists() {
      return Ok(success_response(
        "No Steam shader cache found",
        data_empty_string(),
      ));
    }

    let cleared = remove_dir_contents(&shader_path)?;
    Ok(success_response(
      format!("Steam shader cache cleared: {}", format_size(cleared)),
      data_empty_string(),
    ))
  }

  service_method_full!(clean_steam_download_cache => clean_steam_download_cache_inner);

  fn clean_steam_download_cache_inner(&self) -> MediaResult<Response<Value>> {
    let download_path = CommonPath::SteamDownloadCache
      .path()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;

    if !download_path.exists() {
      return Ok(success_response(
        "No Steam download cache found",
        data_empty_string(),
      ));
    }

    let cleared = remove_dir_contents(&download_path)?;
    Ok(success_response(
      format!("Steam download cache cleared: {}", format_size(cleared)),
      data_empty_string(),
    ))
  }

  pub fn get_spotify_cache_size(&self) -> u64 {
    let cache_path = CommonPath::SpotifyCache.path().unwrap_or_default();
    let local_share = CommonPath::SpotifyLocalShare.path().unwrap_or_default();
    get_dir_size(&cache_path) + get_dir_size(&local_share)
  }

  service_method_full!(clean_spotify_cache => clean_spotify_cache_inner);

  fn clean_spotify_cache_inner(&self) -> MediaResult<Response<Value>> {
    let cache_path = CommonPath::SpotifyCache
      .path()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let local_share = CommonPath::SpotifyLocalShare
      .path()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;

    let mut cleared: u64 = 0;
    if cache_path.exists() {
      cleared += remove_dir_contents(&cache_path)?;
    }
    if local_share.exists() {
      cleared += remove_dir_contents(&local_share)?;
    }

    Ok(success_response(
      format!("Spotify cache cleared: {}", format_size(cleared)),
      data_empty_string(),
    ))
  }

  pub fn get_vlc_cache_size(&self) -> u64 {
    let cache_path = CommonPath::VlcCache.path().unwrap_or_default();
    let config_path = CommonPath::VlcConfig.path().unwrap_or_default();
    get_dir_size(&cache_path) + get_dir_size(&config_path)
  }

  service_method_full!(clean_vlc_cache => clean_vlc_cache_inner);

  fn clean_vlc_cache_inner(&self) -> MediaResult<Response<Value>> {
    let cache_path = CommonPath::VlcCache
      .path()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let config_path = CommonPath::VlcConfig
      .path()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;

    let mut cleared: u64 = 0;
    if cache_path.exists() {
      cleared += remove_dir_contents(&cache_path)?;
    }
    if config_path.exists() {
      cleared += remove_dir_contents(&config_path)?;
    }

    Ok(success_response(
      format!("VLC cache cleared: {}", format_size(cleared)),
      data_empty_string(),
    ))
  }

  pub fn get_thumbnail_cache_size(&self) -> u64 {
    let thumb_path = CommonPath::Thumbnails.path().unwrap_or_default();
    get_dir_size(&thumb_path)
  }

  service_method_full!(clean_thumbnail_cache => clean_thumbnail_cache_inner);

  fn clean_thumbnail_cache_inner(&self) -> MediaResult<Response<Value>> {
    let thumb_path = CommonPath::Thumbnails
      .path()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;

    if !thumb_path.exists() {
      return Ok(success_response(
        "No thumbnail cache found",
        data_empty_string(),
      ));
    }

    let cleared = remove_dir_contents(&thumb_path)?;
    Ok(success_response(
      format!("Thumbnail cache cleared: {}", format_size(cleared)),
      data_empty_string(),
    ))
  }

  pub fn get_icon_cache_size(&self) -> u64 {
    let icon_path = CommonPath::IconCache.path().unwrap_or_default();
    get_dir_size(&icon_path)
  }

  service_method_full!(clean_icon_cache => clean_icon_cache_inner);

  fn clean_icon_cache_inner(&self) -> MediaResult<Response<Value>> {
    let icon_path = CommonPath::IconCache
      .path()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;

    if !icon_path.exists() {
      return Ok(success_response("No icon cache found", data_empty_string()));
    }

    let cleared = remove_dir_contents(&icon_path)?;
    Ok(success_response(
      format!("Icon cache cleared: {}", format_size(cleared)),
      data_empty_string(),
    ))
  }

  pub fn get_media_cache_summary(&self) -> Response<Value> {
    let steam_info = self.get_steam_info();
    Response {
      status: Status::Success,
      message: "Media cache summary retrieved".to_string(),
      data: serde_json::json!({
          "steam_shader_size": steam_info.shader_cache_size,
          "steam_download_size": steam_info.download_cache_size,
          "steam_game_count": steam_info.game_count,
          "spotify_cache_size": self.get_spotify_cache_size(),
          "vlc_cache_size": self.get_vlc_cache_size(),
          "thumbnail_cache_size": self.get_thumbnail_cache_size(),
          "icon_cache_size": self.get_icon_cache_size(),
      }),
    }
  }
}
