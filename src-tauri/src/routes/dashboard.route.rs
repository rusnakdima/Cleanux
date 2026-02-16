/* controllers */
use crate::controllers::dashboard_controller::DashboardController;

/* models */
use crate::models::ResponseModel;

#[tauri::command]
#[allow(non_snake_case)]
pub fn getSystemServices() -> Result<ResponseModel, ResponseModel> {
  let controller = DashboardController::new();
  controller.getRunningServices()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getCacheSummary() -> Result<ResponseModel, ResponseModel> {
  let controller = DashboardController::new();
  controller.getCacheSummary()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getTrashSummary() -> Result<ResponseModel, ResponseModel> {
  let controller = DashboardController::new();
  controller.getTrashSummary()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getLogSummary() -> Result<ResponseModel, ResponseModel> {
  let controller = DashboardController::new();
  controller.getLogSummary()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getLargeFilesSummary() -> Result<ResponseModel, ResponseModel> {
  let controller = DashboardController::new();
  controller.getLargeFilesSummary()
}
