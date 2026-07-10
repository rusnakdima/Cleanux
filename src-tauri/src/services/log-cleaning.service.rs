/* helpers */
use crate::utils::{
  collect_log_file_models, data_empty_string, data_string, models_into_data_array, pkexec_rm_paths,
  service_method_full, stderr_string, success_response,
};
/* models */
use crate::models::LogFileModel;
/* errors */
use crate::models::AppError;
use crate::Response;
use serde_json::Value;
use std::path::Path;
pub struct LogCleaningService;
type CleanResult<T> = Result<T, AppError>;
impl LogCleaningService {
  service_method_full!(get_system_logs => get_system_logs_inner);
  fn get_system_logs_inner(&self) -> CleanResult<Response<Value>> {
    let log_dir = Path::new("/var/log");
    let files: Vec<LogFileModel> = collect_log_file_models(log_dir, 3, 500);
    let data = models_into_data_array(files)?;
    Ok(success_response(data, "System logs retrieved successfully"))
  }
  pub fn clear_selected_log_files(
    &self,
    paths: Vec<String>,
  ) -> Result<Response<Value>, Response<Value>> {
    if paths.is_empty() {
      return Ok(success_response(
        data_empty_string(),
        "No log files selected",
      ));
    }
    let output = pkexec_rm_paths(&paths)
      .map_err(|e| AppError::Unknown(format!("Failed to run pkexec: {}", e)))?;
    if output.status.success() {
      Ok(success_response(
        data_empty_string(),
        format!("Successfully cleared {} log files", paths.len()),
      ))
    } else {
      Err(
        AppError::Unknown(format!(
          "Failed to clear log files: {}",
          stderr_string(&output)
        ))
        .into_response(),
      )
    }
  }
  pub fn clear_all_logs(&self) -> Result<Response<Value>, Response<Value>> {
    let log_dir = Path::new("/var/log");
    let files: Vec<LogFileModel> = collect_log_file_models(log_dir, 3, 500);
    if files.is_empty() {
      return Ok(success_response(
        data_string("0"),
        "No log files found to clear",
      ));
    }
    let paths: Vec<String> = files.iter().map(|f| f.path.clone()).collect();
    let output = pkexec_rm_paths(&paths)
      .map_err(|e| AppError::Unknown(format!("Failed to run pkexec: {}", e)))?;
    if output.status.success() {
      Ok(success_response(
        data_string(files.len().to_string()),
        format!("Cleared {} log files", files.len()),
      ))
    } else {
      Err(
        AppError::Unknown(format!("Failed to clear logs: {}", stderr_string(&output)))
          .into_response(),
      )
    }
  }
}
