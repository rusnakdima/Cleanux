use crate::services::log_manager_service::LogManagerService;
use crate::utils::array_response;
use crate::Response;
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_journal_size() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let size = LogManagerService::get_journal_size();
  Ok(Response::success(
    serde_json::json!({ "size": size }),
    Some("Journal size retrieved"),
  ))
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_journal_usage() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let info = LogManagerService::get_journal_usage();
  Ok(Response::success(
    serde_json::to_value(info).unwrap_or(serde_json::Value::Null),
    Some("Journal usage retrieved"),
  ))
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
  Ok(Response::success(
    serde_json::json!({ "size": size }),
    Some("Rotated logs size retrieved"),
  ))
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_rotated_logs() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let logs = LogManagerService::get_rotated_logs();
  array_response(logs, "Rotated logs retrieved")
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
  array_response(configs, "Logrotate configs retrieved")
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn analyze_logrotate() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let analysis = LogManagerService::analyze_logrotate();
  Ok(Response::success(
    serde_json::to_value(analysis).unwrap_or(serde_json::Value::Null),
    Some("Logrotate analysis retrieved"),
  ))
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_var_log_usage() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let usage = LogManagerService::get_var_log_usage();
  Ok(Response::success(
    serde_json::to_value(usage).unwrap_or(serde_json::Value::Null),
    Some("Var log usage retrieved"),
  ))
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_largest_log_files(
  limit: usize,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let files = LogManagerService::get_largest_log_files(limit);
  array_response(files, "Largest log files retrieved")
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_log_manager_summary() -> Result<Response<serde_json::Value>, Response<serde_json::Value>>
{
  let summary = LogManagerService::get_log_manager_summary();
  Ok(Response::success(
    serde_json::to_value(summary).unwrap_or(serde_json::Value::Null),
    Some("Log manager summary retrieved"),
  ))
}
