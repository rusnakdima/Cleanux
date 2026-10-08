//! APT package manager service

use crate::error::AppError;
use crate::infrastructure::package_service::{PackageCacheInfo, PackageCleanResult};
use std::process::Command;

/// APT cache path
const APT_CACHE_PATH: &str = "/var/cache/apt/archives";

/// Check if APT is available
pub fn is_available() -> bool {
  Command::new("apt")
    .arg("--version")
    .output()
    .map(|o| o.status.success())
    .unwrap_or(false)
}

/// Get APT cache information
pub fn get_cache_info() -> PackageCacheInfo {
  let cache_path = APT_CACHE_PATH.to_string();
  let cache_size = get_dir_size(&cache_path);

  let package_count = std::fs::read_dir(&cache_path)
    .map(|entries| {
      entries
        .filter_map(|e| e.ok())
        .filter(|e| {
          e.path()
            .extension()
            .map(|ext| ext == "deb")
            .unwrap_or(false)
        })
        .count()
    })
    .unwrap_or(0);

  let partial_count = std::fs::read_dir(&cache_path)
    .map(|entries| {
      entries
        .filter_map(|e| e.ok())
        .filter(|e| {
          e.path()
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.ends_with(".partial"))
            .unwrap_or(false)
        })
        .count()
    })
    .unwrap_or(0);

  PackageCacheInfo {
    manager: crate::infrastructure::package_service::PackageManager::Apt,
    cache_size,
    package_count,
    partial_count,
    cache_path,
  }
}

/// Get orphaned packages count for APT
pub fn get_orphaned_count() -> usize {
  // APT doesn't have a direct orphaned package command like other package managers.
  // We check for packages that were installed as dependencies but are no longer required.
  let output = Command::new("apt").args(["-markauto", "--help"]).output();

  if output.is_err() {
    return 0;
  }

  // Use apt-mark to find auto-installed packages that are no longer required
  let output = Command::new("sh")
    .args(["-c", "apt-mark showauto | while read pkg; do apt-mark showmanual \"$pkg\" || echo \"$pkg\"; done 2>/dev/null | head -100"])
    .output();

  match output {
    Ok(o) => String::from_utf8_lossy(&o.stdout)
      .lines()
      .filter(|l| !l.trim().is_empty())
      .count(),
    Err(_) => 0,
  }
}

/// Clean APT cache
pub async fn clean_cache() -> Result<PackageCleanResult, AppError> {
  let before_size = get_dir_size(APT_CACHE_PATH);

  let output = Command::new("apt")
    .args(["clean"])
    .output()
    .map_err(|e| AppError::Internal(format!("apt clean failed: {}", e)))?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(AppError::Internal(format!("apt clean failed: {}", stderr)));
  }

  let after_size = get_dir_size(APT_CACHE_PATH);
  let bytes_freed = before_size.saturating_sub(after_size);

  Ok(PackageCleanResult {
    manager: crate::infrastructure::package_service::PackageManager::Apt,
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
