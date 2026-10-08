//! LargeFileCleanerService — finds and deletes large files
//!
//! Scans a given path for files above a minimum size threshold.

use crate::infrastructure::sys_utils;
use std::path::Path;
use tokio::fs;
use walkdir::WalkDir;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LargeFileItem {
  pub path: String,
  pub size: u64,
  pub modified: String,
}

pub struct LargeFileCleanerService;

impl LargeFileCleanerService {
  pub fn new() -> Self {
    Self
  }
}

impl Default for LargeFileCleanerService {
  fn default() -> Self {
    Self::new()
  }
}

impl LargeFileCleanerService {
  /// Scan a directory for files larger than min_size bytes.
  pub async fn scan_large_files(
    &self,
    path: &str,
    min_size: u64,
  ) -> Result<Vec<LargeFileItem>, String> {
    let root = Path::new(path);
    if !root.exists() {
      return Err(format!("Path does not exist: {}", path));
    }

    let mut items = Vec::new();

    for entry in WalkDir::new(root)
      .into_iter()
      .filter_map(|e| e.ok())
      .filter(|e| e.path().is_file())
    {
      if let Ok(metadata) = entry.metadata() {
        let size = metadata.len();
        if size >= min_size {
          let modified = metadata
            .modified()
            .map(|t| {
              let datetime: chrono::DateTime<chrono::Utc> = t.into();
              datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            })
            .unwrap_or_else(|_| "unknown".to_string());

          items.push(LargeFileItem {
            path: entry.path().to_string_lossy().to_string(),
            size,
            modified,
          });
        }
      }
    }

    // Sort by size descending
    items.sort_by(|a, b| b.size.cmp(&a.size));

    Ok(items)
  }

  /// Delete the specified files, returning bytes freed.
  pub async fn delete_files(&self, paths: Vec<String>) -> Result<u64, String> {
    let mut bytes_freed: u64 = 0;

    for path_str in paths {
      let path = Path::new(&path_str);
      if path.exists() {
        let file_size = if path.is_file() {
          fs::metadata(path).await.map(|m| m.len()).unwrap_or(0)
        } else {
          sys_utils::get_dir_size(path).await.unwrap_or(0)
        };

        let result = if path.is_dir() {
          fs::remove_dir_all(path).await
        } else {
          fs::remove_file(path).await
        };

        if result.is_ok() {
          bytes_freed += file_size;
        }
      }
    }

    Ok(bytes_freed)
  }
}
