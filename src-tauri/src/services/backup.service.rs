use crate::models::AppError;
use crate::models::Response;
use crate::utils::{data_empty_string, data_string, success_response};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde_json::Value;
use std::fs::{self, File};
use std::path::Path;

pub struct BackupService;

type BackupResult<T> = Result<T, AppError>;

impl BackupService {
  pub fn create_backup(
    paths: Vec<String>,
    archive_path: &str,
  ) -> Result<Response<Value>, Response<Value>> {
    Self::create_backup_inner(paths, archive_path).map_err(|e| e.into_response())
  }

  fn create_backup_inner(paths: Vec<String>, archive_path: &str) -> BackupResult<Response<Value>> {
    let tar_gz = File::create(archive_path)?;
    let encoder = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(encoder);

    for path_str in paths {
      let path = Path::new(&path_str);
      if !path.exists() {
        continue;
      }
      if path.is_file() {
        tar.append_path(path)?;
      } else if path.is_dir() {
        let dir_name = path
          .file_name()
          .ok_or_else(|| AppError::InvalidPath("Directory has no name".to_string()))?;
        tar.append_dir_all(dir_name, path)?;
      }
    }

    tar.finish()?;

    let metadata = fs::metadata(archive_path)?;
    let size = metadata.len();

    Ok(success_response(
      format!("Backup created successfully: {} bytes", size),
      data_string(size.to_string()),
    ))
  }

  pub fn restore_backup(
    archive_path: &str,
    destination: &str,
  ) -> Result<Response<Value>, Response<Value>> {
    Self::restore_backup_inner(archive_path, destination).map_err(|e| e.into_response())
  }

  fn restore_backup_inner(archive_path: &str, destination: &str) -> BackupResult<Response<Value>> {
    let file = File::open(archive_path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);

    let dest_path = Path::new(destination);
    fs::create_dir_all(dest_path)?;

    archive.unpack(dest_path)?;

    Ok(success_response(
      format!("Backup restored to {}", destination),
      data_empty_string(),
    ))
  }

  pub fn list_backups() -> Result<Response<Value>, Response<Value>> {
    Self::list_backups_inner().map_err(|e| e.into_response())
  }

  fn list_backups_inner() -> BackupResult<Response<Value>> {
    let backup_dir = Self::get_backup_dir()?;
    if !backup_dir.exists() {
      return Ok(success_response("No backups found", Value::Array(vec![])));
    }

    let mut backups: Vec<serde_json::Value> = Vec::new();
    let entries = fs::read_dir(&backup_dir)?;
    for entry in entries.flatten() {
      let path = entry.path();
      if path.extension().and_then(|s| s.to_str()) == Some("tar.gz") {
        let path_clone = path.clone();
        let metadata = fs::metadata(&path_clone).ok();
        let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
        let modified = metadata
          .as_ref()
          .and_then(|m| m.modified().ok())
          .map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
          .unwrap_or_default();
        let name = path_clone
          .file_name()
          .and_then(|n| n.to_str())
          .unwrap_or("unknown")
          .to_string();

        backups.push(serde_json::json!({
            "name": name,
            "path": path_clone.display().to_string(),
            "size": size,
            "modified": modified
        }));
      }
    }

    backups.sort_by(|a, b| {
      let a_time = a["modified"].as_str().unwrap_or("");
      let b_time = b["modified"].as_str().unwrap_or("");
      b_time.cmp(a_time)
    });

    Ok(success_response(
      format!("Found {} backups", backups.len()),
      Value::Array(backups),
    ))
  }

  pub fn delete_backup(archive_path: &str) -> Result<Response<Value>, Response<Value>> {
    Self::delete_backup_inner(archive_path).map_err(|e| e.into_response())
  }

  fn delete_backup_inner(archive_path: &str) -> BackupResult<Response<Value>> {
    let path = Path::new(archive_path);
    if !path.exists() {
      return Err(AppError::BackupFailed("Backup file not found".to_string()));
    }

    fs::remove_file(path)?;

    Ok(success_response(
      "Backup deleted successfully",
      data_empty_string(),
    ))
  }

  fn get_backup_dir() -> BackupResult<std::path::PathBuf> {
    let config_dir = dirs::config_dir()
      .ok_or_else(|| AppError::InvalidPath("Config directory not found".to_string()))?;
    Ok(config_dir.join("cleanux").join("backups"))
  }

  pub fn get_backup_dir_path() -> String {
    Self::get_backup_dir()
      .map(|p| p.display().to_string())
      .unwrap_or_default()
  }
}
