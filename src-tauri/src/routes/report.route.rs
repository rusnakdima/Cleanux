use crate::helpers::{array_response, define_singleton_service, ResponseBuilder};
use crate::models::DataValue;
use crate::models::ResponseModel;
use crate::services::report_service::{ReportCategories, ReportService};

define_singleton_service!(REPORT_SERVICE, ReportService, init_database);

#[tauri::command]
#[allow(non_snake_case)]
#[allow(clippy::too_many_arguments)]
pub fn generate_cleaning_report(
  items_cleaned: i64,
  space_reclaimed: u64,
  duration: f64,
  cache_items: i64,
  trash_items: i64,
  log_items: i64,
  large_file_items: i64,
  duplicate_items: i64,
) -> Result<ResponseModel, ResponseModel> {
  let categories = ReportCategories {
    cache: cache_items,
    trash: trash_items,
    logs: log_items,
    large_files: large_file_items,
    duplicates: duplicate_items,
  };

  match get_service().generate_cleaning_report(items_cleaned, space_reclaimed, duration, categories)
  {
    Ok(id) => Ok(
      ResponseBuilder::new()
        .success("Cleaning report generated successfully")
        .data(DataValue::Object(serde_json::json!({ "id": id })))
        .build(),
    ),
    Err(e) => Err(
      ResponseBuilder::new()
        .error(&format!("Failed to generate cleaning report: {}", e))
        .build(),
    ),
  }
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_cleaning_history(limit: Option<i64>) -> Result<ResponseModel, ResponseModel> {
  match get_service().get_cleaning_history(limit) {
    Ok(reports) => array_response("Cleaning history retrieved successfully", reports),
    Err(e) => Err(
      ResponseBuilder::new()
        .error(&format!("Failed to get cleaning history: {}", e))
        .build(),
    ),
  }
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn export_to_html(report_id: i64) -> Result<ResponseModel, ResponseModel> {
  match get_service().export_to_html(report_id) {
    Ok(html) => Ok(
      ResponseBuilder::new()
        .success("HTML report exported successfully")
        .data(DataValue::Object(serde_json::json!({ "html": html })))
        .build(),
    ),
    Err(e) => Err(
      ResponseBuilder::new()
        .error(&format!("Failed to export HTML report: {}", e))
        .build(),
    ),
  }
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn compare_snapshots(before_id: i64, after_id: i64) -> Result<ResponseModel, ResponseModel> {
  match get_service().compare_snapshots(before_id, after_id) {
    Ok(comparison) => Ok(
      ResponseBuilder::new()
        .success("Snapshots compared successfully")
        .data(DataValue::Object(
          serde_json::to_value(comparison).map_err(|e| format!("Serialization error: {}", e))?,
        ))
        .build(),
    ),
    Err(e) => Err(
      ResponseBuilder::new()
        .error(&format!("Failed to compare snapshots: {}", e))
        .build(),
    ),
  }
}
