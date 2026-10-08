//! Automation Recipe KAS handlers for Cleanux
//!
//! Delegates to `global_state::routine_service()`.

use crate::error::AppError;
use crate::global_state::routine_service;
use crate::response::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickAction {
  pub id: String,
  pub name: String,
  pub description: String,
  pub icon: String,
}

/// List all available quick actions.
pub fn list_quick_actions() -> Response<Vec<QuickAction>> {
  let actions = vec![
    QuickAction {
      id: "clean_cache".to_string(),
      name: "Clean Cache".to_string(),
      description: "Remove temporary cache files".to_string(),
      icon: "delete_sweep".to_string(),
    },
    QuickAction {
      id: "optimize_memory".to_string(),
      name: "Optimize Memory".to_string(),
      description: "Drop filesystem caches to free RAM".to_string(),
      icon: "memory".to_string(),
    },
    QuickAction {
      id: "clean_logs".to_string(),
      name: "Clean Logs".to_string(),
      description: "Remove old rotated log files".to_string(),
      icon: "description".to_string(),
    },
    QuickAction {
      id: "docker_prune".to_string(),
      name: "Docker Prune".to_string(),
      description: "Remove unused Docker images and containers".to_string(),
      icon: "inventory_2".to_string(),
    },
  ];
  Response::success(actions, Some("Quick actions retrieved"))
}

/// Execute a quick action by ID.
pub async fn execute_quick_action(id: &str) -> Result<Response<Value>, AppError> {
  match id {
    "clean_cache" => {
      let home = dirs::home_dir().unwrap_or_default();
      let freed = crate::infrastructure::sys_utils::delete_old_files(&home.join(".cache"), 0)
        .await
        .unwrap_or(0);
      Ok(Response::success(
        serde_json::json!({"freed": freed}),
        Some("Cache cleaned"),
      ))
    }
    "optimize_memory" => {
      let service = crate::infrastructure::memory_service::MemoryService::new();
      match service.drop_caches().await {
        Ok(r) => Ok(Response::success(
          serde_json::json!({"success": r.caches_cleared}),
          Some("Memory optimized"),
        )),
        Err(e) => Err(AppError::Internal(e.to_string().into())),
      }
    }
    "clean_logs" => {
      let freed = crate::infrastructure::sys_utils::delete_old_files(
        &std::path::PathBuf::from("/var/log"),
        30,
      )
      .await
      .unwrap_or(0);
      Ok(Response::success(
        serde_json::json!({"freed": freed}),
        Some("Logs cleaned"),
      ))
    }
    "docker_prune" => {
      match crate::application::handlers::docker_system_prune()
        .await
        .map_err(AppError::from)
      {
        Ok(r) => {
          let freed = r.data;
          Ok(Response::success(
            serde_json::json!({ "freed": freed }),
            Some(r.message.as_str()),
          ))
        }
        Err(e) => Err(e),
      }
    }
    _ => Err(AppError::ValidationError(
      format!("unknown quick action: {}", id).into(),
    )),
  }
}

/// Get all automation recipes.
pub async fn get_all_recipes() -> Result<Response<Vec<Value>>, AppError> {
  let svc = routine_service();
  let recipes = svc.get_routines()?;
  let value: Vec<Value> = recipes
    .into_iter()
    .map(serde_json::to_value)
    .filter_map(|r| r.ok())
    .collect();
  Ok(Response::success(value, Some("Recipes retrieved")))
}

/// Execute an automation recipe by ID.
pub async fn execute_recipe(id: &str) -> Result<Response<Value>, AppError> {
  let svc = routine_service();
  let result = svc.execute_routine(id)?;
  Ok(Response::success(
    serde_json::to_value(result).map_err(|e| AppError::Internal(e.to_string().into()))?,
    Some("Recipe executed"),
  ))
}

/// Generate a cleaning report from recent history.
pub async fn generate_cleaning_report() -> Result<Response<Value>, AppError> {
  use crate::global_state::get_history_entries;
  let history = get_history_entries();

  let total_cleaned: u64 = history.iter().map(|e| e.space_reclaimed).sum();

  let report = serde_json::json!({
      "total_runs": history.len(),
      "total_bytes_cleaned": total_cleaned,
      "entries": history,
  });

  Ok(Response::success(report, Some("Cleaning report generated")))
}

/// Export cleaning report as formatted text for file save.
pub async fn export_cleaning_report() -> Result<Response<String>, AppError> {
  use crate::global_state::get_history_entries;
  let history = get_history_entries();

  let total_cleaned: u64 = history.iter().map(|e| e.space_reclaimed).sum();

  let mut report = String::new();
  report.push_str("===========================================\n");
  report.push_str("           CLEANUX CLEANING REPORT\n");
  report.push_str("===========================================\n\n");
  report.push_str(&format!(
    "Generated: {}\n",
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
  ));
  report.push_str(&format!("Total Cleaning Runs: {}\n", history.len()));
  report.push_str(&format!(
    "Total Space Reclaimed: {}\n\n",
    format_size(total_cleaned)
  ));

  if history.is_empty() {
    report.push_str("No cleaning history available.\n");
  } else {
    report.push_str("-------------------------------------------n");
    report.push_str("CLEANING HISTORY\n");
    report.push_str("-------------------------------------------n\n");
    for (i, entry) in history.iter().enumerate() {
      report.push_str(&format!("#{}\n", i + 1));
      report.push_str(&format!("  Recipe: {}\n", entry.recipe_name));
      report.push_str(&format!(
        "  Started: {}\n",
        entry.started_at.format("%Y-%m-%d %H:%M:%S")
      ));
      if let Some(completed) = entry.completed_at {
        report.push_str(&format!(
          "  Completed: {}\n",
          completed.format("%Y-%m-%d %H:%M:%S")
        ));
      }
      report.push_str(&format!("  Status: {:?}\n", entry.status));
      report.push_str(&format!(
        "  Space Reclaimed: {}\n",
        format_size(entry.space_reclaimed)
      ));
      report.push_str(&format!("  Items Affected: {}\n", entry.items_affected));
      if let Some(ref err) = entry.error_message {
        report.push_str(&format!("  Error: {}\n", err));
      }
      report.push_str("\n");
    }
  }

  report.push_str("===========================================\n");
  report.push_str("            END OF REPORT\n");
  report.push_str("===========================================\n");

  Ok(Response::success(report, Some("Report exported")))
}

pub fn format_size(bytes: u64) -> String {
  const KB: u64 = 1024;
  const MB: u64 = KB * 1024;
  const GB: u64 = MB * 1024;
  if bytes >= GB {
    format!("{:.2} GB", bytes as f64 / GB as f64)
  } else if bytes >= MB {
    format!("{:.2} MB", bytes as f64 / MB as f64)
  } else if bytes >= KB {
    format!("{:.2} KB", bytes as f64 / KB as f64)
  } else {
    format!("{} B", bytes)
  }
}

/// Compare two health snapshots by ID.
pub async fn compare_snapshots(id1: &str, id2: &str) -> Result<Response<Value>, AppError> {
  use crate::global_state::health_snapshot_service;

  let svc = health_snapshot_service();
  let s1 = svc
    .get(id1)
    .await
    .map_err(|e| AppError::Internal(e.to_string().into()))?;
  let s2 = svc
    .get(id2)
    .await
    .map_err(|e| AppError::Internal(e.to_string().into()))?;

  let comparison = serde_json::json!({
      "before": s1,
      "after": s2,
  });

  Ok(Response::success(comparison, Some("Snapshots compared")))
}

/// Scan for application residue (partial uninstall leftovers).
pub async fn scan_app_residue() -> Result<Response<Vec<Value>>, AppError> {
  // Scan common residue locations
  let home = dirs::home_dir().unwrap_or_default();
  let residue_base = home.join(".local/share");

  let mut items = Vec::new();

  // Check for orphaned app directories
  let known_apps = vec!["code", "sublime-text", "atom", "gitkraken", "postman"];
  for app in known_apps {
    let app_dir = residue_base.join(app);
    if app_dir.exists() {
      if let Ok(size) = crate::infrastructure::sys_utils::get_dir_size(&app_dir).await {
        if size > 0 {
          items.push(serde_json::json!({
              "name": app,
              "path": app_dir.to_string_lossy(),
              "size": size,
          }));
        }
      }
    }
  }

  let count = items.len();
  Ok(Response::success(
    items,
    Some(&format!("Found {} residue entries", count)),
  ))
}

/// Clean residue for a specific application.
pub async fn clean_app_residue(name: &str) -> Result<Response<u64>, AppError> {
  let home = dirs::home_dir().unwrap_or_default();
  let paths = vec![
    home.join(".local/share").join(name),
    home.join(".config").join(name),
    home.join(".cache").join(name),
  ];

  let mut freed = 0u64;
  for path in paths {
    if path.exists() {
      if let Ok(size) = crate::infrastructure::sys_utils::get_dir_size(&path).await {
        if tokio::fs::remove_dir_all(&path).await.is_ok() {
          freed += size;
        }
      }
    }
  }

  Ok(Response::success(
    freed,
    Some(&format!("Freed {} bytes from {} residue", freed, name)),
  ))
}
