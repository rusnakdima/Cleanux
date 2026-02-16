/* controllers */
use crate::controllers::system_controller::SystemController;

/* models */
use crate::models::ResponseModel;

#[tauri::command]
#[allow(non_snake_case)]
pub fn stopService(service: &str) -> Result<ResponseModel, ResponseModel> {
  let controller = SystemController::new();
  controller.stopService(service)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn stopSelectedServices(services: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  let controller = SystemController::new();
  controller.stopSelectedServices(services)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn openFile(path: String, command: Option<String>) -> Result<ResponseModel, ResponseModel> {
  let controller = SystemController::new();
  controller.openFile(&path, command)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getAllServices() -> Result<ResponseModel, ResponseModel> {
  let controller = SystemController::new();
  controller.getAllServices()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn enableService(service: String) -> Result<ResponseModel, ResponseModel> {
  let controller = SystemController::new();
  controller.enableService(&service)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn startService(service: String) -> Result<ResponseModel, ResponseModel> {
  let controller = SystemController::new();
  controller.startService(&service)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn enableSelectedServices(services: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  let controller = SystemController::new();
  controller.enableSelectedServices(services)
}
