use crate::models::ResponseModel;
use crate::services::app_residue_service::{AppResidueService, AppResidueSummary};

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_residue_summary() -> Result<AppResidueSummary, ResponseModel> {
  Ok(AppResidueService.get_residue_summary())
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_user_configs() -> Result<ResponseModel, ResponseModel> {
  AppResidueService.scan_user_configs_response()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_user_data() -> Result<ResponseModel, ResponseModel> {
  AppResidueService.scan_user_data_response()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_user_caches() -> Result<ResponseModel, ResponseModel> {
  AppResidueService.scan_user_caches_response()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_home_residues() -> Result<ResponseModel, ResponseModel> {
  AppResidueService.scan_home_residues_response()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_orphaned_configs() -> Result<ResponseModel, ResponseModel> {
  AppResidueService.get_orphaned_configs_response()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_app_residue(path: String) -> Result<ResponseModel, ResponseModel> {
  AppResidueService.clean_residue(&path)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_multiple_app_residues(paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  AppResidueService.clean_multiple(paths)
}
