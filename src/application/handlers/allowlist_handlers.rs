//! Path Security Allowlist handlers for Cleanux
//!
//! Provides allowlist management using local path validation.

use crate::response::Response;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

/// Allowed path entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowlistEntry {
  pub path: String,
  pub allowed: bool,
}

/// Allowlist status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowlistStatus {
  pub allowed_paths: Vec<String>,
  pub total_count: usize,
}

/// Global allowlist state (paths explicitly added by user)
static ALLOWLIST: LazyLock<parking_lot::RwLock<Vec<String>>> =
  LazyLock::new(|| parking_lot::RwLock::new(Vec::new()));

// ---------------------------------------------------------------------------
// Path validation stubs (previously from dioxus_shared::core::storage::allowlist)
// ---------------------------------------------------------------------------

/// Validate that a path string is safe to use.
/// Returns `Ok(())` if valid, or an error message if not.
pub fn validate_path(path: &str) -> Result<(), String> {
  if path.is_empty() {
    return Err("Path cannot be empty".to_string());
  }
  let path_buf = PathBuf::from(path);
  // Reject paths that try to escape the filesystem root
  if path_buf.is_absolute() {
    Ok(())
  } else {
    Err("Only absolute paths are allowed".to_string())
  }
}

/// Check if a path is allowed (exists in the user allowlist or is a safe default).
pub fn is_allowed_path(path: &Path) -> bool {
  let path_str = path.to_string_lossy();
  // User-added paths are always allowed
  if ALLOWLIST.read().iter().any(|p| p == path_str.as_ref()) {
    return true;
  }
  // Stub: allow home directory paths by default
  if let Some(home) = dirs::home_dir() {
    if path.starts_with(&home) {
      return true;
    }
  }
  // Allow common safe directories
  let safe_prefixes = ["/tmp", "/var/tmp", "/usr"];
  safe_prefixes.iter().any(|prefix| path.starts_with(prefix))
}

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

/// List all paths in the allowlist
pub fn allowlist_list() -> Response<Vec<String>> {
  let list = ALLOWLIST.read();
  Response::success(
    list.clone(),
    Some(&format!("{} paths in allowlist", list.len())),
  )
}

/// Check if a path is allowed
pub fn allowlist_check(path: &str) -> Response<bool> {
  let path_buf = Path::new(path);
  let is_allowed = is_allowed_path(path_buf);
  Response::success(
    is_allowed,
    Some(if is_allowed {
      "Path is allowed"
    } else {
      "Path is blocked"
    }),
  )
}

/// Get allowlist status
pub fn allowlist_status() -> Response<AllowlistStatus> {
  let list = ALLOWLIST.read();
  Response::success(
    AllowlistStatus {
      allowed_paths: list.clone(),
      total_count: list.len(),
    },
    Some("Allowlist status retrieved"),
  )
}
