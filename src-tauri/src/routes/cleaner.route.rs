/* controllers */
use crate::controllers::cleaner_controller::CleanerController;

/* models */
use crate::models::ResponseModel;

#[tauri::command]
#[allow(non_snake_case)]
pub fn getCacheFiles() -> Result<ResponseModel, ResponseModel> {
  let controller = CleanerController::new();
  controller.getCacheFiles()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getTrashFiles() -> Result<ResponseModel, ResponseModel> {
  let controller = CleanerController::new();
  controller.getTrashFiles()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getSystemLogs() -> Result<ResponseModel, ResponseModel> {
  let controller = CleanerController::new();
  controller.getSystemLogs()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn getLargeFiles() -> Result<ResponseModel, ResponseModel> {
  let controller = CleanerController::new();
  controller.getLargeFiles()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn previewFile(path: String) -> Result<ResponseModel, ResponseModel> {
  let controller = CleanerController::new();
  controller.previewFile(path)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearSelectedCacheFiles(paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  let controller = CleanerController::new();
  controller.clearSelectedCacheFiles(paths)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearSelectedTrashFiles(paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  let controller = CleanerController::new();
  controller.clearSelectedTrashFiles(paths)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearSelectedLogFiles(paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  let controller = CleanerController::new();
  controller.clearSelectedLogFiles(paths)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearSelectedLargeFiles(paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  let controller = CleanerController::new();
  controller.clearSelectedLargeFiles(paths)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearTrash() -> Result<ResponseModel, ResponseModel> {
  let controller = CleanerController::new();
  controller.clearTrash()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearCache() -> Result<ResponseModel, ResponseModel> {
  let controller = CleanerController::new();
  controller.clearCache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearAllLogs() -> Result<ResponseModel, ResponseModel> {
  let controller = CleanerController::new();
  controller.clearAllLogs()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearAllLargeFiles() -> Result<ResponseModel, ResponseModel> {
  let controller = CleanerController::new();
  controller.clearAllLargeFiles()
}
