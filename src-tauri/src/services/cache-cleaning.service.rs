use crate::models::AppError;
use crate::utils::{collect_cache_file_models, remove_paths_with_errors};
use crate::Response;
use serde_json::Value;
use std::fs;
pub struct CacheCleaningService;
type CleanResult<T> = Result<T, Response<Value>>;
impl CacheCleaningService {
  pub fn get_cache_files(
    &self,
    limit: Option<usize>,
    offset: Option<usize>,
  ) -> Result<Response<Value>, Response<Value>> {
    self.get_cache_files_inner(limit, offset)
  }
  fn get_cache_files_inner(
    &self,
    limit: Option<usize>,
    offset: Option<usize>,
  ) -> CleanResult<Response<Value>> {
    let cache_dir = dirs::cache_dir()
      .ok_or_else(|| "Cache directory not found".to_string())
      .map_err(Response::error)?;
    let (files, has_more, total) = collect_cache_file_models(cache_dir, offset, limit);
    let paginated = serde_json::json!({
        "data": files,
        "has_more": has_more,
        "total": total
    });
    let data = serde_json::to_value(paginated)
      .map_err(|e| Response::error(format!("Failed to serialize cache data: {}", e)))?;
    Ok(Response::success(
      data,
      Some("Cache files retrieved successfully"),
    ))
  }
  pub fn clear_selected_cache_files(
    &self,
    paths: Vec<String>,
  ) -> Result<Response<Value>, Response<Value>> {
    let outcome = remove_paths_with_errors(paths);
    if outcome.errors.is_empty() {
      Ok(Response::success(
        serde_json::Value::String(String::new()),
        Some(&format!(
          "Successfully cleared {} cache files",
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
  pub fn clear_cache(&self) -> Result<Response<Value>, Response<Value>> {
    self.clear_cache_inner()
  }
  fn clear_cache_inner(&self) -> CleanResult<Response<Value>> {
    let cache_dir = dirs::cache_dir()
      .ok_or_else(|| "Cache directory not found".to_string())
      .map_err(Response::error)?;
    if cache_dir.exists() {
      match fs::remove_dir_all(&cache_dir) {
        Ok(_) => {
          let _ = fs::create_dir_all(&cache_dir);
          Ok(Response::success(
            serde_json::Value::String(String::new()),
            Some("Cache directory cleared successfully"),
          ))
        }
        Err(e) => Err(Response::error(e.to_string())),
      }
    } else {
      Ok(Response::success(
        serde_json::Value::String(String::new()),
        Some("No cache to clear"),
      ))
    }
  }
}
