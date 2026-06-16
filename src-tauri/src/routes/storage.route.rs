use crate::models::ResponseModel;
use crate::services::backup_service::BackupService;
use crate::services::dev_cache_service::DevCacheService;
use crate::services::directory_service::DirectoryService;
use crate::services::junk_cleaner_service::JunkCleanerService;
use crate::services::media_cache_service::MediaCacheService;
use crate::services::scanner_service::ScannerService;

static DEV_CACHE_SERVICE: std::sync::OnceLock<DevCacheService> = std::sync::OnceLock::new();

fn get_dev_cache_service() -> &'static DevCacheService {
  DEV_CACHE_SERVICE.get_or_init(|| DevCacheService)
}

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

#[tauri::command]
#[allow(non_snake_case)]
pub fn find_duplicates(
  path: String,
  extension_filter: Option<String>,
) -> Result<ResponseModel, ResponseModel> {
  ScannerService::scan_for_duplicates(&path, extension_filter)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_junk_summary() -> Result<ResponseModel, ResponseModel> {
  JunkCleanerService.get_junk_summary()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_browser_caches() -> Result<ResponseModel, ResponseModel> {
  JunkCleanerService.scan_browser_caches()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_thumbnail_caches() -> Result<ResponseModel, ResponseModel> {
  JunkCleanerService.scan_thumbnail_caches()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_application_caches() -> Result<ResponseModel, ResponseModel> {
  JunkCleanerService.scan_application_caches()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_system_temp() -> Result<ResponseModel, ResponseModel> {
  JunkCleanerService.scan_system_temp()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_log_rotations() -> Result<ResponseModel, ResponseModel> {
  JunkCleanerService.scan_log_rotations()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_junk_category(category: String) -> Result<ResponseModel, ResponseModel> {
  JunkCleanerService.clean_junk_category(category)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_media_cache_summary() -> ResponseModel {
  MediaCacheService.get_media_cache_summary()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_steam_shader_cache() -> Result<ResponseModel, ResponseModel> {
  MediaCacheService.clean_steam_shader_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_steam_download_cache() -> Result<ResponseModel, ResponseModel> {
  MediaCacheService.clean_steam_download_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_spotify_cache() -> Result<ResponseModel, ResponseModel> {
  MediaCacheService.clean_spotify_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_vlc_cache() -> Result<ResponseModel, ResponseModel> {
  MediaCacheService.clean_vlc_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_thumbnail_cache() -> Result<ResponseModel, ResponseModel> {
  MediaCacheService.clean_thumbnail_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_media_icon_cache() -> Result<ResponseModel, ResponseModel> {
  MediaCacheService.clean_icon_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_dev_cache_summary() -> Result<ResponseModel, ResponseModel> {
  get_dev_cache_service().get_all_dev_caches()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_npm_cache() -> Result<ResponseModel, ResponseModel> {
  get_dev_cache_service().clean_npm_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_pip_cache() -> Result<ResponseModel, ResponseModel> {
  get_dev_cache_service().clean_pip_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_cargo_cache() -> Result<ResponseModel, ResponseModel> {
  get_dev_cache_service().clean_cargo_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_go_cache() -> Result<ResponseModel, ResponseModel> {
  get_dev_cache_service().clean_go_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_maven_cache() -> Result<ResponseModel, ResponseModel> {
  get_dev_cache_service().clean_maven_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_gradle_cache() -> Result<ResponseModel, ResponseModel> {
  get_dev_cache_service().clean_gradle_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_all_dev_caches() -> Result<ResponseModel, ResponseModel> {
  get_dev_cache_service().clean_all_dev_caches()
}
