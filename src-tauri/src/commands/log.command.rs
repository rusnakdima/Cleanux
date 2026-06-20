use crate::models::Response;
use crate::services::log_manager_service::LogManagerService;
use crate::utils::{array_response, ResponseBuilder};
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_journal_size() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let size = LogManagerService::get_journal_size();
  Ok(
    ResponseBuilder::new()
      .success("Journal size retrieved")
      .data(serde_json::json!({ "size": size }))
      .build(),
  )
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_journal_usage() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let info = LogManagerService::get_journal_usage();
  Ok(
    ResponseBuilder::new()
      .success("Journal usage retrieved")
      .data(serde_json::to_value(info).unwrap_or(serde_json::Value::Null))
      .build(),
  )
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn vacuum_journal(
  size_mb: u32,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  LogManagerService::vacuum_journal(size_mb)
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn vacuum_journal_by_days(
  days: u32,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  LogManagerService::vacuum_journal_by_days(days)
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_rotated_logs_size() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let size = LogManagerService::get_rotated_logs_size();
  Ok(
    ResponseBuilder::new()
      .success("Rotated logs size retrieved")
      .data(serde_json::json!({ "size": size }))
      .build(),
  )
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_rotated_logs() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let logs = LogManagerService::get_rotated_logs();
  array_response("Rotated logs retrieved", logs)
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn clean_rotated_logs(
  days: u32,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  LogManagerService::clean_rotated_logs(days)
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_logrotate_configs() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let configs = LogManagerService::get_logrotate_configs();
  array_response("Logrotate configs retrieved", configs)
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn analyze_logrotate() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let analysis = LogManagerService::analyze_logrotate();
  Ok(
    ResponseBuilder::new()
      .success("Logrotate analysis retrieved")
      .data(serde_json::to_value(analysis).unwrap_or(serde_json::Value::Null))
      .build(),
  )
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_var_log_usage() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let usage = LogManagerService::get_var_log_usage();
  Ok(
    ResponseBuilder::new()
      .success("Var log usage retrieved")
      .data(serde_json::to_value(usage).unwrap_or(serde_json::Value::Null))
      .build(),
  )
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_largest_log_files(
  limit: usize,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let files = LogManagerService::get_largest_log_files(limit);
  array_response("Largest log files retrieved", files)
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_log_manager_summary() -> Result<Response<serde_json::Value>, Response<serde_json::Value>>
{
  let summary = LogManagerService::get_log_manager_summary();
  Ok(
    ResponseBuilder::new()
      .success("Log manager summary retrieved")
      .data(serde_json::to_value(summary).unwrap_or(serde_json::Value::Null))
      .build(),
  )
}
