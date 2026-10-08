//! Infrastructure layer for Cleanux
//!
//! External integrations: MCP server
pub mod app_residue_service;
pub mod apt_service;
pub mod automation_service_impl;
pub mod backup_service;
pub mod cleaning_service_impl;
pub mod clipboard_service;
pub mod commands;
pub mod dev_cache_service;
pub mod disk_usage_service;
pub mod dnf_service;
pub mod docker_service;
pub mod downloads_cleaner_service;
pub mod duplicate_finder_service;
pub mod duplicate_scanner;
pub mod health_history_service;
pub mod health_service_impl;
pub mod history;
pub mod journal_service;
pub mod json_storage;
pub mod junk_cleaner_service;
pub mod large_file_cleaner_service;
pub mod log_cleaner_service;
pub mod mcp;
pub mod media_cache_service;
pub mod memory_service;
pub mod monitor_service;
pub mod monitoring_service;
pub mod package_managers;
pub mod package_service;
pub mod pacman_service;
pub mod process_service;
pub mod profile_service;
pub mod repair_service_impl;
pub mod report_service;
pub mod routine_service;
pub mod scanners;
pub mod scheduler_service;
pub mod services;
pub mod settings;
pub mod startup_service;
pub mod sys_utils;
pub mod system_repair_service;
pub mod temperature_service;
pub mod trash_cleaner_service;
pub mod zypper_service;

// Re-export for convenience
#[allow(ambiguous_glob_reexports)]
pub use cleaning_service_impl::*;
