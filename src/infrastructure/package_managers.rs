//! Package managers infrastructure (apt, dnf, pacman, zypper)

use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Package cache info for a single package manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageCacheInfo {
  pub manager: String,
  pub cache_size: u64,
  pub package_count: usize,
  pub partial_count: usize,
  pub orphaned_count: usize,
  pub available: bool,
}

/// Package cache summary for all detected package managers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageCacheSummary {
  pub managers: Vec<PackageCacheInfo>,
  pub total_reclaimable: u64,
}

/// Detects which package managers are available on the system
fn detect_package_managers() -> Vec<&'static str> {
  let mut managers = Vec::new();
  if Command::new("apt")
    .arg("--version")
    .output()
    .map(|o| o.status.success())
    .unwrap_or(false)
  {
    managers.push("apt");
  }
  if Command::new("dnf")
    .arg("--version")
    .output()
    .map(|o| o.status.success())
    .unwrap_or(false)
  {
    managers.push("dnf");
  }
  if Command::new("pacman")
    .arg("--version")
    .output()
    .map(|o| o.status.success())
    .unwrap_or(false)
  {
    managers.push("pacman");
  }
  if Command::new("zypper")
    .arg("--version")
    .output()
    .map(|o| o.status.success())
    .unwrap_or(false)
  {
    managers.push("zypper");
  }
  managers
}

/// Gets the cache directory size for a package manager
fn get_cache_size(manager: &str) -> (u64, usize, usize) {
  match manager {
    "apt" => {
      let cache_path = "/var/cache/apt/archives";
      let output = Command::new("du").args(["-sb", cache_path]).output();
      let size = output
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.split_whitespace().next()?.parse::<u64>().ok())
        .unwrap_or(0);
      let count = std::fs::read_dir(cache_path)
        .map(|d| d.filter_map(|e| e.ok()).count())
        .unwrap_or(0);
      (size, count, 0)
    }
    "dnf" => {
      let cache_path = "/var/cache/dnf";
      let output = Command::new("du").args(["-sb", cache_path]).output();
      let size = output
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.split_whitespace().next()?.parse::<u64>().ok())
        .unwrap_or(0);
      let count = std::fs::read_dir(cache_path)
        .map(|d| d.filter_map(|e| e.ok()).count())
        .unwrap_or(0);
      (size, count, 0)
    }
    "pacman" => {
      let cache_path = "/var/cache/pacman/pkg";
      let output = Command::new("du").args(["-sb", cache_path]).output();
      let size = output
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.split_whitespace().next()?.parse::<u64>().ok())
        .unwrap_or(0);
      let count = std::fs::read_dir(cache_path)
        .map(|d| d.filter_map(|e| e.ok()).count())
        .unwrap_or(0);
      (size, count, 0)
    }
    "zypper" => {
      let cache_path = "/var/cache/zypp";
      let output = Command::new("du").args(["-sb", cache_path]).output();
      let size = output
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.split_whitespace().next()?.parse::<u64>().ok())
        .unwrap_or(0);
      let count = std::fs::read_dir(cache_path)
        .map(|d| d.filter_map(|e| e.ok()).count())
        .unwrap_or(0);
      (size, count, 0)
    }
    _ => (0, 0, 0),
  }
}

/// Gets package cache summary for all detected package managers.
pub async fn get_package_cache_summary() -> Result<PackageCacheSummary, AppError> {
  let managers = detect_package_managers();
  let mut infos = Vec::new();
  let mut total = 0u64;

  for manager in managers {
    let (size, count, partial) = get_cache_size(manager);
    total += size;
    infos.push(PackageCacheInfo {
      manager: manager.to_string(),
      cache_size: size,
      package_count: count,
      partial_count: partial,
      orphaned_count: 0,
      available: true,
    });
  }

  Ok(PackageCacheSummary {
    managers: infos,
    total_reclaimable: total,
  })
}

/// Cleans package manager cache and returns bytes freed.
pub async fn clean_package_cache(manager: &str) -> Result<u64, AppError> {
  // Get size before cleaning
  let (size_before, _, _) = get_cache_size(manager);

  let freed: u64 = match manager {
    "apt" => {
      let output = Command::new("apt")
        .args(["clean"])
        .output()
        .map_err(|e| AppError::Internal(format!("apt clean failed: {}", e)))?;
      if output.status.success() {
        size_before
      } else {
        0
      }
    }
    "dnf" => {
      let output = Command::new("dnf")
        .args(["clean", "all"])
        .output()
        .map_err(|e| AppError::Internal(format!("dnf clean failed: {}", e)))?;
      if output.status.success() {
        size_before
      } else {
        0
      }
    }
    "pacman" => {
      let output = Command::new("paccache")
        .args(["-r"])
        .output()
        .map_err(|e| AppError::Internal(format!("paccache failed: {}", e)))?;
      if output.status.success() {
        size_before
      } else {
        0
      }
    }
    "zypper" => {
      let output = Command::new("zypper")
        .args(["clean"])
        .output()
        .map_err(|e| AppError::Internal(format!("zypper clean failed: {}", e)))?;
      if output.status.success() {
        size_before
      } else {
        0
      }
    }
    _ => {
      return Err(AppError::ValidationError(format!(
        "unknown package manager: {}",
        manager
      )))
    }
  };
  Ok(freed)
}

/// Package manager type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
  Apt,
  Dnf,
  Pacman,
  Zypper,
}
