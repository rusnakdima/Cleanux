/* imports */
// pub mod commands; // removed - moved to routes/
pub mod entities;
pub mod errors;
pub mod helpers;
pub mod logger;
pub mod models;
pub mod routes;
pub mod security;
pub mod services;

use crate::entities::automation_recipe_entity::AutomationRecipeEntity;
use crate::entities::cleaning_profile_entity::CleaningProfileEntity;
use crate::entities::execution_history_entity::ExecutionHistoryEntity;
use crate::entities::health_snapshot_entity::HealthSnapshotEntity;
use nosql_orm::relations::register_relations_for_entity;
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  logger::init_logger();

  tauri::Builder::default()
    .setup(|app| {
      let app_data_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");
      std::fs::create_dir_all(&app_data_dir).ok();

      register_relations_for_entity::<CleaningProfileEntity>();
      register_relations_for_entity::<AutomationRecipeEntity>();
      register_relations_for_entity::<ExecutionHistoryEntity>();
      register_relations_for_entity::<HealthSnapshotEntity>();

      let json_provider =
        tauri::async_runtime::block_on(nosql_orm::providers::JsonProvider::new(&app_data_dir))
          .expect("Failed to create JSON provider");

      let repository_service = Arc::new(services::RepositoryService::new(json_provider));

      app.manage(AppState {
        data: DataState { repository_service },
      });

      Ok(())
    })
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      routes::health_command::crud_get_health_snapshot,
      routes::health_command::crud_get_health_snapshots,
      routes::health_command::crud_create_health_snapshot,
      routes::health_command::crud_get_health_history,
      routes::health_command::crud_get_health_trends,
      routes::health_command::crud_save_health_snapshot,
      routes::report_command::get_cleaning_report,
      routes::report_command::get_cleaning_reports,
      routes::report_command::create_cleaning_report,
      routes::report_command::crud_generate_cleaning_report,
      routes::report_command::crud_get_cleaning_history,
      routes::report_command::crud_compare_snapshots,
      routes::profile_command::get_cleaning_profile,
      routes::profile_command::get_cleaning_profiles,
      routes::profile_command::create_cleaning_profile,
      routes::profile_command::update_cleaning_profile,
      routes::profile_command::delete_cleaning_profile,
      routes::profile_command::apply_cleaning_profile,
      routes::automation_command::get_automation_recipe,
      routes::automation_command::get_automation_recipes,
      routes::automation_command::create_automation_recipe,
      routes::automation_command::update_automation_recipe,
      routes::automation_command::delete_automation_recipe,
      routes::automation_command::crud_get_execution_history,
      routes::automation_command::crud_get_quick_actions,
      routes::automation_command::crud_execute_action,
      routes::automation_command::crud_execute_recipe,
      routes::dashboard_command::get_system_services,
      routes::dashboard_command::get_cache_summary,
      routes::dashboard_command::get_trash_summary,
      routes::dashboard_command::get_log_summary,
      routes::dashboard_command::get_large_files_summary,
      routes::system_command::get_memory_info,
      routes::system_command::get_swap_info,
      routes::system_command::get_process_memory,
      routes::system_command::optimize_memory,
      routes::system_command::get_processes,
      routes::system_command::kill_process,
      routes::system_command::kill_selected_processes,
      routes::system_command::get_current_kernel,
      routes::system_command::get_installed_kernels,
      routes::system_command::get_old_kernels,
      routes::system_command::get_old_kernels_size,
      routes::system_command::remove_kernel,
      routes::system_command::get_old_initramfs,
      routes::system_command::remove_initramfs,
      routes::system_command::get_boot_space_info,
      routes::system_command::update_grub,
      routes::system_command::get_battery_info,
      routes::system_command::get_power_profiles,
      routes::system_command::set_power_profile,
      routes::system_command::get_thermal_info,
      routes::system_command::stop_service,
      routes::system_command::stop_selected_services,
      routes::system_command::open_file,
      routes::system_command::get_all_services,
      routes::system_command::enable_service,
      routes::system_command::start_service,
      routes::system_command::enable_selected_services,
      routes::storage_command::create_backup,
      routes::storage_command::restore_backup,
      routes::storage_command::list_backups,
      routes::storage_command::delete_backup,
      routes::storage_command::get_backup_dir,
      routes::storage_command::scan_directory,
      routes::storage_command::get_directory_size,
      routes::storage_command::find_empty_directories,
      routes::storage_command::find_nested_empty_directories,
      routes::storage_command::remove_empty_directory,
      routes::storage_command::remove_empty_directories,
      routes::storage_command::find_duplicates,
      routes::storage_command::get_junk_summary,
      routes::storage_command::scan_browser_caches,
      routes::storage_command::scan_thumbnail_caches,
      routes::storage_command::scan_application_caches,
      routes::storage_command::scan_system_temp,
      routes::storage_command::scan_log_rotations,
      routes::storage_command::clean_junk_category,
      routes::storage_command::get_media_cache_summary,
      routes::storage_command::clean_steam_shader_cache,
      routes::storage_command::clean_steam_download_cache,
      routes::storage_command::clean_spotify_cache,
      routes::storage_command::clean_vlc_cache,
      routes::storage_command::clean_thumbnail_cache,
      routes::storage_command::clean_media_icon_cache,
      routes::storage_command::get_dev_cache_summary,
      routes::storage_command::clean_npm_cache,
      routes::storage_command::clean_pip_cache,
      routes::storage_command::clean_cargo_cache,
      routes::storage_command::clean_go_cache,
      routes::storage_command::clean_maven_cache,
      routes::storage_command::clean_gradle_cache,
      routes::storage_command::clean_all_dev_caches,
      routes::monitor_command::get_temperatures,
      routes::monitor_command::get_cpu_temperature,
      routes::monitor_command::get_gpu_temperature,
      routes::monitor_command::get_system_stats,
      routes::monitor_command::start_monitoring,
      routes::monitor_command::stop_monitoring,
      routes::cleaner_command::find_broken_symlinks,
      routes::cleaner_command::find_orphaned_packages,
      routes::cleaner_command::clean_font_cache,
      routes::cleaner_command::clean_repair_icon_cache,
      routes::cleaner_command::repair_permissions,
      routes::cleaner_command::remove_broken_symlink,
      routes::cleaner_command::clean_repair_orphaned_pkg,
      routes::cleaner_command::get_startup_items,
      routes::cleaner_command::disable_startup_item,
      routes::cleaner_command::enable_startup_item,
      routes::cleaner_command::get_container_summary,
      routes::cleaner_command::docker_system_prune,
      routes::cleaner_command::docker_image_prune,
      routes::cleaner_command::docker_container_prune,
      routes::cleaner_command::docker_volume_prune,
      routes::cleaner_command::docker_preview_prune,
      routes::cleaner_command::podman_system_prune,
      routes::cleaner_command::podman_image_prune,
      routes::cleaner_command::get_quick_actions,
      routes::cleaner_command::execute_action,
      routes::cleaner_command::get_recipes,
      routes::cleaner_command::save_recipe,
      routes::cleaner_command::delete_recipe,
      routes::cleaner_command::execute_recipe,
      routes::cleaner_command::get_execution_history,
      routes::cleaner_command::get_residue_summary,
      routes::cleaner_command::scan_user_configs,
      routes::cleaner_command::scan_user_data,
      routes::cleaner_command::scan_user_caches,
      routes::cleaner_command::scan_home_residues,
      routes::cleaner_command::get_orphaned_configs,
      routes::cleaner_command::clean_app_residue,
      routes::cleaner_command::clean_multiple_app_residues,
      routes::package_command::get_package_cache_info,
      routes::package_command::clean_package_cache,
      routes::log_command::get_journal_size,
      routes::log_command::get_journal_usage,
      routes::log_command::vacuum_journal,
      routes::log_command::vacuum_journal_by_days,
      routes::log_command::get_rotated_logs_size,
      routes::log_command::get_rotated_logs,
      routes::log_command::clean_rotated_logs,
      routes::log_command::get_logrotate_configs,
      routes::log_command::analyze_logrotate,
      routes::log_command::get_var_log_usage,
      routes::log_command::get_largest_log_files,
      routes::log_command::get_log_manager_summary,
    ])
    .run(tauri::generate_context!())
    .unwrap_or_else(|e| {
      log::error!("error while running tauri application: {}", e);
      std::process::exit(1);
    });
}

pub struct AppState {
  pub data: DataState,
}

pub struct DataState {
  pub repository_service: Arc<services::RepositoryService>,
}
