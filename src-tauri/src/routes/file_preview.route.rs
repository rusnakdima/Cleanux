use crate::helpers::ResponseBuilder;
use crate::models::{DataValue, ResponseModel};
use crate::services::file_preview_service::FilePreviewService;

#[tauri::command]
pub fn preview_file(path: String) -> Result<ResponseModel, ResponseModel> {
  FilePreviewService::preview_file(path)
}

#[tauri::command]
pub fn get_file_info(path: String) -> Result<ResponseModel, ResponseModel> {
  use std::fs;
  use std::path::Path;

  let file_path = Path::new(&path);

  if !file_path.exists() {
    return Err(
      ResponseBuilder::new()
        .error("File not found")
        .data(DataValue::String(String::new()))
        .build(),
    );
  }

  let metadata = fs::metadata(&path).map_err(|e| {
    ResponseBuilder::new()
      .error(format!("Failed to get file info: {}", e))
      .data(DataValue::String(String::new()))
      .build()
  })?;

  let name = file_path
    .file_name()
    .and_then(|n| n.to_str())
    .unwrap_or("unknown")
    .to_string();

  let extension = file_path
    .extension()
    .and_then(|e| e.to_str())
    .unwrap_or("")
    .to_lowercase();

  Ok(ResponseBuilder::new()
    .success("File info retrieved")
    .data(DataValue::Object(serde_json::json!({
      "name": name,
      "path": path,
      "size": metadata.len(),
      "extension": extension,
      "is_file": metadata.is_file(),
      "is_dir": metadata.is_dir(),
    })))
    .build())
}
