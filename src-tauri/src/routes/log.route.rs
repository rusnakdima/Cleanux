/* models */
use crate::helpers::{vec_to_data_array, ResponseBuilder};
use crate::models::{DataValue, ResponseModel};
/* services */
use crate::services::log_service::LogService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn getSystemLogs() -> Result<ResponseModel, ResponseModel> {
  LogService.getSystemLogs()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearSelectedLogFiles(paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
  LogService.clearSelectedLogFiles(paths)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clearAllLogs() -> Result<ResponseModel, ResponseModel> {
  LogService.clearAllLogs()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_journal_size() -> Result<ResponseModel, ResponseModel> {
  Ok(
    ResponseBuilder::new()
      .success("Journal size retrieved")
      .data(DataValue::Number(
        LogService::get_journal_size() as f64
      ))
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_journal_usage() -> Result<ResponseModel, ResponseModel> {
  let info = LogService::get_journal_usage();
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
  LogService::vacuum_journal(size_mb)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn vacuum_journal_by_days(days: u32) -> Result<ResponseModel, ResponseModel> {
  LogService::vacuum_journal_by_days(days)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_rotated_logs_size() -> Result<ResponseModel, ResponseModel> {
  Ok(
    ResponseBuilder::new()
      .success("Rotated logs size retrieved")
      .data(DataValue::Number(
        LogService::get_rotated_logs_size() as f64,
      ))
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_rotated_logs() -> Result<ResponseModel, ResponseModel> {
  let logs = LogService::get_rotated_logs();
  let count = logs.len();
  let data = vec_to_data_array(logs).map_err(|e| {
    ResponseBuilder::new()
      .error(format!("Serialization error: {}", e))
      .data(DataValue::String(String::new()))
      .build()
  })?;
  Ok(
    ResponseBuilder::new()
      .success(format!("Rotated logs retrieved ({})", count))
      .data(data)
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_rotated_logs(days: u32) -> Result<ResponseModel, ResponseModel> {
  LogService::clean_rotated_logs(days)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_logrotate_configs() -> Result<ResponseModel, ResponseModel> {
  let configs = LogService::get_logrotate_configs();
  let count = configs.len();
  let data = vec_to_data_array(configs).map_err(|e| {
    ResponseBuilder::new()
      .error(format!("Serialization error: {}", e))
      .data(DataValue::String(String::new()))
      .build()
  })?;
  Ok(
    ResponseBuilder::new()
      .success(format!("Logrotate configs retrieved ({})", count))
      .data(data)
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn analyze_logrotate() -> Result<ResponseModel, ResponseModel> {
  let analysis = LogService::analyze_logrotate();
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
  let usage = LogService::get_var_log_usage();
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
  let files = LogService::get_largest_log_files(limit);
  let count = files.len();
  let data = vec_to_data_array(files).map_err(|e| {
    ResponseBuilder::new()
      .error(format!("Serialization error: {}", e))
      .data(DataValue::String(String::new()))
      .build()
  })?;
  Ok(
    ResponseBuilder::new()
      .success(format!("Largest log files retrieved ({})", count))
      .data(data)
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_log_manager_summary() -> Result<ResponseModel, ResponseModel> {
  let summary = LogService::get_log_manager_summary();
  Ok(
    ResponseBuilder::new()
      .success("Log manager summary retrieved")
      .data(DataValue::Object(
        serde_json::to_value(summary).map_err(|e| format!("Serialization error: {}", e))?,
      ))
      .build(),
  )
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_log_summary() -> Result<ResponseModel, ResponseModel> {
  LogService.get_log_summary()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_log_entries(category: String) -> Result<ResponseModel, ResponseModel> {
  LogService.get_log_entries(category)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_old_logs(days: u32) -> Result<ResponseModel, ResponseModel> {
  LogService.clean_old_logs(days)
}
