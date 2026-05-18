use crate::models::ResponseModel;
use crate::services::memory_service::MemoryService;

#[tauri::command]
pub fn get_memory_info() -> Result<ResponseModel, ResponseModel> {
  MemoryService::get_memory_info()
}

#[tauri::command]
pub fn get_swap_info() -> Result<ResponseModel, ResponseModel> {
  MemoryService::get_swap_info()
}

#[tauri::command]
pub fn get_process_memory() -> Result<ResponseModel, ResponseModel> {
  MemoryService::get_process_memory()
}

#[tauri::command]
pub fn optimize_memory() -> Result<ResponseModel, ResponseModel> {
  MemoryService::optimize_memory()
}
