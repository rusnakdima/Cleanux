/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::dashboard_service::DashboardService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn getSystemServices() -> Result<ResponseModel, ResponseModel> {
  DashboardService.getRunningServices()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getCacheSummary() -> Result<ResponseModel, ResponseModel> {
  DashboardService.getCacheSummary()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getTrashSummary() -> Result<ResponseModel, ResponseModel> {
  DashboardService.getTrashSummary()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getLogSummary() -> Result<ResponseModel, ResponseModel> {
  DashboardService.getLogSummary()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getLargeFilesSummary() -> Result<ResponseModel, ResponseModel> {
  DashboardService.getLargeFilesSummary()
}
