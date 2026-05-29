/* helpers */
use crate::helpers::{
  collect_cache_file_models, data_empty_string, remove_paths_with_errors, success_response,
};
/* models */
use crate::models::{PaginatedData, ResponseModel};
/* errors */
use crate::models::AppError;

use std::fs;

pub struct CacheCleaningService;

type CleanResult<T> = Result<T, ResponseModel>;

#[allow(non_snake_case)]
impl CacheCleaningService {
  pub fn getCacheFiles(
    &self,
    limit: Option<usize>,
    offset: Option<usize>,
  ) -> Result<ResponseModel, ResponseModel> {
    self.get_cache_files_inner(limit, offset)
  }

  fn get_cache_files_inner(
    &self,
    limit: Option<usize>,
    offset: Option<usize>,
  ) -> CleanResult<ResponseModel> {
    let cache_dir = dirs::cache_dir()
      .ok_or_else(|| AppError::InvalidPath("Cache directory not found".to_string()))?;
    let (files, has_more, total) = collect_cache_file_models(cache_dir, offset, limit);
    let paginated = PaginatedData::new(files, has_more, total);
    let data = serde_json::to_value(paginated)
      .map_err(|e| AppError::Unknown(format!("Failed to serialize cache data: {}", e)))?;
    Ok(success_response(
      "Cache files retrieved successfully",
      crate::models::DataValue::Object(data),
    ))
  }

  pub fn clearSelectedCacheFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    let outcome = remove_paths_with_errors(paths);
    if outcome.errors.is_empty() {
      Ok(success_response(
        format!("Successfully cleared {} cache files", outcome.cleared),
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

  pub fn clearCache(&self) -> Result<ResponseModel, ResponseModel> {
    self.clear_cache_inner()
  }

  fn clear_cache_inner(&self) -> CleanResult<ResponseModel> {
    let cache_dir = dirs::cache_dir()
      .ok_or_else(|| AppError::InvalidPath("Cache directory not found".to_string()))?;
    if cache_dir.exists() {
      match fs::remove_dir_all(&cache_dir) {
        Ok(_) => {
          let _ = fs::create_dir_all(&cache_dir);
          Ok(success_response(
            "Cache directory cleared successfully",
            data_empty_string(),
          ))
        }
        Err(e) => Err(AppError::Io(e).into()),
      }
    } else {
      Ok(success_response("No cache to clear", data_empty_string()))
    }
  }
}
