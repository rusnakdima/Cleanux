/* services module */

#[path = "automation.service.rs"]
pub mod automation_service;

#[path = "backup.service.rs"]
pub mod backup_service;

#[path = "cache_cleaning.service.rs"]
pub mod cache_cleaning_service;

#[path = "dashboard.service.rs"]
pub mod dashboard_service;

#[path = "directory.service.rs"]
pub mod directory_service;

#[path = "file_preview.service.rs"]
pub mod file_preview_service;

#[path = "health_history.service.rs"]
pub mod health_history_service;

#[path = "large_file_cleaning.service.rs"]
pub mod large_file_cleaning_service;

#[path = "log_cleaning.service.rs"]
pub mod log_cleaning_service;

#[path = "media_cache.service.rs"]
pub mod media_cache_service;

#[path = "memory.service.rs"]
pub mod memory_service;

#[path = "monitor.service.rs"]
pub mod monitor_service;

#[path = "package.service.rs"]
pub mod package_service;

#[path = "process.service.rs"]
pub mod process_service;

#[path = "profile.service.rs"]
pub mod profile_service;

#[path = "repair.service.rs"]
pub mod repair_service;

#[path = "scanner.service.rs"]
pub mod scanner_service;

#[path = "startup.service.rs"]
pub mod startup_service;

#[path = "system.service.rs"]
pub mod system_service;

#[path = "power.service.rs"]
pub mod power_service;

#[path = "temperature.service.rs"]
pub mod temperature_service;

#[path = "report.service.rs"]
pub mod report_service;

#[path = "container.service.rs"]
pub mod container_service;

#[path = "trash_cleaning.service.rs"]
pub mod trash_cleaning_service;

#[path = "junk_cleaner.service.rs"]
pub mod junk_cleaner_service;

#[path = "app_residue.service.rs"]
pub mod app_residue_service;

#[path = "log_manager.service.rs"]
pub mod log_manager_service;

#[path = "kernel_cleaner.service.rs"]
pub mod kernel_cleaner_service;

#[path = "dev_cache.service.rs"]
pub mod dev_cache_service;
