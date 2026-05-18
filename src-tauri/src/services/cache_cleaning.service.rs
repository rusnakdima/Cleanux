/* helpers */
use crate::helpers::{
  collect_cache_file_models, data_empty_string, models_into_data_array, remove_paths_with_errors,
  success_response,
};
/* models */
use crate::models::{CacheFileModel, ResponseModel};
/* errors */
use crate::models::AppError;

use std::fs;

pub struct CacheCleaningService;

type CleanResult<T> = Result<T, AppError>;

#[allow(non_snake_case)]
impl CacheCleaningService {
  pub fn getCacheFiles(&self) -> Result<ResponseModel, ResponseModel> {
    self.get_cache_files_inner().map_err(|e| e.into_response())
  }

  fn get_cache_files_inner(&self) -> CleanResult<ResponseModel> {
    let cache_dir = dirs::cache_dir()
      .ok_or_else(|| AppError::InvalidPath("Cache directory not found".to_string()))?;
    let files: Vec<CacheFileModel> = collect_cache_file_models(cache_dir);
    let data = models_into_data_array(files)?;
    Ok(success_response("Cache files retrieved successfully", data))
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
    self.clear_cache_inner().map_err(|e| e.into_response())
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
        Err(e) => Err(AppError::Io(e)),
      }
    } else {
      Ok(success_response("No cache to clear", data_empty_string()))
    }
  }
}
