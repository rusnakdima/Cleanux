/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::media_cache_service::MediaCacheService;

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
