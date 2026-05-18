/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::startup_service::StartupService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_startup_items() -> Result<ResponseModel, ResponseModel> {
  StartupService::get_startup_items()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn disable_startup_item(path: String) -> Result<ResponseModel, ResponseModel> {
  StartupService::disable_startup_item(&path)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn enable_startup_item(path: String) -> Result<ResponseModel, ResponseModel> {
  StartupService::enable_startup_item(&path)
}
