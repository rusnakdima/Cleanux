//! System utilities for Cleanux
//!
//! Provides cross-platform helpers for file scanning, CPU temp, etc.

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Get home directory with cross-platform fallback
pub fn home_dir() -> PathBuf {
  dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
}

/// Get total size of all files in a directory (recursive)
pub async fn get_dir_size(path: &Path) -> Result<u64, String> {
  if !path.exists() {
    return Ok(0);
  }

  let mut size = 0u64;
  let mut stack = vec![path.to_path_buf()];

  while let Some(dir) = stack.pop() {
    let mut entries = match tokio::fs::read_dir(&dir).await {
      Ok(v) => v,
      Err(_) => continue,
    };

    while let Ok(Some(entry)) = entries.next_entry().await {
      let entry_path = entry.path();
      if let Ok(metadata) = entry.metadata().await {
        if metadata.is_file() {
          size += metadata.len();
        } else if metadata.is_dir() {
          stack.push(entry_path);
        }
      }
    }
  }

  Ok(size)
}

/// Walk directory, collect (path_string, size) for files under max_depth
pub async fn walkdir_size(
  dir: &Path,
  results: &mut Vec<(String, u64)>,
  max_depth: usize,
) -> Result<(), String> {
  if !dir.exists() {
    return Ok(());
  }

  for entry in WalkDir::new(dir)
    .max_depth(max_depth)
    .follow_links(false)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    let path = entry.path();
    if path.is_file() {
      if let Ok(metadata) = entry.metadata() {
        results.push((path.to_string_lossy().to_string(), metadata.len()));
      }
    }
  }

  Ok(())
}

/// Delete files older than max_age_days in path. Returns bytes freed.
pub async fn delete_old_files(path: &Path, max_age_days: u32) -> Result<u64, String> {
  if !path.exists() {
    return Ok(0);
  }

  let use_cutoff = max_age_days > 0;
  let cutoff = Utc::now() - chrono::Duration::days(max_age_days as i64);

  let mut freed = 0u64;
  let mut stack = vec![path.to_path_buf()];

  while let Some(dir) = stack.pop() {
    let mut entries = match tokio::fs::read_dir(&dir).await {
      Ok(v) => v,
      Err(_) => continue,
    };

    while let Ok(Some(entry)) = entries.next_entry().await {
      let entry_path = entry.path();
      if let Ok(metadata) = entry.metadata().await {
        if metadata.is_dir() {
          stack.push(entry_path);
        } else if metadata.is_file() {
          // Check modification time
          if let Ok(modified) = metadata.modified() {
            let modified: DateTime<Utc> = modified.into();
            if !use_cutoff || modified < cutoff {
              let size = metadata.len();
              if tokio::fs::remove_file(&entry_path).await.is_ok() {
                freed += size;
              }
            }
          }
        }
      }
    }
  }

  Ok(freed)
}

/// Read CPU temperature from /sys/class/thermal/
pub async fn get_cpu_temp() -> Result<f64, String> {
  let mut temps = Vec::new();

  // Try thermal zones
  for entry in WalkDir::new("/sys/class/thermal")
    .max_depth(2)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    let path = entry.path();
    if path
      .file_name()
      .is_some_and(|n| n.to_string_lossy().starts_with("thermal_zone"))
    {
      let temp_file = path.join("temp");
      if let Ok(content) = tokio::fs::read_to_string(&temp_file).await {
        if let Ok(temp_milli) = content.trim().parse::<u32>() {
          temps.push(temp_milli as f64 / 1000.0);
        }
      }
    }
  }

  if temps.is_empty() {
    return Ok(f64::NAN);
  }

  Ok(temps.iter().sum::<f64>() / temps.len() as f64)
}

/// Find duplicate files by hashing
pub async fn find_duplicates_size(root: &Path, min_size: u64) -> Result<u64, String> {
  if !root.exists() {
    return Ok(0);
  }

  // Phase 1: collect files by size
  let mut size_groups: std::collections::HashMap<u64, Vec<PathBuf>> =
    std::collections::HashMap::new();

  for entry in WalkDir::new(root)
    .follow_links(false)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    let path = entry.path();
    if path.is_file() {
      if let Ok(metadata) = entry.metadata() {
        let size = metadata.len();
        if size >= min_size {
          size_groups
            .entry(size)
            .or_default()
            .push(path.to_path_buf());
        }
      }
    }
  }

  // Phase 2: hash files with same size
  let mut hash_groups: std::collections::HashMap<String, Vec<PathBuf>> =
    std::collections::HashMap::new();

  for (_, paths) in size_groups {
    if paths.len() < 2 {
      continue;
    }

    let mut file_hashes: std::collections::HashMap<String, PathBuf> =
      std::collections::HashMap::new();

    for path in paths {
      if let Ok(content) = tokio::fs::read(&path).await {
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let hash = format!("{:x}", hasher.finalize());
        file_hashes.insert(hash, path);
      }
    }

    // Keep only hashes with duplicates
    for (hash, path) in file_hashes {
      let count = hash_groups.entry(hash.clone()).or_default().len();
      if count == 0 {
        // First file with this hash
        hash_groups.insert(hash, vec![path]);
      } else {
        hash_groups.get_mut(&hash).unwrap().push(path);
      }
    }
  }

  // Count wasted space (size * (count - 1) for each group)
  let mut wasted = 0u64;
  for paths in hash_groups.values() {
    if paths.len() > 1 {
      // Get size from first path
      if let Ok(metadata) = tokio::fs::metadata(&paths[0]).await {
        let size = metadata.len();
        wasted += size * (paths.len() - 1) as u64;
      }
    }
  }

  Ok(wasted)
}

/// Get size of log directory
pub async fn get_log_dir_size() -> Result<u64, String> {
  let log_dir = PathBuf::from("/var/log");
  if !log_dir.exists() {
    return Ok(0);
  }

  let mut size = 0u64;

  for entry in WalkDir::new(&log_dir)
    .follow_links(false)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    let path = entry.path();
    if path.is_file() {
      // Skip rotated logs
      let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
      if name.ends_with(".gz") || name.ends_with(".1") || name.ends_with(".old") {
        continue;
      }
      if let Ok(metadata) = entry.metadata() {
        size += metadata.len();
      }
    }
  }

  Ok(size)
}

/// Read GPU temperature by probing nvidia-smi or AMD sysfs.
/// Returns Ok(f64) with temperature in Celsius, or Err if unavailable.
pub async fn get_gpu_temp() -> Result<f64, String> {
  // Try NVIDIA first
  if let Ok(output) = tokio::process::Command::new("nvidia-smi")
    .args(["-q", "-d", "TEMPERATURE"])
    .output()
    .await
  {
    if output.status.success() {
      let stdout = String::from_utf8_lossy(&output.stdout);
      if let Some(line) = stdout.lines().find(|l| l.contains("GPU Current Temp")) {
        let temp_part = line.split(':').nth(1).unwrap_or("").trim();
        let temp_str = temp_part.split_whitespace().next().unwrap_or("");
        if let Ok(temp) = temp_str.parse::<f64>() {
          return Ok(temp);
        }
      }
    }
  }

  // Try AMD GPU sysfs
  for gpu_path in ["/sys/class/drm/card0/device", "/sys/class/drm/card1/device"] {
    let hwmon_path = format!("{}/hwmon", gpu_path);
    if let Ok(entries) = std::fs::read_dir(&hwmon_path) {
      for entry in entries.filter_map(|e| e.ok()) {
        let name_file = entry.path().join("name");
        if let Ok(name) = std::fs::read_to_string(&name_file) {
          if name.trim().contains("amdgpu") || name.trim().contains("radeon") {
            let temp_file = entry.path().join("temp1_input");
            if let Ok(content) = tokio::fs::read_to_string(&temp_file).await {
              if let Ok(temp_milli) = content.trim().parse::<u32>() {
                return Ok(temp_milli as f64 / 1000.0);
              }
            }
          }
        }
      }
    }
  }

  Err("GPU temperature not available (no nvidia-smi or AMD sysfs found)".to_string())
}

// ---------------------------------------------------------------------------
// Fan speed
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, serde::Serialize)]
pub struct FanInfo {
  pub name: String,
  pub rpm: u32,
}

/// Read fan speeds from /sys/class/hwmon/.
/// Used by the bridge layer which cannot call async functions directly.
pub fn get_fan_info() -> Result<Vec<FanInfo>, String> {
  let mut fans = Vec::new();

  let hwmon_base = PathBuf::from("/sys/class/hwmon");
  if !hwmon_base.exists() {
    return Ok(fans);
  }

  if let Ok(entries) = std::fs::read_dir(&hwmon_base) {
    for entry in entries.filter_map(|e| e.ok()) {
      let path = entry.path();
      let name = path.file_name().unwrap_or_default().to_string_lossy();

      if !name.starts_with("hwmon") {
        continue;
      }

      if let Ok(fan_entries) = std::fs::read_dir(&path) {
        for fan_entry in fan_entries.filter_map(|e| e.ok()) {
          let fan_path = fan_entry.path();
          let fan_name = fan_path.file_name().unwrap_or_default().to_string_lossy();

          if fan_name.starts_with("fan") && fan_name.ends_with("_input") {
            let fan_num = fan_name
              .strip_prefix("fan")
              .and_then(|s| s.strip_suffix("_input"))
              .unwrap_or("?");
            let label_path = fan_path.with_file_name(format!("fan{}_label", fan_num));
            let label: String = if label_path.exists() {
              std::fs::read_to_string(&label_path)
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|_| format!("Fan {}", fan_num))
            } else {
              format!("Fan {}", fan_num)
            };

            if let Ok(content) = std::fs::read_to_string(&fan_path) {
              if let Ok(rpm) = content.trim().parse::<u32>() {
                fans.push(FanInfo { name: label, rpm });
              }
            }
          }
        }
      }
    }
  }

  Ok(fans)
}
