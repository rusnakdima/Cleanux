/* helpers */
use crate::utils::{
  data_empty_string, data_string, home_dir, remove_paths_with_errors, scan_large_file_models,
  success_response,
};
/* models */
use crate::models::Response;
/* errors */
use crate::models::AppError;

use serde_json::Value;
use std::fs;

pub struct LargeFileCleaningService;

type CleanResult<T> = Result<T, Response<Value>>;

impl LargeFileCleaningService {
  pub fn get_large_files(
    &self,
    limit: Option<usize>,
    offset: Option<usize>,
  ) -> Result<Response<Value>, Response<Value>> {
    self.get_large_files_inner(limit, offset)
  }

  fn get_large_files_inner(
    &self,
    limit: Option<usize>,
    offset: Option<usize>,
  ) -> CleanResult<Response<Value>> {
    let home = home_dir().map_err(|e| e.into_response())?;
    let (files, has_more, total) = scan_large_file_models(&home, 3, 50, Some(200), offset, limit);
    let paginated = serde_json::json!({
        "data": files,
        "has_more": has_more,
        "total": total
    });
    let data = serde_json::to_value(paginated)
      .map_err(|e| AppError::Unknown(format!("Failed to serialize large files data: {}", e)))?;
    Ok(success_response("Large files retrieved successfully", data))
  }

  pub fn clear_selected_large_files(
    &self,
    paths: Vec<String>,
  ) -> Result<Response<Value>, Response<Value>> {
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

  pub fn clear_all_large_files(&self) -> Result<Response<Value>, Response<Value>> {
    self.clear_all_large_files_inner()
  }

  fn clear_all_large_files_inner(&self) -> CleanResult<Response<Value>> {
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
