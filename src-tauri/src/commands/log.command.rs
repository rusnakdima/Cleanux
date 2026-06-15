use crate::helpers::{array_response, ResponseBuilder};
use crate::models::{DataValue, ResponseModel};
use crate::services::log_manager_service::LogManagerService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_journal_size() -> Result<ResponseModel, ResponseModel> {
  Ok(
    ResponseBuilder::new()
      .success("Journal size retrieved")
      .data(DataValue::Number(
        LogManagerService::get_journal_size() as f64
      ))
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_journal_usage() -> Result<ResponseModel, ResponseModel> {
  let info = LogManagerService::get_journal_usage();
  Ok(
    ResponseBuilder::new()
      .success("Journal usage retrieved")
      .data(DataValue::Object(
        serde_json::to_value(info).map_err(|e| format!("Serialization error: {}", e))?,
      ))
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn vacuum_journal(size_mb: u32) -> Result<ResponseModel, ResponseModel> {
  LogManagerService::vacuum_journal(size_mb)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn vacuum_journal_by_days(days: u32) -> Result<ResponseModel, ResponseModel> {
  LogManagerService::vacuum_journal_by_days(days)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_rotated_logs_size() -> Result<ResponseModel, ResponseModel> {
  Ok(
    ResponseBuilder::new()
      .success("Rotated logs size retrieved")
      .data(DataValue::Number(
        LogManagerService::get_rotated_logs_size() as f64,
      ))
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_rotated_logs() -> Result<ResponseModel, ResponseModel> {
  let logs = LogManagerService::get_rotated_logs();
  array_response("Rotated logs retrieved", logs)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_rotated_logs(days: u32) -> Result<ResponseModel, ResponseModel> {
  LogManagerService::clean_rotated_logs(days)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_logrotate_configs() -> Result<ResponseModel, ResponseModel> {
  let configs = LogManagerService::get_logrotate_configs();
  array_response("Logrotate configs retrieved", configs)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn analyze_logrotate() -> Result<ResponseModel, ResponseModel> {
  let analysis = LogManagerService::analyze_logrotate();
  Ok(
    ResponseBuilder::new()
      .success("Logrotate analysis retrieved")
      .data(DataValue::Object(
        serde_json::to_value(analysis).map_err(|e| format!("Serialization error: {}", e))?,
      ))
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_var_log_usage() -> Result<ResponseModel, ResponseModel> {
  let usage = LogManagerService::get_var_log_usage();
  Ok(
    ResponseBuilder::new()
      .success("Var log usage retrieved")
      .data(DataValue::Object(
        serde_json::to_value(usage).map_err(|e| format!("Serialization error: {}", e))?,
      ))
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_largest_log_files(limit: usize) -> Result<ResponseModel, ResponseModel> {
  let files = LogManagerService::get_largest_log_files(limit);
  array_response("Largest log files retrieved", files)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_log_manager_summary() -> Result<ResponseModel, ResponseModel> {
  let summary = LogManagerService::get_log_manager_summary();
  Ok(
    ResponseBuilder::new()
      .success("Log manager summary retrieved")
      .data(DataValue::Object(
        serde_json::to_value(summary).map_err(|e| format!("Serialization error: {}", e))?,
      ))
      .build(),
  )
}
