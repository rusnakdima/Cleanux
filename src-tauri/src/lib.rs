/* imports */
// pub mod commands; // removed - moved to routes/
pub mod commands;
pub mod entities;
pub mod errors;
pub mod models;
pub mod repositories;
pub mod security;
pub mod services;
pub mod utils;
use crate::entities::automation_recipe_entity::AutomationRecipeEntity;
use crate::entities::cleaning_profile_entity::CleaningProfileEntity;
use crate::entities::execution_history_entity::ExecutionHistoryEntity;
use crate::entities::health_snapshot_entity::HealthSnapshotEntity;
use nosql_orm::relations::register_relations_for_entity;
use std::sync::Arc;
use tauri::Manager;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
      let repository_service = Arc::new(repositories::service::RepositoryService::new(
        json_provider.clone(),
      ));
      let crud_service = Arc::new(services::crud_service::CrudService::new(json_provider));
      app.manage(AppState {
        data: DataState {
          repository_service,
          crud_service,
        },
      });
      Ok(())
    })
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_mcp_bridge::init())
    .invoke_handler(tauri::generate_handler![
      commands::crud_command::crud_execute,
      commands::health_command::crud_get_health_snapshot,
      commands::health_command::crud_get_health_snapshots,
      commands::health_command::crud_create_health_snapshot,
      commands::health_command::crud_get_health_history,
      commands::health_command::crud_get_health_trends,
      commands::health_command::crud_save_health_snapshot,
      commands::report_command::get_cleaning_report,
      commands::report_command::get_cleaning_reports,
      commands::report_command::create_cleaning_report,
      commands::report_command::crud_generate_cleaning_report,
      commands::report_command::crud_get_cleaning_history,
      commands::report_command::crud_compare_snapshots,
      commands::profile_command::get_cleaning_profile,
      commands::profile_command::get_cleaning_profiles,
      commands::profile_command::create_cleaning_profile,
      commands::profile_command::update_cleaning_profile,
      commands::profile_command::delete_cleaning_profile,
      commands::profile_command::apply_cleaning_profile,
      commands::automation_command::get_automation_recipe,
      commands::automation_command::get_automation_recipes,
      commands::automation_command::create_automation_recipe,
      commands::automation_command::update_automation_recipe,
      commands::automation_command::delete_automation_recipe,
      commands::automation_command::crud_get_execution_history,
      commands::automation_command::crud_get_quick_actions,
      commands::automation_command::crud_execute_action,
      commands::automation_command::crud_execute_recipe,
      commands::dashboard_command::get_system_services,
      commands::dashboard_command::get_cache_summary,
      commands::dashboard_command::get_trash_summary,
      commands::dashboard_command::get_log_summary,
      commands::dashboard_command::get_large_files_summary,
      commands::system_command::get_memory_info,
      commands::system_command::get_swap_info,
      commands::system_command::get_process_memory,
      commands::system_command::optimize_memory,
      commands::system_command::get_processes,
      commands::system_command::kill_process,
      commands::system_command::kill_selected_processes,
      commands::system_command::get_current_kernel,
      commands::system_command::get_installed_kernels,
      commands::system_command::get_old_kernels,
      commands::system_command::get_old_kernels_size,
      commands::system_command::remove_kernel,
      commands::system_command::get_old_initramfs,
      commands::system_command::remove_initramfs,
      commands::system_command::get_boot_space_info,
      commands::system_command::update_grub,
      commands::system_command::get_battery_info,
      commands::system_command::get_power_profiles,
      commands::system_command::set_power_profile,
      commands::system_command::get_thermal_info,
      commands::system_command::stop_service,
      commands::system_command::stop_selected_services,
      commands::system_command::open_file,
      commands::system_command::get_all_services,
      commands::system_command::enable_service,
      commands::system_command::start_service,
      commands::system_command::enable_selected_services,
      commands::storage_command::create_backup,
      commands::storage_command::restore_backup,
      commands::storage_command::list_backups,
      commands::storage_command::delete_backup,
      commands::storage_command::get_backup_dir,
      commands::storage_command::scan_directory,
      commands::storage_command::get_directory_size,
      commands::storage_command::find_empty_directories,
      commands::storage_command::find_nested_empty_directories,
      commands::storage_command::remove_empty_directory,
      commands::storage_command::remove_empty_directories,
      commands::storage_command::find_duplicates,
      commands::storage_command::get_junk_summary,
      commands::storage_command::scan_browser_caches,
      commands::storage_command::scan_thumbnail_caches,
      commands::storage_command::scan_application_caches,
      commands::storage_command::scan_system_temp,
      commands::storage_command::scan_log_rotations,
      commands::storage_command::clean_junk_category,
      commands::storage_command::get_media_cache_summary,
      commands::storage_command::clean_steam_shader_cache,
      commands::storage_command::clean_steam_download_cache,
      commands::storage_command::clean_spotify_cache,
      commands::storage_command::clean_vlc_cache,
      commands::storage_command::clean_thumbnail_cache,
      commands::storage_command::clean_media_icon_cache,
      commands::storage_command::get_dev_cache_summary,
      commands::storage_command::clean_npm_cache,
      commands::storage_command::clean_pip_cache,
      commands::storage_command::clean_cargo_cache,
      commands::storage_command::clean_go_cache,
      commands::storage_command::clean_maven_cache,
      commands::storage_command::clean_gradle_cache,
      commands::storage_command::clean_all_dev_caches,
      commands::monitor_command::get_temperatures,
      commands::monitor_command::get_cpu_temperature,
      commands::monitor_command::get_gpu_temperature,
      commands::monitor_command::get_system_stats,
      commands::monitor_command::start_monitoring,
      commands::monitor_command::stop_monitoring,
      commands::cleaner_command::find_broken_symlinks,
      commands::cleaner_command::find_orphaned_packages,
      commands::cleaner_command::clean_font_cache,
      commands::cleaner_command::clean_repair_icon_cache,
      commands::cleaner_command::repair_permissions,
      commands::cleaner_command::remove_broken_symlink,
      commands::cleaner_command::clean_repair_orphaned_pkg,
      commands::cleaner_command::get_startup_items,
      commands::cleaner_command::disable_startup_item,
      commands::cleaner_command::enable_startup_item,
      commands::cleaner_command::get_container_summary,
      commands::cleaner_command::docker_system_prune,
      commands::cleaner_command::docker_image_prune,
      commands::cleaner_command::docker_container_prune,
      commands::cleaner_command::docker_volume_prune,
      commands::cleaner_command::docker_preview_prune,
      commands::cleaner_command::podman_system_prune,
      commands::cleaner_command::podman_image_prune,
      commands::cleaner_command::get_quick_actions,
      commands::cleaner_command::execute_action,
      commands::cleaner_command::get_recipes,
      commands::cleaner_command::save_recipe,
      commands::cleaner_command::delete_recipe,
      commands::cleaner_command::execute_recipe,
      commands::cleaner_command::get_execution_history,
      commands::cleaner_command::get_residue_summary,
      commands::cleaner_command::scan_user_configs,
      commands::cleaner_command::scan_user_data,
      commands::cleaner_command::scan_user_caches,
      commands::cleaner_command::scan_home_residues,
      commands::cleaner_command::get_orphaned_configs,
      commands::cleaner_command::clean_app_residue,
      commands::cleaner_command::clean_multiple_app_residues,
      commands::package_command::get_package_cache_info,
      commands::package_command::clean_package_cache,
      commands::log_command::get_journal_size,
      commands::log_command::get_journal_usage,
      commands::log_command::vacuum_journal,
      commands::log_command::vacuum_journal_by_days,
      commands::log_command::get_rotated_logs_size,
      commands::log_command::get_rotated_logs,
      commands::log_command::clean_rotated_logs,
      commands::log_command::get_logrotate_configs,
      commands::log_command::analyze_logrotate,
      commands::log_command::get_var_log_usage,
      commands::log_command::get_largest_log_files,
      commands::log_command::get_log_manager_summary,
    ])
    .run(tauri::generate_context!())
    .unwrap_or_else(|e| {
      std::process::exit(1);
    });
}
pub struct AppState {
  pub data: DataState,
}
pub struct DataState {
  pub repository_service: Arc<repositories::service::RepositoryService>,
  pub crud_service: Arc<services::crud_service::CrudService>,
}
