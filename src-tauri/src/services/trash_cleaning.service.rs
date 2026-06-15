/* helpers */
use crate::helpers::common_paths::CommonPath;
use crate::helpers::{
  collect_trash_file_models, data_empty_string, models_into_data_array, remove_paths_with_errors,
  service_method_full, success_response,
};
/* models */
use crate::models::{ResponseModel, TrashFileModel};
/* errors */
use crate::models::AppError;

use log;
use std::fs;

pub struct TrashCleaningService;

type CleanResult<T> = Result<T, AppError>;

impl TrashCleaningService {
  service_method_full!(get_trash_files => get_trash_files_inner);

  fn get_trash_files_inner(&self) -> CleanResult<ResponseModel> {
    let trash_dir = CommonPath::TrashFiles
      .path()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let trash_files: Vec<TrashFileModel> = collect_trash_file_models(&trash_dir);
    let data = models_into_data_array(trash_files)?;
    Ok(success_response("Trash files retrieved successfully", data))
  }

  pub fn clear_selected_trash_files(
    &self,
    paths: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    log::info!("Clearing {} selected trash files", paths.len());
    let outcome = remove_paths_with_errors(paths);
    if outcome.errors.is_empty() {
      log::info!("Successfully cleared {} trash files", outcome.cleared);
      Ok(success_response(
        format!("Successfully cleared {} trash files", outcome.cleared),
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

  pub fn clear_trash(&self) -> Result<ResponseModel, ResponseModel> {
    log::info!("Clearing trash directory");
    let trash_dir = CommonPath::TrashFiles
      .path()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    match fs::read_dir(&trash_dir) {
      Ok(entries) => {
        for entry in entries.flatten() {
          let path = entry.path();
          if path.is_file() {
            if let Err(e) = fs::remove_file(&path) {
              log::error!("Failed to remove file from trash: {}", e);
              return Err(AppError::Io(e).into_response());
            }
          }
        }
        log::info!("Trash cleared successfully");
        Ok(success_response(
          "Trash cleared successfully",
          data_empty_string(),
        ))
      }
      Err(e) => {
        log::error!("Failed to read trash directory: {}", e);
        Err(AppError::Io(e).into_response())
      }
    }
  }
}
