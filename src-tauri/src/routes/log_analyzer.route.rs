/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::log_analyzer_service::LogAnalyzerService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_log_summary() -> Result<ResponseModel, ResponseModel> {
  LogAnalyzerService.get_log_summary()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_log_entries(category: String) -> Result<ResponseModel, ResponseModel> {
  LogAnalyzerService.get_log_entries(category)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_old_logs(days: u32) -> Result<ResponseModel, ResponseModel> {
  LogAnalyzerService.clean_old_logs(days)
}
