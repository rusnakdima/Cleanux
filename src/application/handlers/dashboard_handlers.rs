//! Dashboard handlers — provide data for dashboard summary cards.
//!
//! Implements: system services summary, cache summary, trash summary,
//! log summary, large files summary.

use crate::infrastructure::sys_utils::{get_dir_size, get_log_dir_size, home_dir};
use crate::response::Response;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Summary of system services status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSummary {
  pub running: usize,
  pub stopped: usize,
  pub failed: usize,
}

/// Summary of cache sizes by category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSummary {
  pub browser_bytes: u64,
  pub thumbnails_bytes: u64,
  pub app_caches_bytes: u64,
  pub dev_caches_bytes: u64,
  pub total_bytes: u64,
  pub total_human: String,
}

/// Summary of trash directories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrashSummary {
  pub files_bytes: u64,
  pub expunged_bytes: u64,
  pub total_bytes: u64,
  pub total_human: String,
}

/// Summary of log directory sizes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogSummary {
  pub var_log_bytes: u64,
  pub journal_bytes: u64,
  pub total_bytes: u64,
  pub total_human: String,
}

/// Summary of large files found on the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargeFilesSummary {
  pub count: usize,
  pub total_bytes: u64,
  pub total_human: String,
}

/// Format bytes to human-readable string.
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

/// Get system services summary — counts running/stopped/failed services.
pub async fn get_system_services() -> Result<Response<ServiceSummary>, String> {
  let output = tokio::process::Command::new("systemctl")
    .args([
      "list-units",
      "--type=service",
      "--all",
      "--no-pager",
      "--no-legend",
    ])
    .output()
    .await
    .map_err(|e| e.to_string())?;

  let stdout = String::from_utf8_lossy(&output.stdout);
  let mut running = 0usize;
  let mut stopped = 0usize;
  let mut failed = 0usize;

  for line in stdout.lines() {
    let trimmed = line.trim();
    if trimmed.is_empty() {
      continue;
    }
    // Output format: UNIT LOAD ACTIVE SUB DESCRIPTION
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.len() >= 4 {
      let active = parts[2];
      match active {
        "active" => running += 1,
        "inactive" | "dead" => stopped += 1,
        "failed" => failed += 1,
        _ => {}
      }
    }
  }

  Ok(Response::success(
    ServiceSummary {
      running,
      stopped,
      failed,
    },
    Some("Service summary retrieved"),
  ))
}

/// Get cache summary — browser, thumbnails, app caches, dev caches.
pub async fn get_cache_summary() -> Result<Response<CacheSummary>, String> {
  let home = home_dir();

  let browser = home.join(".cache");
  let thumbnails = home.join(".cache/thumbnails");
  let app_caches = home.join(".cache");
  let dev_npm = home.join(".npm");
  let dev_pip = home.join(".cache/pip");
  let dev_cargo = home.join(".cargo");
  let dev_go = home.join("go/pkg/mod");
  let dev_maven = home.join(".m2/repository");
  let dev_gradle = home.join(".gradle/caches");

  let browser_bytes = get_dir_size(&browser).await.unwrap_or(0);
  let thumbnails_bytes = get_dir_size(&thumbnails).await.unwrap_or(0);

  // App caches = total cache minus known subdirs
  let app_caches_bytes = get_dir_size(&app_caches).await.unwrap_or(0);

  // Dev caches
  let dev_caches_bytes = get_dir_size(&dev_npm).await.unwrap_or(0)
    + get_dir_size(&dev_pip).await.unwrap_or(0)
    + get_dir_size(&dev_cargo).await.unwrap_or(0)
    + get_dir_size(&dev_go).await.unwrap_or(0)
    + get_dir_size(&dev_maven).await.unwrap_or(0)
    + get_dir_size(&dev_gradle).await.unwrap_or(0);

  let total_bytes = browser_bytes + thumbnails_bytes + app_caches_bytes + dev_caches_bytes;

  Ok(Response::success(
    CacheSummary {
      browser_bytes,
      thumbnails_bytes,
      app_caches_bytes,
      dev_caches_bytes,
      total_bytes,
      total_human: format_bytes(total_bytes),
    },
    Some("Cache summary retrieved"),
  ))
}

/// Get trash summary — ~/.local/share/Trash/files and expunged.
pub async fn get_trash_summary() -> Result<Response<TrashSummary>, String> {
  let home = home_dir();
  let trash_files = home.join(".local/share/Trash/files");
  let trash_expunged = home.join(".local/share/Trash/expunged");

  let files_bytes = get_dir_size(&trash_files).await.unwrap_or(0);
  let expunged_bytes = get_dir_size(&trash_expunged).await.unwrap_or(0);
  let total_bytes = files_bytes + expunged_bytes;

  Ok(Response::success(
    TrashSummary {
      files_bytes,
      expunged_bytes,
      total_bytes,
      total_human: format_bytes(total_bytes),
    },
    Some("Trash summary retrieved"),
  ))
}

/// Get log summary — /var/log size and journal size.
pub async fn get_log_summary() -> Result<Response<LogSummary>, String> {
  let journal = PathBuf::from("/var/log/journal");

  let var_log_bytes = get_log_dir_size().await.unwrap_or(0);
  let journal_bytes = get_dir_size(&journal).await.unwrap_or(0);
  let total_bytes = var_log_bytes + journal_bytes;

  Ok(Response::success(
    LogSummary {
      var_log_bytes,
      journal_bytes,
      total_bytes,
      total_human: format_bytes(total_bytes),
    },
    Some("Log summary retrieved"),
  ))
}

/// Get large files summary — scan home for files > 100MB.
pub async fn get_large_files_summary() -> Result<Response<LargeFilesSummary>, String> {
  let home = home_dir();
  const THRESHOLD: u64 = 100 * 1024 * 1024; // 100 MB

  let mut total_bytes: u64 = 0;
  let mut count: usize = 0;
  let mut stack = vec![home];

  while let Some(dir) = stack.pop() {
    let mut entries = match tokio::fs::read_dir(&dir).await {
      Ok(v) => v,
      Err(_) => continue,
    };
    while let Ok(Some(entry)) = entries.next_entry().await {
      let path = entry.path();
      if let Ok(meta) = entry.metadata().await {
        if meta.is_file() && meta.len() >= THRESHOLD {
          total_bytes += meta.len();
          count += 1;
        } else if meta.is_dir() {
          stack.push(path);
        }
      }
    }
  }

  Ok(Response::success(
    LargeFilesSummary {
      count,
      total_bytes,
      total_human: format_bytes(total_bytes),
    },
    Some("Large files summary retrieved"),
  ))
}
