//! Journal Log Management KAS handlers for Cleanux
//!
//! Provides journald vacuum, logrotate analysis, var-log usage, and largest-files commands.

use crate::error::AppError;
use crate::response::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use tokio::process::Command;

/// Journal usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalUsage {
  pub total_bytes: u64,
  pub total_entries: u64,
  pub oldest_timestamp: Option<String>,
  pub newest_timestamp: Option<String>,
}

/// Journal size information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalSize {
  pub bytes: u64,
  pub human: String,
}

/// Rotated log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotatedLog {
  pub path: String,
  pub size: u64,
  pub size_human: String,
}

/// Logrotate configuration entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogrotateConfig {
  pub path: String,
  pub size: u64,
}

/// Largest log file entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargestLogFile {
  pub path: String,
  pub size: u64,
  pub size_human: String,
  pub modified: String,
}

/// Log manager summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogManagerSummary {
  pub journal_size_bytes: u64,
  pub journal_size_human: String,
  pub var_log_size_bytes: u64,
  pub var_log_size_human: String,
  pub rotated_logs_count: usize,
  pub largest_log_files: Vec<LargestLogFile>,
}

/// Vacuum journal by size (e.g., "500M", "1G")
pub async fn vacuum_journal(size: &str) -> Result<Response<u64>, AppError> {
  tracing::info!("Vacuuming journal to {}", size);

  let output = Command::new("journalctl")
    .args(["--vacuum-size=".to_string() + size])
    .output()
    .await
    .map_err(|e| AppError::Internal(format!("journalctl failed: {}", e)))?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(AppError::Internal(format!(
      "journalctl vacuum-size failed: {}",
      stderr
    )));
  }

  let stdout = String::from_utf8_lossy(&output.stdout);
  tracing::info!("Journal vacuum result: {}", stdout);
  Ok(Response::success(
    0,
    Some(&format!("Journal vacuumed to {}", size)),
  ))
}

/// Vacuum journal by days (keep only last N days)
pub async fn vacuum_journal_by_days(days: u32) -> Result<Response<u64>, AppError> {
  tracing::info!("Vacuuming journal to {} days", days);

  let output = Command::new("journalctl")
    .args(["--vacuum-time=".to_string() + &days.to_string() + "d"])
    .output()
    .await
    .map_err(|e| AppError::Internal(format!("journalctl failed: {}", e)))?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(AppError::Internal(format!(
      "journalctl vacuum-time failed: {}",
      stderr
    )));
  }

  Ok(Response::success(
    0,
    Some(&format!("Journal vacuumed to {} days", days)),
  ))
}

/// Get journal usage statistics
pub async fn get_journal_usage() -> Result<Response<JournalUsage>, AppError> {
  let entries_output = Command::new("journalctl")
    .args(["--no-pager", "-b", "-n", "1", "--output=short-iso"])
    .output()
    .await;

  let oldest = entries_output
    .as_ref()
    .ok()
    .and_then(|o| String::from_utf8(o.stdout.clone()).ok())
    .and_then(|s| s.lines().next().map(|l| l.to_string()));

  let newest_output = Command::new("journalctl")
    .args(["--no-pager", "--output=short-iso"])
    .output()
    .await;

  let newest = newest_output
    .ok()
    .and_then(|o| String::from_utf8(o.stdout).ok())
    .and_then(|s| s.lines().last().map(|l| l.to_string()));

  let count_output = Command::new("journalctl")
    .args(["--no-pager", "--output=short-iso"])
    .output()
    .await
    .map_err(|e| AppError::Internal(format!("journalctl failed: {}", e)))?;

  let total_entries = String::from_utf8_lossy(&count_output.stdout)
    .lines()
    .count() as u64;

  Ok(Response::success(
    JournalUsage {
      total_bytes: 0,
      total_entries,
      oldest_timestamp: oldest,
      newest_timestamp: newest,
    },
    Some("Journal usage retrieved"),
  ))
}

/// Get journal size on disk
pub async fn get_journal_size() -> Result<Response<JournalSize>, AppError> {
  let output = Command::new("journalctl")
    .args(["--disk-usage"])
    .output()
    .await
    .map_err(|e| AppError::Internal(format!("journalctl disk-usage failed: {}", e)))?;

  if !output.status.success() {
    return Err(AppError::Internal(
      "journalctl --disk-usage failed".to_string(),
    ));
  }

  let stdout = String::from_utf8_lossy(&output.stdout);
  let line = stdout.lines().next().unwrap_or("");
  let size_str = line.split(':').nth(1).map(|s| s.trim()).unwrap_or("0");

  let bytes = parse_size_to_bytes(size_str);
  Ok(Response::success(
    JournalSize {
      bytes,
      human: size_str.to_string(),
    },
    Some("Journal size retrieved"),
  ))
}

/// Get /var/log usage
pub async fn get_var_log_usage() -> Result<Response<u64>, AppError> {
  let path = PathBuf::from("/var/log");
  if !path.exists() {
    return Ok(Response::success(0u64, Some("/var/log not found")));
  }

  let size = crate::infrastructure::sys_utils::get_dir_size(&path)
    .await
    .unwrap_or(0);

  Ok(Response::success(size, Some("Var log usage retrieved")))
}

/// Get largest log files in /var/log
pub async fn get_largest_log_files(
  limit: usize,
) -> Result<Response<Vec<LargestLogFile>>, AppError> {
  let log_dir = PathBuf::from("/var/log");
  let mut files: Vec<(String, u64, String)> = Vec::new();

  if log_dir.exists() {
    for entry in walkdir::WalkDir::new(&log_dir)
      .max_depth(4)
      .follow_links(false)
      .into_iter()
      .filter_map(|e| e.ok())
    {
      let path = entry.path();
      if path.is_file() {
        if let Ok(metadata) = entry.metadata() {
          let modified = metadata
            .modified()
            .ok()
            .and_then(|t| {
              chrono::DateTime::<chrono::Utc>::from(t)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
                .into()
            })
            .unwrap_or_else(|| "unknown".to_string());

          files.push((path.to_string_lossy().to_string(), metadata.len(), modified));
        }
      }
    }
  }

  files.sort_by_key(|a| std::cmp::Reverse(a.1));
  let result: Vec<LargestLogFile> = files
    .into_iter()
    .take(limit)
    .map(|(path, size, modified)| LargestLogFile {
      path,
      size,
      size_human: format_bytes(size),
      modified,
    })
    .collect();

  Ok(Response::success(
    result,
    Some("Largest log files retrieved"),
  ))
}

/// Get rotated logs (gz, .1, .old, etc.)
pub async fn get_rotated_logs() -> Result<Response<Vec<RotatedLog>>, AppError> {
  let log_dir = PathBuf::from("/var/log");
  let mut logs: Vec<RotatedLog> = Vec::new();

  if log_dir.exists() {
    for entry in walkdir::WalkDir::new(&log_dir)
      .max_depth(4)
      .follow_links(false)
      .into_iter()
      .filter_map(|e| e.ok())
    {
      let path = entry.path();
      if path.is_file() {
        let name = path
          .file_name()
          .map(|n| n.to_string_lossy().into_owned())
          .unwrap_or_default();

        if name.ends_with(".gz")
          || name.ends_with(".1")
          || name.ends_with(".old")
          || name.ends_with(".2")
          || name.ends_with(".bz2")
          || name.ends_with(".xz")
        {
          if let Ok(metadata) = entry.metadata() {
            let size = metadata.len();
            logs.push(RotatedLog {
              path: path.to_string_lossy().to_string(),
              size,
              size_human: format_bytes(size),
            });
          }
        }
      }
    }
  }

  logs.sort_by_key(|a| std::cmp::Reverse(a.size));
  Ok(Response::success(logs, Some("Rotated logs retrieved")))
}

/// Get total size of rotated logs
pub async fn get_rotated_logs_size() -> Result<Response<u64>, AppError> {
  let logs = get_rotated_logs().await?;
  let total: u64 = logs
    .data
    .as_ref()
    .map(|v| v.iter().map(|l| l.size).sum())
    .unwrap_or(0);
  Ok(Response::success(
    total,
    Some("Rotated logs size retrieved"),
  ))
}

/// Clean rotated logs
pub async fn clean_rotated_logs() -> Result<Response<u64>, AppError> {
  let logs = get_rotated_logs().await?;
  let mut freed: u64 = 0;

  if let Some(ref log_entries) = logs.data {
    for log in log_entries {
      if std::fs::remove_file(&log.path).is_ok() {
        freed += log.size;
      }
    }
  }

  tracing::info!("Cleaned rotated logs: {} bytes freed", freed);
  Ok(Response::success(
    freed,
    Some(&format!("Freed {} bytes of rotated logs", freed)),
  ))
}

/// Get logrotate configurations
pub async fn get_logrotate_configs() -> Result<Response<Vec<LogrotateConfig>>, AppError> {
  let mut configs: Vec<LogrotateConfig> = Vec::new();

  let config_dirs = vec!["/etc/logrotate.d", "/etc/logrotate.conf"];

  for config_path in config_dirs {
    let path = PathBuf::from(config_path);
    if path.exists() {
      if path.is_file() {
        if let Ok(metadata) = path.metadata() {
          configs.push(LogrotateConfig {
            path: config_path.to_string(),
            size: metadata.len(),
          });
        }
      } else if path.is_dir() {
        if let Ok(entries) = std::fs::read_dir(&path) {
          for entry in entries.filter_map(|e| e.ok()) {
            let entry_path = entry.path();
            if entry_path.is_file() {
              if let Ok(metadata) = entry_path.metadata() {
                configs.push(LogrotateConfig {
                  path: entry_path.to_string_lossy().to_string(),
                  size: metadata.len(),
                });
              }
            }
          }
        }
      }
    }
  }

  Ok(Response::success(
    configs,
    Some("Logrotate configs retrieved"),
  ))
}

/// Analyze logrotate configuration for a given path
pub async fn analyze_logrotate(path: &str) -> Result<Response<Value>, AppError> {
  let config_dir = PathBuf::from("/etc/logrotate.d");
  let mut result = serde_json::json!({
      "path": path,
      "configs": Vec::<Value>::new(),
      "has_config": false,
  });

  if config_dir.exists() {
    if let Ok(entries) = std::fs::read_dir(&config_dir) {
      let mut configs: Vec<Value> = Vec::new();
      for entry in entries.filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        if entry_path.is_file() {
          if let Ok(content) = std::fs::read_to_string(&entry_path) {
            if content.contains(path) {
              configs.push(serde_json::json!({
                  "config": entry_path.to_string_lossy().to_string(),
                  "content": content,
              }));
            }
          }
        }
      }
      result["has_config"] = serde_json::json!(!configs.is_empty());
      result["configs"] = serde_json::json!(configs);
    }
  }

  Ok(Response::success(
    result,
    Some("Logrotate analysis complete"),
  ))
}

/// Scan log rotations for a path
pub async fn scan_log_rotations(path: &str) -> Result<Response<Vec<RotatedLog>>, AppError> {
  let target = PathBuf::from(path);
  let parent = target.parent().unwrap_or(&target);
  let basename = target
    .file_name()
    .map(|n| n.to_string_lossy().into_owned())
    .unwrap_or_default();

  let mut rotations: Vec<RotatedLog> = Vec::new();

  if let Ok(entries) = std::fs::read_dir(parent) {
    for entry in entries.filter_map(|e| e.ok()) {
      let entry_path = entry.path();
      let name = entry_path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();

      if name == basename || name.starts_with(&basename) {
        if name.ends_with(".gz")
          || name.ends_with(".1")
          || name.ends_with(".old")
          || name.ends_with(".2")
          || name.ends_with(".bz2")
          || name.ends_with(".xz")
          || name == format!("{}.1", basename)
          || name == format!("{}.gz", basename)
        {
          if let Ok(metadata) = entry_path.metadata() {
            rotations.push(RotatedLog {
              path: entry_path.to_string_lossy().to_string(),
              size: metadata.len(),
              size_human: format_bytes(metadata.len()),
            });
          }
        }
      }
    }
  }

  rotations.sort_by_key(|a| std::cmp::Reverse(a.size));
  Ok(Response::success(rotations, Some("Log rotations scanned")))
}

/// Get combined log manager summary
pub async fn get_log_manager_summary() -> Result<Response<LogManagerSummary>, AppError> {
  let journal_size = get_journal_size().await?;
  let var_log_size = get_var_log_usage().await?;
  let largest_logs = get_largest_log_files(10).await?;
  let rotated = get_rotated_logs().await?;

  let var_log_size_val = var_log_size.data.unwrap_or(0);
  let rotated_count = rotated.data.as_ref().map(|v| v.len()).unwrap_or(0);
  let largest_from_response = largest_logs.data.unwrap_or_default();

  Ok(Response::success(
    LogManagerSummary {
      journal_size_bytes: journal_size.data.as_ref().map(|d| d.bytes).unwrap_or(0),
      journal_size_human: journal_size
        .data
        .as_ref()
        .map(|d| d.human.clone())
        .unwrap_or_default(),
      var_log_size_bytes: var_log_size_val,
      var_log_size_human: format_bytes(var_log_size_val),
      rotated_logs_count: rotated_count,
      largest_log_files: largest_from_response,
    },
    Some("Log manager summary retrieved"),
  ))
}

/// Get log summary (simple version)
pub async fn get_log_summary() -> Result<Response<Value>, AppError> {
  let journal_size = get_journal_size().await?;
  let var_log_size = get_var_log_usage().await?;
  let rotated = get_rotated_logs_size().await?;

  let var_log_size_val = var_log_size.data.unwrap_or(0);
  let rotated_size_val = rotated.data.unwrap_or(0);

  Ok(Response::success(
    serde_json::json!({
        "journal_size_bytes": journal_size.data.as_ref().map(|d| d.bytes).unwrap_or(0),
        "journal_size_human": journal_size.data.as_ref().map(|d| d.human.clone()).unwrap_or_default(),
        "var_log_size_bytes": var_log_size_val,
        "var_log_size_human": format_bytes(var_log_size_val),
        "rotated_logs_size_bytes": rotated_size_val,
        "rotated_logs_size_human": format_bytes(rotated_size_val),
    }),
    Some("Log summary retrieved"),
  ))
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

fn parse_size_to_bytes(s: &str) -> u64 {
  let s = s.trim().to_uppercase();
  let multiplier: u64 = if s.ends_with('G') {
    1024 * 1024 * 1024
  } else if s.ends_with('M') {
    1024 * 1024
  } else if s.ends_with('K') {
    1024
  } else {
    return s.parse::<u64>().unwrap_or(0);
  };
  let num_str: String = s
    .chars()
    .take_while(|c| c.is_ascii_digit() || *c == '.')
    .collect();
  let num: f64 = num_str.parse().unwrap_or(0.0);
  (num as u64).saturating_mul(multiplier)
}

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
