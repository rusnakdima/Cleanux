use crate::models::ResponseModel;
use crate::services::dashboard_service::DashboardService;

#[tauri::command]
pub fn get_system_services() -> Result<ResponseModel, ResponseModel> {
  DashboardService.get_running_services()
}

#[tauri::command]
pub fn get_cache_summary() -> Result<ResponseModel, ResponseModel> {
  DashboardService.get_cache_summary()
}

#[tauri::command]
pub fn get_trash_summary() -> Result<ResponseModel, ResponseModel> {
  DashboardService.get_trash_summary()
}

#[tauri::command]
pub fn get_log_summary() -> Result<ResponseModel, ResponseModel> {
  DashboardService.get_log_summary()
}

#[tauri::command]
pub fn get_large_files_summary() -> Result<ResponseModel, ResponseModel> {
  DashboardService.get_large_files_summary()
}
