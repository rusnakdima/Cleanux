use crate::helpers::array_response;
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
  let residues = AppResidueService.scan_user_configs();
  array_response(
    format!("Found {} config residues", residues.len()),
    residues,
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_user_data() -> Result<ResponseModel, ResponseModel> {
  let residues = AppResidueService.scan_user_data();
  array_response(format!("Found {} data residues", residues.len()), residues)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_user_caches() -> Result<ResponseModel, ResponseModel> {
  let residues = AppResidueService.scan_user_caches();
  array_response(format!("Found {} cache residues", residues.len()), residues)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_home_residues() -> Result<ResponseModel, ResponseModel> {
  let residues = AppResidueService.scan_home_residues();
  array_response(format!("Found {} home residues", residues.len()), residues)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_orphaned_configs() -> Result<ResponseModel, ResponseModel> {
  let orphaned = AppResidueService.get_orphaned_configs();
  array_response(
    format!("Found {} orphaned configs", orphaned.len()),
    orphaned,
  )
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
