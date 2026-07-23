/* helpers */
use crate::utils::common_paths::CommonPath;
use crate::utils::{collect_trash_file_models, remove_paths_with_errors, service_method_full};
/* models */
use crate::models::TrashFileModel;
/* errors */
use crate::models::AppError;
use std::fs;
use tauri_shared::response::Response;
pub struct TrashCleaningService;
type CleanResult<T> = Result<T, AppError>;
impl TrashCleaningService {
  service_method_full!(get_trash_files => get_trash_files_inner);
  fn get_trash_files_inner(&self) -> CleanResult<Response<serde_json::Value>> {
    let trash_dir = CommonPath::TrashFiles.path().ok_or(AppError::InvalidPath(
      "Home directory not found".to_string(),
    ))?;
    let trash_files: Vec<TrashFileModel> = collect_trash_file_models(&trash_dir);
    let data: Vec<serde_json::Value> = trash_files
      .into_iter()
      .map(serde_json::to_value)
      .collect::<Result<_, _>>()?;
    Ok(Response::success(
      serde_json::Value::Array(data),
      Some("Trash files retrieved successfully"),
    ))
  }
  pub fn clear_selected_trash_files(
    &self,
    paths: Vec<String>,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    let outcome = remove_paths_with_errors(paths);
    if outcome.errors.is_empty() {
      Ok(Response::success(
        serde_json::Value::String(String::new()),
        Some(&format!(
          "Successfully cleared {} trash files",
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
  pub fn clear_trash(&self) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    let trash_dir = CommonPath::TrashFiles
      .path()
      .ok_or_else(|| Response::error("Home directory not found".to_string()))?;
    match fs::read_dir(&trash_dir) {
      Ok(entries) => {
        for entry in entries.flatten() {
          let path = entry.path();
          if path.is_file() {
            if let Err(e) = fs::remove_file(&path) {
              return Err(AppError::from(e).into_response());
            }
          }
        }
        Ok(Response::success(
          serde_json::Value::String(String::new()),
          Some("Trash cleared successfully"),
        ))
      }
      Err(e) => Err(AppError::from(e).into_response()),
    }
  }
}
