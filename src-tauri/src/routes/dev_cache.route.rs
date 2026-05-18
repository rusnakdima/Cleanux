/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::dev_cache_service::DevCacheService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_dev_cache_summary() -> ResponseModel {
  DevCacheService {}.get_all_dev_caches()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_npm_cache() -> ResponseModel {
  DevCacheService {}.clean_npm_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_pip_cache() -> ResponseModel {
  DevCacheService {}.clean_pip_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_cargo_cache() -> ResponseModel {
  DevCacheService {}.clean_cargo_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_go_cache() -> ResponseModel {
  DevCacheService {}.clean_go_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_maven_cache() -> ResponseModel {
  DevCacheService {}.clean_maven_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_gradle_cache() -> ResponseModel {
  DevCacheService {}.clean_gradle_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_all_dev_caches() -> ResponseModel {
  DevCacheService {}.clean_all_dev_caches()
}
