//! Zypper package manager service

use crate::error::AppError;
use crate::infrastructure::package_service::{PackageCacheInfo, PackageCleanResult};
use std::process::Command;

/// Zypper cache path
const ZYPPER_CACHE_PATH: &str = "/var/cache/zypp";

/// Check if Zypper is available
pub fn is_available() -> bool {
  Command::new("zypper")
    .arg("--version")
    .output()
    .map(|o| o.status.success())
    .unwrap_or(false)
}

/// Get Zypper cache information
pub fn get_cache_info() -> PackageCacheInfo {
  let cache_path = ZYPPER_CACHE_PATH.to_string();
  let cache_size = get_dir_size(&cache_path);

  let package_count = std::fs::read_dir(&cache_path)
    .map(|entries| {
      entries
        .filter_map(|e| e.ok())
        .filter(|e| {
          e.path()
            .extension()
            .map(|ext| ext == "rpm")
            .unwrap_or(false)
        })
        .count()
    })
    .unwrap_or(0);

  PackageCacheInfo {
    manager: crate::infrastructure::package_service::PackageManager::Zypper,
    cache_size,
    package_count,
    partial_count: 0,
    cache_path,
  }
}

/// Get orphaned packages count for Zypper
pub fn get_orphaned_count() -> usize {
  let output = Command::new("zypper")
    .args(["packages", "--unneeded"])
    .output();

  match output {
    Ok(out) if out.status.success() => {
      String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|l| !l.trim().is_empty())
        .count()
    }
    _ => 0,
  }
}

/// Clean Zypper cache
pub async fn clean_cache() -> Result<PackageCleanResult, AppError> {
  let before_size = get_dir_size(ZYPPER_CACHE_PATH);

  let output = Command::new("zypper")
    .args(["clean"])
    .output()
    .map_err(|e| AppError::Internal(format!("zypper clean failed: {}", e)))?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(AppError::Internal(format!("zypper clean failed: {}", stderr)));
  }

  let after_size = get_dir_size(ZYPPER_CACHE_PATH);
  let bytes_freed = before_size.saturating_sub(after_size);

  Ok(PackageCleanResult {
    manager: crate::infrastructure::package_service::PackageManager::Zypper,
    bytes_freed,
    packages_removed: None,
  })
}

/// Get directory size in bytes using du
fn get_dir_size(dir: &str) -> u64 {
  let output = Command::new("du").args(["-sb", dir]).output();

  match output {
    Ok(out) if out.status.success() => {
      let stdout = String::from_utf8_lossy(&out.stdout);
      stdout
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0)
    }
    _ => 0,
  }
}
