use crate::crud_create_command;
use crate::crud_get_all_command;
use crate::crud_get_command;

crud_get_command!(get_cleaning_report, "cleaning_reports");
crud_get_all_command!(get_cleaning_reports, "cleaning_reports");
crud_create_command!(create_cleaning_report, "cleaning_reports");

use crate::models::{Response, Status};
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn crud_generate_cleaning_report(
  state: State<'_, AppState>,
  items_cleaned: i64,
  space_reclaimed: u64,
  duration: f64,
  categories: serde_json::Value,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let data = serde_json::json!({
      "date": chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
      "items_cleaned": items_cleaned,
      "space_reclaimed": space_reclaimed,
      "duration": duration,
      "categories": categories,
  });

  state
    .data
    .repository_service
    .insert("cleaning_reports", data)
    .await
    .map(|doc| Response::success("Report generated".to_string(), doc))
    .map_err(|e| Response::error(Status::Error, e.to_string()))
}

#[tauri::command]
pub async fn crud_get_cleaning_history(
  state: State<'_, AppState>,
  limit: Option<u64>,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  state
    .data
    .repository_service
    .find_many("cleaning_reports", None, None, limit, Some("date"), false)
    .await
    .map(|docs| {
      Response::success(
        "Cleaning history retrieved".to_string(),
        serde_json::to_value(docs).unwrap_or(serde_json::Value::Null),
      )
    })
    .map_err(|e| Response::error(Status::Error, e.to_string()))
}

#[tauri::command]
pub async fn crud_compare_snapshots(
  state: State<'_, AppState>,
  before_id: String,
  after_id: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  use crate::entities::cleaning_report_entity::{ComparisonDetails, SnapshotComparison};

  let before_doc = state
    .data
    .repository_service
    .find_by_id("cleaning_reports", &before_id)
    .await
    .map_err(|e| Response::error(Status::Error, e.to_string()))?
    .ok_or_else(|| Response::error(Status::NotFound, "Before report not found".to_string()))?;

  let after_doc = state
    .data
    .repository_service
    .find_by_id("cleaning_reports", &after_id)
    .await
    .map_err(|e| Response::error(Status::Error, e.to_string()))?
    .ok_or_else(|| Response::error(Status::NotFound, "After report not found".to_string()))?;

  let before_space = before_doc
    .get("space_reclaimed")
    .and_then(|v| v.as_u64())
    .unwrap_or(0);
  let after_space = after_doc
    .get("space_reclaimed")
    .and_then(|v| v.as_u64())
    .unwrap_or(0);
  let before_items = before_doc
    .get("items_cleaned")
    .and_then(|v| v.as_i64())
    .unwrap_or(0);
  let after_items = after_doc
    .get("items_cleaned")
    .and_then(|v| v.as_i64())
    .unwrap_or(0);

  let before_categories = before_doc.get("categories");
  let after_categories = after_doc.get("categories");

  let comparison = SnapshotComparison {
    before_id,
    after_id,
    space_reclaimed: after_space.saturating_sub(before_space),
    items_cleaned: after_items.saturating_sub(before_items),
    health_improvement: 0.0,
    details: ComparisonDetails {
      cache_change: get_category_change(before_categories, after_categories, "cache"),
      trash_change: get_category_change(before_categories, after_categories, "trash"),
      log_change: get_category_change(before_categories, after_categories, "logs"),
      large_file_change: get_category_change(before_categories, after_categories, "large_files"),
    },
  };

  Ok(Response::success(
    "Snapshots compared".to_string(),
    serde_json::to_value(comparison).unwrap_or_default(),
  ))
}

fn get_category_change(
  before: Option<&serde_json::Value>,
  after: Option<&serde_json::Value>,
  key: &str,
) -> i64 {
  let before_val = before
    .and_then(|v| v.get(key))
    .and_then(|v| v.as_i64())
    .unwrap_or(0);
  let after_val = after
    .and_then(|v| v.get(key))
    .and_then(|v| v.as_i64())
    .unwrap_or(0);
  after_val - before_val
}
