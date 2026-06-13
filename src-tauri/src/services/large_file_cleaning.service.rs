/* helpers */
use crate::helpers::{
  data_empty_string, data_string, home_dir, remove_paths_with_errors, scan_large_file_models,
  success_response,
};
/* models */
use crate::models::{PaginatedData, ResponseModel};
/* errors */
use crate::models::AppError;

use std::fs;

pub struct LargeFileCleaningService;

type CleanResult<T> = Result<T, ResponseModel>;

#[allow(non_snake_case)]
impl LargeFileCleaningService {
  pub fn getLargeFiles(
    &self,
    limit: Option<usize>,
    offset: Option<usize>,
  ) -> Result<ResponseModel, ResponseModel> {
    self.get_large_files_inner(limit, offset)
  }

  fn get_large_files_inner(
    &self,
    limit: Option<usize>,
    offset: Option<usize>,
  ) -> CleanResult<ResponseModel> {
    let home = home_dir().map_err(|e| e.into_response())?;
    let (files, has_more, total) = scan_large_file_models(&home, 3, 50, Some(200), offset, limit);
    let paginated = PaginatedData::new(files, has_more, total);
    let data = serde_json::to_value(paginated)
      .map_err(|e| AppError::Unknown(format!("Failed to serialize large files data: {}", e)))?;
    Ok(success_response(
      "Large files retrieved successfully",
      crate::models::DataValue::Object(data),
    ))
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
    self.clear_all_large_files_inner()
  }

  fn clear_all_large_files_inner(&self) -> CleanResult<ResponseModel> {
    let home = home_dir().map_err(|e| e.into_response())?;
    let (files, _, _) = scan_large_file_models(&home, 3, 50, None, None, None);
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
