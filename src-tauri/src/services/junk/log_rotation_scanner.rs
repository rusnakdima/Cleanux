use crate::models::AppError;
use crate::services::junk::types::{JunkCategory, JunkItem};
use std::fs;
use std::path::Path;

pub struct LogRotationScanner;

impl LogRotationScanner {
  pub fn scan() -> Result<Vec<JunkItem>, AppError> {
    let mut items = Vec::new();
    let log_dir = Path::new("/var/log");

    if !log_dir.exists() {
      return Ok(Vec::new());
    }

    let mut total_size = 0u64;
    let mut total_count = 0u32;

    if let Ok(entries) = fs::read_dir(log_dir) {
      for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
          if let Some(ext) = path.extension() {
            if ext == "gz" {
              if let Ok(metadata) = fs::metadata(&path) {
                total_size += metadata.len();
                total_count += 1;
              }
            }
          }
          if let Some(filename) = path.file_name() {
            let name = filename.to_string_lossy();
            if name.ends_with(".old")
              || name.ends_with(".bak")
              || name.ends_with(".1")
              || name.ends_with(".2")
            {
              if let Ok(metadata) = fs::metadata(&path) {
                total_size += metadata.len();
                total_count += 1;
              }
            }
          }
        }
      }
    }

    if total_count > 0 {
      items.push(JunkItem {
        path: "/var/log".to_string(),
        size: total_size,
        category: JunkCategory::Logs,
        description: "Rotated and archived log files".to_string(),
        file_count: total_count,
      });
    }

    Ok(items)
  }

  pub fn clean() -> Result<u64, AppError> {
    let log_dir = Path::new("/var/log");
    if !log_dir.exists() {
      return Ok(0);
    }

    let mut cleaned_count = 0u64;

    if let Ok(entries) = fs::read_dir(log_dir) {
      for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
          let should_delete = path.extension().map(|e| e == "gz").unwrap_or(false)
            || path
              .file_name()
              .map(|name| {
                let name = name.to_string_lossy();
                name.ends_with(".old")
                  || name.ends_with(".bak")
                  || name.ends_with(".1")
                  || name.ends_with(".2")
              })
              .unwrap_or(false);

          if should_delete {
            if fs::remove_file(&path).is_ok() {
              cleaned_count += 1;
            }
          }
        }
      }
    }

    Ok(cleaned_count)
  }
}
