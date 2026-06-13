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

use std::fs;

pub struct TrashCleaningService;

type CleanResult<T> = Result<T, AppError>;

#[allow(non_snake_case)]
impl TrashCleaningService {
  service_method_full!(getTrashFiles => get_trash_files_inner);

  fn get_trash_files_inner(&self) -> CleanResult<ResponseModel> {
    let trash_dir = CommonPath::TrashFiles
      .path()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let trash_files: Vec<TrashFileModel> = collect_trash_file_models(&trash_dir);
    let data = models_into_data_array(trash_files)?;
    Ok(success_response("Trash files retrieved successfully", data))
  }

  pub fn clearSelectedTrashFiles(
    &self,
    paths: Vec<String>,
  ) -> Result<ResponseModel, ResponseModel> {
    let outcome = remove_paths_with_errors(paths);
    if outcome.errors.is_empty() {
      Ok(success_response(
        format!("Successfully cleared {} trash files", outcome.cleared),
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

  pub fn clearTrash(&self) -> Result<ResponseModel, ResponseModel> {
    let trash_dir = CommonPath::TrashFiles
      .path()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    match fs::read_dir(&trash_dir) {
      Ok(entries) => {
        for entry in entries.flatten() {
          let path = entry.path();
          if path.is_file() {
            if let Err(e) = fs::remove_file(&path) {
              return Err(AppError::Io(e).into_response());
            }
          }
        }
        Ok(success_response(
          "Trash cleared successfully",
          data_empty_string(),
        ))
      }
      Err(e) => Err(AppError::Io(e).into_response()),
    }
  }
}
