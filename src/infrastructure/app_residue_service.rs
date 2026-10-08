//! AppResidueService — detects and cleans application leftover files
//!
//! Scans user config, data, cache directories for orphaned app residue
//! after partial uninstalls or abandoned software.

use crate::error::AppError;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

pub type Result<T> = std::result::Result<T, AppError>;

/// Type of app residue
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ResidueType {
  Config,
  Data,
  Cache,
  Log,
  Runtime,
  Temp,
  Other,
}

impl std::fmt::Display for ResidueType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ResidueType::Config => write!(f, "config"),
      ResidueType::Data => write!(f, "data"),
      ResidueType::Cache => write!(f, "cache"),
      ResidueType::Log => write!(f, "log"),
      ResidueType::Runtime => write!(f, "runtime"),
      ResidueType::Temp => write!(f, "temp"),
      ResidueType::Other => write!(f, "other"),
    }
  }
}

/// A single app residue item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppResidueItem {
  /// Full path to the residue entry
  pub path: String,
  /// Derived app name from path
  pub app_name: String,
  /// Category of residue
  pub residue_type: ResidueType,
  /// Size in bytes
  pub size: u64,
  /// Last modified timestamp (epoch seconds)
  pub modified: i64,
  /// Whether this is a directory
  pub is_dir: bool,
}

impl AppResidueItem {
  /// Create from a path, inferring app_name and residue_type
  fn from_path(path: &Path) -> Option<Self> {
    let path_str = path.to_string_lossy().to_string();
    let home = dirs::home_dir()?;
    let rel = path.strip_prefix(&home).ok()?.to_string_lossy().to_string();

    let (residue_type, app_name) = if rel.starts_with(".config/") {
      (
        ResidueType::Config,
        rel
          .trim_start_matches(".config/")
          .split('/')
          .next()?
          .to_string(),
      )
    } else if rel.starts_with(".local/share/") {
      (
        ResidueType::Data,
        rel
          .trim_start_matches(".local/share/")
          .split('/')
          .next()?
          .to_string(),
      )
    } else if rel.starts_with(".cache/") {
      (
        ResidueType::Cache,
        rel
          .trim_start_matches(".cache/")
          .split('/')
          .next()?
          .to_string(),
      )
    } else if rel.starts_with(".local/state/") {
      (
        ResidueType::Runtime,
        rel
          .trim_start_matches(".local/state/")
          .split('/')
          .next()?
          .to_string(),
      )
    } else if rel.starts_with(".")
      && !rel.starts_with(".local")
      && !rel.starts_with(".config")
      && !rel.starts_with(".cache")
    {
      (
        ResidueType::Other,
        rel
          .trim_start_matches('.')
          .trim_start_matches('/')
          .split('/')
          .next()?
          .to_string(),
      )
    } else {
      return None;
    };

    let metadata = fs::metadata(path).ok()?;
    let size = if metadata.is_dir() {
      dir_size(path).unwrap_or(0)
    } else {
      metadata.len()
    };
    let modified = metadata
      .modified()
      .ok()
      .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
      .map(|d| d.as_secs() as i64)
      .unwrap_or(0);

    Some(Self {
      path: path_str,
      app_name,
      residue_type,
      size,
      modified,
      is_dir: metadata.is_dir(),
    })
  }
}

/// Calculate total size of a directory recursively
fn dir_size(path: &Path) -> std::io::Result<u64> {
  let mut size = 0u64;
  for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
    if entry.file_type().is_file() {
      size += entry.metadata().map(|m| m.len()).unwrap_or(0);
    }
  }
  Ok(size)
}

/// Get the user's home directory
fn home_dir() -> PathBuf {
  dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
}

/// Scan ~/.config/* for app residue
pub fn scan_user_configs() -> Result<Vec<AppResidueItem>> {
  let home = home_dir();
  let config_dir = home.join(".config");
  scan_dir(&config_dir)
}

/// Scan ~/.local/share/* and ~/.* for app data residue
pub fn scan_user_data() -> Result<Vec<AppResidueItem>> {
  let home = home_dir();
  let mut items = Vec::new();

  // Scan ~/.local/share/*
  let local_share = home.join(".local/share");
  if local_share.is_dir() {
    items.extend(scan_dir(&local_share)?);
  }

  // Scan ~/.* dotfiles/dirs (excluding standard ones)
  if let Ok(entries) = fs::read_dir(&home) {
    for entry in entries.filter_map(|e| e.ok()) {
      let name = entry.file_name().to_string_lossy().to_string();
      if name.starts_with('.') && !is_standard_dotdir(&name) {
        if let Some(item) = AppResidueItem::from_path(&entry.path()) {
          items.push(item);
        }
      }
    }
  }

  Ok(items)
}

/// Scan ~/.cache/* for app cache residue
pub fn scan_user_caches() -> Result<Vec<AppResidueItem>> {
  let home = home_dir();
  let cache_dir = home.join(".cache");
  scan_dir(&cache_dir)
}

/// Standard dot directories that are not app residue
fn is_standard_dotdir(name: &str) -> bool {
  matches!(
    name,
    ".local"
      | ".config"
      | ".cache"
      | ".profile"
      | ".bashrc"
      | ".zshrc"
      | ".zprofile"
      | ".xinitrc"
      | ".xsession"
      | ".pam_environment"
      | ".bash_profile"
      | ".kshrc"
      | ".cshrc"
      | ".fishrc"
  )
}

/// Scan a directory for app residue items (non-recursive, top-level only)
fn scan_dir(dir: &Path) -> Result<Vec<AppResidueItem>> {
  let mut items = Vec::new();
  if !dir.is_dir() {
    return Ok(items);
  }

  if let Ok(entries) = fs::read_dir(dir) {
    for entry in entries.filter_map(|e| e.ok()) {
      let path = entry.path();
      if let Some(item) = AppResidueItem::from_path(&path) {
        items.push(item);
      }
    }
  }
  Ok(items)
}

/// Scan for orphaned directories in ~/.config, ~/.local/share, ~/.cache
/// that don't correspond to any known installed package.
pub fn scan_home_residues() -> Result<Vec<AppResidueItem>> {
  let home = home_dir();
  let mut all_items = Vec::new();

  // Config residue
  let configs = home.join(".config");
  if configs.is_dir() {
    all_items.extend(scan_dir(&configs)?);
  }

  // Data residue
  let local_share = home.join(".local/share");
  if local_share.is_dir() {
    all_items.extend(scan_dir(&local_share)?);
  }

  // Cache residue
  let cache_dir = home.join(".cache");
  if cache_dir.is_dir() {
    all_items.extend(scan_dir(&cache_dir)?);
  }

  Ok(all_items)
}

/// Get orphaned configs by matching against known packages.
/// Currently returns all items not in the known_apps allowlist.
pub fn get_orphaned_configs() -> Result<Vec<AppResidueItem>> {
  let items = scan_home_residues()?;

  // Filter to configs that might be orphaned
  let orphaned: Vec<AppResidueItem> = items
    .into_iter()
    .filter(|item| {
      // Keep configs from uninstalled apps
      // This is a heuristic: config dirs with no associated binary in PATH
      !is_known_app(&item.app_name)
    })
    .collect();

  Ok(orphaned)
}

/// Check if an app name corresponds to a known installed application
fn is_known_app(_app_name: &str) -> bool {
  // TODO: Cross-reference with package database or PATH binaries
  // For now, return false to surface all items for review
  false
}

/// Result of a cleaning operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanResult {
  /// Number of items cleaned
  pub items_cleaned: u32,
  /// Total bytes freed
  pub bytes_freed: u64,
  /// List of paths that were deleted
  pub cleaned_paths: Vec<String>,
  /// Any errors encountered
  pub errors: Vec<String>,
}

/// Delete residue items and return cleanup summary.
pub fn clean_residue(items: Vec<AppResidueItem>) -> CleanResult {
  let mut bytes_freed = 0u64;
  let mut cleaned_paths = Vec::new();
  let mut errors = Vec::new();

  for item in items {
    let path = Path::new(&item.path);
    let result = if path.is_dir() {
      fs::remove_dir_all(path)
    } else {
      fs::remove_file(path)
    };

    match result {
      Ok(_) => {
        bytes_freed += item.size;
        cleaned_paths.push(item.path);
      }
      Err(e) => {
        errors.push(format!("{}: {}", item.path, e));
      }
    }
  }

  CleanResult {
    items_cleaned: cleaned_paths.len() as u32,
    bytes_freed,
    cleaned_paths,
    errors,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_residue_type_display() {
    assert_eq!(ResidueType::Config.to_string(), "config");
    assert_eq!(ResidueType::Data.to_string(), "data");
  }

  #[test]
  fn test_is_standard_dotdir() {
    assert!(is_standard_dotdir(".config"));
    assert!(is_standard_dotdir(".cache"));
    assert!(!is_standard_dotdir(".myapp"));
  }
}
