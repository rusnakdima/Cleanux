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
  Ok(ResponseModel {
    status: crate::models::ResponseStatus::Success,
    message: format!("Found {} config residues", residues.len()),
    data: crate::models::DataValue::Array(
      residues
        .into_iter()
        .map(serde_json::to_value)
        .filter_map(|r| r.ok())
        .collect(),
    ),
  })
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_user_data() -> Result<ResponseModel, ResponseModel> {
  let residues = AppResidueService.scan_user_data();
  Ok(ResponseModel {
    status: crate::models::ResponseStatus::Success,
    message: format!("Found {} data residues", residues.len()),
    data: crate::models::DataValue::Array(
      residues
        .into_iter()
        .map(serde_json::to_value)
        .filter_map(|r| r.ok())
        .collect(),
    ),
  })
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_user_caches() -> Result<ResponseModel, ResponseModel> {
  let residues = AppResidueService.scan_user_caches();
  Ok(ResponseModel {
    status: crate::models::ResponseStatus::Success,
    message: format!("Found {} cache residues", residues.len()),
    data: crate::models::DataValue::Array(
      residues
        .into_iter()
        .map(serde_json::to_value)
        .filter_map(|r| r.ok())
        .collect(),
    ),
  })
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn scan_home_residues() -> Result<ResponseModel, ResponseModel> {
  let residues = AppResidueService.scan_home_residues();
  Ok(ResponseModel {
    status: crate::models::ResponseStatus::Success,
    message: format!("Found {} home residues", residues.len()),
    data: crate::models::DataValue::Array(
      residues
        .into_iter()
        .map(serde_json::to_value)
        .filter_map(|r| r.ok())
        .collect(),
    ),
  })
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_orphaned_configs() -> Result<ResponseModel, ResponseModel> {
  let orphaned = AppResidueService.get_orphaned_configs();
  Ok(ResponseModel {
    status: crate::models::ResponseStatus::Success,
    message: format!("Found {} orphaned configs", orphaned.len()),
    data: crate::models::DataValue::Array(
      orphaned
        .into_iter()
        .map(serde_json::to_value)
        .filter_map(|r| r.ok())
        .collect(),
    ),
  })
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

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_multiple_residues(paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  AppResidueService.clean_multiple(paths)
}
