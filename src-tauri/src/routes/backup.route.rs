/* services */
use crate::services::backup_service::BackupService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn create_backup(
  paths: Vec<String>,
  archive_path: String,
) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
  BackupService::create_backup(paths, &archive_path)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn restore_backup(
  archive_path: String,
  destination: String,
) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
  BackupService::restore_backup(&archive_path, &destination)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn list_backups() -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
  BackupService::list_backups()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn delete_backup(
  archive_path: String,
) -> Result<crate::models::ResponseModel, crate::models::ResponseModel> {
  BackupService::delete_backup(&archive_path)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_backup_dir() -> String {
  BackupService::get_backup_dir_path()
}
