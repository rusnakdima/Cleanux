/* helpers */
use crate::helpers::{
  collect_cache_file_models, collect_log_file_models, collect_trash_file_models,
  data_empty_string, data_string, error_response, info_response, models_into_data_array,
  pkexec_rm_paths, remove_paths_with_errors, scan_large_file_models, stderr_message,
  success_response,
};
/* models */
use crate::models::{
  AppError, CacheFileModel, LargeFileModel, LogFileModel, ResponseModel, TrashFileModel,
};
/* services */
use crate::services::file_preview_service::FilePreviewService;
/* sys lib */
use std::fs;
use std::path::Path;

pub struct CleanerService;

type CleanResult<T> = Result<T, AppError>;

#[allow(non_snake_case)]
impl CleanerService {
  pub fn getCacheFiles(&self) -> Result<ResponseModel, ResponseModel> {
    self
      .get_cache_files_inner()
      .map_err(|e| e.into_response())
  }

  fn get_cache_files_inner(&self) -> CleanResult<ResponseModel> {
    let cache_dir = dirs::cache_dir()
      .ok_or_else(|| AppError::message("Cache directory not found"))?;
    let files: Vec<CacheFileModel> = collect_cache_file_models(cache_dir);
    let data = models_into_data_array(files)?;
    Ok(success_response(
      "Cache files retrieved successfully",
      data,
    ))
  }

  pub fn getTrashFiles(&self) -> Result<ResponseModel, ResponseModel> {
    self
      .get_trash_files_inner()
      .map_err(|e| e.into_response())
  }

  fn get_trash_files_inner(&self) -> CleanResult<ResponseModel> {
    let home = dirs::home_dir().ok_or_else(|| AppError::message("Home directory not found"))?;
    let trash_dir = home.join(".local/share/Trash/files");
    let trash_files: Vec<TrashFileModel> = collect_trash_file_models(&trash_dir);
    let data = models_into_data_array(trash_files)?;
    Ok(success_response(
      "Trash files retrieved successfully",
      data,
    ))
  }

  pub fn getSystemLogs(&self) -> Result<ResponseModel, ResponseModel> {
    self
      .get_system_logs_inner()
      .map_err(|e| e.into_response())
  }

  fn get_system_logs_inner(&self) -> CleanResult<ResponseModel> {
    let log_dir = Path::new("/var/log");
    let files: Vec<LogFileModel> = collect_log_file_models(log_dir, 3, 500);
    let data = models_into_data_array(files)?;
    Ok(success_response(
      "System logs retrieved successfully",
      data,
    ))
  }

  pub fn getLargeFiles(&self) -> Result<ResponseModel, ResponseModel> {
    self
      .get_large_files_inner()
      .map_err(|e| e.into_response())
  }

  fn get_large_files_inner(&self) -> CleanResult<ResponseModel> {
    let home = dirs::home_dir().ok_or_else(|| AppError::message("Home directory not found"))?;
    let files: Vec<LargeFileModel> =
      scan_large_file_models(&home, 3, 50, Some(200));
    let data = models_into_data_array(files)?;
    Ok(success_response(
      "Large files retrieved successfully",
      data,
    ))
  }

  pub fn clearSelectedCacheFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    map_bulk_remove(paths, "cache files")
  }

  pub fn clearSelectedTrashFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    map_bulk_remove(paths, "trash files")
  }

  pub fn clearSelectedLargeFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    map_bulk_remove(paths, "large files")
  }

  pub fn clearSelectedLogFiles(&self, paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
    if paths.is_empty() {
      return Ok(success_response(
        "No log files selected",
        data_empty_string(),
      ));
    }
    let output = pkexec_rm_paths(&paths).map_err(|e| {
      error_response(format!("Failed to run pkexec: {}", e), data_empty_string())
    })?;
    if output.status.success() {
      Ok(success_response(
        format!("Successfully cleared {} log files", paths.len()),
        data_empty_string(),
      ))
    } else {
      Err(error_response(
        format!(
          "Failed to clear log files: {}",
          stderr_message(&output)
        ),
        data_empty_string(),
      ))
    }
  }

  pub fn clearTrash(&self) -> Result<ResponseModel, ResponseModel> {
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::message("Home directory not found"))
      .map_err(|e| e.into_response())?;
    let trash_dir = home.join(".local/share/Trash/files");
    match fs::read_dir(&trash_dir) {
      Ok(entries) => {
        for entry in entries.flatten() {
          let path = entry.path();
          if path.is_file() {
            if let Err(e) = fs::remove_file(&path) {
              return Err(error_response(
                format!("Failed to remove {}: {}", path.display(), e),
                data_empty_string(),
              ));
            }
          }
        }
        Ok(success_response(
          "Trash cleared successfully",
          data_empty_string(),
        ))
      }
      Err(e) => Err(error_response(
        format!("Failed to read trash: {}", e),
        data_empty_string(),
      )),
    }
  }

  pub fn clearCache(&self) -> Result<ResponseModel, ResponseModel> {
    self
      .clear_cache_inner()
      .map_err(|e| e.into_response())
  }

  fn clear_cache_inner(&self) -> CleanResult<ResponseModel> {
    let cache_dir = dirs::cache_dir()
      .ok_or_else(|| AppError::message("Cache directory not found"))?;
    if cache_dir.exists() {
      match fs::remove_dir_all(&cache_dir) {
        Ok(_) => {
          let _ = fs::create_dir_all(&cache_dir);
          Ok(success_response(
            "Cache directory cleared successfully",
            data_empty_string(),
          ))
        }
        Err(e) => Err(AppError::message(format!(
          "Failed to clear cache directory: {}",
          e
        ))),
      }
    } else {
      Ok(info_response("No cache to clear", data_empty_string()))
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
    let output = pkexec_rm_paths(&paths).map_err(|e| {
      error_response(
        format!("Failed to run pkexec: {}", e),
        data_string("0"),
      )
    })?;
    if output.status.success() {
      Ok(success_response(
        format!("Cleared {} log files", files.len()),
        data_string(files.len().to_string()),
      ))
    } else {
      Err(error_response(
        format!("Failed to clear logs: {}", stderr_message(&output)),
        data_string("0"),
      ))
    }
  }

  pub fn clearAllLargeFiles(&self) -> Result<ResponseModel, ResponseModel> {
    self
      .clear_all_large_files_inner()
      .map_err(|e| e.into_response())
  }

  fn clear_all_large_files_inner(&self) -> CleanResult<ResponseModel> {
    let home = dirs::home_dir().ok_or_else(|| AppError::message("Home directory not found"))?;
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

  pub fn previewFile(&self, path: String) -> Result<ResponseModel, ResponseModel> {
    FilePreviewService::preview_file(path)
  }
}

fn map_bulk_remove(paths: Vec<String>, label: &str) -> Result<ResponseModel, ResponseModel> {
  let outcome = remove_paths_with_errors(paths);
  if outcome.errors.is_empty() {
    Ok(success_response(
      format!("Successfully cleared {} {}", outcome.cleared, label),
      data_empty_string(),
    ))
  } else {
    Err(error_response(
      format!(
        "Cleared {} files, failed on: {}",
        outcome.cleared,
        outcome.errors.join("; ")
      ),
      data_empty_string(),
    ))
  }
}
