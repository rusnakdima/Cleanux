/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::scanner_service::ScannerService;

#[tauri::command]
#[allow(non_snake_case)]
pub fn find_duplicates(
  path: String,
  extension_filter: Option<String>,
) -> Result<ResponseModel, ResponseModel> {
  ScannerService::scan_for_duplicates(&path, extension_filter)
}
