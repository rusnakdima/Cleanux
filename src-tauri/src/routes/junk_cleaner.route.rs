/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::junk_cleaner_service::JunkCleanerService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_junk_summary() -> Result<ResponseModel, ResponseModel> {
  JunkCleanerService.get_junk_summary()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_browser_caches() -> Result<ResponseModel, ResponseModel> {
  JunkCleanerService.scan_browser_caches()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_thumbnail_caches() -> Result<ResponseModel, ResponseModel> {
  JunkCleanerService.scan_thumbnail_caches()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_application_caches() -> Result<ResponseModel, ResponseModel> {
  JunkCleanerService.scan_application_caches()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_system_temp() -> Result<ResponseModel, ResponseModel> {
  JunkCleanerService.scan_system_temp()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_log_rotations() -> Result<ResponseModel, ResponseModel> {
  JunkCleanerService.scan_log_rotations()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_junk_category(category: String) -> Result<ResponseModel, ResponseModel> {
  JunkCleanerService.clean_junk_category(category)
}
