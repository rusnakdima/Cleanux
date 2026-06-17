use crate::crud_create_command;
use crate::crud_delete_command;
use crate::crud_get_all_command;
use crate::crud_get_command;
use crate::crud_update_command;

crud_get_command!(get_cleaning_profile, "cleaning_profiles");
crud_get_all_command!(get_cleaning_profiles, "cleaning_profiles");
crud_create_command!(create_cleaning_profile, "cleaning_profiles");
crud_update_command!(update_cleaning_profile, "cleaning_profiles");
crud_delete_command!(delete_cleaning_profile, "cleaning_profiles");

use crate::models::{Response, Status};
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn apply_cleaning_profile(
  state: State<'_, AppState>,
  name: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let filter = serde_json::json!({
      "name": name
  });

  let filter = nosql_orm::query::Filter::from_json(&filter)
    .map_err(|e| Response::error(Status::Error, e.to_string()))?;

  let profiles = state
    .data
    .repository_service
    .find_many("cleaning_profiles", Some(filter), None, Some(1), None, true)
    .await
    .map_err(|e| Response::error(Status::Error, e.to_string()))?;

  let profile = profiles
    .into_iter()
    .next()
    .ok_or_else(|| Response::error(Status::NotFound, "Profile not found".to_string()))?;

  let clean_cache = profile
    .get("clean_cache")
    .and_then(|v| v.as_bool())
    .unwrap_or(false);
  let clean_trash = profile
    .get("clean_trash")
    .and_then(|v| v.as_bool())
    .unwrap_or(false);
  let clean_logs = profile
    .get("clean_logs")
    .and_then(|v| v.as_bool())
    .unwrap_or(false);
  let min_large_file_size = profile
    .get("min_large_file_size")
    .and_then(|v| v.as_u64())
    .unwrap_or(0);

  let mut results: Vec<String> = Vec::new();

  if clean_cache {
    if let Some(cache_dir) = dirs::cache_dir() {
      if cache_dir.exists() {
        match std::fs::remove_dir_all(&cache_dir) {
          Ok(_) => {
            let _ = std::fs::create_dir_all(&cache_dir);
            results.push("Cache cleared".to_string());
          }
          Err(e) => results.push(format!("Cache clear failed: {}", e)),
        }
      }
    }
  }

  if clean_trash {
    if let Some(home) = dirs::home_dir() {
      let trash_dir = home.join(".local/share/Trash/files");
      if trash_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&trash_dir) {
          for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
              if let Err(e) = std::fs::remove_file(&path) {
                results.push(format!("Trash clear partial: {}", e));
                break;
              }
            }
          }
          results.push("Trash cleared".to_string());
        }
      }
    }
  }

  if clean_logs {
    let log_dir = std::path::PathBuf::from("/var/log");
    if log_dir.exists() {
      results.push("Log cleaning requires elevated permissions".to_string());
    }
  }

  if min_large_file_size > 0 {
    results.push(format!(
      "Large file cleaning with threshold {} bytes",
      min_large_file_size
    ));
  }

  Ok(Response::success(
    format!("Profile '{}' applied: {}", name, results.join(", ")),
    serde_json::json!({ "applied": true, "results": results }),
  ))
}
