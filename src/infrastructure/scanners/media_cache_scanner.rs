//! Media cache scanner for Cleanux.
//!
//! Scans and cleans media-related cache directories:
//! - Steam shader cache (~/.steam/steamapps/shadercache/)
//! - Steam download cache (~/.steam/steamapps/download/)
//! - Spotify cache (~/.cache/spotify/)
//! - VLC cache (~/.cache/vlc/)
//! - Thumbnail cache (~/.cache/thumbnails/)
//! - Media artwork cache (~/.cache/media-art/)

use crate::infrastructure::sys_utils::{get_dir_size, home_dir};
use dioxus_shared::Result;
use serde::{Deserialize, Serialize};

/// Media cache item with path and size info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaCacheItem {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub size_human: String,
}

/// Summary of all media caches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaCacheSummary {
    pub steam_shader_bytes: u64,
    pub steam_download_bytes: u64,
    pub spotify_bytes: u64,
    pub vlc_bytes: u64,
    pub thumbnail_bytes: u64,
    pub media_art_bytes: u64,
    pub total_bytes: u64,
    pub total_human: String,
}

/// Format bytes to human-readable string
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Get Steam shader cache path (~/.steam/steamapps/shadercache/)
fn steam_shader_path() -> std::path::PathBuf {
    home_dir().join(".steam/steamapps/shadercache")
}

/// Get Steam download cache path (~/.steam/steamapps/download/)
fn steam_download_path() -> std::path::PathBuf {
    home_dir().join(".steam/steamapps/download")
}

/// Get Spotify cache path (~/.cache/spotify/)
fn spotify_cache_path() -> std::path::PathBuf {
    home_dir().join(".cache/spotify")
}

/// Get VLC cache path (~/.cache/vlc/)
fn vlc_cache_path() -> std::path::PathBuf {
    home_dir().join(".cache/vlc")
}

/// Get thumbnail cache path (~/.cache/thumbnails/)
fn thumbnail_cache_path() -> std::path::PathBuf {
    home_dir().join(".cache/thumbnails")
}

/// Get media artwork cache path (~/.cache/media-art/)
fn media_art_cache_path() -> std::path::PathBuf {
    home_dir().join(".cache/media-art")
}

/// Scan a single media cache directory
pub async fn scan_media_cache(path: &std::path::Path, name: &str) -> MediaCacheItem {
    let size = get_dir_size(path).await.unwrap_or(0);
    MediaCacheItem {
        name: name.to_string(),
        path: path.display().to_string(),
        size_bytes: size,
        size_human: format_bytes(size),
    }
}

/// Get media cache summary for all tracked caches
pub async fn get_media_cache_summary() -> Result<MediaCacheSummary> {
    let shader_path = steam_shader_path();
    let download_path = steam_download_path();
    let spotify_path = spotify_cache_path();
    let vlc_path = vlc_cache_path();
    let thumb_path = thumbnail_cache_path();
    let art_path = media_art_cache_path();

    let shader_bytes = get_dir_size(&shader_path).await.unwrap_or(0);
    let download_bytes = get_dir_size(&download_path).await.unwrap_or(0);
    let spotify_bytes = get_dir_size(&spotify_path).await.unwrap_or(0);
    let vlc_bytes = get_dir_size(&vlc_path).await.unwrap_or(0);
    let thumb_bytes = get_dir_size(&thumb_path).await.unwrap_or(0);
    let art_bytes = get_dir_size(&art_path).await.unwrap_or(0);

    let total = shader_bytes + download_bytes + spotify_bytes + vlc_bytes + thumb_bytes + art_bytes;

    Ok(MediaCacheSummary {
        steam_shader_bytes: shader_bytes,
        steam_download_bytes: download_bytes,
        spotify_bytes,
        vlc_bytes,
        thumbnail_bytes: thumb_bytes,
        media_art_bytes: art_bytes,
        total_bytes: total,
        total_human: format_bytes(total),
    })
}

/// Clean Steam shader cache
pub async fn clean_steam_shader_cache() -> Result<u64> {
    let path = steam_shader_path();
    let size = get_dir_size(&path).await.unwrap_or(0);
    if path.exists() {
        tokio::fs::remove_dir_all(&path).await.ok();
        tokio::fs::create_dir_all(&path).await.ok();
    }
    Ok(size)
}

/// Clean Steam download cache
pub async fn clean_steam_download_cache() -> Result<u64> {
    let path = steam_download_path();
    let size = get_dir_size(&path).await.unwrap_or(0);
    if path.exists() {
        tokio::fs::remove_dir_all(&path).await.ok();
        tokio::fs::create_dir_all(&path).await.ok();
    }
    Ok(size)
}

/// Clean Spotify cache
pub async fn clean_spotify_cache() -> Result<u64> {
    let path = spotify_cache_path();
    let size = get_dir_size(&path).await.unwrap_or(0);
    if path.exists() {
        tokio::fs::remove_dir_all(&path).await.ok();
        tokio::fs::create_dir_all(&path).await.ok();
    }
    Ok(size)
}

/// Clean VLC cache
pub async fn clean_vlc_cache() -> Result<u64> {
    let path = vlc_cache_path();
    let size = get_dir_size(&path).await.unwrap_or(0);
    if path.exists() {
        tokio::fs::remove_dir_all(&path).await.ok();
        tokio::fs::create_dir_all(&path).await.ok();
    }
    Ok(size)
}

/// Clean thumbnail cache
pub async fn clean_thumbnail_cache() -> Result<u64> {
    let path = thumbnail_cache_path();
    let size = get_dir_size(&path).await.unwrap_or(0);
    if path.exists() {
        tokio::fs::remove_dir_all(&path).await.ok();
        tokio::fs::create_dir_all(&path).await.ok();
    }
    Ok(size)
}

/// Clean media artwork cache
pub async fn clean_media_art_cache() -> Result<u64> {
    let path = media_art_cache_path();
    let size = get_dir_size(&path).await.unwrap_or(0);
    if path.exists() {
        tokio::fs::remove_dir_all(&path).await.ok();
        tokio::fs::create_dir_all(&path).await.ok();
    }
    Ok(size)
}
