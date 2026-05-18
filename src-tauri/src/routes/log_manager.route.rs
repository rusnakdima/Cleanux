/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::log_manager_service::{
  JournalInfo, LogFileInfo, LogManagerService, LogManagerSummary, LogrotateAnalysis,
  LogrotateConfig, RotatedLogInfo, VarLogUsage,
};

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_journal_size() -> u64 {
  LogManagerService::get_journal_size()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_journal_usage() -> JournalInfo {
  LogManagerService::get_journal_usage()
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
pub fn get_rotated_logs_size() -> u64 {
  LogManagerService::get_rotated_logs_size()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_rotated_logs() -> Vec<RotatedLogInfo> {
  LogManagerService::get_rotated_logs()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_rotated_logs(days: u32) -> Result<ResponseModel, ResponseModel> {
  LogManagerService::clean_rotated_logs(days)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_logrotate_configs() -> Vec<LogrotateConfig> {
  LogManagerService::get_logrotate_configs()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn analyze_logrotate() -> LogrotateAnalysis {
  LogManagerService::analyze_logrotate()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_var_log_usage() -> VarLogUsage {
  LogManagerService::get_var_log_usage()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_largest_log_files(limit: usize) -> Vec<LogFileInfo> {
  LogManagerService::get_largest_log_files(limit)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_log_manager_summary() -> LogManagerSummary {
  LogManagerService::get_log_manager_summary()
}
