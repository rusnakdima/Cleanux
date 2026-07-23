use crate::services::health_history_service::{HealthHistoryService, HealthSnapshot};
use crate::services::monitor_service::MonitorService;
use crate::services::temperature_service::TemperatureService;
use crate::Response;
static HEALTH_SERVICE: std::sync::OnceLock<HealthHistoryService> = std::sync::OnceLock::new();
fn get_health_service() -> &'static HealthHistoryService {
  HEALTH_SERVICE.get_or_init(|| {
    let svc = HealthHistoryService::new();
    svc.init_database().ok();
    svc
  })
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_temperatures() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  TemperatureService::get_temperatures()
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_cpu_temperature() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  TemperatureService::get_cpu_temperature()
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_gpu_temperature() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  TemperatureService::get_gpu_temperature()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn save_health_snapshot(
  health_score: f64,
  cache_size: u64,
  trash_size: u64,
  log_size: u64,
  large_files_count: i64,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let snapshot = HealthSnapshot {
    id: None,
    timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    health_score,
    cache_size,
    trash_size,
    log_size,
    large_files_count,
  };
  match get_health_service().save_health_snapshot(snapshot) {
    Ok(id) => Ok(Response::success(
      serde_json::json!({ "id": id }),
      Some("Health snapshot saved successfully"),
    )),
    Err(e) => Err(Response::error(format!(
      "Failed to save health snapshot: {}",
      e
    ))),
  }
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_health_history(
  days: u32,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  match get_health_service().get_health_history(days) {
    Ok(history) => {
      let data: Vec<serde_json::Value> = history
        .into_iter()
        .map(serde_json::to_value)
        .collect::<Result<_, _>>()
        .map_err(|e| Response::error(format!("Serialization error: {}", e)))?;
      Ok(Response::success(
        serde_json::Value::Array(data),
        Some("Health history retrieved successfully"),
      ))
    }
    Err(e) => Err(Response::error(format!(
      "Failed to get health history: {}",
      e
    ))),
  }
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_health_trends(
  days: u32,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  match get_health_service().get_health_trends(days) {
    Ok(trend) => Ok(Response::success(
      serde_json::to_value(trend)
        .map_err(|e| Response::error(format!("Serialization error: {}", e)))?,
      Some("Health trends retrieved successfully"),
    )),
    Err(e) => Err(Response::error(format!(
      "Failed to get health trends: {}",
      e
    ))),
  }
}
#[tauri::command(rename_all = "camelCase")]
pub fn get_system_stats() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  MonitorService::get_system_stats()
}
#[tauri::command(rename_all = "camelCase")]
pub fn start_monitoring() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  MonitorService::start_monitoring()
}
#[tauri::command(rename_all = "camelCase")]
pub fn stop_monitoring() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  MonitorService::stop_monitoring()
}
