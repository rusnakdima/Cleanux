//! DownloadsCleanerService — scans and cleans old downloads
//!
//! Scans ~/Downloads for files older than a specified threshold.

use crate::infrastructure::sys_utils;
use chrono::{DateTime, Duration, Utc};
use tokio::fs;
use walkdir::WalkDir;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DownloadItem {
  pub path: String,
  pub size: u64,
  pub modified: String,
}

pub struct DownloadsCleanerService;

impl DownloadsCleanerService {
  pub fn new() -> Self {
    Self
  }
}

impl Default for DownloadsCleanerService {
  fn default() -> Self {
    Self::new()
  }
}

impl DownloadsCleanerService {
  /// Scan the Downloads directory.
  pub async fn scan_downloads(&self) -> Result<Vec<DownloadItem>, String> {
    let home = sys_utils::home_dir();
    let downloads = home.join("Downloads");

    if !downloads.exists() {
      return Ok(Vec::new());
    }

    let mut items = Vec::new();

    for entry in WalkDir::new(&downloads)
      .max_depth(1) // Only top-level files in Downloads
      .into_iter()
      .filter_map(|e| e.ok())
      .filter(|e| e.path().is_file())
    {
      if let Ok(metadata) = entry.metadata() {
        let size = metadata.len();
        let modified = metadata
          .modified()
          .map(|t| {
            let datetime: chrono::DateTime<chrono::Utc> = t.into();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
          })
          .unwrap_or_else(|_| "unknown".to_string());

        items.push(DownloadItem {
          path: entry.path().to_string_lossy().to_string(),
          size,
          modified,
        });
      }
    }

    Ok(items)
  }

  /// Delete downloads older than specified days, returning bytes freed.
  pub async fn clean_old_downloads(&self, older_than_days: u32) -> Result<u64, String> {
    let home = sys_utils::home_dir();
    let downloads = home.join("Downloads");

    if !downloads.exists() {
      return Ok(0);
    }

    let cutoff = Utc::now() - Duration::days(older_than_days as i64);
    let mut bytes_freed: u64 = 0;

    for entry in WalkDir::new(&downloads)
      .max_depth(1)
      .into_iter()
      .filter_map(|e| e.ok())
      .filter(|e| e.path().is_file())
    {
      if let Ok(metadata) = entry.metadata() {
        let modified = metadata.modified();
        let should_delete = if let Ok(modified) = modified {
          let datetime: DateTime<Utc> = modified.into();
          datetime < cutoff
        } else {
          false
        };

        if should_delete {
          let path = entry.path();
          let size = metadata.len();
          if fs::remove_file(path).await.is_ok() {
            bytes_freed += size;
          }
        }
      }
    }

    Ok(bytes_freed)
  }
}
