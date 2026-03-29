/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::cleaner_service::CleanerService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn getCacheFiles() -> Result<ResponseModel, ResponseModel> {
  CleanerService.getCacheFiles()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getTrashFiles() -> Result<ResponseModel, ResponseModel> {
  CleanerService.getTrashFiles()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getSystemLogs() -> Result<ResponseModel, ResponseModel> {
  CleanerService.getSystemLogs()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getLargeFiles() -> Result<ResponseModel, ResponseModel> {
  CleanerService.getLargeFiles()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn previewFile(path: String) -> Result<ResponseModel, ResponseModel> {
  CleanerService.previewFile(path)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearSelectedCacheFiles(paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  CleanerService.clearSelectedCacheFiles(paths)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearSelectedTrashFiles(paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  CleanerService.clearSelectedTrashFiles(paths)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearSelectedLogFiles(paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  CleanerService.clearSelectedLogFiles(paths)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearSelectedLargeFiles(paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  CleanerService.clearSelectedLargeFiles(paths)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearTrash() -> Result<ResponseModel, ResponseModel> {
  CleanerService.clearTrash()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearCache() -> Result<ResponseModel, ResponseModel> {
  CleanerService.clearCache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearAllLogs() -> Result<ResponseModel, ResponseModel> {
  CleanerService.clearAllLogs()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearAllLargeFiles() -> Result<ResponseModel, ResponseModel> {
  CleanerService.clearAllLargeFiles()
}
