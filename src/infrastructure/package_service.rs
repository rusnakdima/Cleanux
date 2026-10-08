//! PackageService — package cache info and cleaning for apt/dnf/pacman/zypper

use crate::error::AppError;
use crate::response::Response;
use crate::infrastructure::{apt_service, dnf_service, pacman_service, zypper_service};
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Result type alias for this module.
pub type Result<T, E = AppError> = std::result::Result<T, E>;

/// Package manager type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageManager {
  Apt,
  Dnf,
  Pacman,
  Zypper,
}

impl std::fmt::Display for PackageManager {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      PackageManager::Apt => write!(f, "apt"),
      PackageManager::Dnf => write!(f, "dnf"),
      PackageManager::Pacman => write!(f, "pacman"),
      PackageManager::Zypper => write!(f, "zypper"),
    }
  }
}

/// Package cache information for a single package manager
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageCacheInfo {
  pub manager: PackageManager,
  pub cache_size: u64,
  pub package_count: usize,
  pub partial_count: usize,
  pub cache_path: String,
}

/// Result of a package cache cleaning operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageCleanResult {
  pub manager: PackageManager,
  pub bytes_freed: u64,
  pub packages_removed: Option<usize>,
}

/// Detect which package managers are available on the system
pub fn detect_package_managers() -> Vec<PackageManager> {
  let mut managers = Vec::new();

  if Command::new("apt")
    .arg("--version")
    .output()
    .map(|o| o.status.success())
    .unwrap_or(false)
  {
    managers.push(PackageManager::Apt);
  }
  if Command::new("dnf")
    .arg("--version")
    .output()
    .map(|o| o.status.success())
    .unwrap_or(false)
  {
    managers.push(PackageManager::Dnf);
  }
  if Command::new("pacman")
    .arg("--version")
    .output()
    .map(|o| o.status.success())
    .unwrap_or(false)
  {
    managers.push(PackageManager::Pacman);
  }
  if Command::new("zypper")
    .arg("--version")
    .output()
    .map(|o| o.status.success())
    .unwrap_or(false)
  {
    managers.push(PackageManager::Zypper);
  }

  managers
}

/// Get cache directory size in bytes using du
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

/// Get apt cache info
fn get_apt_cache_info() -> PackageCacheInfo {
  apt_service::get_cache_info()
}

/// Get dnf cache info
fn get_dnf_cache_info() -> PackageCacheInfo {
  dnf_service::get_cache_info()
}

/// Get pacman cache info
fn get_pacman_cache_info() -> PackageCacheInfo {
  pacman_service::get_cache_info()
}

/// Get zypper cache info
fn get_zypper_cache_info() -> PackageCacheInfo {
  zypper_service::get_cache_info()
}

/// Get package cache information for a specific package manager
pub fn get_package_cache_info(manager: PackageManager) -> Result<PackageCacheInfo> {
  match manager {
    PackageManager::Apt => Ok(get_apt_cache_info()),
    PackageManager::Dnf => Ok(get_dnf_cache_info()),
    PackageManager::Pacman => Ok(get_pacman_cache_info()),
    PackageManager::Zypper => Ok(get_zypper_cache_info()),
  }
}

/// Get all available package cache infos
pub fn get_all_package_cache_infos() -> Vec<PackageCacheInfo> {
  detect_package_managers()
    .into_iter()
    .filter_map(|m| get_package_cache_info(m).ok())
    .collect()
}

/// Clean apt cache
async fn clean_apt_cache() -> Result<PackageCleanResult> {
  apt_service::clean_cache().await
}

/// Clean dnf cache
async fn clean_dnf_cache() -> Result<PackageCleanResult> {
  dnf_service::clean_cache().await
}

/// Clean pacman cache (keep last 3 versions)
async fn clean_pacman_cache() -> Result<PackageCleanResult> {
  pacman_service::clean_cache().await
}

/// Clean zypper cache
async fn clean_zypper_cache() -> Result<PackageCleanResult> {
  zypper_service::clean_cache().await
}

/// Clean package cache for a specific package manager
pub async fn clean_package_cache(manager: PackageManager) -> Result<PackageCleanResult> {
  match manager {
    PackageManager::Apt => clean_apt_cache().await,
    PackageManager::Dnf => clean_dnf_cache().await,
    PackageManager::Pacman => clean_pacman_cache().await,
    PackageManager::Zypper => clean_zypper_cache().await,
  }
}

/// Clean all available package caches
pub async fn clean_all_package_caches() -> Vec<PackageCleanResult> {
  let managers = detect_package_managers();
  let mut results = Vec::new();

  for manager in managers {
    if let Ok(result) = clean_package_cache(manager).await {
      results.push(result);
    }
  }

  results
}

// ---------------------------------------------------------------------------
// Handler wrappers (return Response<T>)
// ---------------------------------------------------------------------------

/// Get package cache info for a specific manager
pub async fn get_cache_info(manager: &str) -> Result<Response<PackageCacheInfo>, String> {
  let mgr = match manager.to_lowercase().as_str() {
    "apt" => PackageManager::Apt,
    "dnf" => PackageManager::Dnf,
    "pacman" => PackageManager::Pacman,
    "zypper" => PackageManager::Zypper,
    _ => {
      return Ok(Response::validation_error(format!(
        "unknown package manager: {}",
        manager
      )))
    }
  };

  match get_package_cache_info(mgr) {
    Ok(info) => Ok(Response::success(info, None)),
    Err(e) => Ok(Response::error(e.to_string())),
  }
}

/// Get all available package cache infos
pub async fn get_all_cache_infos() -> Result<Response<Vec<PackageCacheInfo>>, String> {
  let infos = get_all_package_cache_infos();
  Ok(Response::success(infos, None))
}

/// Clean package cache for a specific manager
pub async fn clean_cache(manager: &str) -> Result<Response<PackageCleanResult>, String> {
  let mgr = match manager.to_lowercase().as_str() {
    "apt" => PackageManager::Apt,
    "dnf" => PackageManager::Dnf,
    "pacman" => PackageManager::Pacman,
    "zypper" => PackageManager::Zypper,
    _ => {
      return Ok(Response::validation_error(format!(
        "unknown package manager: {}",
        manager
      )))
    }
  };

  match clean_package_cache(mgr).await {
    Ok(result) => Ok(Response::success(result, None)),
    Err(e) => Ok(Response::error(e.to_string())),
  }
}

/// Clean all available package caches
pub async fn clean_all_caches() -> Result<Response<Vec<PackageCleanResult>>, String> {
  let results = clean_all_package_caches().await;
  Ok(Response::success(results, None))
}

/// Detect available package managers
pub async fn get_available_managers() -> Result<Response<Vec<PackageManager>>, String> {
  let managers = detect_package_managers();
  Ok(Response::success(managers, None))
}
