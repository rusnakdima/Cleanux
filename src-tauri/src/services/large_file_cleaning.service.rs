/* helpers */
use crate::helpers::{
  data_empty_string, data_string, models_into_data_array, remove_paths_with_errors,
  scan_large_file_models, success_response,
};
/* models */
use crate::models::{LargeFileModel, ResponseModel};
/* errors */
use crate::models::AppError;

use std::fs;

pub struct LargeFileCleaningService;

type CleanResult<T> = Result<T, AppError>;

#[allow(non_snake_case)]
impl LargeFileCleaningService {
  pub fn getLargeFiles(&self) -> Result<ResponseModel, ResponseModel> {
    self.get_large_files_inner().map_err(|e| e.into_response())
  }

  fn get_large_files_inner(&self) -> CleanResult<ResponseModel> {
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let files: Vec<LargeFileModel> = scan_large_file_models(&home, 3, 50, Some(200));
    let data = models_into_data_array(files)?;
    Ok(success_response("Large files retrieved successfully", data))
  }

  pub fn clearSelectedLargeFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    let outcome = remove_paths_with_errors(paths);
    if outcome.errors.is_empty() {
      Ok(success_response(
        format!("Successfully cleared {} large files", outcome.cleared),
        data_empty_string(),
      ))
    } else {
      Err(
        AppError::Unknown(format!(
          "Cleared {} files, failed on: {}",
          outcome.cleared,
          outcome.errors.join("; ")
        ))
        .into_response(),
      )
    }
  }

  pub fn clearAllLargeFiles(&self) -> Result<ResponseModel, ResponseModel> {
    self
      .clear_all_large_files_inner()
      .map_err(|e| e.into_response())
  }

  fn clear_all_large_files_inner(&self) -> CleanResult<ResponseModel> {
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let files: Vec<LargeFileModel> = scan_large_file_models(&home, 3, 50, None);
    let mut cleared_count = 0;
    for file in files {
      if fs::remove_file(file.path).is_ok() {
        cleared_count += 1;
      }
    }
    Ok(success_response(
      format!("Cleared {} large files", cleared_count),
      data_string(cleared_count.to_string()),
    ))
  }
}
