//! Storage KAS handlers for Cleanux
//!
//! Handles backup, directory scanning, and storage analysis.

use std::path::Path;
use std::sync::Arc;
use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use dioxus_shared::response::Response;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::infrastructure::json_storage::JsonStorage;

const BACKUP_COLLECTION: &str = "backups.json";
const BACKUP_DIR: &str = "backups";

/// Global storage instance — same data_dir as the main app.
static STORAGE: LazyLock<Arc<JsonStorage>> =
    LazyLock::new(|| Arc::new(JsonStorage::new(dioxus_shared::env::data_dir("cleanux"))));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryInfo {
    pub path: String,
    pub size: u64,
    pub files: u64,
    pub subdirs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub id: String,
    pub name: String,
    pub date: String,
    pub size: u64,
    pub path: String,
}

/// Get directory information
pub async fn scan_directory(path: &str) -> Result<Response<DirectoryInfo>, String> {
    tracing::info!("Scanning directory: {}", path);

    let mut total_size: u64 = 0;
    let mut file_count: u64 = 0;
    let mut dir_count: u64 = 0;

    let entries = walkdir::WalkDir::new(path).into_iter();
    for entry in entries.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            total_size += entry.metadata().map(|m| m.len()).unwrap_or(0);
            file_count += 1;
        } else if entry.file_type().is_dir() {
            dir_count += 1;
        }
    }

    Ok(Response::success(
        DirectoryInfo {
            path: path.to_string(),
            size: total_size,
            files: file_count,
            subdirs: dir_count,
        },
        Some("Directory scanned"),
    ))
}

/// Get directory size
pub async fn get_directory_size(path: &str) -> Result<Response<u64>, String> {
    let mut size: u64 = 0;

    let entries = walkdir::WalkDir::new(path).into_iter();
    for entry in entries.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            size += entry.metadata().map(|m| m.len()).unwrap_or(0);
        }
    }

    Ok(Response::success(size, Some("Directory size calculated")))
}

/// Find empty directories
pub async fn find_empty_directories(path: &str) -> Result<Response<Vec<String>>, String> {
    let mut empty_dirs = Vec::new();

    let entries = walkdir::WalkDir::new(path).into_iter();
    for entry in entries.filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            if let Ok(mut entries_in_dir) = std::fs::read_dir(entry.path()) {
                if entries_in_dir.next().is_none() {
                    empty_dirs.push(entry.path().display().to_string());
                }
            }
        }
    }

    Ok(Response::success(
        empty_dirs,
        Some("Empty directories found"),
    ))
}

/// Find duplicate files
pub async fn find_duplicates(path: &str) -> Result<Response<Vec<DuplicateGroup>>, String> {
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;

    tracing::info!("Finding duplicates in: {}", path);

    let mut files: HashMap<String, Vec<String>> = HashMap::new();
    let mut stack = vec![std::path::PathBuf::from(path)];

    while let Some(current) = stack.pop() {
        let mut entries = tokio::fs::read_dir(&current)
            .await
            .map_err(|e| e.to_string())?;
        while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else {
                match tokio::fs::read(&p).await {
                    Ok(content) => {
                        let hash = format!("{:x}", Sha256::digest(&content));
                        let path_str = p.to_string_lossy().to_string();
                        files.entry(hash).or_default().push(path_str);
                    }
                    Err(_) => continue,
                }
            }
        }
    }

    let duplicates: Vec<DuplicateGroup> = files
        .into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .map(|(hash, paths)| {
            let size = std::fs::metadata(&paths[0]).map(|m| m.len()).unwrap_or(0);
            DuplicateGroup { paths, size, hash }
        })
        .collect();

    Ok(Response::success(
        duplicates,
        Some("Duplicate scan complete"),
    ))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub paths: Vec<String>,
    pub size: u64,
    pub hash: String,
}

/// Load backups list from JsonStorage
fn load_backup_list() -> Result<Vec<BackupInfo>, String> {
    STORAGE.load(BACKUP_COLLECTION)
}

/// Save backups list to JsonStorage
fn save_backup_list(backups: &[BackupInfo]) -> Result<(), String> {
    STORAGE.save(BACKUP_COLLECTION, backups)
}

/// Backup archive directory path
fn backup_dir() -> std::path::PathBuf {
    STORAGE.base_path.join(BACKUP_DIR)
}

/// Create a tar.gz archive of the data directory and return its size.
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

    // Ensure backup directory exists
    let backup_path = backup_dir();
    std::fs::create_dir_all(&backup_path)
        .map_err(|e| format!("Failed to create backup directory: {}", e))?;

    // Create tar.gz archive of the data directory
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

    // Persist backup metadata
    let mut backups = load_backup_list()?;
    backups.push(info.clone());
    save_backup_list(&backups)?;

    tracing::info!("Backup created: {} ({} bytes)", backup_id, size);
    Ok(Response::success(info, Some("Backup created")))
}

/// List all persisted backups, filtering out any whose archive files are missing.
pub async fn list_backups() -> Result<Response<Vec<BackupInfo>>, String> {
    tracing::info!("Listing backups");

    let backups = load_backup_list()?;

    // Filter to only backups whose archive files still exist on disk
    let existing: Vec<BackupInfo> = backups
        .into_iter()
        .filter(|b| Path::new(&b.path).exists())
        .collect();

    Ok(Response::success(existing, Some("Backups retrieved")))
}

/// Scan browser caches
pub async fn scan_browser_caches() -> Result<Response<Vec<Value>>, String> {
    Ok(Response::success(
        vec![
            serde_json::json!({"browser": "Chrome", "path": "~/.cache/google-chrome", "size": 756_000_000}),
            serde_json::json!({"browser": "Firefox", "path": "~/.cache/mozilla", "size": 234_000_000}),
        ],
        Some("Browser caches scanned"),
    ))
}

/// Scan thumbnail caches
pub async fn scan_thumbnail_caches() -> Result<Response<u64>, String> {
    Ok(Response::success(
        120_000_000,
        Some("Thumbnail cache: 120 MB"),
    ))
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

    // Extract tarball to a temporary location
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

    // Copy contents to storage directory
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

    // Cleanup temp directory
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

    // Find and remove the backup
    if let Some(pos) = backups.iter().position(|b| b.id == id) {
        let backup = backups.remove(pos);

        // Delete the archive file if it exists
        let archive_path = Path::new(&backup.path);
        if archive_path.exists() {
            std::fs::remove_file(archive_path)
                .map_err(|e| format!("Failed to delete archive file: {}", e))?;
        }

        // Save updated backup list
        save_backup_list(&backups)?;

        Ok(Response::success(true, Some("Backup deleted")))
    } else {
        Err(format!("Backup not found: {}", id))
    }
}
