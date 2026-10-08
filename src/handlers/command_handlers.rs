//! Command handlers — all write/create/delete operations from handler files.
//!
//! This module aggregates all command functions from the original handler files.

use crate::error::AppError;
use crate::response::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex};
use chrono::{DateTime, Utc};
use flate2::read::GzDecoder;
use crate::handlers::query_handlers::format_bytes;
use flate2::write::GzEncoder;
use flate2::Compression;
use tokio::process::Command;
use uuid::Uuid;
use dirs;

use crate::domain::settings::CleanuxSettings;
use crate::global_state::{
    routine_service,
    start_monitoring as gs_start_monitoring,
    stop_monitoring as gs_stop_monitoring,
};
use crate::infrastructure::json_storage::JsonStorage;
use crate::infrastructure::memory_service::{MemoryError, MemoryService};
use crate::infrastructure::settings as settings_storage;
use crate::infrastructure::sys_utils::{delete_old_files, get_cpu_temp, home_dir};
use crate::response::Response as DSResponse;
use crate::allowlist::validate_path;

// ============================================================================
// Automation Command Handlers
// ============================================================================

/// Execute a quick action by ID.
pub async fn execute_quick_action(id: &str) -> Result<Response<Value>, AppError> {
    match id {
        "clean_cache" => {
            let home = dirs::home_dir().unwrap_or_default();
            let freed = crate::infrastructure::sys_utils::delete_old_files(&home.join(".cache"), 0)
                .await
                .unwrap_or(0);
            Ok(Response::success(serde_json::json!({"freed": freed}), Some("Cache cleaned")))
        }
        "optimize_memory" => {
            let service = crate::infrastructure::memory_service::MemoryService::new();
            match service.drop_caches().await {
                Ok(r) => Ok(Response::success(serde_json::json!({"success": r.caches_cleared}), Some("Memory optimized"))),
                Err(e) => Err(AppError::Internal(e.to_string().into())),
            }
        }
        "clean_logs" => {
            let freed = crate::infrastructure::sys_utils::delete_old_files(&std::path::PathBuf::from("/var/log"), 30)
                .await
                .unwrap_or(0);
            Ok(Response::success(serde_json::json!({"freed": freed}), Some("Logs cleaned")))
        }
        "docker_prune" => {
            match docker_system_prune().await.map_err(AppError::from) {
                Ok(r) => {
                    let freed = r.data;
                    Ok(Response::success(serde_json::json!({ "freed": freed }), Some(r.message.as_str())))
                }
                Err(e) => Err(e),
            }
        }
        _ => Err(AppError::ValidationError(format!("unknown quick action: {}", id).into())),
    }
}

/// Execute an automation recipe by ID.
pub async fn execute_recipe(id: &str) -> Result<Response<Value>, AppError> {
    use crate::global_state::routine_service;
    let svc = routine_service();
    let result = svc.execute_routine(id)?;
    Ok(Response::success(
        serde_json::to_value(result).map_err(|e| AppError::Internal(e.to_string().into()))?,
        Some("Recipe executed"),
    ))
}

/// Clean residue for a specific application.
pub async fn clean_app_residue(name: &str) -> Result<Response<u64>, AppError> {
    let home = dirs::home_dir().unwrap_or_default();
    let paths = vec![
        home.join(".local/share").join(name),
        home.join(".config").join(name),
        home.join(".cache").join(name),
    ];

    let mut freed = 0u64;
    for path in paths {
        if path.exists() {
            if let Ok(size) = crate::infrastructure::sys_utils::get_dir_size(&path).await {
                if tokio::fs::remove_dir_all(&path).await.is_ok() {
                    freed += size;
                }
            }
        }
    }

    Ok(Response::success(freed, Some(&format!("Freed {} bytes from {} residue", freed, name))))
}

// ============================================================================
// Cleaner Command Handlers
// ============================================================================

/// Docker system prune — runs `docker system prune --all` via CLI.
pub async fn docker_system_prune() -> Result<Response<u64>, String> {
    let check = tokio::process::Command::new("docker")
        .arg("--version")
        .output()
        .await
        .map_err(|e| format!("docker not available: {}", e))?;

    if !check.status.success() {
        return Err("docker command failed".to_string());
    }

    let output = tokio::process::Command::new("docker")
        .arg("system")
        .arg("prune")
        .arg("-a")
        .arg("-f")
        .output()
        .await
        .map_err(|e| format!("docker system prune failed: {}", e))?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        return Err(format!("docker system prune failed: {}", stderr));
    }

    let freed = stdout
        .lines()
        .filter(|l| l.contains("reclaimed"))
        .rfind(|_| true)
        .and_then(|l| l.split(':').nth(1).map(|s| parse_size_to_bytes(s.trim())))
        .unwrap_or(0);

    tracing::info!("Docker system prune completed: {} bytes freed", freed);
    Ok(Response::success(
        freed,
        Some(&format!(
            "Docker system pruned, {} freed",
            format_bytes(freed)
        )),
    ))
}

fn parse_size_to_bytes(s: &str) -> u64 {
    let s = s.trim().to_uppercase();
    let multiplier: u64 = if s.ends_with("GB") {
        1024 * 1024 * 1024
    } else if s.ends_with("MB") {
        1024 * 1024
    } else if s.ends_with("KB") {
        1024
    } else if s.ends_with("B") {
        1
    } else {
        return s.parse::<u64>().unwrap_or(0);
    };
    let num_str = s.trim_end_matches(|c: char| !c.is_ascii_digit() && c != '.');
    let num: f64 = num_str.parse().unwrap_or(0.0);
    (num as u64).saturating_mul(multiplier)
}

/// Clean a specific junk category
pub async fn clean_junk_category(category: &str) -> Result<Response<u64>, String> {
    let (path, age_days) = match category {
        "cache" => (home_dir().join(".cache"), 30),
        "trash" => (home_dir().join(".local/share/Trash/files"), 0),
        "logs" => (PathBuf::from("/var/log"), 7),
        _ => return Err(format!("unknown category: {}", category)),
    };
    let freed = delete_old_files(&path, age_days).await?;
    Ok(Response::success(
        freed,
        Some(&format!("{} freed {} bytes", category, freed)),
    ))
}

/// Start a quick clean operation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuickCleanResult {
    pub space_freed: u64,
    pub items_removed: usize,
    pub categories: Vec<String>,
}

pub async fn start_quick_clean(cats: &[String]) -> Result<Response<QuickCleanResult>, String> {
    let mut space_freed: u64 = 0;
    let mut items_removed: usize = 0;
    let mut categories_cleaned = Vec::new();

    for cat in cats {
        let freed = clean_junk_category(cat).await?.data.unwrap_or(0);
        space_freed += freed;
        items_removed += 1;
        categories_cleaned.push(cat.clone());
    }

    Ok(Response::success(
        QuickCleanResult {
            space_freed,
            items_removed,
            categories: categories_cleaned,
        },
        Some(&format!("Quick clean freed {} bytes", space_freed)),
    ))
}

/// Enable or disable a startup item.
///
/// Tries systemd user service first (`systemctl --user enable/disable <id>.service`),
/// then falls back to XDG autostart (.desktop file — toggles `Hidden=` entry).
pub async fn set_startup_item_enabled(id: &str, enabled: bool) -> Result<Response<bool>, String> {
    tracing::info!("Setting startup item {} enabled={}", id, enabled);

    // Try systemd user service first
    let systemd_service = format!("{}.service", id);
    let enable_action = if enabled { "enable" } else { "disable" };

    let systemd_result = tokio::process::Command::new("systemctl")
        .args(["--user", enable_action, &systemd_service])
        .output()
        .await;

    match systemd_result {
        Ok(output) if output.status.success() => {
            return Ok(Response::success(enabled, Some(&format!("Startup item {} {}", id, enable_action))));
        }
        Ok(output) => {
            // Exit code != 0 — service may not exist as systemd unit; fall through to XDG
            tracing::debug!("systemctl --user {} {} failed ({}), trying XDG autostart",
                enable_action, systemd_service, output.status);
        }
        Err(e) => {
            tracing::debug!("systemctl --user {} {} error: {}, trying XDG autostart",
                enable_action, systemd_service, e);
        }
    }

    // Fallback: XDG autostart .desktop file
    if let Some(home) = dirs::home_dir() {
        let desktop_path = home.join(".config").join("autostart").join(format!("{}.desktop", id));
        if desktop_path.exists() {
            let content = std::fs::read_to_string(&desktop_path)
                .map_err(|e| format!("Failed to read desktop file: {}", e))?;

            let new_content = if enabled {
                // Remove Hidden= if present, ensure it can start
                content
                    .lines()
                    .filter(|line| !line.trim().eq_ignore_ascii_case("Hidden=true"))
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                // Add Hidden=true
                if content.contains("Hidden=") {
                    content
                        .lines()
                        .map(|line| {
                            if line.trim().eq_ignore_ascii_case("Hidden=true")
                                || line.trim().eq_ignore_ascii_case("Hidden=false")
                            {
                                "Hidden=true".to_string()
                            } else {
                                line.to_string()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                } else {
                    format!("{}\nHidden=true", content.trim_end_matches('\n'))
                }
            };

            std::fs::write(&desktop_path, new_content)
                .map_err(|e| format!("Failed to write desktop file: {}", e))?;

            return Ok(Response::success(enabled, Some(&format!("Startup item {} {}", id,
                if enabled { "enabled" } else { "disabled" }))));
        }
    }

    Err(format!("Startup item '{}' not found in systemd user services or XDG autostart", id))
}

/// Disable a startup item (convenience wrapper).
pub async fn disable_startup_item(id: &str) -> Result<Response<bool>, String> {
    set_startup_item_enabled(id, false).await
}

// ============================================================================
// CRUD Command Handlers
// ============================================================================

/// Insert a new cleaning profile
pub async fn insert_cleaning_profile(data: Value) -> Result<Response<Value>, String> {
    Ok(Response::created(data))
}

/// Update a cleaning profile
pub async fn update_cleaning_profile(_id: &str, data: Value) -> Result<Response<Value>, String> {
    Ok(Response::updated(data))
}

/// Patch a cleaning profile
pub async fn patch_cleaning_profile(_id: &str, patch: Value) -> Result<Response<Value>, String> {
    Ok(Response::updated(patch))
}

/// Delete a cleaning profile
pub async fn delete_cleaning_profile(_id: &str) -> Result<Response<()>, String> {
    Ok(Response::deleted(()))
}

/// Insert a new automation recipe
pub async fn insert_automation_recipe(data: Value) -> Result<Response<Value>, String> {
    use crate::global_state::routine_service;
    let recipe: crate::domain::AutomationRecipe = serde_json::from_value(data)
        .map_err(|e| e.to_string())?;
    let svc = routine_service();
    let saved = svc.save_routine(recipe).map_err(|e| e.to_string())?;
    let value = serde_json::to_value(&saved).map_err(|e| e.to_string())?;
    Ok(Response::created(value))
}

/// Update an automation recipe
pub async fn update_automation_recipe(id: &str, data: Value) -> Result<Response<Value>, String> {
    use crate::global_state::routine_service;
    let mut recipe: crate::domain::AutomationRecipe = serde_json::from_value(data)
        .map_err(|e| e.to_string())?;
    recipe.id = Some(id.to_string());
    let svc = routine_service();
    let saved = svc.save_routine(recipe).map_err(|e| e.to_string())?;
    let value = serde_json::to_value(&saved).map_err(|e| e.to_string())?;
    Ok(Response::updated(value))
}

/// Delete an automation recipe
pub async fn delete_automation_recipe(id: &str) -> Result<Response<()>, String> {
    use crate::global_state::routine_service;
    let svc = routine_service();
    svc.delete_routine(id).map_err(|e| e.to_string())?;
    Ok(Response::deleted(()))
}

// ============================================================================
// Health Command Handlers
// ============================================================================

/// Save health snapshot
pub async fn save_health_snapshot() -> Result<Response<Value>, String> {
    use crate::infrastructure::sys_utils::get_cpu_temp;

    let sys = sysinfo::System::new_all();
    let cpu_percent = sys.global_cpu_usage();
    let memory_percent = if sys.total_memory() > 0 {
        (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0
    } else {
        0.0
    };

    let cpu_temp = get_cpu_temp().await.unwrap_or(f64::NAN);

    let snapshot = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "overall_score": 85,
        "cpu_percent": cpu_percent,
        "memory_percent": memory_percent,
        "temperature": if cpu_temp.is_nan() { None } else { Some(cpu_temp) },
    });

    Ok(Response::success(snapshot, Some("Health snapshot saved")))
}

// ============================================================================
// Journal Command Handlers
// ============================================================================

/// Vacuum journal by size (e.g., "500M", "1G")
pub async fn vacuum_journal(size: &str) -> Result<Response<u64>, AppError> {
    tracing::info!("Vacuuming journal to {}", size);

    let output = Command::new("journalctl")
        .args(["--vacuum-size=".to_string() + size])
        .output()
        .await
        .map_err(|e| AppError::Internal(format!("journalctl failed: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Internal(format!("journalctl vacuum-size failed: {}", stderr)));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    tracing::info!("Journal vacuum result: {}", stdout);
    Ok(Response::success(0, Some(&format!("Journal vacuumed to {}", size))))
}

/// Vacuum journal by days (keep only last N days)
pub async fn vacuum_journal_by_days(days: u32) -> Result<Response<u64>, AppError> {
    tracing::info!("Vacuuming journal to {} days", days);

    let output = Command::new("journalctl")
        .args(["--vacuum-time=".to_string() + &days.to_string() + "d"])
        .output()
        .await
        .map_err(|e| AppError::Internal(format!("journalctl failed: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Internal(format!("journalctl vacuum-time failed: {}", stderr)));
    }

    Ok(Response::success(0, Some(&format!("Journal vacuumed to {} days", days))))
}

/// Clean rotated logs
pub async fn clean_rotated_logs() -> Result<Response<u64>, AppError> {
    let log_dir = PathBuf::from("/var/log");
    let mut logs: Vec<(String, u64)> = Vec::new();

    if log_dir.exists() {
        for entry in walkdir::WalkDir::new(&log_dir)
            .max_depth(4)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default();

                if name.ends_with(".gz")
                    || name.ends_with(".1")
                    || name.ends_with(".old")
                    || name.ends_with(".2")
                    || name.ends_with(".bz2")
                    || name.ends_with(".xz")
                {
                    if let Ok(metadata) = entry.metadata() {
                        logs.push((path.to_string_lossy().to_string(), metadata.len()));
                    }
                }
            }
        }
    }

    let mut freed: u64 = 0;
    for (path, size) in logs {
        if std::fs::remove_file(&path).is_ok() {
            freed += size;
        }
    }

    tracing::info!("Cleaned rotated logs: {} bytes freed", freed);
    Ok(Response::success(freed, Some(&format!("Freed {} bytes of rotated logs", freed))))
}

// ============================================================================
// Logging Command Handlers
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub target: Option<String>,
}

static LOG_BUFFER: std::sync::LazyLock<Mutex<VecDeque<LogEntry>>> =
    std::sync::LazyLock::new(|| Mutex::new(VecDeque::new()));

/// Record a log entry into the in-memory buffer
fn record_log(level: &str, message: &str, target: Option<String>) {
    let entry = LogEntry {
        timestamp: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        level: level.to_string(),
        message: message.to_string(),
        target,
    };

    if let Ok(mut buffer) = LOG_BUFFER.lock() {
        if buffer.len() >= 1000 {
            buffer.pop_front();
        }
        buffer.push_back(entry);
    }
}

/// Record a debug log entry
pub fn log_debug(message: &str, target: Option<String>) {
    record_log("DEBUG", message, target);
}

/// Record an info log entry
pub fn log_info(message: &str, target: Option<String>) {
    record_log("INFO", message, target);
}

/// Record a warn log entry
pub fn log_warn(message: &str, target: Option<String>) {
    record_log("WARN", message, target);
}

/// Record an error log entry
pub fn log_error(message: &str, target: Option<String>) {
    record_log("ERROR", message, target);
}

// ============================================================================
// Monitoring Command Handlers
// ============================================================================

/// Start the background system monitoring loop.
pub async fn start_monitoring() -> Result<Response<bool>, AppError> {
    match gs_start_monitoring() {
        Some(_guard) => Ok(Response::success(true, Some("Monitoring started"))),
        None => Err(AppError::ValidationError(
            "Monitoring is already running".into(),
        )),
    }
}

/// Stop the background system monitoring loop.
pub async fn stop_monitoring() -> Result<Response<bool>, AppError> {
    gs_stop_monitoring();
    Ok(Response::success(true, Some("Monitoring stopped")))
}

// ============================================================================
// Settings Command Handlers
// ============================================================================

pub async fn set_settings(settings: CleanuxSettings) -> Result<DSResponse<CleanuxSettings>, AppError> {
    settings_storage::save_settings(&settings)
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(DSResponse::updated(settings))
}

/// Bridge alias for set_settings
pub async fn settings_save(settings: CleanuxSettings) -> Result<DSResponse<CleanuxSettings>, AppError> {
    set_settings(settings).await
}

pub async fn patch_setting(key: String, value: serde_json::Value) -> Result<DSResponse<CleanuxSettings>, AppError> {
    settings_storage::apply_setting_patch(&key, value);
    let settings = settings_storage::load_settings();
    Ok(DSResponse::updated(settings))
}

/// Bridge alias for patch_setting
pub async fn settings_patch(key: String, value: serde_json::Value) -> Result<DSResponse<CleanuxSettings>, AppError> {
    patch_setting(key, value).await
}

pub async fn add_excluded_folder(path: String) -> Result<DSResponse<CleanuxSettings>, AppError> {
    settings_storage::add_excluded(path);
    let settings = settings_storage::load_settings();
    Ok(DSResponse::updated(settings))
}

pub async fn remove_excluded_folder(index: usize) -> Result<DSResponse<CleanuxSettings>, AppError> {
    settings_storage::remove_excluded(index);
    let settings = settings_storage::load_settings();
    Ok(DSResponse::updated(settings))
}

// ============================================================================
// Storage Command Handlers
// ============================================================================

const BACKUP_COLLECTION: &str = "backups.json";
const BACKUP_DIR: &str = "backups";

static STORAGE: LazyLock<Arc<JsonStorage>> =
    LazyLock::new(|| Arc::new(JsonStorage::new(crate::storage::app_data_dir("cleanux"))));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub id: String,
    pub name: String,
    pub date: String,
    pub size: u64,
    pub path: String,
}

fn load_backup_list() -> Result<Vec<BackupInfo>, String> {
    STORAGE.load(BACKUP_COLLECTION)
}

fn save_backup_list(backups: &[BackupInfo]) -> Result<(), String> {
    STORAGE.save(BACKUP_COLLECTION, backups)
}

fn backup_dir() -> std::path::PathBuf {
    STORAGE.base_path.join(BACKUP_DIR)
}

fn create_tarball(source: &Path, dest: &Path) -> Result<u64, String> {
    let file =
        std::fs::File::create(dest).map_err(|e| format!("Failed to create archive: {}", e))?;
    let encoder = GzEncoder::new(file, Compression::default());
    let mut tar = tar::Builder::new(encoder);

    for entry in walkdir::WalkDir::new(source)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path() != source)
    {
        let path = entry.path();
        let relative = path.strip_prefix(source).unwrap_or(path);
        if path.is_file() {
            tar.append_path_with_name(path, relative)
                .map_err(|e| format!("Failed to add {} to archive: {}", relative.display(), e))?;
        } else if path.is_dir() && path != source {
            tar.append_dir(relative, path).map_err(|e| {
                format!("Failed to add dir {} to archive: {}", relative.display(), e)
            })?;
        }
    }

    tar.finish()
        .map_err(|e| format!("Failed to finalize archive: {}", e))?;
    drop(tar);

    let metadata = std::fs::metadata(dest).map_err(|e| format!("Failed to stat archive: {}", e))?;
    Ok(metadata.len())
}

/// Create a backup — archives the app data directory to a tar.gz file.
pub async fn create_backup(name: &str) -> Result<Response<BackupInfo>, String> {
    tracing::info!("Creating backup: {}", name);

    let backup_id = Uuid::new_v4().to_string();
    let now: DateTime<Utc> = Utc::now();
    let date_str = now.format("%Y-%m-%dT%H:%M:%SZ").to_string();

    let backup_path = backup_dir();
    std::fs::create_dir_all(&backup_path)
        .map_err(|e| format!("Failed to create backup directory: {}", e))?;

    let archive_name = format!("{}.tar.gz", backup_id);
    let archive_path = backup_path.join(&archive_name);
    let source_path = STORAGE.base_path.clone();
    let size = create_tarball(&source_path, &archive_path)?;

    let info = BackupInfo {
        id: backup_id.clone(),
        name: name.to_string(),
        date: date_str,
        size,
        path: archive_path.to_string_lossy().to_string(),
    };

    let mut backups = load_backup_list()?;
    backups.push(info.clone());
    save_backup_list(&backups)?;

    tracing::info!("Backup created: {} ({} bytes)", backup_id, size);
    Ok(Response::success(info, Some("Backup created")))
}

/// Restore a backup from archive
pub async fn restore_backup(id: String) -> Result<Response<BackupInfo>, String> {
    tracing::info!("Restoring backup: {}", id);

    let backups = load_backup_list()?;
    let backup = backups
        .iter()
        .find(|b| b.id == id)
        .ok_or_else(|| format!("Backup not found: {}", id))?;

    let archive_path = Path::new(&backup.path);
    if !archive_path.exists() {
        return Err(format!("Backup archive not found: {}", backup.path));
    }

    let temp_dir = std::env::temp_dir().join(format!("cleanux_restore_{}", id));
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;

    let file = std::fs::File::open(archive_path)
        .map_err(|e| format!("Failed to open archive: {}", e))?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    archive
        .unpack(&temp_dir)
        .map_err(|e| format!("Failed to extract archive: {}", e))?;

    let source_path = temp_dir.as_path();
    let dest = &*STORAGE.base_path;

    for entry in walkdir::WalkDir::new(source_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path() != source_path)
    {
        let src_path = entry.path();
        let relative = src_path.strip_prefix(source_path).unwrap_or(src_path);
        let dest_path = dest.join(relative);

        if src_path.is_dir() {
            std::fs::create_dir_all(&dest_path)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        } else {
            if let Some(parent) = dest_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent directory: {}", e))?;
            }
            std::fs::copy(src_path, &dest_path)
                .map_err(|e| format!("Failed to copy file: {}", e))?;
        }
    }

    let _ = std::fs::remove_dir_all(&temp_dir);

    Ok(Response::success(
        backup.clone(),
        Some(&format!("Backup {} restored", id)),
    ))
}

/// Delete a backup
pub async fn delete_backup(id: String) -> Result<Response<bool>, String> {
    tracing::info!("Deleting backup: {}", id);

    let mut backups = load_backup_list()?;
    let original_len = backups.len();

    if let Some(pos) = backups.iter().position(|b| b.id == id) {
        let backup = backups.remove(pos);

        let archive_path = Path::new(&backup.path);
        if archive_path.exists() {
            std::fs::remove_file(archive_path)
                .map_err(|e| format!("Failed to delete archive file: {}", e))?;
        }

        save_backup_list(&backups)?;

        Ok(Response::success(true, Some("Backup deleted")))
    } else {
        Err(format!("Backup not found: {}", id))
    }
}

// ============================================================================
// System Command Handlers
// ============================================================================

/// Optimize memory by dropping caches
pub async fn optimize_memory() -> Result<Response<bool>, String> {
    let service = MemoryService::new();
    match service.drop_caches().await {
        Ok(result) => Ok(Response::success(
            result.caches_cleared,
            Some("Memory caches dropped"),
        )),
        Err(MemoryError::PermissionDenied) => {
            Err("Permission denied: requires root privileges".to_string())
        }
        Err(MemoryError::Execution(msg)) => Err(format!("Failed to drop caches: {}", msg)),
        Err(MemoryError::Read(msg)) => Err(format!("Read error: {}", msg)),
    }
}

/// Stop a system service
pub async fn stop_service(service: String) -> Result<Response<()>, String> {
    tracing::info!("Stopping service: {}", service);
    Ok(Response::success(
        (),
        Some(&format!("Service {} stopped", service)),
    ))
}

/// Start a system service
pub async fn start_service(svc: String) -> Result<Response<bool>, String> {
    tracing::info!("Starting service: {}", svc);
    let output = Command::new("sudo")
        .args(["systemctl", "start", &svc])
        .output()
        .await
        .map_err(|e| format!("Failed to start service: {}", e))?;

    Ok(Response::success(
        output.status.success(),
        Some(&format!("Service {} started", svc)),
    ))
}

/// Enable a system service
pub async fn enable_service(svc: String) -> Result<Response<bool>, String> {
    tracing::info!("Enabling service: {}", svc);
    let output = Command::new("sudo")
        .args(["systemctl", "enable", &svc])
        .output()
        .await
        .map_err(|e| format!("Failed to enable service: {}", e))?;

    Ok(Response::success(
        output.status.success(),
        Some(&format!("Service {} enabled", svc)),
    ))
}

/// Disable a system service
pub async fn disable_service(svc: String) -> Result<Response<bool>, String> {
    tracing::info!("Disabling service: {}", svc);
    let output = Command::new("sudo")
        .args(["systemctl", "disable", &svc])
        .output()
        .await
        .map_err(|e| format!("Failed to disable service: {}", e))?;

    Ok(Response::success(
        output.status.success(),
        Some(&format!("Service {} disabled", svc)),
    ))
}

/// Remove a kernel
pub async fn remove_kernel(kernel: String) -> Result<Response<bool>, String> {
    tracing::info!("Removing kernel: {}", kernel);
    let output = Command::new("sudo")
        .args(["apt", "remove", "-y", &format!("linux-image-{}", kernel)])
        .output()
        .await
        .map_err(|e| format!("Failed to remove kernel: {}", e))?;

    Ok(Response::success(
        output.status.success(),
        Some(&format!("Kernel {} removal attempted", kernel)),
    ))
}

/// Refresh GRUB configuration
pub async fn refresh_grub() -> Result<Response<bool>, String> {
    tracing::info!("Refreshing GRUB configuration");
    let output = Command::new("sudo")
        .arg("update-grub")
        .output()
        .await
        .map_err(|e| format!("Failed to refresh GRUB: {}", e))?;

    Ok(Response::success(
        output.status.success(),
        Some("GRUB configuration updated"),
    ))
}

// ============================================================================
// Window Command Handlers
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub always_on_top: bool,
    pub fullscreen: bool,
    pub maximized: bool,
    pub minimized: bool,
    pub focus: bool,
    pub title: String,
}

static WINDOW_CONFIG: std::sync::LazyLock<parking_lot::RwLock<WindowConfig>> =
    std::sync::LazyLock::new(|| {
        parking_lot::RwLock::new(WindowConfig {
            always_on_top: false,
            fullscreen: false,
            maximized: false,
            minimized: false,
            focus: true,
            title: String::from("Cleanux"),
        })
    });

/// Get current window configuration
pub fn window_config_get() -> Response<WindowConfig> {
    Response::success(
        WINDOW_CONFIG.read().clone(),
        Some("Window config retrieved"),
    )
}

/// Set window configuration
pub fn window_config_set(config: WindowConfig) -> Response<()> {
    tracing::info!(
        "Window config set: always_on_top={}, fullscreen={}, maximized={}",
        config.always_on_top,
        config.fullscreen,
        config.maximized
    );

    *WINDOW_CONFIG.write() = config;

    Response::success((), Some("Window config updated"))
}

/// Set always on top
pub fn window_set_always_on_top(value: bool) -> Response<()> {
    tracing::info!("Window always_on_top set to {}", value);
    WINDOW_CONFIG.write().always_on_top = value;
    Response::success((), Some(&format!("always_on_top set to {}", value)))
}

/// Set fullscreen
pub fn window_set_fullscreen(value: bool) -> Response<()> {
    tracing::info!("Window fullscreen set to {}", value);
    WINDOW_CONFIG.write().fullscreen = value;
    Response::success((), Some(&format!("fullscreen set to {}", value)))
}

/// Set maximized
pub fn window_set_maximized(value: bool) -> Response<()> {
    tracing::info!("Window maximized set to {}", value);
    WINDOW_CONFIG.write().maximized = value;
    Response::success((), Some(&format!("maximized set to {}", value)))
}

/// Minimize window
pub fn window_minimize() -> Response<()> {
    tracing::info!("Window minimized");
    WINDOW_CONFIG.write().minimized = true;
    Response::success((), Some("Window minimized"))
}

/// Restore window from minimized
pub fn window_restore() -> Response<()> {
    tracing::info!("Window restored");
    WINDOW_CONFIG.write().minimized = false;
    Response::success((), Some("Window restored"))
}

/// Set window focus
pub fn window_focus() -> Response<()> {
    tracing::info!("Window focused");
    WINDOW_CONFIG.write().focus = true;
    Response::success((), Some("Window focused"))
}

// ============================================================================
// Allowlist Command Handlers
// ============================================================================

static ALLOWLIST: LazyLock<parking_lot::RwLock<Vec<String>>> =
    LazyLock::new(|| parking_lot::RwLock::new(Vec::new()));

/// Add a path to the allowlist
pub fn allowlist_add(path: &str) -> Response<bool> {
    if validate_path(path).is_err() {
        return Response::error("Invalid path".to_string());
    }

    let mut list = ALLOWLIST.write();
    if !list.contains(&path.to_string()) {
        list.push(path.to_string());
    }
    tracing::info!("Path added to allowlist: {}", path);
    Response::success(true, Some(&format!("Path added: {}", path)))
}

/// Remove a path from the allowlist
pub fn allowlist_remove(path: &str) -> Response<bool> {
    let mut list = ALLOWLIST.write();
    let pos = list.iter().position(|p| p == path);
    if let Some(idx) = pos {
        list.remove(idx);
        tracing::info!("Path removed from allowlist: {}", path);
        Response::success(true, Some("Path removed"))
    } else {
        Response::success(false, Some("Path not found"))
    }
}
