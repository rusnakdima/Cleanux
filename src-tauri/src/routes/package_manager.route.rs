/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::package_manager_service::PackageManagerService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_package_cache_info() -> Result<ResponseModel, ResponseModel> {
  PackageManagerService::get_package_cache_info()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_package_cache(manager: String) -> Result<ResponseModel, ResponseModel> {
  PackageManagerService::clean_package_cache(&manager)
}
