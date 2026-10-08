//! MCP bridge integration for Cleanux.
//!
//! Dispatches `commands_invoke` name/payload pairs to handler functions.

use crate::bridge_state::{BridgeState, EvalRequest, Response as BridgeMsg};
use crate::error::AppError;
use crate::infrastructure::commands;
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

  pub fn invoke_app_command(&self, name: &str, payload: &Value) -> Result<Value, AppError> {
    // Most handlers are async — run them on the current Tokio runtime.
    let rt = tokio::runtime::Handle::current();
    rt.block_on(self.invoke_app_command_async(name, payload))
  }

  async fn invoke_app_command_async(&self, name: &str, payload: &Value) -> Result<Value, AppError> {
    use crate::application::handlers::*;

    match name {
      // ── App info ──────────────────────────────────────────────────────────
      "app_info" => Ok(serde_json::json!({
        "name": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
        "platform": "dioxus-desktop"
      })),

      // ── Power profiles ─────────────────────────────────────────────────────
      "power_profiles" => {
        let info = get_power_profiles()?;
        serde_json::to_value(info).map_err(|e| AppError::Serialization(e.to_string()))
      }
      "power_profile_set" => {
        let profile = payload
          .get("profile")
          .and_then(|v| v.as_str())
          .ok_or_else(|| AppError::ValidationError("missing 'profile'".into()))?;
        set_power_profile(profile)?;
        Ok(serde_json::json!({ "success": true, "profile": profile }))
      }
      "thermal_info" => {
        let info = get_thermal_info()?;
        serde_json::to_value(info).map_err(|e| AppError::Serialization(e.to_string()))
      }

      // ── Commands list / health ──────────────────────────────────────────────
      "commands_list" => Ok(serde_json::json!({
        "commands": commands::invoke_app_commands()
      })),
      "health" => Ok(serde_json::json!({ "healthy": true })),
      "logs_read" => Ok(serde_json::json!({
        "entries": self.state.get_logs()
      })),

      // ── System commands ────────────────────────────────────────────────────
      "system_sleep" => {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(system_handlers::system_sleep())
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "system_shutdown" => {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(system_handlers::system_shutdown())
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "system_lock" => {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(system_handlers::system_lock())
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "system_reboot" => {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(system_handlers::system_reboot())
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "system_logout" => {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(system_handlers::system_logout())
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }

      // ── Memory / system stats ──────────────────────────────────────────────
      "memory_get_info" => system_handlers::get_memory_info()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "memory_get_swap" => system_handlers::get_swap_info()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "memory_optimize" => system_handlers::optimize_memory()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "memory_get_process_memory" => system_handlers::get_process_memory()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "system_kill_process" => {
        let pid = payload.get("pid").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        system_handlers::system_kill_process(pid)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "system_get_processes" => system_handlers::system_get_processes()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "system_kill_selected_processes" => {
        let pids = payload
          .get("pids")
          .and_then(|v| v.as_array())
          .map(|arr| {
            arr
              .iter()
              .filter_map(|v| v.as_u64())
              .map(|n| n as u32)
              .collect::<Vec<u32>>()
          })
          .unwrap_or_default();
        system_handlers::system_kill_selected_processes(pids)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "system_get_battery" => system_handlers::get_battery_info()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "system_get_all_services" => system_handlers::get_all_services()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "system_service_start" => {
        let svc = payload
          .get("service")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        system_handlers::start_service(svc.to_string())
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "system_service_stop" => {
        let svc = payload
          .get("service")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        system_handlers::stop_service(svc.to_string())
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "system_service_enable" => {
        let svc = payload
          .get("service")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        system_handlers::enable_service(svc.to_string())
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "system_service_disable" => {
        let svc = payload
          .get("service")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        system_handlers::disable_service(svc.to_string())
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "system_get_current_kernel" => system_handlers::get_current_kernel()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "system_get_installed_kernels" => system_handlers::get_installed_kernels()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "system_get_old_kernels" => system_handlers::get_old_kernels()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "system_remove_kernel" => {
        let kernel = payload
          .get("kernel")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        system_handlers::remove_kernel(kernel.to_string())
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "system_refresh_grub" => system_handlers::refresh_grub()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "system_get_old_initramfs" => system_handlers::system_get_old_initramfs()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "system_remove_initramfs" => {
        let initramfs = payload
          .get("initramfs")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        system_handlers::system_remove_initramfs(initramfs.to_string())
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "system_get_boot_space_info" => system_handlers::system_get_boot_space_info()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),

      // ── Health ─────────────────────────────────────────────────────────────
      "health_get_temperatures" => health_handlers::get_temperatures()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "health_get_system_stats" => health_handlers::get_system_stats()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "health_get_history" => health_handlers::get_health_history()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "health_get_trends" => health_handlers::get_health_trends()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "health_save_snapshot" => health_handlers::save_health_snapshot()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "health_get_snapshot" => health_handlers::get_health_snapshot()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),

      // ── Monitoring ─────────────────────────────────────────────────────────
      "monitoring_start" => monitoring_handlers::start_monitoring()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "monitoring_stop" => monitoring_handlers::stop_monitoring()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "monitoring_get_data" => monitoring_handlers::get_monitoring_data()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "monitoring_is_active" => monitoring_handlers::is_monitoring_active()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),

      // ── Cleaning ───────────────────────────────────────────────────────────
      "clean_scan_cache" => cleaner_handlers::scan_cache()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "clean_scan_trash" => cleaner_handlers::scan_trash()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "clean_scan_logs" => cleaner_handlers::scan_logs()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "clean_find_broken_symlinks" => cleaner_handlers::find_broken_symlinks()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "clean_find_orphaned" => cleaner_handlers::find_orphaned_packages()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "repair_icon_cache" => {
        match std::process::Command::new("fc-cache")
          .args(["-f", "-v"])
          .output()
        {
          Ok(out) => Ok(serde_json::json!({
            "success": out.status.success(),
            "message": String::from_utf8_lossy(&out.stderr)
          })),
          Err(e) => Ok(serde_json::json!({
            "success": false,
            "message": e.to_string()
          })),
        }
      }
      "repair_permissions" => {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
        match std::process::Command::new("bash")
          .args([
            "-c",
            &format!(
              "find '{}' -type d -exec chmod 755 {{}} \\; 2>/dev/null; \
             find '{}' -type f -executable -exec chmod 755 {{}} \\; 2>/dev/null; \
             echo 'done'",
              home, home
            ),
          ])
          .output()
        {
          Ok(out) => Ok(serde_json::json!({
            "success": out.status.success(),
            "message": "Permission repair completed".to_string(),
            "dirs_fixed": 0,
            "files_fixed": 0
          })),
          Err(e) => Ok(serde_json::json!({
            "success": false,
            "message": e.to_string(),
            "dirs_fixed": 0,
            "files_fixed": 0
          })),
        }
      }
      "repair_remove_broken_symlinks" => {
        let paths: Vec<String> = payload
          .get("paths")
          .and_then(|v| serde_json::from_value(v.clone()).ok())
          .unwrap_or_default();
        let mut removed = 0;
        for path in &paths {
          if std::process::Command::new("rm")
            .arg(path)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
          {
            removed += 1;
          }
        }
        Ok(serde_json::json!({
          "success": true,
          "removed": removed,
          "message": format!("Removed {} broken symlinks", removed)
        }))
      }
      "repair_remove_orphaned" => {
        let packages: Vec<String> = payload
          .get("packages")
          .and_then(|v| serde_json::from_value(v.clone()).ok())
          .unwrap_or_default();
        let mut removed = 0;
        for pkg in &packages {
          if std::process::Command::new("sudo")
            .args(["pacman", "-Rns", "--noconfirm", pkg])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
          {
            removed += 1;
          }
        }
        Ok(serde_json::json!({
          "success": true,
          "removed": removed,
          "message": format!("Removed {} orphaned packages", removed)
        }))
      }
      "clean_get_container_summary" => cleaner_handlers::get_container_summary()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "clean_docker_prune" => cleaner_handlers::docker_system_prune()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "clean_junk_category" => {
        let category = payload
          .get("category")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        cleaner_handlers::clean_junk_category(category)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "clean_get_junk_summary" => cleaner_handlers::get_junk_summary()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "clean_get_startup_items" => cleaner_handlers::get_startup_items()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "clean_set_startup_enabled" => {
        let id = payload
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        let enabled = payload
          .get("enabled")
          .and_then(|v| v.as_bool())
          .unwrap_or(false);
        cleaner_handlers::set_startup_item_enabled(id, enabled)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "clean_disable_startup_item" => {
        let id = payload
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        cleaner_handlers::disable_startup_item(id)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }

      "get_category_sizes" => cleaner_handlers::get_category_sizes()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "scan_cache_categories" => {
        let cats: Vec<String> = payload
          .get("categories")
          .and_then(|v| v.as_array())
          .map(|arr| {
            arr
              .iter()
              .filter_map(|v| v.as_str().map(String::from))
              .collect()
          })
          .unwrap_or_default();
        cleaner_handlers::scan_cache_categories(&cats)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "start_quick_clean" => {
        let cats: Vec<String> = payload
          .get("categories")
          .and_then(|v| v.as_array())
          .map(|arr| {
            arr
              .iter()
              .filter_map(|v| v.as_str().map(String::from))
              .collect()
          })
          .unwrap_or_default();
        cleaner_handlers::start_quick_clean(&cats)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }

      // ── Cleaning (app-level commands) ─────────────────────────────────────
      "cleaning.scan" => automation_handlers::scan_app_residue()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "cleaning.execute" => {
        let name = payload
          .get("name")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        automation_handlers::clean_app_residue(name)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }

      // ── Storage ────────────────────────────────────────────────────────────
      "storage_scan_directory" => {
        let path = payload
          .get("path")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        storage_handlers::scan_directory(path)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "storage_directory_size" => {
        let path = payload
          .get("path")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        storage_handlers::get_directory_size(path)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "storage_find_empty_dirs" => {
        let path = payload
          .get("path")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        storage_handlers::find_empty_directories(path)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "storage_find_duplicates" => {
        let path = payload
          .get("path")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        storage_handlers::find_duplicates(path)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "storage_scan_browser_caches" => storage_handlers::scan_browser_caches()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "storage_scan_thumbnail_caches" => storage_handlers::scan_thumbnail_caches()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),

      // ── Backup ─────────────────────────────────────────────────────────────
      "backup_create" => {
        let name = payload
          .get("name")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        storage_handlers::create_backup(name)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "backup_list" => storage_handlers::list_backups()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "backup_restore" => {
        let id = payload
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or_default()
          .to_string();
        storage_handlers::restore_backup(id)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "backup_delete" => {
        let id = payload
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or_default()
          .to_string();
        storage_handlers::delete_backup(id)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "storage_get_backup_dir" => storage_handlers::get_backup_dir()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),

      // ── Dashboard ─────────────────────────────────────────────────────────
      "dashboard_get_system_services" => dashboard_handlers::get_system_services()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "dashboard_get_cache_summary" => dashboard_handlers::get_cache_summary()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "dashboard_get_trash_summary" => dashboard_handlers::get_trash_summary()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "dashboard_get_log_summary" => dashboard_handlers::get_log_summary()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "dashboard_get_large_files_summary" => dashboard_handlers::get_large_files_summary()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),

      // ── Journal ────────────────────────────────────────────────────────────
      "journal_vacuum_by_size" => {
        let size = payload
          .get("size")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        journal_handlers::vacuum_journal(size)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }
      "journal_vacuum_by_days" => {
        let days = payload.get("days").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        journal_handlers::vacuum_journal_by_days(days)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }
      "journal_get_usage" => journal_handlers::get_journal_usage()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "journal_get_size" => journal_handlers::get_journal_size()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "journal_get_var_log_usage" => journal_handlers::get_var_log_usage()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "journal_get_largest_files" => {
        let limit = payload.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
        journal_handlers::get_largest_log_files(limit)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }
      "journal_get_rotated_logs" => journal_handlers::get_rotated_logs()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "journal_get_rotated_logs_size" => journal_handlers::get_rotated_logs_size()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "journal_clean_rotated_logs" => journal_handlers::clean_rotated_logs()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "journal_get_logrotate_configs" => journal_handlers::get_logrotate_configs()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "journal_analyze_logrotate" => {
        let path = payload
          .get("path")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        journal_handlers::analyze_logrotate(path)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }
      "journal_scan_log_rotations" => {
        let path = payload
          .get("path")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        journal_handlers::scan_log_rotations(path)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }
      "journal_get_summary" => journal_handlers::get_log_manager_summary()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "journal_get_log_summary" => journal_handlers::get_log_summary()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),

      // ── Log / logging ──────────────────────────────────────────────────────
      "log_export" => {
        let filter = payload.clone();
        logging_handlers::logs_export(serde_json::from_value(filter).unwrap_or_default())
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }
      "log_query" => {
        let filter = payload.clone();
        logging_handlers::logs_query(serde_json::from_value(filter).unwrap_or_default())
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }

      // ── Media cache ────────────────────────────────────────────────────────
      "media_cache_get_summary" => {
        let summary =
          crate::infrastructure::scanners::media_cache_scanner::get_media_cache_summary()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        serde_json::to_value(summary).map_err(|e| AppError::Serialization(e.to_string()))
      }
      "media_cache_clean_steam_shader" => {
        let freed =
          crate::infrastructure::scanners::media_cache_scanner::clean_steam_shader_cache()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(serde_json::json!({ "freed": freed }))
      }
      "media_cache_clean_steam_download" => {
        let freed =
          crate::infrastructure::scanners::media_cache_scanner::clean_steam_download_cache()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(serde_json::json!({ "freed": freed }))
      }
      "media_cache_clean_spotify" => {
        let freed = crate::infrastructure::scanners::media_cache_scanner::clean_spotify_cache()
          .await
          .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(serde_json::json!({ "freed": freed }))
      }
      "media_cache_clean_vlc" => {
        let freed = crate::infrastructure::scanners::media_cache_scanner::clean_vlc_cache()
          .await
          .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(serde_json::json!({ "freed": freed }))
      }
      "media_cache_clean_thumbnails" => {
        let freed = crate::infrastructure::scanners::media_cache_scanner::clean_thumbnail_cache()
          .await
          .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(serde_json::json!({ "freed": freed }))
      }
      "media_cache_clean_media_art" => {
        let freed = crate::infrastructure::scanners::media_cache_scanner::clean_media_art_cache()
          .await
          .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(serde_json::json!({ "freed": freed }))
      }

      // ── Dev cache ─────────────────────────────────────────────────────────
      "dev_cache_get_summary" => {
        let summary = crate::infrastructure::scanners::dev_cache_scanner::dev_cache_summary().await;
        serde_json::to_value(summary).map_err(|e| AppError::Serialization(e.to_string()))
      }
      "dev_cache_clean_npm" => {
        let freed = crate::infrastructure::scanners::dev_cache_scanner::clean_npm().await;
        Ok(serde_json::json!({ "freed": freed }))
      }
      "dev_cache_clean_pip" => {
        let freed = crate::infrastructure::scanners::dev_cache_scanner::clean_pip().await;
        Ok(serde_json::json!({ "freed": freed }))
      }
      "dev_cache_clean_cargo" => {
        let freed = crate::infrastructure::scanners::dev_cache_scanner::clean_cargo().await;
        Ok(serde_json::json!({ "freed": freed }))
      }
      "dev_cache_clean_go" => {
        let freed = crate::infrastructure::scanners::dev_cache_scanner::clean_go().await;
        Ok(serde_json::json!({ "freed": freed }))
      }
      "dev_cache_clean_maven" => {
        let freed = crate::infrastructure::scanners::dev_cache_scanner::clean_maven().await;
        Ok(serde_json::json!({ "freed": freed }))
      }
      "dev_cache_clean_gradle" => {
        let freed = crate::infrastructure::scanners::dev_cache_scanner::clean_gradle().await;
        Ok(serde_json::json!({ "freed": freed }))
      }
      "dev_cache_clean_all" => {
        let npm = crate::infrastructure::scanners::dev_cache_scanner::clean_npm().await;
        let pip = crate::infrastructure::scanners::dev_cache_scanner::clean_pip().await;
        let cargo = crate::infrastructure::scanners::dev_cache_scanner::clean_cargo().await;
        let go = crate::infrastructure::scanners::dev_cache_scanner::clean_go().await;
        let maven = crate::infrastructure::scanners::dev_cache_scanner::clean_maven().await;
        let gradle = crate::infrastructure::scanners::dev_cache_scanner::clean_gradle().await;
        let total = npm + pip + cargo + go + maven + gradle;
        Ok(serde_json::json!({ "freed": total }))
      }

      // ── App residue ────────────────────────────────────────────────────────
      "app_residue_scan" => automation_handlers::scan_app_residue()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "app_residue_clean" => {
        let name = payload
          .get("name")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        automation_handlers::clean_app_residue(name)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }

      // ── App residue (extended) ─────────────────────────────────────────────
      "clean_scan_user_configs" => cleaner_handlers::scan_user_configs()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "clean_scan_user_data" => cleaner_handlers::scan_user_data()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "clean_scan_user_caches" => cleaner_handlers::scan_user_caches()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "clean_scan_home_residues" => cleaner_handlers::scan_home_residues()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "clean_get_orphaned_configs" => cleaner_handlers::get_orphaned_configs()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "clean_multiple_app_residues" => {
        let items: Vec<cleaner_handlers::AppResidueItem> = payload
          .get("items")
          .and_then(|v| serde_json::from_value(v.clone()).ok())
          .unwrap_or_default();
        cleaner_handlers::clean_multiple_app_residues(items)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "clean_get_residue_summary" => cleaner_handlers::get_residue_summary()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),

      // ── Package cache ─────────────────────────────────────────────────────
      "package_get_cache_summary" => {
        let summary = crate::infrastructure::package_managers::get_package_cache_summary()
          .await
          .map_err(|e| AppError::Internal(e.to_string()))?;
        serde_json::to_value(summary).map_err(|e| AppError::Serialization(e.to_string()))
      }
      "package_cache_clean" => {
        let manager = payload
          .get("manager")
          .and_then(|v| v.as_str())
          .unwrap_or("apt");
        let freed = crate::infrastructure::package_managers::clean_package_cache(manager)
          .await
          .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(serde_json::json!({ "freed": freed }))
      }
      "package_clean_package_cache" => {
        let manager = payload
          .get("manager")
          .and_then(|v| v.as_str())
          .unwrap_or("apt");
        let freed = crate::infrastructure::package_managers::clean_package_cache(manager)
          .await
          .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(serde_json::json!({ "freed": freed }))
      }

      // ── Window ─────────────────────────────────────────────────────────────
      "window_config_get" => {
        let cfg = window_handlers::window_config_get();
        Ok(serde_json::to_value(cfg).unwrap_or(Value::Null))
      }
      "window_config_set" => {
        let cfg = serde_json::from_value(payload.clone())
          .map_err(|e| AppError::ValidationError(e.to_string()))?;
        window_handlers::window_config_set(cfg);
        Ok(Value::Null)
      }
      "window_set_always_on_top" => {
        let value = payload
          .get("value")
          .and_then(|v| v.as_bool())
          .unwrap_or(false);
        window_handlers::window_set_always_on_top(value);
        Ok(Value::Null)
      }
      "window_set_fullscreen" => {
        let value = payload
          .get("value")
          .and_then(|v| v.as_bool())
          .unwrap_or(false);
        window_handlers::window_set_fullscreen(value);
        Ok(Value::Null)
      }
      "window_set_maximized" => {
        let value = payload
          .get("value")
          .and_then(|v| v.as_bool())
          .unwrap_or(false);
        window_handlers::window_set_maximized(value);
        Ok(Value::Null)
      }
      "window_minimize" => {
        window_handlers::window_minimize();
        Ok(Value::Null)
      }
      "window_restore" => {
        window_handlers::window_restore();
        Ok(Value::Null)
      }
      "window_focus" => {
        window_handlers::window_focus();
        Ok(Value::Null)
      }

      // ── Opener ─────────────────────────────────────────────────────────────
      "opener_open" => {
        let path = payload
          .get("path")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        opener_handlers::opener_open(path)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }
      "opener_open_folder" => {
        let path = payload
          .get("path")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        opener_handlers::opener_open_folder(path)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }
      "opener_open_url" => {
        let url = payload
          .get("url")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        opener_handlers::opener_open_url(url)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }
      "opener_get_mime_type" => {
        let path = payload
          .get("path")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        opener_handlers::opener_get_mime_type(path)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }

      // ── Allowlist ──────────────────────────────────────────────────────────
      "allowlist_add" => {
        let path = payload
          .get("path")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        let r = allowlist_handlers::allowlist_add(path);
        Ok(serde_json::to_value(r).unwrap_or(Value::Null))
      }
      "allowlist_remove" => {
        let path = payload
          .get("path")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        let r = allowlist_handlers::allowlist_remove(path);
        Ok(serde_json::to_value(r).unwrap_or(Value::Null))
      }
      "allowlist_list" => {
        let r = allowlist_handlers::allowlist_list();
        Ok(serde_json::to_value(r).unwrap_or(Value::Null))
      }
      "allowlist_check" => {
        let path = payload
          .get("path")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        let r = allowlist_handlers::allowlist_check(path);
        Ok(serde_json::to_value(r).unwrap_or(Value::Null))
      }
      "allowlist_status" => {
        let r = allowlist_handlers::allowlist_status();
        Ok(serde_json::to_value(r).unwrap_or(Value::Null))
      }

      // ── Settings ───────────────────────────────────────────────────────────
      "settings_get" => settings_handlers::settings_get()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "settings_save" => {
        let settings = serde_json::from_value(payload.clone())
          .map_err(|e| AppError::ValidationError(e.to_string()))?;
        settings_handlers::settings_save(settings)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }
      "settings_patch" => {
        let key = payload
          .get("key")
          .and_then(|v| v.as_str())
          .unwrap_or_default()
          .to_string();
        let value = payload.get("value").cloned().unwrap_or(Value::Null);
        settings_handlers::settings_patch(key, value)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }

      // ── Shell / pages ─────────────────────────────────────────────────────
      "shell_pages_list" => {
        let r = shell_pages_handlers::shell_pages_list();
        Ok(serde_json::to_value(r).unwrap_or(Value::Null))
      }
      "shell_get_active_page" => {
        let r = shell_pages_handlers::shell_get_active_page();
        Ok(serde_json::to_value(r).unwrap_or(Value::Null))
      }

      // ── Automation ────────────────────────────────────────────────────────
      "quick_actions_list" => {
        let r = automation_handlers::list_quick_actions();
        Ok(serde_json::to_value(r).unwrap_or(Value::Null))
      }
      "quick_actions_execute" => {
        let id = payload
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        automation_handlers::execute_quick_action(id)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }
      "automation_get_recipes" => automation_handlers::get_all_recipes()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "automation_execute_recipe" => {
        let id = payload
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        automation_handlers::execute_recipe(id)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }
      "automation_export_report" => automation_handlers::export_cleaning_report()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),

      // ── Schedule ───────────────────────────────────────────────────────────
      "schedule_list" => crud_handlers::find_all_schedules()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "schedule_add" => crud_handlers::insert_schedule(payload.clone())
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "schedule_edit" => {
        let id = payload
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        crud_handlers::update_schedule(id, payload.clone())
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "schedule_delete" => {
        let id = payload
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        crud_handlers::delete_schedule(id)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }

      // ── CRUD ───────────────────────────────────────────────────────────────
      "crud_find_cleaning_profile" => {
        let id = payload
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        crud_handlers::find_cleaning_profile(id)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "crud_find_all_cleaning_profiles" => crud_handlers::find_all_cleaning_profiles()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "crud_insert_cleaning_profile" => crud_handlers::insert_cleaning_profile(payload.clone())
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "crud_update_cleaning_profile" => {
        let id = payload
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        crud_handlers::update_cleaning_profile(id, payload.clone())
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "crud_patch_cleaning_profile" => {
        let id = payload
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        crud_handlers::patch_cleaning_profile(id, payload.clone())
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "crud_delete_cleaning_profile" => {
        let id = payload
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        crud_handlers::delete_cleaning_profile(id)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "crud_find_automation_recipe" => {
        let id = payload
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        crud_handlers::find_automation_recipe(id)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "crud_find_all_automation_recipes" => crud_handlers::find_all_automation_recipes()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "crud_insert_automation_recipe" => crud_handlers::insert_automation_recipe(payload.clone())
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "crud_update_automation_recipe" => {
        let id = payload
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        crud_handlers::update_automation_recipe(id, payload.clone())
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "crud_delete_automation_recipe" => {
        let id = payload
          .get("id")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        crud_handlers::delete_automation_recipe(id)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e))
      }
      "crud_find_cleaning_reports" => crud_handlers::find_cleaning_reports()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "crud_generate_cleaning_report" => automation_handlers::generate_cleaning_report()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e.to_string())),
      "crud_compare_snapshots" => {
        let id1 = payload
          .get("id1")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        let id2 = payload
          .get("id2")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        automation_handlers::compare_snapshots(id1, id2)
          .await
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }
      "crud_find_health_snapshots" => crud_handlers::find_all_health_snapshots()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),
      "crud_find_execution_history" => crud_handlers::find_execution_history()
        .await
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .map_err(|e| AppError::Internal(e)),

      // ── Fan info ───────────────────────────────────────────────────────────
      "fan_info" => {
        let info = get_thermal_info()?;
        serde_json::to_value(info).map_err(|e| AppError::Serialization(e.to_string()))
      }

      // ── Catch-all ──────────────────────────────────────────────────────────
      _ => Err(AppError::Internal(format!(
        "command not implemented: {}",
        name
      ))),
    }
  }

  pub fn invoke_ui_action(&self, action: &str, params: &Value) -> Result<Value, AppError> {
    match action {
      "navigate" => {
        let route = params.get("route").and_then(|v| v.as_str()).unwrap_or("/");
        self.state.set_navigation(route.to_string());
        Ok(serde_json::json!({ "navigated": route }))
      }
      "open_url" => {
        let url = params
          .get("url")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        let rt = tokio::runtime::Handle::current();
        rt.block_on(crate::application::handlers::opener_handlers::opener_open_url(url))
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }
      "open_folder" => {
        let path = params
          .get("path")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        let rt = tokio::runtime::Handle::current();
        rt.block_on(crate::application::handlers::opener_handlers::opener_open_folder(path))
          .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
          .map_err(|e| AppError::Internal(e.to_string()))
      }
      "show_notification" => {
        let message = params
          .get("message")
          .and_then(|v| v.as_str())
          .unwrap_or_default();
        self.state.push_notification(message.to_string());
        Ok(serde_json::json!({ "shown": true }))
      }
      "set_theme" => {
        let theme = params
          .get("theme")
          .and_then(|v| v.as_str())
          .unwrap_or("system");
        self.state.set_theme(theme.to_string());
        Ok(serde_json::json!({ "theme": theme }))
      }
      _ => Err(AppError::Internal(format!(
        "action not implemented: {}",
        action
      ))),
    }
  }
}

/// Free function wrapper so pages can call `bridge::invoke_app_command(...)`.
pub fn invoke_app_command(name: &str, payload: &Value) -> Result<Value, AppError> {
  let state = Arc::new(BridgeState::default());
  let bridge = CleanuxBridge::new(state);
  bridge.invoke_app_command(name, payload)
}

/// Start the MCP bridge and return bridge state + port.
pub fn start_mcp_bridge() -> (CleanuxBridge, Arc<BridgeState>) {
  let state = Arc::new(BridgeState::default());
  let bridge = CleanuxBridge::new(state.clone());
  (bridge, state)
}

/// Legacy bridge consumer loop for Cleanux's custom DOM/eval handling.
pub fn bridge_consumer_loop(state: Arc<BridgeState>) {
  loop {
    for (id, result) in state.dequeue_js_results() {
      let response = BridgeMsg {
        id,
        result: serde_json::json!(result),
        error: None,
      };
      state.enqueue_response(response);
    }
    for cmd in state.dequeue_all() {
      if matches!(cmd.method.as_str(), "evaluate_js" | "dom_snapshot") {
        state.enqueue_eval_request(EvalRequest {
          id: cmd.id,
          method: cmd.method.clone(),
          params: cmd.params.clone(),
        });
        continue;
      }
      let response = match cmd.method.as_str() {
        "ping" => BridgeMsg {
          id: cmd.id.clone(),
          result: serde_json::json!({ "pong": true }),
          error: None,
        },
        "app_info" => BridgeMsg {
          id: cmd.id.clone(),
          result: serde_json::json!({ "name": env!("CARGO_PKG_NAME"), "version": env!("CARGO_PKG_VERSION"), "platform": "dioxus-desktop" }),
          error: None,
        },
        "initialize" => BridgeMsg {
          id: cmd.id.clone(),
          result: serde_json::json!({ "protocolVersion": "2024-11-05", "capabilities": { "tools": true }, "serverInfo": { "name": env!("CARGO_PKG_NAME"), "version": env!("CARGO_PKG_VERSION") } }),
          error: None,
        },
        "health" => BridgeMsg {
          id: cmd.id.clone(),
          result: serde_json::json!({ "healthy": true }),
          error: None,
        },
        "commands_list" => BridgeMsg {
          id: cmd.id.clone(),
          result: serde_json::json!({ "commands": commands::invoke_app_commands() }),
          error: None,
        },
        "logs_read" => BridgeMsg {
          id: cmd.id.clone(),
          result: serde_json::json!({ "entries": state.get_logs() }),
          error: None,
        },
        "commands_invoke" => {
          let name = cmd
            .params
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("");
          let payload = cmd.params.get("payload").cloned().unwrap_or_default();
          match invoke_app_command(name, &payload) {
            Ok(v) => BridgeMsg {
              id: cmd.id.clone(),
              result: v,
              error: None,
            },
            Err(e) => BridgeMsg {
              id: cmd.id.clone(),
              result: serde_json::Value::Null,
              error: Some(e.to_string()),
            },
          }
        }
        "ui_invoke_action" => {
          let action = cmd
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
          let params = cmd.params.get("params").cloned().unwrap_or_default();
          match CleanuxBridge::new(state.clone()).invoke_ui_action(action, &params) {
            Ok(v) => BridgeMsg {
              id: cmd.id.clone(),
              result: v,
              error: None,
            },
            Err(e) => BridgeMsg {
              id: cmd.id.clone(),
              result: serde_json::Value::Null,
              error: Some(e.to_string()),
            },
          }
        }
        "evaluate_js" | "dom_snapshot" | "webview_screenshot" => {
          state.enqueue_eval_request(EvalRequest {
            id: cmd.id,
            method: cmd.method.clone(),
            params: cmd.params.clone(),
          });
          continue;
        }
        other => BridgeMsg {
          id: cmd.id.clone(),
          result: serde_json::Value::Null,
          error: Some(format!("unknown method: {}", other)),
        },
      };
      state.enqueue_response(response);
    }
    thread::sleep(Duration::from_millis(50));
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
  pub zone_type: String,
  pub temp: f64,
}

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
  let parsed: serde_json::Value = serde_json::from_str(&stdout)
    .map_err(|e| AppError::Internal(format!("failed to parse powerprofilesctl output: {}", e)))?;

  let current = parsed
    .get("active")
    .and_then(|v| v.get("profile"))
    .and_then(|v| v.as_str())
    .unwrap_or("performance")
    .to_string();

  let available: Vec<String> = parsed
    .get("profiles")
    .and_then(|v| v.as_array())
    .map(|arr| {
      arr
        .iter()
        .filter_map(|v| v.get("profile").and_then(|p| p.as_str()))
        .map(|s| s.to_string())
        .collect()
    })
    .unwrap_or_default();

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
    return Err(AppError::Internal(format!(
      "powerprofilesctl set failed: {}",
      stderr
    )));
  }
  Ok(())
}

/// Read thermal zones from /sys/class/thermal/ and return zone type + temp.
pub fn get_thermal_info() -> Result<Vec<ThermalInfo>, AppError> {
  let mut zones = Vec::new();
  let thermal_base = std::path::PathBuf::from("/sys/class/thermal");

  let entries = std::fs::read_dir(&thermal_base)
    .map_err(|e| AppError::Internal(format!("failed to read /sys/class/thermal: {}", e)))?;

  for entry in entries.filter_map(|e| e.ok()) {
    let path = entry.path();
    if path
      .file_name()
      .map(|s| s.to_string_lossy().starts_with("thermal_zone"))
      .unwrap_or(false)
    {
      let zone_type = std::fs::read_to_string(path.join("type"))
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
      let temp_str = std::fs::read_to_string(path.join("temp"))
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
      let temp: f64 = temp_str.parse().unwrap_or(0.0) as f64 / 1000.0;

      zones.push(ThermalInfo { zone_type, temp });
    }
  }
  Ok(zones)
}
