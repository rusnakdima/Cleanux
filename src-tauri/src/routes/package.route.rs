use crate::models::{DataValue, ResponseModel};
use crate::services::apt_service::OrphanedPackage;
use crate::services::package_service::PackageService;

#[tauri::command]
pub fn get_package_summary() -> Result<ResponseModel, ResponseModel> {
  let service = PackageService;
  let summary = service.get_package_summary();
  Ok(ResponseModel {
    status: crate::models::ResponseStatus::Success,
    message: "Package manager summary retrieved".to_string(),
    data: DataValue::Object(serde_json::to_value(summary).unwrap_or(serde_json::Value::Null)),
  })
}

#[tauri::command]
pub fn deep_clean_all() -> Result<ResponseModel, ResponseModel> {
  let service = PackageService;
  service.deep_clean_all().map_err(|e| e.into_response())
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_apt_cache_size() -> u64 {
  let service = PackageService;
  service.get_apt_cache_size_internal()
}

#[tauri::command]
pub fn apt_clean() -> Result<ResponseModel, ResponseModel> {
  let service = PackageService;
  service.apt_clean().map_err(|e| e.into_response())
}

#[tauri::command]
pub fn apt_autoremove() -> Result<ResponseModel, ResponseModel> {
  let service = PackageService;
  service.apt_autoremove().map_err(|e| e.into_response())
}

#[tauri::command]
pub fn apt_autoclean() -> Result<ResponseModel, ResponseModel> {
  let service = PackageService;
  service.apt_autoclean().map_err(|e| e.into_response())
}

#[tauri::command]
pub fn get_orphaned_packages() -> Vec<OrphanedPackage> {
  let service = PackageService;
  service.get_orphaned_packages()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn deep_clean_remove_orphaned_package(name: String) -> Result<ResponseModel, ResponseModel> {
  let service = PackageService;
  service
    .remove_orphaned_package(&name)
    .map_err(|e| e.into_response())
}

#[tauri::command]
pub fn get_partial_downloads() -> Vec<String> {
  let service = PackageService;
  service.get_partial_downloads()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_dnf_cache_size() -> u64 {
  let service = PackageService;
  service.get_dnf_cache_size_internal()
}

#[tauri::command]
pub fn dnf_clean_all() -> Result<ResponseModel, ResponseModel> {
  let service = PackageService;
  service.dnf_clean_all().map_err(|e| e.into_response())
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_pacman_cache_size() -> u64 {
  let service = PackageService;
  service.get_pacman_cache_size_internal()
}

#[tauri::command]
pub fn pacman_clean(keep_recent: u32) -> Result<ResponseModel, ResponseModel> {
  let service = PackageService;
  service
    .pacman_clean(keep_recent)
    .map_err(|e| e.into_response())
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn pacman_full_clean() -> Result<ResponseModel, ResponseModel> {
  let service = PackageService;
  service.pacman_full_clean().map_err(|e| e.into_response())
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_zypper_cache_size() -> u64 {
  let service = PackageService;
  service.get_zypper_cache_size_internal()
}

#[tauri::command]
pub fn zypper_clean() -> Result<ResponseModel, ResponseModel> {
  let service = PackageService;
  service.zypper_clean().map_err(|e| e.into_response())
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_package_cache_info() -> Result<ResponseModel, ResponseModel> {
  PackageService::get_package_cache_info()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_package_cache(manager: String) -> Result<ResponseModel, ResponseModel> {
  PackageService::clean_package_cache(&manager)
}
