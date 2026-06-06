/* imports */
pub mod errors;
pub mod helpers;
pub mod models;
pub mod routes;
pub mod security;
pub mod services;

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
  scheduler_route::{
    delete_schedule_config, get_schedule_config, run_cleaning_now, save_schedule_config,
  },
  startup_route::{disable_startup_item, enable_startup_item, get_startup_items},
  system_route::{
    enableSelectedServices, enableService, getAllServices, openFile, startService,
    stopSelectedServices, stopService,
  },
  temperature_route::{get_cpu_temperature, get_gpu_temperature, get_temperatures},
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[allow(non_snake_case)]
pub fn run() {
  #[cfg(target_os = "linux")]
  {
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING", "1");
    std::env::set_var("WEBKIT_FORCE_SANDBOX", "0");
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
  }

  #[allow(unused_mut)]
  let mut builder = tauri::Builder::default();

  #[cfg(all(feature = "mcp-bridge", debug_assertions))]
  {
    let bridge_addr =
      std::env::var("CLEANUX_MCP_BRIDGE_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string());
    let bridge_port: u16 = std::env::var("CLEANUX_MCP_BRIDGE_PORT")
      .ok()
      .and_then(|v| v.parse().ok())
      .unwrap_or(9223);

    let bridge_plugin = tauri_plugin_mcp_bridge::Builder::new()
      .bind_address(&bridge_addr)
      .base_port(bridge_port)
      .build();

    builder = builder.plugin(bridge_plugin);
  }

  builder
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_opener::init())
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
      get_schedule_config,
      save_schedule_config,
      delete_schedule_config,
      run_cleaning_now,
    ])
    .run(tauri::generate_context!())
    .unwrap_or_else(|e| {
      eprintln!("error while running tauri application: {}", e);
      std::process::exit(1);
    });
}
