//! BackupService — manages backup creation, restoration, and listing.
//!
//! Backs up the Cleanux data directory as tar.gz archives.

use crate::infrastructure::json_storage::JsonStorage;
use chrono::{DateTime, Utc};
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::LazyLock;
use uuid::Uuid;

const BACKUP_COLLECTION: &str = "backups.json";
const BACKUP_DIR: &str = "backups";

/// Global storage instance — same data_dir as the main app.
static STORAGE: LazyLock<Arc<JsonStorage>> =
  LazyLock::new(|| Arc::new(JsonStorage::new(crate::env::data_dir("cleanux"))));

pub type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

/// A persisted backup entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupEntry {
  pub id: String,
  pub name: String,
  pub date: String,
  pub size: u64,
  pub path: String,
  #[serde(default)]
  pub backup_type: String,
}

/// Load backup list from JsonStorage.
fn load_backup_list() -> Result<Vec<BackupEntry>> {
  STORAGE
    .load::<Vec<BackupEntry>>(BACKUP_COLLECTION)
    .map_err(anyhow::Error::msg)
}

/// Save backup list to JsonStorage.
fn save_backup_list(backups: &[BackupEntry]) -> Result<()> {
  STORAGE
    .save(BACKUP_COLLECTION, backups)
    .map_err(anyhow::Error::msg)
}

/// Backup archive directory path.
fn backup_dir() -> PathBuf {
  STORAGE.base_path.join(BACKUP_DIR)
}

/// Get the configured backup directory path.
pub fn get_backup_dir() -> String {
  backup_dir().to_string_lossy().to_string()
}

/// Create a tar.gz archive of the data directory and return its size.
fn create_tarball(source: &Path, dest: &Path) -> Result<u64> {
  let file =
    std::fs::File::create(dest).map_err(|e| anyhow::anyhow!("Failed to create archive: {}", e))?;
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
      tar
        .append_path_with_name(path, relative)
        .map_err(|e| anyhow::anyhow!("Failed to add {} to archive: {}", relative.display(), e))?;
    } else if path.is_dir() && path != source {
      tar.append_dir(relative, path).map_err(|e| {
        anyhow::anyhow!("Failed to add dir {} to archive: {}", relative.display(), e)
      })?;
    }
  }

  tar
    .finish()
    .map_err(|e| anyhow::anyhow!("Failed to finalize archive: {}", e))?;
  drop(tar);

  let metadata =
    std::fs::metadata(dest).map_err(|e| anyhow::anyhow!("Failed to stat archive: {}", e))?;
  Ok(metadata.len())
}

/// Create a new backup.
pub async fn create_backup(name: &str) -> Result<BackupEntry> {
  let backup_id = Uuid::new_v4().to_string();
  let now: DateTime<Utc> = Utc::now();
  let date_str = now.format("%Y-%m-%dT%H:%M:%SZ").to_string();

  // Ensure backup directory exists
  let backup_path = backup_dir();
  std::fs::create_dir_all(&backup_path)
    .map_err(|e| anyhow::anyhow!("Failed to create backup directory: {}", e))?;

  // Create tar.gz archive
  let archive_name = format!("{}.tar.gz", backup_id);
  let archive_path = backup_path.join(&archive_name);
  let source_path = STORAGE.base_path.clone();
  let size = create_tarball(&source_path, &archive_path)?;

  let entry = BackupEntry {
    id: backup_id.clone(),
    name: name.to_string(),
    date: date_str,
    size,
    path: archive_path.to_string_lossy().to_string(),
    backup_type: "manual".to_string(),
  };

  // Persist backup metadata
  let mut backups = load_backup_list()?;
  backups.push(entry.clone());
  save_backup_list(&backups)?;

  Ok(entry)
}

/// List all backups.
pub async fn list_backups() -> Result<Vec<BackupEntry>> {
  let backups = load_backup_list()?;

  // Filter to only backups whose archive files still exist on disk
  let existing: Vec<BackupEntry> = backups
    .into_iter()
    .filter(|b| Path::new(&b.path).exists())
    .collect();

  Ok(existing)
}

/// Restore a backup from archive.
pub async fn restore_backup(id: &str) -> Result<BackupEntry> {
  let backups = load_backup_list()?;
  let backup = backups
    .iter()
    .find(|b| b.id == id)
    .ok_or_else(|| anyhow::anyhow!("Backup not found: {}", id))?
    .clone();

  let archive_path = Path::new(&backup.path);
  if !archive_path.exists() {
    return Err(anyhow::anyhow!("Backup archive not found: {}", backup.path));
  }

  // Extract tarball to a temporary location
  let temp_dir = std::env::temp_dir().join(format!("cleanux_restore_{}", id));
  std::fs::create_dir_all(&temp_dir)
    .map_err(|e| anyhow::anyhow!("Failed to create temp directory: {}", e))?;

  let file = std::fs::File::open(archive_path)
    .map_err(|e| anyhow::anyhow!("Failed to open archive: {}", e))?;
  let decoder = flate2::read::GzDecoder::new(file);
  let mut archive = tar::Archive::new(decoder);
  archive
    .unpack(&temp_dir)
    .map_err(|e| anyhow::anyhow!("Failed to extract archive: {}", e))?;

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
        .map_err(|e| anyhow::anyhow!("Failed to create directory: {}", e))?;
    } else {
      if let Some(parent) = dest_path.parent() {
        std::fs::create_dir_all(parent)
          .map_err(|e| anyhow::anyhow!("Failed to create parent directory: {}", e))?;
      }
      std::fs::copy(src_path, &dest_path)
        .map_err(|e| anyhow::anyhow!("Failed to copy file: {}", e))?;
    }
  }

  // Cleanup temp directory
  let _ = std::fs::remove_dir_all(&temp_dir);

  Ok(backup)
}

/// Delete a backup.
pub async fn delete_backup(id: &str) -> Result<()> {
  let mut backups = load_backup_list()?;

  if let Some(pos) = backups.iter().position(|b| b.id == id) {
    let backup = backups.remove(pos);

    // Delete the archive file if it exists
    let archive_path = Path::new(&backup.path);
    if archive_path.exists() {
      std::fs::remove_file(archive_path)
        .map_err(|e| anyhow::anyhow!("Failed to delete archive file: {}", e))?;
    }

    save_backup_list(&backups)?;
  }

  Ok(())
}
