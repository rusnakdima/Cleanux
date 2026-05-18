use crate::models::ResponseModel;
use crate::services::directory_service::DirectoryService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_directory(
  path: String,
  max_depth: Option<u32>,
) -> Result<ResponseModel, ResponseModel> {
  let depth = max_depth.unwrap_or(3);
  DirectoryService::scan_directory(&path, depth)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_directory_size(path: String) -> Result<ResponseModel, ResponseModel> {
  DirectoryService::get_directory_size(&path)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn find_empty_directories(path: String) -> Result<ResponseModel, ResponseModel> {
  DirectoryService::find_empty_directories(&path)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn find_nested_empty_directories(path: String) -> Result<ResponseModel, ResponseModel> {
  DirectoryService::find_nested_empty_directories(&path)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn remove_empty_directory(path: String) -> Result<ResponseModel, ResponseModel> {
  DirectoryService::remove_empty_directory(&path)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn remove_empty_directories(paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  DirectoryService::remove_empty_directories(paths)
}
