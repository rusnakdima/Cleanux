use crate::crud_create_command;
use crate::crud_get_all_command;
use crate::crud_get_command;

crud_get_command!(crud_get_health_snapshot, "health_snapshots");
crud_get_all_command!(crud_get_health_snapshots, "health_snapshots");
crud_create_command!(crud_create_health_snapshot, "health_snapshots");

use crate::models::{Response, Status};
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn crud_get_health_history(
  state: State<'_, AppState>,
  days: Option<u32>,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let days = days.unwrap_or(30);
  let cutoff = chrono::Utc::now() - chrono::Duration::days(days as i64);
  let cutoff_str = cutoff.format("%Y-%m-%d %H:%M:%S").to_string();

  let filter = serde_json::json!({
      "timestamp": { "$gte": cutoff_str }
  });

  let filter = nosql_orm::query::Filter::from_json(&filter)
    .map_err(|e| Response::error(Status::Error, e.to_string()))?;

  let docs = state
    .data
    .repository_service
    .find_many(
      "health_snapshots",
      Some(filter),
      None,
      None,
      Some("timestamp"),
      true,
    )
    .await
    .map_err(|e| Response::error(Status::Error, e.to_string()))?;

  Ok(Response::success(
    "Health history retrieved".to_string(),
    serde_json::to_value(docs).unwrap_or(serde_json::Value::Null),
  ))
}

#[tauri::command]
pub async fn crud_get_health_trends(
  state: State<'_, AppState>,
  days: Option<u32>,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  use crate::entities::health_snapshot_entity::HealthTrendEntity;

  let days = days.unwrap_or(30);
  let cutoff = chrono::Utc::now() - chrono::Duration::days(days as i64);
  let cutoff_str = cutoff.format("%Y-%m-%d %H:%M:%S").to_string();

  let filter = serde_json::json!({
      "timestamp": { "$gte": cutoff_str }
  });

  let filter = nosql_orm::query::Filter::from_json(&filter)
    .map_err(|e| Response::error(Status::Error, e.to_string()))?;

  let docs = state
    .data
    .repository_service
    .find_many(
      "health_snapshots",
      Some(filter),
      None,
      None,
      Some("timestamp"),
      true,
    )
    .await
    .map_err(|e| Response::error(Status::Error, e.to_string()))?;

  if docs.len() < 2 {
    let trend = HealthTrendEntity {
      trend: "insufficient_data".to_string(),
      change_percent: 0.0,
      days_analyzed: days,
    };
    return Ok(Response::success(
      "Health trends calculated".to_string(),
      serde_json::to_value(trend).unwrap_or_default(),
    ));
  }

  let first = &docs[0];
  let last = &docs[docs.len() - 1];

  let first_score = first
    .get("health_score")
    .and_then(|v| v.as_f64())
    .unwrap_or(0.0);
  let last_score = last
    .get("health_score")
    .and_then(|v| v.as_f64())
    .unwrap_or(0.0);

  let change_percent = if first_score > 0.0 {
    ((last_score - first_score) / first_score) * 100.0
  } else {
    0.0
  };

  let trend = if change_percent > 5.0 {
    "improving"
  } else if change_percent < -5.0 {
    "declining"
  } else {
    "stable"
  };

  let trend_entity = HealthTrendEntity {
    trend: trend.to_string(),
    change_percent,
    days_analyzed: days,
  };

  Ok(Response::success(
    "Health trends calculated".to_string(),
    serde_json::to_value(trend_entity).unwrap_or_default(),
  ))
}

#[tauri::command]
pub async fn crud_save_health_snapshot(
  state: State<'_, AppState>,
  data: serde_json::Value,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let doc = state
    .data
    .repository_service
    .insert("health_snapshots", data)
    .await
    .map_err(|e| Response::error(Status::Error, e.to_string()))?;

  Ok(Response::success("Health snapshot saved".to_string(), doc))
}
