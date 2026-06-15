/* imports */
pub mod commands;
pub mod errors;
pub mod helpers;
pub mod models;
pub mod security;
pub mod services;

use tauri::Manager;
use tauri_logger::{init_file_logger, log_error, log_warn, FileLogger, LogFileInfo};

use commands::{
  cleaner_command, dashboard_command, log_command, monitor_command, package_command,
  profile_command, report_command, storage_command, system_command,
};

#[tauri::command]
async fn write_log_to_file(
  state: tauri::State<'_, AppState>,
  level: String,
  message: String,
  source: String,
) -> Result<(), String> {
  state.file_logger.write_log(&level, &source, &message)
}

#[tauri::command]
async fn get_log_file_info(state: tauri::State<'_, AppState>) -> Result<LogFileInfo, String> {
  let path = state
    .file_logger
    .get_log_file_path()
    .ok_or("Log file not initialized")?;

  let metadata =
    std::fs::metadata(&path).map_err(|e| format!("Failed to get log file metadata: {}", e))?;

  let created_at = metadata
    .created()
    .ok()
    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
    .map(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, 0))
    .flatten()
    .map(|dt| dt.to_rfc3339())
    .unwrap_or_default();

  Ok(LogFileInfo {
    path: path.to_string_lossy().to_string(),
    name: path
      .file_name()
      .map(|n| n.to_string_lossy().to_string())
      .unwrap_or_default(),
    size: metadata.len(),
    created_at,
  })
}

pub struct AppState {
  pub file_logger: FileLogger,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
      let file_logger = init_file_logger(&app.handle()).unwrap_or_else(|e| {
        log_warn!("Failed to initialize file logger: {}", e);
        FileLogger::new()
      });
      app.manage(AppState { file_logger });
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      dashboard_command::get_system_services,
      dashboard_command::get_cache_summary,
      dashboard_command::get_trash_summary,
      dashboard_command::get_log_summary,
      dashboard_command::get_large_files_summary,
      system_command::get_memory_info,
      system_command::get_swap_info,
      system_command::get_process_memory,
      system_command::optimize_memory,
      system_command::get_processes,
      system_command::kill_process,
      system_command::kill_selected_processes,
      system_command::get_current_kernel,
      system_command::get_installed_kernels,
      system_command::get_old_kernels,
      system_command::get_old_kernels_size,
      system_command::remove_kernel,
      system_command::get_old_initramfs,
      system_command::remove_initramfs,
      system_command::get_boot_space_info,
      system_command::update_grub,
      system_command::get_battery_info,
      system_command::get_power_profiles,
      system_command::set_power_profile,
      system_command::get_thermal_info,
      system_command::stop_service,
      system_command::stop_selected_services,
      system_command::open_file,
      system_command::get_all_services,
      system_command::enable_service,
      system_command::start_service,
      system_command::enable_selected_services,
      storage_command::create_backup,
      storage_command::restore_backup,
      storage_command::list_backups,
      storage_command::delete_backup,
      storage_command::get_backup_dir,
      storage_command::scan_directory,
      storage_command::get_directory_size,
      storage_command::find_empty_directories,
      storage_command::find_nested_empty_directories,
      storage_command::remove_empty_directory,
      storage_command::remove_empty_directories,
      storage_command::find_duplicates,
      storage_command::get_junk_summary,
      storage_command::scan_browser_caches,
      storage_command::scan_thumbnail_caches,
      storage_command::scan_application_caches,
      storage_command::scan_system_temp,
      storage_command::scan_log_rotations,
      storage_command::clean_junk_category,
      storage_command::get_media_cache_summary,
      storage_command::clean_steam_shader_cache,
      storage_command::clean_steam_download_cache,
      storage_command::clean_spotify_cache,
      storage_command::clean_vlc_cache,
      storage_command::clean_thumbnail_cache,
      storage_command::clean_media_icon_cache,
      storage_command::get_dev_cache_summary,
      storage_command::clean_npm_cache,
      storage_command::clean_pip_cache,
      storage_command::clean_cargo_cache,
      storage_command::clean_go_cache,
      storage_command::clean_maven_cache,
      storage_command::clean_gradle_cache,
      storage_command::clean_all_dev_caches,
      monitor_command::get_temperatures,
      monitor_command::get_cpu_temperature,
      monitor_command::get_gpu_temperature,
      monitor_command::save_health_snapshot,
      monitor_command::get_health_history,
      monitor_command::get_health_trends,
      monitor_command::get_system_stats,
      monitor_command::start_monitoring,
      monitor_command::stop_monitoring,
      cleaner_command::find_broken_symlinks,
      cleaner_command::find_orphaned_packages,
      cleaner_command::clean_font_cache,
      cleaner_command::clean_repair_icon_cache,
      cleaner_command::repair_permissions,
      cleaner_command::remove_broken_symlink,
      cleaner_command::clean_repair_orphaned_pkg,
      cleaner_command::get_startup_items,
      cleaner_command::disable_startup_item,
      cleaner_command::enable_startup_item,
      cleaner_command::get_container_summary,
      cleaner_command::docker_system_prune,
      cleaner_command::docker_image_prune,
      cleaner_command::docker_container_prune,
      cleaner_command::docker_volume_prune,
      cleaner_command::docker_preview_prune,
      cleaner_command::podman_system_prune,
      cleaner_command::podman_image_prune,
      cleaner_command::get_quick_actions,
      cleaner_command::execute_action,
      cleaner_command::get_recipes,
      cleaner_command::save_recipe,
      cleaner_command::delete_recipe,
      cleaner_command::execute_recipe,
      cleaner_command::get_execution_history,
      cleaner_command::get_residue_summary,
      cleaner_command::scan_user_configs,
      cleaner_command::scan_user_data,
      cleaner_command::scan_user_caches,
      cleaner_command::scan_home_residues,
      cleaner_command::get_orphaned_configs,
      cleaner_command::clean_app_residue,
      cleaner_command::clean_multiple_app_residues,
      profile_command::save_profile,
      profile_command::load_profile,
      profile_command::list_profiles,
      profile_command::delete_profile,
      profile_command::apply_profile,
      package_command::get_package_cache_info,
      package_command::clean_package_cache,
      log_command::get_journal_size,
      log_command::get_journal_usage,
      log_command::vacuum_journal,
      log_command::vacuum_journal_by_days,
      log_command::get_rotated_logs_size,
      log_command::get_rotated_logs,
      log_command::clean_rotated_logs,
      log_command::get_logrotate_configs,
      log_command::analyze_logrotate,
      log_command::get_var_log_usage,
      log_command::get_largest_log_files,
      log_command::get_log_manager_summary,
      report_command::generate_cleaning_report,
      report_command::get_cleaning_history,
      report_command::export_to_html,
      report_command::compare_snapshots,
      write_log_to_file,
      get_log_file_info,
    ])
    .run(tauri::generate_context!())
    .unwrap_or_else(|e| {
      log_error!("error while running tauri application: {}", e);
      std::process::exit(1);
    });
}
