/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::monitor_service::MonitorService;

#[tauri::command]
pub fn get_system_stats() -> Result<ResponseModel, ResponseModel> {
  MonitorService::get_system_stats()
}

#[tauri::command]
pub fn start_monitoring() -> Result<ResponseModel, ResponseModel> {
  MonitorService::start_monitoring()
}

#[tauri::command]
pub fn stop_monitoring() -> Result<ResponseModel, ResponseModel> {
  MonitorService::stop_monitoring()
}
