/* security module */

#[path = "allowlist.rs"]
pub mod allowlist;

#[path = "privilege.rs"]
pub mod privilege;

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::AppError;
use crate::security::allowlist::is_path_allowed;

pub struct PathValidator;

impl PathValidator {
  pub fn validate(path: &str) -> Result<PathBuf, AppError> {
    if path.is_empty() {
      return Err(AppError::InvalidPath("Path cannot be empty".to_string()));
    }

    if path.contains('\0') {
      return Err(AppError::InvalidPath("Path contains null byte".to_string()));
    }

    if path.contains("..") {
      return Err(AppError::InvalidPath(
        "Path traversal not allowed".to_string(),
      ));
    }

    let path_buf = PathBuf::from(path);

    let canonical = path_buf
      .canonicalize()
      .map_err(|e| AppError::InvalidPath(format!("Failed to canonicalize '{}': {}", path, e)))?;

    if !canonical.exists() {
      return Err(AppError::InvalidPath(format!(
        "Path does not exist: {}",
        path
      )));
    }

    if !is_path_allowed(&canonical) {
      return Err(AppError::PathOutsideAllowed(format!(
        "Path '{}' is outside allowed directories",
        canonical.display()
      )));
    }

    if Self::is_symlink(&canonical) {
      return Err(AppError::InvalidPath(format!(
        "Symlinks are not allowed: {}",
        canonical.display()
      )));
    }

    Ok(canonical)
  }

  pub fn is_symlink(path: &PathBuf) -> bool {
    fs::symlink_metadata(path)
      .map(|meta| meta.file_type().is_symlink())
      .unwrap_or(false)
  }

  pub fn validate_within_home(path: &str, home_dir: &Path) -> Result<PathBuf, AppError> {
    let validated = Self::validate(path)?;

    let home_canonical = home_dir
      .canonicalize()
      .map_err(|e| AppError::InvalidPath(format!("Failed to canonicalize home: {}", e)))?;

    if !validated.starts_with(&home_canonical) {
      return Err(AppError::PathOutsideAllowed(
        "Path must be within home directory".to_string(),
      ));
    }

    Ok(validated)
  }
}

pub fn sanitize_path(path: &str) -> String {
  let mut result = path.to_string();

  result = result.replace('\0', "");
  result = result.trim().to_string();

  loop {
    let prev = result.clone();
    result = result.replace("..", "");
    if prev == result {
      break;
    }
  }

  result
}

pub fn sanitize_input(input: &str) -> String {
  let mut result = input.to_string();

  result = result.replace('\0', "");
  result = result.replace('\r', "");
  result = result.replace('\n', "");
  result = result.trim().to_string();

  result
}

pub fn get_timestamp() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs()
}
