/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::dev_cache_service::DevCacheService;

static SERVICE: std::sync::OnceLock<DevCacheService> = std::sync::OnceLock::new();

fn get_service() -> &'static DevCacheService {
  SERVICE.get_or_init(|| DevCacheService)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_dev_cache_summary() -> Result<ResponseModel, ResponseModel> {
  get_service().get_all_dev_caches()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_npm_cache() -> Result<ResponseModel, ResponseModel> {
  get_service().clean_npm_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_pip_cache() -> Result<ResponseModel, ResponseModel> {
  get_service().clean_pip_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_cargo_cache() -> Result<ResponseModel, ResponseModel> {
  get_service().clean_cargo_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_go_cache() -> Result<ResponseModel, ResponseModel> {
  get_service().clean_go_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_maven_cache() -> Result<ResponseModel, ResponseModel> {
  get_service().clean_maven_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_gradle_cache() -> Result<ResponseModel, ResponseModel> {
  get_service().clean_gradle_cache()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_all_dev_caches() -> Result<ResponseModel, ResponseModel> {
  get_service().clean_all_dev_caches()
}
