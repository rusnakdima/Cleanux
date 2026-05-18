/* helpers */
use crate::helpers::{data_empty_string, success_response};
/* models */
use crate::models::{DataValue, ResponseModel};
/* errors */
use crate::models::AppError;

use std::fs;
use std::path::PathBuf;

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

fn get_dir_size(path: &PathBuf) -> u64 {
  if !path.exists() {
    return 0;
  }
  fs::read_dir(path)
    .map(|entries| {
      entries
        .filter_map(|entry| entry.ok())
        .map(|entry| {
          let metadata = entry.metadata().ok();
          (entry, metadata)
        })
        .filter_map(|(entry, metadata)| metadata.map(|m| (entry, m)))
        .map(|(entry, metadata)| {
          if metadata.is_dir() {
            get_dir_size(&entry.path())
          } else {
            metadata.len()
          }
        })
        .sum()
    })
    .unwrap_or(0)
}

fn remove_dir_contents(path: &PathBuf) -> MediaResult<u64> {
  if !path.exists() {
    return Ok(0);
  }
  let mut cleared: u64 = 0;
  for entry in fs::read_dir(path)? {
    let entry = entry?;
    let metadata = entry.metadata()?;
    if metadata.is_dir() {
      cleared += get_dir_size(&entry.path());
      fs::remove_dir_all(entry.path())?;
    } else {
      cleared += metadata.len();
      fs::remove_file(entry.path())?;
    }
  }
  Ok(cleared)
}

#[allow(non_snake_case)]
impl MediaCacheService {
  pub fn get_steam_cache_size(&self) -> u64 {
    let steam_home = dirs::home_dir()
      .map(|h| h.join(".steam/steam/steamapps/shader"))
      .unwrap_or_default();
    let local_share = dirs::home_dir()
      .map(|h| h.join(".local/share/Steam"))
      .unwrap_or_default();
    get_dir_size(&steam_home) + get_dir_size(&local_share)
  }

  pub fn get_steam_info(&self) -> SteamInfo {
    let steam_root = dirs::home_dir()
      .map(|h| h.join(".steam/steam"))
      .unwrap_or_default();
    let game_count = if steam_root.exists() {
      fs::read_dir(&steam_root)
        .ok()
        .map(|entries| {
          entries
            .filter_map(|e| e.ok())
            .filter(|e| {
              e.path()
                .components()
                .last()
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

    let shader_path = dirs::home_dir()
      .map(|h| h.join(".steam/steam/steamapps/shader"))
      .unwrap_or_default();
    let download_path = dirs::home_dir()
      .map(|h| h.join(".local/share/Steam"))
      .unwrap_or_default();

    SteamInfo {
      game_count,
      shader_cache_size: get_dir_size(&shader_path),
      download_cache_size: get_dir_size(&download_path),
    }
  }

  pub fn clean_steam_shader_cache(&self) -> Result<ResponseModel, ResponseModel> {
    self
      .clean_steam_shader_cache_inner()
      .map_err(|e| e.into_response())
  }

  fn clean_steam_shader_cache_inner(&self) -> MediaResult<ResponseModel> {
    let shader_path = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?
      .join(".steam/steam/steamapps/shader");

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

  pub fn clean_steam_download_cache(&self) -> Result<ResponseModel, ResponseModel> {
    self
      .clean_steam_download_cache_inner()
      .map_err(|e| e.into_response())
  }

  fn clean_steam_download_cache_inner(&self) -> MediaResult<ResponseModel> {
    let download_path = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?
      .join(".local/share/Steam");

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
    let cache_path = dirs::home_dir()
      .map(|h| h.join(".cache/spotify"))
      .unwrap_or_default();
    let local_share = dirs::home_dir()
      .map(|h| h.join(".local/share/spotify"))
      .unwrap_or_default();
    get_dir_size(&cache_path) + get_dir_size(&local_share)
  }

  pub fn clean_spotify_cache(&self) -> Result<ResponseModel, ResponseModel> {
    self
      .clean_spotify_cache_inner()
      .map_err(|e| e.into_response())
  }

  fn clean_spotify_cache_inner(&self) -> MediaResult<ResponseModel> {
    let cache_path = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?
      .join(".cache/spotify");
    let local_share = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?
      .join(".local/share/spotify");

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
    let cache_path = dirs::home_dir()
      .map(|h| h.join(".cache/vlc"))
      .unwrap_or_default();
    let config_path = dirs::home_dir()
      .map(|h| h.join(".config/vlc"))
      .unwrap_or_default();
    get_dir_size(&cache_path) + get_dir_size(&config_path)
  }

  pub fn clean_vlc_cache(&self) -> Result<ResponseModel, ResponseModel> {
    self.clean_vlc_cache_inner().map_err(|e| e.into_response())
  }

  fn clean_vlc_cache_inner(&self) -> MediaResult<ResponseModel> {
    let cache_path = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?
      .join(".cache/vlc");
    let config_path = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?
      .join(".config/vlc");

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
    let thumb_path = dirs::home_dir()
      .map(|h| h.join(".cache/thumbnails"))
      .unwrap_or_default();
    get_dir_size(&thumb_path)
  }

  pub fn clean_thumbnail_cache(&self) -> Result<ResponseModel, ResponseModel> {
    self
      .clean_thumbnail_cache_inner()
      .map_err(|e| e.into_response())
  }

  fn clean_thumbnail_cache_inner(&self) -> MediaResult<ResponseModel> {
    let thumb_path = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?
      .join(".cache/thumbnails");

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
    let icon_path = dirs::home_dir()
      .map(|h| h.join(".cache/icons"))
      .unwrap_or_default();
    get_dir_size(&icon_path)
  }

  pub fn clean_icon_cache(&self) -> Result<ResponseModel, ResponseModel> {
    self.clean_icon_cache_inner().map_err(|e| e.into_response())
  }

  fn clean_icon_cache_inner(&self) -> MediaResult<ResponseModel> {
    let icon_path = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?
      .join(".cache/icons");

    if !icon_path.exists() {
      return Ok(success_response("No icon cache found", data_empty_string()));
    }

    let cleared = remove_dir_contents(&icon_path)?;
    Ok(success_response(
      format!("Icon cache cleared: {}", format_size(cleared)),
      data_empty_string(),
    ))
  }

  pub fn get_media_cache_summary(&self) -> ResponseModel {
    let steam_info = self.get_steam_info();
    ResponseModel {
      status: crate::models::ResponseStatus::Success,
      message: "Media cache summary retrieved".to_string(),
      data: DataValue::Object(serde_json::json!({
          "steamShaderSize": steam_info.shader_cache_size,
          "steamDownloadSize": steam_info.download_cache_size,
          "steamGameCount": steam_info.game_count,
          "spotifyCacheSize": self.get_spotify_cache_size(),
          "vlcCacheSize": self.get_vlc_cache_size(),
          "thumbnailCacheSize": self.get_thumbnail_cache_size(),
          "iconCacheSize": self.get_icon_cache_size(),
      })),
    }
  }
}

fn format_size(bytes: u64) -> String {
  const KB: u64 = 1024;
  const MB: u64 = KB * 1024;
  const GB: u64 = MB * 1024;

  if bytes >= GB {
    format!("{:.2} GB", bytes as f64 / GB as f64)
  } else if bytes >= MB {
    format!("{:.2} MB", bytes as f64 / MB as f64)
  } else if bytes >= KB {
    format!("{:.2} KB", bytes as f64 / KB as f64)
  } else {
    format!("{} B", bytes)
  }
}
