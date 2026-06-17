use crate::models::AppError;
use crate::utils::format_size;
use chrono::{DateTime, Local};
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RotatedLogInfo {
  pub path: String,
  pub size_bytes: u64,
  pub size_human: String,
  pub modified: String,
  pub compression_ratio: Option<f64>,
}

pub struct RotatedLogHandler;

impl RotatedLogHandler {
  const LOG_DIRS: &'static [&'static str] = &[
    "/var/log",
    "/var/log/apache2",
    "/var/log/nginx",
    "/var/log/apache",
  ];

  const ROTATION_PATTERNS: &'static [&'static str] =
    &[".gz", ".old", ".1", ".2", ".bz2", ".xz", ".lz4"];

  pub fn get_size() -> u64 {
    Self::get_size_inner().unwrap_or(0)
  }

  fn get_size_inner() -> Result<u64, AppError> {
    let mut total_size = 0u64;

    for dir_path in Self::LOG_DIRS {
      let dir = Path::new(dir_path);
      if dir.exists() {
        if let Ok(entries) = fs::read_dir(dir) {
          for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
              if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if Self::ROTATION_PATTERNS.iter().any(|p| ext_str.contains(p))
                  || path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.ends_with(".1") || n.ends_with(".old") || n.contains("rotate"))
                    .unwrap_or(false)
                {
                  if let Ok(meta) = fs::metadata(&path) {
                    total_size += meta.len();
                  }
                }
              }
            }
          }
        }
      }
    }

    Ok(total_size)
  }

  pub fn get_logs() -> Vec<RotatedLogInfo> {
    Self::get_logs_inner().unwrap_or_default()
  }

  fn get_logs_inner() -> Result<Vec<RotatedLogInfo>, AppError> {
    let mut logs = Vec::new();

    for dir_path in Self::LOG_DIRS {
      let dir = Path::new(dir_path);
      if dir.exists() {
        if let Ok(entries) = fs::read_dir(dir) {
          for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
              let is_rotated = path
                .extension()
                .map(|e| {
                  let ext = e.to_string_lossy().to_lowercase();
                  Self::ROTATION_PATTERNS.iter().any(|p| ext.contains(p))
                })
                .unwrap_or(false);

              let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
              let is_old_log = filename.ends_with(".1")
                || filename.ends_with(".old")
                || filename.contains("rotate")
                || filename.ends_with(".2")
                || filename.ends_with(".gz");

              if is_rotated || is_old_log {
                if let Ok(meta) = fs::metadata(&path) {
                  let modified: DateTime<Local> =
                    meta.modified().unwrap_or(SystemTime::UNIX_EPOCH).into();
                  logs.push(RotatedLogInfo {
                    path: path.to_string_lossy().into_owned(),
                    size_bytes: meta.len(),
                    size_human: format_size(meta.len()),
                    modified: modified.format("%Y-%m-%d %H:%M:%S").to_string(),
                    compression_ratio: None,
                  });
                }
              }
            }
          }
        }
      }
    }

    logs.sort_by_key(|b| std::cmp::Reverse(b.size_bytes));
    Ok(logs)
  }

  pub fn clean_old_logs(days: u32) -> Result<(u64, Vec<String>), AppError> {
    let cutoff = SystemTime::now()
      .checked_sub(std::time::Duration::from_secs(days as u64 * 86400))
      .unwrap_or(SystemTime::UNIX_EPOCH);

    let logs = Self::get_logs_inner()?;
    let mut cleaned_count = 0u64;
    let mut errors: Vec<String> = Vec::new();

    for log in logs {
      if let Ok(meta) = fs::metadata(&log.path) {
        if let Ok(modified) = meta.modified() {
          if modified < cutoff {
            match fs::remove_file(&log.path) {
              Ok(_) => cleaned_count += 1,
              Err(e) => errors.push(format!("{}: {}", log.path, e)),
            }
          }
        }
      }
    }

    Ok((cleaned_count, errors))
  }
}
