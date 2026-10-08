//! MediaCacheService — scans and cleans media-related cache directories.
//!
//! Handles Steam shader/download cache, Spotify, VLC, thumbnails, and media artwork.

use crate::infrastructure::scanners::media_cache_scanner::{
  self, MediaCacheItem, MediaCacheSummary,
};
use serde::{Deserialize, Serialize};

pub type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

/// Result of a media cache cleaning operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaCacheCleanResult {
  pub category: String,
  pub freed_bytes: u64,
  pub success: bool,
}

/// Get summary of all media caches.
pub async fn get_media_cache_summary() -> Result<MediaCacheSummary> {
  media_cache_scanner::get_media_cache_summary()
    .await
    .map_err(anyhow::Error::msg)
}

/// Get individual media cache items.
pub async fn get_media_cache_items() -> Result<Vec<MediaCacheItem>> {
  let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/"));

  let paths = [
    ("Steam Shader", home.join(".steam/steamapps/shadercache")),
    ("Steam Download", home.join(".steam/steamapps/download")),
    ("Spotify", home.join(".cache/spotify")),
    ("VLC", home.join(".cache/vlc")),
    ("Thumbnails", home.join(".cache/thumbnails")),
    ("Media Art", home.join(".cache/media-art")),
  ];

  let mut items = Vec::new();
  for (name, path) in paths {
    let item = media_cache_scanner::scan_media_cache(&path, name).await;
    items.push(item);
  }
  Ok(items)
}

/// Clean Steam shader cache.
pub async fn clean_steam_shader() -> Result<MediaCacheCleanResult> {
  let freed = media_cache_scanner::clean_steam_shader_cache()
    .await
    .map_err(anyhow::Error::msg)?;
  Ok(MediaCacheCleanResult {
    category: "steam_shader".into(),
    freed_bytes: freed,
    success: true,
  })
}

/// Clean Steam download cache.
pub async fn clean_steam_download() -> Result<MediaCacheCleanResult> {
  let freed = media_cache_scanner::clean_steam_download_cache()
    .await
    .map_err(anyhow::Error::msg)?;
  Ok(MediaCacheCleanResult {
    category: "steam_download".into(),
    freed_bytes: freed,
    success: true,
  })
}

/// Clean Spotify cache.
pub async fn clean_spotify() -> Result<MediaCacheCleanResult> {
  let freed = media_cache_scanner::clean_spotify_cache()
    .await
    .map_err(anyhow::Error::msg)?;
  Ok(MediaCacheCleanResult {
    category: "spotify".into(),
    freed_bytes: freed,
    success: true,
  })
}

/// Clean VLC cache.
pub async fn clean_vlc() -> Result<MediaCacheCleanResult> {
  let freed = media_cache_scanner::clean_vlc_cache()
    .await
    .map_err(anyhow::Error::msg)?;
  Ok(MediaCacheCleanResult {
    category: "vlc".into(),
    freed_bytes: freed,
    success: true,
  })
}

/// Clean thumbnail cache.
pub async fn clean_thumbnails() -> Result<MediaCacheCleanResult> {
  let freed = media_cache_scanner::clean_thumbnail_cache()
    .await
    .map_err(anyhow::Error::msg)?;
  Ok(MediaCacheCleanResult {
    category: "thumbnails".into(),
    freed_bytes: freed,
    success: true,
  })
}

/// Clean media artwork cache.
pub async fn clean_media_art() -> Result<MediaCacheCleanResult> {
  let freed = media_cache_scanner::clean_media_art_cache()
    .await
    .map_err(anyhow::Error::msg)?;
  Ok(MediaCacheCleanResult {
    category: "media_art".into(),
    freed_bytes: freed,
    success: true,
  })
}

/// Clean all media caches.
pub async fn clean_all() -> Result<Vec<MediaCacheCleanResult>> {
  let mut results = Vec::new();
  results.push(clean_steam_shader().await?);
  results.push(clean_steam_download().await?);
  results.push(clean_spotify().await?);
  results.push(clean_vlc().await?);
  results.push(clean_thumbnails().await?);
  results.push(clean_media_art().await?);
  Ok(results)
}
