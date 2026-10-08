//! JunkCleanerService — scans and cleans common junk locations
//!
//! Scans browser cache (~/.cache), temp files (/tmp), and thumbnails (~/.cache/thumbnails).

use crate::infrastructure::sys_utils;
use std::path::PathBuf;
use tokio::fs;
use walkdir::WalkDir;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JunkItem {
  pub path: String,
  pub category: String,
  pub size: u64,
  pub modified: String,
}

pub struct JunkCleanerService;

impl JunkCleanerService {
  pub fn new() -> Self {
    Self
  }
}

impl Default for JunkCleanerService {
  fn default() -> Self {
    Self::new()
  }
}

impl JunkCleanerService {
  /// Scan common junk locations: browser cache, temp files, thumbnails.
  pub async fn scan_junk(&self) -> Result<Vec<JunkItem>, String> {
    let mut items = Vec::new();
    let home = sys_utils::home_dir();

    // Browser cache: ~/.cache
    let cache_dir = home.join(".cache");
    if cache_dir.exists() {
      let cache_items = self.scan_dir(cache_dir.as_path(), "browser_cache").await?;
      items.extend(cache_items);
    }

    // Temp files: /tmp
    let temp_dir = PathBuf::from("/tmp");
    if temp_dir.exists() {
      let temp_items = self.scan_dir(temp_dir.as_path(), "temp_files").await?;
      items.extend(temp_items);
    }

    // Thumbnails: ~/.cache/thumbnails
    let thumbnails_dir = home.join(".cache").join("thumbnails");
    if thumbnails_dir.exists() {
      let thumb_items = self
        .scan_dir(thumbnails_dir.as_path(), "thumbnails")
        .await?;
      items.extend(thumb_items);
    }

    Ok(items)
  }

  /// Clean the provided junk items, returning bytes freed.
  pub async fn clean_junk(&self, items: Vec<JunkItem>) -> Result<u64, String> {
    let mut bytes_freed: u64 = 0;

    for item in items {
      let path = PathBuf::from(&item.path);
      if path.exists() {
        match fs::metadata(&path).await {
          Ok(metadata) => {
            let file_size = if metadata.is_file() {
              metadata.len()
            } else {
              // For directories, compute size first
              sys_utils::get_dir_size(&path).await.unwrap_or(0)
            };

            match fs::remove_file(&path).await {
              Ok(_) => bytes_freed += file_size,
              Err(_) => {
                // Try removing as directory if file removal failed
                let _ = fs::remove_dir_all(&path).await;
              }
            }
          }
          Err(_) => {}
        }
      }
    }

    Ok(bytes_freed)
  }

  async fn scan_dir(&self, dir: &std::path::Path, category: &str) -> Result<Vec<JunkItem>, String> {
    let mut items = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
      let path = entry.path();
      if path.is_file() {
        if let Ok(metadata) = entry.metadata() {
          let size = metadata.len();
          let modified = metadata
            .modified()
            .map(|t| {
              let datetime: chrono::DateTime<chrono::Utc> = t.into();
              datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            })
            .unwrap_or_else(|_| "unknown".to_string());

          items.push(JunkItem {
            path: path.to_string_lossy().to_string(),
            category: category.to_string(),
            size,
            modified,
          });
        }
      }
    }

    Ok(items)
  }
}
