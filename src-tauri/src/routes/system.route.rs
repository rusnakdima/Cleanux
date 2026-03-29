/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::system_service::SystemService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn stopService(service: &str) -> Result<ResponseModel, ResponseModel> {
  SystemService.stopService(service)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn stopSelectedServices(services: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  SystemService.stopSelectedServices(services)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn openFile(path: String, command: Option<String>) -> Result<ResponseModel, ResponseModel> {
  SystemService.openFile(&path, command)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getAllServices() -> Result<ResponseModel, ResponseModel> {
  SystemService.getAllServices()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn enableService(service: String) -> Result<ResponseModel, ResponseModel> {
  SystemService.enableService(&service)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn startService(service: String) -> Result<ResponseModel, ResponseModel> {
  SystemService.startService(&service)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn enableSelectedServices(services: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  SystemService.enableSelectedServices(services)
}
