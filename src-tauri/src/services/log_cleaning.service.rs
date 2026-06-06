/* helpers */
use crate::helpers::{
  collect_log_file_models, data_empty_string, data_string, models_into_data_array, pkexec_rm_paths,
  service_method_full, stderr_message, success_response,
};
/* models */
use crate::models::{LogFileModel, ResponseModel};
/* errors */
use crate::models::AppError;

use std::path::Path;

pub struct LogCleaningService;

type CleanResult<T> = Result<T, AppError>;

#[allow(non_snake_case)]
impl LogCleaningService {
  service_method_full!(getSystemLogs => get_system_logs_inner);

  fn get_system_logs_inner(&self) -> CleanResult<ResponseModel> {
    let log_dir = Path::new("/var/log");
    let files: Vec<LogFileModel> = collect_log_file_models(log_dir, 3, 500);
    let data = models_into_data_array(files)?;
    Ok(success_response("System logs retrieved successfully", data))
  }

  pub fn clearSelectedLogFiles(&self, paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
    if paths.is_empty() {
      return Ok(success_response(
        "No log files selected",
        data_empty_string(),
      ));
    }
    let output = pkexec_rm_paths(&paths)
      .map_err(|e| AppError::Unknown(format!("Failed to run pkexec: {}", e)))?;
    if output.status.success() {
      Ok(success_response(
        format!("Successfully cleared {} log files", paths.len()),
        data_empty_string(),
      ))
    } else {
      Err(
        AppError::Unknown(format!(
          "Failed to clear log files: {}",
          stderr_message(&output)
        ))
        .into_response(),
      )
    }
  }

  pub fn clearAllLogs(&self) -> Result<ResponseModel, ResponseModel> {
    let log_dir = Path::new("/var/log");
    let files: Vec<LogFileModel> = collect_log_file_models(log_dir, 3, 500);
    if files.is_empty() {
      return Ok(success_response(
        "No log files found to clear",
        data_string("0"),
      ));
    }
    let paths: Vec<String> = files.iter().map(|f| f.path.clone()).collect();
    let output = pkexec_rm_paths(&paths)
      .map_err(|e| AppError::Unknown(format!("Failed to run pkexec: {}", e)))?;
    if output.status.success() {
      Ok(success_response(
        format!("Cleared {} log files", files.len()),
        data_string(files.len().to_string()),
      ))
    } else {
      Err(
        AppError::Unknown(format!("Failed to clear logs: {}", stderr_message(&output)))
          .into_response(),
      )
    }
  }
}
