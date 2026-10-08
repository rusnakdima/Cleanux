//! Disk usage analysis service for Cleanux
//!
//! Provides disk space analysis, directory scanning, and size breakdown utilities.

use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::process::Command;

pub type Result<T, E = AppError> = std::result::Result<T, E>;

/// A single entry in the disk usage scan results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskUsageEntry {
  pub path: String,
  pub size: u64,
  pub is_dir: bool,
}

/// Result of a directory scan
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
  pub entries: Vec<DiskUsageEntry>,
  pub total_size: u64,
  pub entry_count: usize,
}

/// Size breakdown by category
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SizeBreakdown {
  pub total_space: u64,
  pub used_space: u64,
  pub free_space: u64,
  pub used_percent: f64,
}

/// Scan a directory recursively for disk usage.
///
/// Returns all entries sorted by size (largest first).
pub async fn scan_directory(path: &Path) -> Result<ScanResult> {
  let path_str = path.to_string_lossy();

  let output = Command::new("du")
    .args(["-a", "-b", &path_str])
    .output()
    .await
    .map_err(|e| AppError::Internal(format!("du command failed: {}", e)))?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(AppError::Internal(format!(
      "du scan failed for '{}': {}",
      path_str, stderr
    )));
  }

  let stdout = String::from_utf8_lossy(&output.stdout);
  let mut entries: Vec<DiskUsageEntry> = Vec::new();
  let mut total_size: u64 = 0;

  for line in stdout.lines() {
    let parts: Vec<&str> = line.splitn(2, '\t').collect();
    if parts.len() == 2 {
      if let Ok(size) = parts[0].parse::<u64>() {
        let entry_path = parts[1].to_string();
        let is_dir = entry_path.is_empty() || std::path::Path::new(&entry_path).is_dir();
        total_size += size;
        entries.push(DiskUsageEntry {
          path: entry_path,
          size,
          is_dir,
        });
      }
    }
  }

  // Sort by size descending
  entries.sort_by(|a, b| b.size.cmp(&a.size));

  Ok(ScanResult {
    entry_count: entries.len(),
    total_size,
    entries,
  })
}

/// Get disk space breakdown for a mount point or path.
///
/// Uses `df` to report total, used, and free space.
pub async fn get_size_breakdown(path: &Path) -> Result<SizeBreakdown> {
  let path_str = path.to_string_lossy();

  let output = Command::new("df")
    .args(["-B1", &path_str])
    .output()
    .await
    .map_err(|e| AppError::Internal(format!("df command failed: {}", e)))?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(AppError::Internal(format!(
      "df failed for '{}': {}",
      path_str, stderr
    )));
  }

  let stdout = String::from_utf8_lossy(&output.stdout);
  let mut lines = stdout.lines();

  // Skip header line
  let _header = lines.next();

  if let Some(data_line) = lines.next() {
    let fields: Vec<&str> = data_line.split_whitespace().collect();
    if fields.len() >= 4 {
      let total_space = fields[1]
        .parse::<u64>()
        .map_err(|_| AppError::Internal("failed to parse total space".into()))?;
      let used_space = fields[2]
        .parse::<u64>()
        .map_err(|_| AppError::Internal("failed to parse used space".into()))?;
      let free_space = fields[3]
        .parse::<u64>()
        .map_err(|_| AppError::Internal("failed to parse free space".into()))?;

      let used_percent = if total_space > 0 {
        (used_space as f64 / total_space as f64) * 100.0
      } else {
        0.0
      };

      return Ok(SizeBreakdown {
        total_space,
        used_space,
        free_space,
        used_percent,
      });
    }
  }

  Err(AppError::Internal("failed to parse df output".into()))
}
