use crate::models::Response;
use crate::services::dashboard_service::DashboardService;

#[tauri::command]
pub fn get_system_services() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  DashboardService.get_running_services()
}

#[tauri::command]
pub fn get_cache_summary() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  DashboardService.get_cache_summary()
}

#[tauri::command]
pub fn get_trash_summary() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  DashboardService.get_trash_summary()
}

#[tauri::command]
pub fn get_log_summary() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  DashboardService.get_log_summary()
}

#[tauri::command]
pub fn get_large_files_summary() -> Result<Response<serde_json::Value>, Response<serde_json::Value>>
{
  DashboardService.get_large_files_summary()
}
