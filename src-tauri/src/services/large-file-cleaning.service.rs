/* helpers */
use crate::utils::{home_dir, remove_paths_with_errors, scan_large_file_models};
/* models */
use crate::Response;
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
      .map_err(|e| Response::error(format!("Failed to serialize large files data: {}", e)))?;
    Ok(Response::success(
      data,
      Some("Large files retrieved successfully"),
    ))
  }
  pub fn clear_selected_large_files(
    &self,
    paths: Vec<String>,
  ) -> Result<Response<Value>, Response<Value>> {
    let outcome = remove_paths_with_errors(paths);
    if outcome.errors.is_empty() {
      Ok(Response::success(
        serde_json::Value::String(String::new()),
        Some(&format!(
          "Successfully cleared {} large files",
          outcome.cleared
        )),
      ))
    } else {
      Err(
        AppError::Internal(format!(
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
    Ok(Response::success(
      serde_json::Value::String(cleared_count.to_string()),
      Some(&format!("Cleared {} large files", cleared_count)),
    ))
  }
}
