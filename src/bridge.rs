//! MCP bridge integration for Cleanux.
//!
//! Dispatches `commands_invoke` name/payload pairs to handler functions.
//! Uses `dioxus_shared::mcp::bridge::AppBridge` trait.

use dioxus_shared::error::AppError;
use dioxus_shared::mcp::bridge::{BridgeState, Response};
use serde_json::Value;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// ---------------------------------------------------------------------------
// Bridge state
// ---------------------------------------------------------------------------

pub struct CleanuxBridge {
    pub state: Arc<BridgeState>,
}

impl CleanuxBridge {
    pub fn new(state: Arc<BridgeState>) -> Self {
        Self { state }
    }
}

/// Power profile info returned by powerprofilesctl.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PowerProfileInfo {
    pub current: String,
    pub available: Vec<String>,
}

/// Thermal zone info from /sys/class/thermal/.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ThermalInfo {
    pub zone: String,
    pub temp_c: f64,
    pub zone_type: String,
}

// ---------------------------------------------------------------------------
// Power management — powerprofilesctl
// ---------------------------------------------------------------------------

/// Get current and available power profiles via `powerprofilesctl list`.
pub fn get_power_profiles() -> Result<PowerProfileInfo, AppError> {
    let output = std::process::Command::new("powerprofilesctl")
        .args(["list", "--json"])
        .output()
        .map_err(|e| AppError::Internal(format!("powerprofilesctl failed: {}", e)))?;

    if !output.status.success() {
        return Err(AppError::Internal(format!(
            "powerprofilesctl exited with {}",
            output.status
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Parse JSON looking for active profile and available list
    let parsed: serde_json::Value = serde_json::from_str(&stdout)
        .map_err(|e| AppError::Internal(format!("failed to parse powerprofilesctl output: {}", e)))?;

    let current = parsed.get("active")
        .and_then(|v| v.get("Profile"))
        .and_then(|v| v.as_str())
        .unwrap_or("balanced")
        .to_string();

    let available: Vec<String> = parsed.get("profiles")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.get("Profile").and_then(|p| p.as_str()).map(String::from))
                .collect()
        })
        .unwrap_or_else(|| vec!["balanced".into(), "performance".into(), "power-saver".into()]);

    Ok(PowerProfileInfo { current, available })
}

/// Set the active power profile via `powerprofilesctl set <profile>`.
pub fn set_power_profile(profile: &str) -> Result<(), AppError> {
    let output = std::process::Command::new("powerprofilesctl")
        .args(["set", profile])
        .output()
        .map_err(|e| AppError::Internal(format!("powerprofilesctl set failed: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Internal(format!("powerprofilesctl set failed: {}", stderr)));
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Thermal zones — /sys/class/thermal/
// ---------------------------------------------------------------------------

/// Read thermal zones from /sys/class/thermal/ and return zone type + temp.
pub fn get_thermal_info() -> Result<Vec<ThermalInfo>, AppError> {
    let mut zones = Vec::new();
    let thermal_base = std::path::PathBuf::from("/sys/class/thermal");

    if !thermal_base.exists() {
        return Ok(zones);
    }

    let entries = std::fs::read_dir(&thermal_base)
        .map_err(|e| AppError::Internal(format!("failed to read /sys/class/thermal: {}", e)))?;

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();

        // Only thermal zones: thermal_zone0, thermal_zone1, ...
        if !name.starts_with("thermal_zone") {
            continue;
        }

        // Read type
        let zone_type_path = path.join("type");
        let zone_type = if zone_type_path.exists() {
            std::fs::read_to_string(&zone_type_path)
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|_| "unknown".into())
        } else {
            "unknown".into()
        };

        // Read temperature (millidegrees Celsius)
        let temp_path = path.join("temp");
        let temp_c = if temp_path.exists() {
            std::fs::read_to_string(&temp_path)
                .ok()
                .and_then(|s| s.trim().parse::<f64>().ok())
                .map(|t| t / 1000.0)
                .unwrap_or(0.0)
        } else {
            continue;
        };

        zones.push(ThermalInfo {
            zone: name,
            temp_c,
            zone_type,
        });
    }

    Ok(zones)
}

// ---------------------------------------------------------------------------
// Async command runner
// ---------------------------------------------------------------------------

async fn run_cmd(program: &str, args: &[&str]) -> Result<Value, AppError> {
    let output = tokio::process::Command::new(program)
        .args(args)
        .output()
        .await
        .map_err(|e| AppError::Internal(format!("failed to run {}: {}", program, e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Internal(format!(
            "{} exited with {}: {}",
            program,
            output.status,
            stderr
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::to_value(stdout.trim())
        .map_err(|e| AppError::Serialization(e.to_string()))
}

// ---------------------------------------------------------------------------
// Command dispatcher
// ---------------------------------------------------------------------------

impl CleanuxBridge {
    pub fn invoke_app_commands() -> Vec<String> {
        crate::infrastructure::commands::invoke_app_commands()
    }

    pub fn invoke_app_command(name: &str, payload: &Value) -> Result<Value, AppError> {
        use tokio::runtime::Handle;

        let rt = Handle::current();

        match name {
            // ── Power management ──────────────────────────────────────────────
            "power_profiles" => {
                let info = get_power_profiles()?;
                return serde_json::to_value(info).map_err(|e| AppError::Serialization(e.to_string())).into();
            }
            "power_profile_set" => {
                let profile = payload
                    .get("profile")
                    .and_then(|v| v.as_str())
                    .ok_or(AppError::ValidationError("missing 'profile'".into()))?;
                set_power_profile(profile)?;
                return Ok(serde_json::json!({ "success": true, "profile": profile }));
            }
            "thermal_info" => {
                let info = get_thermal_info()?;
                return Ok(serde_json::to_value(info).map_err(|e| e.to_string())?);
            }

            // ── Power actions ────────────────────────────────────────────────
            "system_sleep" => {
                return rt.block_on(async {
                    match run_cmd("systemctl", &["suspend"]).await {
                        Ok(v) => Ok(v),
                        Err(_) => run_cmd("loginctl", &["lock-session"]).await,
                    }
                });
            }
            "system_shutdown" => {
                return rt.block_on(run_cmd("systemctl", &["poweroff"]));
            }
            "system_lock" => {
                return rt.block_on(run_cmd("loginctl", &["lock-session"]));
            }
            "system_reboot" => {
                return rt.block_on(run_cmd("systemctl", &["reboot"]));
            }
            "system_logout" => {
                return rt.block_on(async {
                    match run_cmd("loginctl", &["terminate-session", "self"]).await {
                        Ok(v) => Ok(v),
                        Err(_) => run_cmd("loginctl", &["terminate-user", "self"]).await,
                    }
                });
            }

            // ── Schedules ───────────────────────────────────────────────────
            "schedule_list" => {
                let schedules = crate::global_state::get_schedules();
                return Ok(serde_json::to_value(schedules).map_err(|e| AppError::Serialization(e.to_string()))?);
            }
            "schedule_add" => {
                let entry = payload
                    .get("schedule")
                    .cloned()
                    .ok_or_else(|| AppError::ValidationError("missing 'schedule' payload".into()))?;
                let entry: crate::global_state::ScheduleEntry =
                    serde_json::from_value(entry)
                    .map_err(|e| AppError::Serialization(format!("invalid schedule: {}", e)))?;
                crate::global_state::schedule_add(entry.clone());
                return Ok(serde_json::to_value(entry).map_err(|e| AppError::Serialization(e.to_string()))?);
            }
            "schedule_edit" => {
                let id = payload.get("id").and_then(|v| v.as_str())
                    .ok_or_else(|| AppError::ValidationError("missing 'id'".into()))?;
                let entry = payload
                    .get("schedule")
                    .cloned()
                    .ok_or_else(|| AppError::ValidationError("missing 'schedule' payload".into()))?;
                let entry: crate::global_state::ScheduleEntry =
                    serde_json::from_value(entry)
                    .map_err(|e| AppError::Serialization(format!("invalid schedule: {}", e)))?;
                crate::global_state::schedule_edit(id, entry.clone());
                return Ok(serde_json::to_value(entry).map_err(|e| AppError::Serialization(e.to_string()))?);
            }
            "schedule_delete" => {
                let id = payload.get("id").and_then(|v| v.as_str())
                    .ok_or_else(|| AppError::ValidationError("missing 'id'".into()))?;
                crate::global_state::schedule_delete(id);
                return Ok(serde_json::json!({ "success": true }));
            }

            // ── Fan info ───────────────────────────────────────────────────
            "fan_info" => {
                let fans = crate::infrastructure::sys_utils::get_fan_info()?;
                return Ok(serde_json::to_value(fans).map_err(|e| AppError::Serialization(e.to_string()))?);
            }

            // ── Quick clean / category sizes ────────────────────────────────
            "get_category_sizes" => {
                let resp = rt
                    .block_on(crate::application::handlers::get_category_sizes())
                    .map_err(|e| e.to_string())?;
                let data = resp.data.ok_or_else(|| "no data".to_string())?;
                return Ok(serde_json::json!({ "data": { "items": data } }));
            }
            "scan_cache_categories" => {
                let cats = payload
                    .get("categories")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_else(|| vec!["cache".into(), "trash".into(), "logs".into()]);
                return rt.block_on(crate::application::handlers::scan_cache_categories(&cats)).map_err(AppError::from)
                    .map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "start_quick_clean" => {
                let cats = payload
                    .get("categories")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_else(|| vec!["cache".into(), "trash".into(), "logs".into()]);
                return rt.block_on(crate::application::handlers::start_quick_clean(&cats)).map_err(AppError::from)
                    .map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| "no data".to_string())?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }

            // ── Health ─────────────────────────────────────────────────────
            "health_save_snapshot" => {
                return rt.block_on(crate::application::handlers::save_health_snapshot()).map_err(AppError::from)
                    .map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| "no data".to_string())?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "health_get_history" => {
                return rt.block_on(crate::application::handlers::get_health_history()).map_err(AppError::from)
                    .map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| "no data".to_string())?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "health_get_snapshot" => {
                return rt.block_on(async {
                    let stats = crate::application::handlers::get_system_stats().await.map_err(AppError::from)?;
                    let temps = crate::application::handlers::get_temperatures().await.map_err(AppError::from)?;
                    let data = serde_json::json!({
                        "stats": stats.data,
                        "temperatures": temps.data,
                    });
                    Ok(serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))?)
                });
            }
            "health_get_trends" => {
                return rt.block_on(crate::application::handlers::get_health_trends()).map_err(AppError::from)
                    .map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| "no data".to_string())?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "health_get_temperatures" => {
                return rt.block_on(crate::application::handlers::get_temperatures()).map_err(AppError::from)
                    .map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| "no data".to_string())?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "health_get_system_stats" => {
                return rt.block_on(crate::application::handlers::get_system_stats()).map_err(AppError::from)
                    .map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| "no data".to_string())?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }

            // ── Monitoring ─────────────────────────────────────────────────
            "monitoring_start" => {
                return rt.block_on(crate::application::handlers::start_monitoring()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "monitoring_stop" => {
                return rt.block_on(crate::application::handlers::stop_monitoring()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "monitoring_get_data" => {
                return rt.block_on(crate::application::handlers::get_monitoring_data()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "monitoring_is_active" => {
                return rt.block_on(crate::application::handlers::is_monitoring_active()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }

            // ── Cleaning ───────────────────────────────────────────────────
            "clean_scan_cache" => {
                return rt.block_on(crate::application::handlers::scan_cache()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "clean_scan_trash" => {
                return rt.block_on(crate::application::handlers::scan_trash()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "clean_scan_logs" => {
                return rt.block_on(crate::application::handlers::scan_logs()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "clean_find_broken_symlinks" => {
                return rt.block_on(crate::application::handlers::find_broken_symlinks()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "clean_find_orphaned" => {
                return rt.block_on(crate::application::handlers::find_orphaned_packages()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "clean_get_container_summary" => {
                return rt.block_on(crate::application::handlers::get_container_summary()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "clean_docker_prune" => {
                return rt.block_on(crate::application::handlers::docker_system_prune()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "clean_junk_category" => {
                let cat = payload.get("category").and_then(|v| v.as_str()).unwrap_or("cache");
                return rt.block_on(crate::application::handlers::clean_junk_category(cat)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "clean_get_junk_summary" => {
                return rt.block_on(crate::application::handlers::get_junk_summary()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "clean_get_startup_items" => {
                return rt.block_on(crate::application::handlers::get_startup_items()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "clean_set_startup_enabled" => {
                let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");
                let enabled = payload.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);
                return rt.block_on(crate::application::handlers::set_startup_item_enabled(id, enabled)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "clean_disable_startup_item" => {
                let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");
                return rt.block_on(crate::application::handlers::disable_startup_item(id)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }

            // ── Storage ─────────────────────────────────────────────────────
            "storage_scan_directory" => {
                let path = payload.get("path").and_then(|v| v.as_str()).unwrap_or("/");
                return rt.block_on(crate::application::handlers::scan_directory(path)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "storage_directory_size" => {
                let path = payload.get("path").and_then(|v| v.as_str()).unwrap_or("/");
                return rt.block_on(crate::application::handlers::get_directory_size(path)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "storage_find_empty_dirs" => {
                let path = payload.get("path").and_then(|v| v.as_str()).unwrap_or("/");
                return rt.block_on(crate::application::handlers::find_empty_directories(path)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "storage_find_duplicates" => {
                let path = payload.get("path").and_then(|v| v.as_str()).unwrap_or("/");
                return rt.block_on(crate::application::handlers::find_duplicates(path)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "storage_scan_browser_caches" => {
                return rt.block_on(crate::application::handlers::scan_browser_caches()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "storage_scan_thumbnail_caches" => {
                return rt.block_on(crate::application::handlers::scan_thumbnail_caches()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "backup_create" => {
                let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("default");
                return rt.block_on(crate::application::handlers::create_backup(name)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "backup_list" => {
                return rt.block_on(crate::application::handlers::list_backups()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "backup_restore" => {
                let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                return rt.block_on(crate::application::handlers::restore_backup(id)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "backup_delete" => {
                let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                return rt.block_on(crate::application::handlers::delete_backup(id)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }

            // ── Memory & system ───────────────────────────────────────────
            "memory_get_info" => {
                return rt.block_on(crate::application::handlers::get_memory_info()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "memory_optimize" => {
                return rt.block_on(crate::application::handlers::optimize_memory()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "memory_get_swap" => {
                return rt.block_on(crate::application::handlers::get_swap_info()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "memory_get_process_memory" => {
                return rt.block_on(crate::application::handlers::get_process_memory()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "system_get_current_kernel" => {
                return rt.block_on(crate::application::handlers::get_current_kernel()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "system_get_installed_kernels" => {
                return rt.block_on(crate::application::handlers::get_installed_kernels()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "system_get_old_kernels" => {
                return rt.block_on(crate::application::handlers::get_old_kernels()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "system_remove_kernel" => {
                let kernel = payload.get("kernel").and_then(|v| v.as_str()).unwrap_or("").to_string();
                return rt.block_on(crate::application::handlers::remove_kernel(kernel)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "system_refresh_grub" => {
                return rt.block_on(crate::application::handlers::refresh_grub()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "system_get_battery" => {
                return rt.block_on(crate::application::handlers::get_battery_info()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "system_get_all_services" => {
                return rt.block_on(crate::application::handlers::get_all_services()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "system_service_start" => {
                let svc = payload.get("service").and_then(|v| v.as_str()).unwrap_or("").to_string();
                return rt.block_on(crate::application::handlers::start_service(svc)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "system_service_stop" => {
                let svc = payload.get("service").and_then(|v| v.as_str()).unwrap_or("").to_string();
                return rt.block_on(crate::application::handlers::stop_service(svc)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "system_service_enable" => {
                let svc = payload.get("service").and_then(|v| v.as_str()).unwrap_or("").to_string();
                return rt.block_on(crate::application::handlers::enable_service(svc)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "system_service_disable" => {
                let svc = payload.get("service").and_then(|v| v.as_str()).unwrap_or("").to_string();
                return rt.block_on(crate::application::handlers::disable_service(svc)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }

            // ── CRUD ──────────────────────────────────────────────────────
            "crud_find_cleaning_profile" => {
                let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");
                return rt.block_on(crate::application::handlers::find_cleaning_profile(id)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "crud_find_all_cleaning_profiles" => {
                return rt.block_on(crate::application::handlers::find_all_cleaning_profiles()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "crud_insert_cleaning_profile" => {
                let data = payload.get("data").cloned().unwrap_or(serde_json::json!({}));
                return rt.block_on(crate::application::handlers::insert_cleaning_profile(data)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "crud_update_cleaning_profile" => {
                let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");
                let data = payload.get("data").cloned().unwrap_or(serde_json::json!({}));
                return rt.block_on(crate::application::handlers::update_cleaning_profile(id, data)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "crud_patch_cleaning_profile" => {
                let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");
                let patch = payload.get("patch").cloned().unwrap_or(serde_json::json!({}));
                return rt.block_on(crate::application::handlers::patch_cleaning_profile(id, patch)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "crud_delete_cleaning_profile" => {
                let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");
                return rt.block_on(crate::application::handlers::delete_cleaning_profile(id)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "crud_find_automation_recipe" => {
                let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");
                return rt.block_on(crate::application::handlers::find_automation_recipe(id)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "crud_find_all_automation_recipes" => {
                return rt.block_on(crate::application::handlers::find_all_automation_recipes()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "crud_insert_automation_recipe" => {
                let data = payload.get("data").cloned().unwrap_or(serde_json::json!({}));
                return rt.block_on(crate::application::handlers::insert_automation_recipe(data)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "crud_update_automation_recipe" => {
                let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");
                let data = payload.get("data").cloned().unwrap_or(serde_json::json!({}));
                return rt.block_on(crate::application::handlers::update_automation_recipe(id, data)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "crud_delete_automation_recipe" => {
                let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");
                return rt.block_on(crate::application::handlers::delete_automation_recipe(id)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "crud_find_cleaning_reports" => {
                return rt.block_on(crate::application::handlers::find_cleaning_reports()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "crud_generate_cleaning_report" => {
                return rt.block_on(crate::application::handlers::generate_cleaning_report()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "crud_compare_snapshots" => {
                let id1 = payload.get("id1").and_then(|v| v.as_str()).unwrap_or("");
                let id2 = payload.get("id2").and_then(|v| v.as_str()).unwrap_or("");
                return rt.block_on(crate::application::handlers::compare_snapshots(id1, id2)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "crud_find_health_snapshots" => {
                return rt.block_on(crate::application::handlers::find_all_health_snapshots()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "crud_find_execution_history" => {
                return rt.block_on(crate::application::handlers::find_execution_history()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }

            // ── Journal / logs ─────────────────────────────────────────────
            "journal_vacuum_by_size" => {
                let size = payload.get("size").and_then(|v| v.as_str()).unwrap_or("500M");
                return rt.block_on(crate::application::handlers::vacuum_journal(size)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "journal_vacuum_by_days" => {
                let days = payload.get("days").and_then(|v| v.as_u64()).unwrap_or(7) as u32;
                return rt.block_on(crate::application::handlers::vacuum_journal_by_days(days)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "journal_get_usage" => {
                return rt.block_on(crate::application::handlers::get_journal_usage()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "journal_get_size" => {
                return rt.block_on(crate::application::handlers::get_journal_size()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "journal_get_var_log_usage" => {
                return rt.block_on(crate::application::handlers::get_var_log_usage()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "journal_get_largest_files" => {
                let limit = payload.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
                return rt.block_on(crate::application::handlers::get_largest_log_files(limit)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "journal_get_rotated_logs" => {
                return rt.block_on(crate::application::handlers::get_rotated_logs()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "journal_get_rotated_logs_size" => {
                return rt.block_on(crate::application::handlers::get_rotated_logs_size()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "journal_clean_rotated_logs" => {
                return rt.block_on(crate::application::handlers::clean_rotated_logs()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "journal_get_logrotate_configs" => {
                return rt.block_on(crate::application::handlers::get_logrotate_configs()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "journal_analyze_logrotate" => {
                let path = payload.get("path").and_then(|v| v.as_str()).unwrap_or("/var/log/syslog");
                return rt.block_on(crate::application::handlers::analyze_logrotate(path)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "journal_scan_log_rotations" => {
                let path = payload.get("path").and_then(|v| v.as_str()).unwrap_or("/var/log");
                return rt.block_on(crate::application::handlers::scan_log_rotations(path)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "journal_get_summary" => {
                return rt.block_on(crate::application::handlers::get_log_manager_summary()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "journal_get_log_summary" => {
                return rt.block_on(crate::application::handlers::get_log_summary()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }

            // ── Structured log export / query ───────────────────────────────
            "log_export" => {
                let filter = crate::application::handlers::LogFilter::default();
                return rt.block_on(crate::application::handlers::logs_export(filter)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "log_query" => {
                let filter = crate::application::handlers::LogFilter::default();
                return rt.block_on(crate::application::handlers::logs_query(filter)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }

            // ── Media cache cleaning ───────────────────────────────────────
            "media_cache_get_summary" => {
                return rt.block_on(crate::infrastructure::scanners::media_cache_scanner::get_media_cache_summary())
                    .and_then(|r| serde_json::to_value(r).map_err(|e| AppError::Serialization(e.to_string())));
            }
            "media_cache_clean_steam_shader" => {
                let freed = rt.block_on(crate::infrastructure::scanners::media_cache_scanner::clean_steam_shader_cache())?;
                return Ok(serde_json::json!({ "freed": freed }));
            }
            "media_cache_clean_steam_download" => {
                let freed = rt.block_on(crate::infrastructure::scanners::media_cache_scanner::clean_steam_download_cache())?;
                return Ok(serde_json::json!({ "freed": freed }));
            }
            "media_cache_clean_spotify" => {
                let freed = rt.block_on(crate::infrastructure::scanners::media_cache_scanner::clean_spotify_cache())?;
                return Ok(serde_json::json!({ "freed": freed }));
            }
            "media_cache_clean_vlc" => {
                let freed = rt.block_on(crate::infrastructure::scanners::media_cache_scanner::clean_vlc_cache())?;
                return Ok(serde_json::json!({ "freed": freed }));
            }
            "media_cache_clean_thumbnails" => {
                let freed = rt.block_on(crate::infrastructure::scanners::media_cache_scanner::clean_thumbnail_cache())?;
                return Ok(serde_json::json!({ "freed": freed }));
            }
            "media_cache_clean_media_art" => {
                let freed = rt.block_on(crate::infrastructure::scanners::media_cache_scanner::clean_media_art_cache())?;
                return Ok(serde_json::json!({ "freed": freed }));
            }

            // ── Dev cache cleaning ─────────────────────────────────────────
            "dev_cache_get_summary" => {
                let summary = rt.block_on(crate::infrastructure::scanners::dev_cache_scanner::dev_cache_summary());
                return serde_json::to_value(summary).map_err(|e| AppError::Serialization(e.to_string()));
            }
            "dev_cache_clean_npm" => {
                let freed = rt.block_on(crate::infrastructure::scanners::dev_cache_scanner::clean_npm());
                return Ok(serde_json::json!({ "freed": freed }));
            }
            "dev_cache_clean_pip" => {
                let freed = rt.block_on(crate::infrastructure::scanners::dev_cache_scanner::clean_pip());
                return Ok(serde_json::json!({ "freed": freed }));
            }
            "dev_cache_clean_cargo" => {
                let freed = rt.block_on(crate::infrastructure::scanners::dev_cache_scanner::clean_cargo());
                return Ok(serde_json::json!({ "freed": freed }));
            }
            "dev_cache_clean_go" => {
                let freed = rt.block_on(crate::infrastructure::scanners::dev_cache_scanner::clean_go());
                return Ok(serde_json::json!({ "freed": freed }));
            }
            "dev_cache_clean_maven" => {
                let freed = rt.block_on(crate::infrastructure::scanners::dev_cache_scanner::clean_maven());
                return Ok(serde_json::json!({ "freed": freed }));
            }
            "dev_cache_clean_gradle" => {
                let freed = rt.block_on(crate::infrastructure::scanners::dev_cache_scanner::clean_gradle());
                return Ok(serde_json::json!({ "freed": freed }));
            }

            // ── App residue & package manager cleaning ──────────────────────
            "app_residue_scan" => {
                return rt.block_on(crate::application::handlers::scan_app_residue()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "app_residue_clean" => {
                let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("");
                return rt.block_on(crate::application::handlers::clean_app_residue(name)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "package_cache_clean" => {
                let manager = payload.get("manager").and_then(|v| v.as_str()).unwrap_or("apt");
                let freed = rt.block_on(crate::infrastructure::package_managers::clean_package_cache(manager))?;
                return Ok(serde_json::json!({ "freed": freed, "manager": manager }));
            }

            // ── Window ─────────────────────────────────────────────────────
            "window_config_get" => {
                return Ok(serde_json::to_value(crate::application::handlers::window_config_get())
                    .map_err(|e| e.to_string())?);
            }
            "window_config_set" => {
                let config = payload.clone();
                return Ok(serde_json::to_value(crate::application::handlers::window_config_set(
                    serde_json::from_value(config).map_err(|e| e.to_string())?,
                )).map_err(|e| e.to_string())?);
            }
            "window_set_always_on_top" => {
                let val = payload.get("value").and_then(|v| v.as_bool()).unwrap_or(false);
                return Ok(serde_json::to_value(crate::application::handlers::window_set_always_on_top(val))
                    .map_err(|e| e.to_string())?);
            }
            "window_set_fullscreen" => {
                let val = payload.get("value").and_then(|v| v.as_bool()).unwrap_or(false);
                return Ok(serde_json::to_value(crate::application::handlers::window_set_fullscreen(val))
                    .map_err(|e| e.to_string())?);
            }
            "window_set_maximized" => {
                let val = payload.get("value").and_then(|v| v.as_bool()).unwrap_or(false);
                return Ok(serde_json::to_value(crate::application::handlers::window_set_maximized(val))
                    .map_err(|e| e.to_string())?);
            }
            "window_minimize" => {
                return Ok(serde_json::to_value(crate::application::handlers::window_minimize())
                    .map_err(|e| e.to_string())?);
            }
            "window_restore" => {
                return Ok(serde_json::to_value(crate::application::handlers::window_restore())
                    .map_err(|e| e.to_string())?);
            }
            "window_focus" => {
                return Ok(serde_json::to_value(crate::application::handlers::window_focus())
                    .map_err(|e| e.to_string())?);
            }

            // ── Opener ─────────────────────────────────────────────────────
            "opener_open" => {
                let path = payload.get("path").and_then(|v| v.as_str()).unwrap_or("");
                return rt.block_on(crate::application::handlers::opener_open(path)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "opener_open_folder" => {
                let path = payload.get("path").and_then(|v| v.as_str()).unwrap_or("");
                return rt.block_on(crate::application::handlers::opener_open_folder(path)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "opener_open_url" => {
                let url = payload.get("url").and_then(|v| v.as_str()).unwrap_or("");
                return rt.block_on(crate::application::handlers::opener_open_url(url)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "opener_get_mime_type" => {
                let path = payload.get("path").and_then(|v| v.as_str()).unwrap_or("");
                return rt.block_on(crate::application::handlers::opener_get_mime_type(path)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }

            // ── Allowlist ─────────────────────────────────────────────────
            "allowlist_add" => {
                let path = payload.get("path").and_then(|v| v.as_str()).unwrap_or("");
                return Ok(serde_json::to_value(crate::application::handlers::allowlist_add(path))
                    .map_err(|e| e.to_string())?);
            }
            "allowlist_remove" => {
                let path = payload.get("path").and_then(|v| v.as_str()).unwrap_or("");
                return Ok(serde_json::to_value(crate::application::handlers::allowlist_remove(path))
                    .map_err(|e| e.to_string())?);
            }
            "allowlist_list" => {
                return Ok(serde_json::to_value(crate::application::handlers::allowlist_list())
                    .map_err(|e| e.to_string())?);
            }
            "allowlist_check" => {
                let path = payload.get("path").and_then(|v| v.as_str()).unwrap_or("");
                return Ok(serde_json::to_value(crate::application::handlers::allowlist_check(path))
                    .map_err(|e| e.to_string())?);
            }
            "allowlist_status" => {
                return Ok(serde_json::to_value(crate::application::handlers::allowlist_status())
                    .map_err(|e| e.to_string())?);
            }

            // ── Settings ──────────────────────────────────────────────────
            "settings_get" => {
                return rt.block_on(crate::application::handlers::settings_get()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "settings_save" => {
                let settings = payload.clone();
                return rt.block_on(crate::application::handlers::settings_save(
                    serde_json::from_value(settings).map_err(|e| AppError::ValidationError(e.to_string()))?,
                ))
                    .map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "settings_patch" => {
                let key = payload.get("key").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let value = payload.get("value").cloned().unwrap_or(serde_json::json!(null));
                return rt.block_on(crate::application::handlers::settings_patch(key, value)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }

            // ── Shell pages ────────────────────────────────────────────────
            "shell_pages_list" => {
                return Ok(serde_json::to_value(crate::application::handlers::shell_pages_list())
                    .map_err(|e| e.to_string())?);
            }
            "shell_get_active_page" => {
                return Ok(serde_json::to_value(crate::application::handlers::shell_get_active_page())
                    .map_err(|e| e.to_string())?);
            }

            // ── Quick actions ──────────────────────────────────────────────
            "quick_actions_list" => {
                return Ok(serde_json::to_value(crate::application::handlers::list_quick_actions())
                    .map_err(|e| e.to_string())?);
            }
            "quick_actions_execute" => {
                let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");
                return rt.block_on(crate::application::handlers::execute_quick_action(id)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }

            // ── Automation ─────────────────────────────────────────────────
            "automation_get_recipes" => {
                return rt.block_on(crate::application::handlers::get_all_recipes()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "automation_execute_recipe" => {
                let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");
                return rt.block_on(crate::application::handlers::execute_recipe(id)).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }
            "automation_export_report" => {
                return rt.block_on(crate::application::handlers::export_cleaning_report()).map_err(AppError::from)
                    .and_then(|r| {
                        let data = r.data.ok_or_else(|| AppError::NotFound("no data".into()))?;
                        serde_json::to_value(data).map_err(|e| AppError::Serialization(e.to_string()))
                    });
            }

            _ => Err(AppError::Internal(format!("command not implemented: {}", name)).into())
        }
    }

    pub fn invoke_ui_action(action: &str, _params: &Value) -> Result<Value, AppError> {
        Err(AppError::Internal(format!("action not implemented: {}", action)))
    }

    pub fn read_ui() -> Result<Value, AppError> {
        Ok(serde_json::json!({
            "route": "dashboard",
            "signals": {},
            "page": "Cleanux",
            "dark_mode": false
        }))
    }
}

/// Free function wrapper so pages can call `bridge::invoke_app_command(...)`.
pub fn invoke_app_command(name: &str, payload: &Value) -> Result<Value, AppError> {
    CleanuxBridge::invoke_app_command(name, payload)
}

impl dioxus_shared::mcp::bridge::AppBridge for CleanuxBridge {
    fn invoke_app_command(
        &self,
        name: &str,
        payload: &serde_json::Value,
    ) -> std::result::Result<serde_json::Value, String> {
        CleanuxBridge::invoke_app_command(name, payload).map_err(|e| e.to_string())
    }

    fn invoke_ui_action(
        &self,
        action: &str,
        params: &serde_json::Value,
    ) -> std::result::Result<serde_json::Value, String> {
        CleanuxBridge::invoke_ui_action(action, params).map_err(|e| e.to_string())
    }

    fn invoke_app_commands(&self) -> Vec<String> {
        crate::infrastructure::commands::invoke_app_commands()
    }

    fn read_ui(&self) -> std::result::Result<serde_json::Value, String> {
        Ok(serde_json::json!({
            "route": "dashboard",
            "signals": {},
            "page": "Cleanux",
            "dark_mode": false
        }))
    }

    fn dom_snapshot(&self) -> std::result::Result<serde_json::Value, String> {
        Ok(serde_json::json!({ "snapshot": "not_implemented" }))
    }
}

/// Start the MCP bridge and return bridge state + port.
///
/// Call from `main.rs` before `dioxus::LaunchBuilder::desktop().launch(...)`:
/// ```ignore
/// let (bridge, bridge_state) = start_mcp_bridge();
/// thread::spawn(move || bridge.run());
/// ```
pub fn start_mcp_bridge() -> (CleanuxBridge, Arc<BridgeState>) {
    let state = Arc::new(BridgeState::default());
    let bridge = CleanuxBridge::new(state.clone());
    (bridge, state)
}

/// Legacy bridge consumer loop for Cleanux's custom DOM/eval handling.
pub fn bridge_consumer_loop(state: Arc<BridgeState>) {
    loop {
        if state.is_shutdown() {
            break;
        }
        for (id, result) in state.dequeue_js_results() {
            state.set_response(
                id,
                Response {
                    result: Some(serde_json::json!(result)),
                    error: None,
                },
            );
        }
        for cmd in state.dequeue_all() {
            if matches!(cmd.method.as_str(), "evaluate_js" | "dom_snapshot") {
                state.enqueue_eval_request(dioxus_shared::mcp::bridge::state::EvalRequest {
                    id: cmd.id,
                    method: cmd.method.clone(),
                    params: cmd.params.clone(),
                });
                continue;
            }
            let response = match cmd.method.as_str() {
                "ping" => Response {
                    result: Some(serde_json::json!({ "pong": true })),
                    error: None,
                },
                "app_info" => Response {
                    result: Some(
                        serde_json::json!({ "name": env!("CARGO_PKG_NAME"), "version": env!("CARGO_PKG_VERSION"), "platform": "dioxus-desktop" }),
                    ),
                    error: None,
                },
                "initialize" => Response {
                    result: Some(
                        serde_json::json!({ "protocolVersion": "2024-11-05", "capabilities": { "tools": true }, "serverInfo": { "name": env!("CARGO_PKG_NAME"), "version": env!("CARGO_PKG_VERSION") } }),
                    ),
                    error: None,
                },
                "health" => Response {
                    result: Some(serde_json::json!({ "healthy": true })),
                    error: None,
                },
                "commands_list" => Response {
                    result: Some(serde_json::json!({ "commands": crate::infrastructure::commands::invoke_app_commands() })),
                    error: None,
                },
                "logs_read" => Response {
                    result: Some(serde_json::json!({ "entries": state.get_logs() })),
                    error: None,
                },
                "commands_invoke" => {
                    let name = cmd.params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                    let payload = cmd.params.get("payload").cloned().unwrap_or_default();
                    match CleanuxBridge::invoke_app_command(name, &payload) {
                        Ok(v) => Response { result: Some(v), error: None },
                        Err(e) => Response { result: None, error: Some(e.to_string()) },
                    }
                }
                "ui_invoke_action" => {
                    let action = cmd.params.get("action").and_then(|v| v.as_str()).unwrap_or("");
                    let params = cmd.params.get("params").cloned().unwrap_or_default();
                    match CleanuxBridge::invoke_ui_action(action, &params) {
                        Ok(v) => Response { result: Some(v), error: None },
                        Err(e) => Response { result: None, error: Some(e.to_string()) },
                    }
                }
                "evaluate_js" | "dom_snapshot" | "webview_screenshot" => {
                    state.enqueue_eval_request(dioxus_shared::mcp::bridge::state::EvalRequest {
                        id: cmd.id,
                        method: cmd.method.clone(),
                        params: cmd.params.clone(),
                    });
                    continue;
                }
                other => Response {
                    result: None,
                    error: Some(format!("unknown method: {}", other)),
                },
            };
            state.set_response(cmd.id, response);
        }
        thread::sleep(Duration::from_millis(25));
    }
}
