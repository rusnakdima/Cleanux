use crate::models::{DataValue, ResponseModel};
use crate::services::package_deep_clean_service::{OrphanedPackage, PackageDeepCleanService};

#[tauri::command]
pub fn get_package_summary() -> Result<ResponseModel, ResponseModel> {
  let service = PackageDeepCleanService;
  let summary = service.get_package_summary();
  Ok(ResponseModel {
    status: crate::models::ResponseStatus::Success,
    message: "Package manager summary retrieved".to_string(),
    data: DataValue::Object(serde_json::to_value(summary).unwrap_or(serde_json::Value::Null)),
  })
}

#[tauri::command]
pub fn deep_clean_all() -> Result<ResponseModel, ResponseModel> {
  let service = PackageDeepCleanService;
  service.deep_clean_all()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_apt_cache_size() -> u64 {
  let service = PackageDeepCleanService;
  service.get_apt_cache_size()
}

#[tauri::command]
pub fn apt_clean() -> Result<ResponseModel, ResponseModel> {
  let service = PackageDeepCleanService;
  service.apt_clean()
}

#[tauri::command]
pub fn apt_autoremove() -> Result<ResponseModel, ResponseModel> {
  let service = PackageDeepCleanService;
  service.apt_autoremove()
}

#[tauri::command]
pub fn apt_autoclean() -> Result<ResponseModel, ResponseModel> {
  let service = PackageDeepCleanService;
  service.apt_autoclean()
}

#[tauri::command]
pub fn get_orphaned_packages() -> Vec<OrphanedPackage> {
  let service = PackageDeepCleanService;
  service.get_orphaned_packages()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn deep_clean_remove_orphaned_package(name: String) -> Result<ResponseModel, ResponseModel> {
  let service = PackageDeepCleanService;
  service.remove_orphaned_package(&name)
}

#[tauri::command]
pub fn get_partial_downloads() -> Vec<String> {
  let service = PackageDeepCleanService;
  service.get_partial_downloads()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_dnf_cache_size() -> u64 {
  let service = PackageDeepCleanService;
  service.get_dnf_cache_size()
}

#[tauri::command]
pub fn dnf_clean_all() -> Result<ResponseModel, ResponseModel> {
  let service = PackageDeepCleanService;
  service.dnf_clean_all()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_pacman_cache_size() -> u64 {
  let service = PackageDeepCleanService;
  service.get_pacman_cache_size()
}

#[tauri::command]
pub fn pacman_clean(keep_recent: u32) -> Result<ResponseModel, ResponseModel> {
  let service = PackageDeepCleanService;
  service.pacman_clean(keep_recent)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn pacman_full_clean() -> Result<ResponseModel, ResponseModel> {
  let service = PackageDeepCleanService;
  service.pacman_full_clean()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_zypper_cache_size() -> u64 {
  let service = PackageDeepCleanService;
  service.get_zypper_cache_size()
}

#[tauri::command]
pub fn zypper_clean() -> Result<ResponseModel, ResponseModel> {
  let service = PackageDeepCleanService;
  service.zypper_clean()
}
