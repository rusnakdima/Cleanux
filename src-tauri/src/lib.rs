/* imports */
pub mod errors;
pub mod helpers;
pub mod models;
pub mod routes;
pub mod security;
pub mod services;

use tauri::Manager;
use tauri_logger::{init_file_logger, log_error, log_warn, FileLogger, LogFileInfo};

/* routes */
use routes::{
  app_residue_route::{
    clean_app_residue, clean_multiple_app_residues, get_orphaned_configs, get_residue_summary,
    scan_home_residues, scan_user_caches, scan_user_configs, scan_user_data,
  },
  automation_route::{
    delete_recipe, execute_action, execute_recipe, get_execution_history, get_quick_actions,
    get_recipes, save_recipe,
  },
  backup_route::{create_backup, delete_backup, get_backup_dir, list_backups, restore_backup},
  container_route::{
    docker_container_prune, docker_image_prune, docker_preview_prune, docker_system_prune,
    docker_volume_prune, get_container_summary, podman_image_prune, podman_system_prune,
  },
  dashboard_route::{
    getCacheSummary, getLargeFilesSummary, getLogSummary, getSystemServices, getTrashSummary,
  },
  dev_cache_route::{
    clean_all_dev_caches, clean_cargo_cache, clean_go_cache, clean_gradle_cache, clean_maven_cache,
    clean_npm_cache, clean_pip_cache, get_dev_cache_summary,
  },
  directory_route::{
    find_empty_directories, find_nested_empty_directories, get_directory_size,
    remove_empty_directories, scan_directory,
  },
  duplicates_route::find_duplicates,
  health_history_route::{get_health_history, get_health_trends, save_health_snapshot},
  junk_cleaner_route::{
    clean_junk_category, get_junk_summary, scan_application_caches, scan_browser_caches,
    scan_log_rotations, scan_system_temp, scan_thumbnail_caches,
  },
  kernel_cleaner_route::{
    get_boot_space_info, get_current_kernel, get_installed_kernels, get_old_initramfs,
    get_old_kernels, get_old_kernels_size, remove_initramfs, remove_kernel, update_grub,
  },
  log_manager_route::{
    analyze_logrotate, clean_rotated_logs, get_journal_size, get_journal_usage,
    get_largest_log_files, get_log_manager_summary, get_logrotate_configs, get_rotated_logs,
    get_rotated_logs_size, get_var_log_usage, vacuum_journal, vacuum_journal_by_days,
  },
  media_cache_route::{
    clean_media_icon_cache, clean_spotify_cache, clean_steam_download_cache,
    clean_steam_shader_cache, clean_thumbnail_cache, clean_vlc_cache, get_media_cache_summary,
  },
  memory_route::{get_memory_info, get_process_memory, get_swap_info, optimize_memory},
  monitor_route::{get_system_stats, start_monitoring, stop_monitoring},
  package_route::{
    apt_autoclean, apt_autoremove, apt_clean, clean_package_cache, deep_clean_all,
    deep_clean_remove_orphaned_package, dnf_clean_all, get_apt_cache_size, get_dnf_cache_size,
    get_orphaned_packages, get_package_cache_info, get_package_summary, get_pacman_cache_size,
    get_partial_downloads, get_zypper_cache_size, pacman_clean, pacman_full_clean, zypper_clean,
  },
  power_route::{get_battery_info, get_power_profiles, get_thermal_info, set_power_profile},
  process_route::{getProcesses, killProcess, killSelectedProcesses},
  profile_route::{apply_profile, delete_profile, list_profiles, load_profile, save_profile},
  repair_route::{
    clean_font_cache, clean_repair_icon_cache, clean_repair_orphaned_pkg, find_broken_symlinks,
    find_orphaned_packages, remove_broken_symlink, repair_permissions,
  },
  report_route::{
    compare_snapshots, export_to_html, generate_cleaning_report, get_cleaning_history,
  },
  startup_route::{disable_startup_item, enable_startup_item, get_startup_items},
  system_route::{
    enableSelectedServices, enableService, getAllServices, openFile, startService,
    stopSelectedServices, stopService,
  },
  temperature_route::{get_cpu_temperature, get_gpu_temperature, get_temperatures},
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
#[allow(non_snake_case)]
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
      getSystemServices,
      getCacheSummary,
      getTrashSummary,
      getLogSummary,
      getLargeFilesSummary,
      stopSelectedServices,
      stopService,
      getAllServices,
      enableService,
      startService,
      enableSelectedServices,
      openFile,
      get_system_stats,
      start_monitoring,
      stop_monitoring,
      get_memory_info,
      get_swap_info,
      get_process_memory,
      optimize_memory,
      getProcesses,
      killProcess,
      killSelectedProcesses,
      find_duplicates,
      create_backup,
      restore_backup,
      list_backups,
      delete_backup,
      get_backup_dir,
      get_startup_items,
      disable_startup_item,
      enable_startup_item,
      save_profile,
      load_profile,
      list_profiles,
      delete_profile,
      apply_profile,
      scan_directory,
      get_directory_size,
      find_empty_directories,
      find_nested_empty_directories,
      remove_empty_directories,
      find_broken_symlinks,
      find_orphaned_packages,
      clean_font_cache,
      clean_repair_icon_cache,
      repair_permissions,
      remove_broken_symlink,
      clean_repair_orphaned_pkg,
      get_junk_summary,
      scan_browser_caches,
      scan_thumbnail_caches,
      scan_application_caches,
      scan_system_temp,
      scan_log_rotations,
      clean_junk_category,
      get_residue_summary,
      scan_user_configs,
      scan_user_data,
      scan_user_caches,
      scan_home_residues,
      get_orphaned_configs,
      clean_app_residue,
      clean_multiple_app_residues,
      get_quick_actions,
      execute_action,
      get_recipes,
      save_recipe,
      delete_recipe,
      execute_recipe,
      get_execution_history,
      get_battery_info,
      get_power_profiles,
      set_power_profile,
      get_thermal_info,
      generate_cleaning_report,
      get_cleaning_history,
      export_to_html,
      compare_snapshots,
      get_container_summary,
      docker_system_prune,
      docker_image_prune,
      docker_container_prune,
      docker_volume_prune,
      docker_preview_prune,
      podman_system_prune,
      podman_image_prune,
      get_current_kernel,
      get_installed_kernels,
      get_old_kernels,
      get_old_kernels_size,
      remove_kernel,
      get_old_initramfs,
      remove_initramfs,
      get_boot_space_info,
      update_grub,
      get_journal_size,
      get_journal_usage,
      vacuum_journal,
      vacuum_journal_by_days,
      get_rotated_logs_size,
      get_rotated_logs,
      clean_rotated_logs,
      get_logrotate_configs,
      analyze_logrotate,
      get_var_log_usage,
      get_largest_log_files,
      get_log_manager_summary,
      get_package_summary,
      deep_clean_all,
      get_apt_cache_size,
      apt_clean,
      apt_autoremove,
      apt_autoclean,
      get_orphaned_packages,
      deep_clean_remove_orphaned_package,
      get_partial_downloads,
      get_dnf_cache_size,
      dnf_clean_all,
      get_pacman_cache_size,
      pacman_clean,
      pacman_full_clean,
      get_zypper_cache_size,
      zypper_clean,
      get_media_cache_summary,
      clean_steam_shader_cache,
      clean_steam_download_cache,
      clean_spotify_cache,
      clean_vlc_cache,
      clean_thumbnail_cache,
      clean_media_icon_cache,
      get_dev_cache_summary,
      clean_npm_cache,
      clean_pip_cache,
      clean_cargo_cache,
      clean_go_cache,
      clean_maven_cache,
      clean_gradle_cache,
      clean_all_dev_caches,
      get_health_history,
      get_health_trends,
      save_health_snapshot,
      get_temperatures,
      get_cpu_temperature,
      get_gpu_temperature,
      clean_package_cache,
      get_package_cache_info,
      write_log_to_file,
      get_log_file_info,
    ])
    .run(tauri::generate_context!())
    .unwrap_or_else(|e| {
      log_error!("error while running tauri application: {}", e);
      std::process::exit(1);
    });
}
