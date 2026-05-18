/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::repair_service::RepairService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn find_broken_symlinks() -> Result<ResponseModel, ResponseModel> {
  RepairService::find_broken_symlinks()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn find_orphaned_packages() -> Result<ResponseModel, ResponseModel> {
  RepairService::find_orphaned_packages()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_font_cache() -> Result<ResponseModel, ResponseModel> {
  RepairService::clean_font_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_repair_icon_cache() -> Result<ResponseModel, ResponseModel> {
  RepairService::clean_icon_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn repair_permissions() -> Result<ResponseModel, ResponseModel> {
  RepairService::repair_permissions()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn remove_broken_symlink(path: String) -> Result<ResponseModel, ResponseModel> {
  RepairService::remove_broken_symlink(&path)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_repair_orphaned_pkg(path: String) -> Result<ResponseModel, ResponseModel> {
  RepairService::remove_orphaned_package(&path)
}
