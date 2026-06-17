/* helpers */
use crate::utils::{
  collect_cache_file_models, data_empty_string, remove_paths_with_errors, success_response,
};
/* models */
use crate::models::Response;
/* errors */
use crate::models::AppError;

use log;
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
    log::info!(
      "Fetching cache files with limit: {:?}, offset: {:?}",
      limit,
      offset
    );
    let cache_dir = dirs::cache_dir()
      .ok_or_else(|| AppError::InvalidPath("Cache directory not found".to_string()))?;
    let (files, has_more, total) = collect_cache_file_models(cache_dir, offset, limit);
    log::info!(
      "Found {} cache files (total: {}, has_more: {})",
      files.len(),
      total,
      has_more
    );
    let paginated = serde_json::json!({
        "data": files,
        "has_more": has_more,
        "total": total
    });
    let data = serde_json::to_value(paginated)
      .map_err(|e| AppError::Unknown(format!("Failed to serialize cache data: {}", e)))?;
    Ok(success_response("Cache files retrieved successfully", data))
  }

  pub fn clear_selected_cache_files(
    &self,
    paths: Vec<String>,
  ) -> Result<Response<Value>, Response<Value>> {
    log::info!("Clearing {} selected cache files", paths.len());
    let outcome = remove_paths_with_errors(paths);
    if outcome.errors.is_empty() {
      log::info!("Successfully cleared {} cache files", outcome.cleared);
      Ok(success_response(
        format!("Successfully cleared {} cache files", outcome.cleared),
        data_empty_string(),
      ))
    } else {
      log::error!(
        "Cleared {} files, failed on: {}",
        outcome.cleared,
        outcome.errors.join("; ")
      );
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

  pub fn clear_cache(&self) -> Result<Response<Value>, Response<Value>> {
    self.clear_cache_inner()
  }

  fn clear_cache_inner(&self) -> CleanResult<Response<Value>> {
    log::info!("Clearing cache directory");
    let cache_dir = dirs::cache_dir()
      .ok_or_else(|| AppError::InvalidPath("Cache directory not found".to_string()))?;
    if cache_dir.exists() {
      match fs::remove_dir_all(&cache_dir) {
        Ok(_) => {
          let _ = fs::create_dir_all(&cache_dir);
          log::info!("Cache directory cleared successfully");
          Ok(success_response(
            "Cache directory cleared successfully",
            data_empty_string(),
          ))
        }
        Err(e) => {
          log::error!("Failed to clear cache directory: {}", e);
          Err(AppError::from(e).into())
        }
      }
    } else {
      log::info!("No cache to clear");
      Ok(success_response("No cache to clear", data_empty_string()))
    }
  }
}
