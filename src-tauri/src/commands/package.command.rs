use crate::models::Response;
use crate::services::package_service::PackageService;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageManagerSummary {
  pub apt_available: bool,
  pub apt_cache_size: u64,
  pub apt_autoremove_size: u64,
  pub apt_orphaned_count: usize,
  pub apt_partial_downloads: usize,
  pub dnf_available: bool,
  pub dnf_cache_size: u64,
  pub pacman_available: bool,
  pub pacman_cache_size: u64,
  pub zypper_available: bool,
  pub zypper_cache_size: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanResult {
  pub command: String,
  pub space_freed: u64,
  pub message: String,
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_package_cache_info() -> Result<Response<serde_json::Value>, Response<serde_json::Value>>
{
  PackageService::get_package_cache_info()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn clean_package_cache(
  manager: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  PackageService::clean_package_cache(manager.as_str())
}
