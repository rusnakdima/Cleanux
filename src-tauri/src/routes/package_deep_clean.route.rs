use crate::models::ResponseModel;
use crate::services::package_deep_clean_service::{OrphanedPackage, PackageDeepCleanService};

#[tauri::command]
pub fn get_package_summary() -> Result<ResponseModel, ResponseModel> {
  PackageDeepCleanService.get_package_summary_response()
}

#[tauri::command]
pub fn deep_clean_all() -> Result<ResponseModel, ResponseModel> {
  PackageDeepCleanService.deep_clean_all()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_apt_cache_size() -> u64 {
  PackageDeepCleanService.get_apt_cache_size()
}

#[tauri::command]
pub fn apt_clean() -> Result<ResponseModel, ResponseModel> {
  PackageDeepCleanService.apt_clean()
}

#[tauri::command]
pub fn apt_autoremove() -> Result<ResponseModel, ResponseModel> {
  PackageDeepCleanService.apt_autoremove()
}

#[tauri::command]
pub fn apt_autoclean() -> Result<ResponseModel, ResponseModel> {
  PackageDeepCleanService.apt_autoclean()
}

#[tauri::command]
pub fn get_orphaned_packages() -> Vec<OrphanedPackage> {
  PackageDeepCleanService.get_orphaned_packages()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn deep_clean_remove_orphaned_package(name: String) -> Result<ResponseModel, ResponseModel> {
  PackageDeepCleanService.remove_orphaned_package(&name)
}

#[tauri::command]
pub fn get_partial_downloads() -> Vec<String> {
  PackageDeepCleanService.get_partial_downloads()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_dnf_cache_size() -> u64 {
  PackageDeepCleanService.get_dnf_cache_size()
}

#[tauri::command]
pub fn dnf_clean_all() -> Result<ResponseModel, ResponseModel> {
  PackageDeepCleanService.dnf_clean_all()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_pacman_cache_size() -> u64 {
  PackageDeepCleanService.get_pacman_cache_size()
}

#[tauri::command]
pub fn pacman_clean(keep_recent: u32) -> Result<ResponseModel, ResponseModel> {
  PackageDeepCleanService.pacman_clean(keep_recent)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn pacman_full_clean() -> Result<ResponseModel, ResponseModel> {
  PackageDeepCleanService.pacman_full_clean()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_zypper_cache_size() -> u64 {
  PackageDeepCleanService.get_zypper_cache_size()
}

#[tauri::command]
pub fn zypper_clean() -> Result<ResponseModel, ResponseModel> {
  PackageDeepCleanService.zypper_clean()
}
