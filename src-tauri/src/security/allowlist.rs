/* security/allowlist.rs - Path allowlist functionality */

use std::fs;
use std::path::PathBuf;

use crate::models::AppError;
use crate::security::PathValidator;
use crate::utils::home_dir;

pub const ALLOWED_PATHS: &[&str] = &[
  "/home",
  "/tmp",
  "/var/cache",
  "/var/tmp",
  "/snap",
  "/srv",
  "/opt",
];

const BLOCKED_PATHS: &[&str] = &["/proc", "/sys", "/dev"];

const EXCEPTION_PATHS: &[&str] = &["/proc/self", "/proc/curproc"];

pub fn is_path_allowed(path: &PathBuf) -> bool {
  let canonical = match fs::canonicalize(path) {
    Ok(p) => p,
    Err(_) => return false,
  };

  let path_str = canonical.to_string_lossy();

  for blocked in BLOCKED_PATHS {
    if path_str.starts_with(blocked) {
      let is_exception = EXCEPTION_PATHS.iter().any(|exc| path_str.starts_with(exc));
      if !is_exception {
        return false;
      }
    }
  }

  for allowed in ALLOWED_PATHS {
    if path_str.starts_with(allowed) {
      return true;
    }
  }

  if let Ok(home) = home_dir() {
    if let Ok(home_canonical) = fs::canonicalize(&home) {
      if path_str.starts_with(home_canonical.to_string_lossy().as_ref()) {
        return true;
      }
    }

    let trash_path = home.join(".local/share/Trash");
    if let Ok(trash_canonical) = fs::canonicalize(trash_path) {
      if path_str.starts_with(trash_canonical.to_string_lossy().as_ref()) {
        return true;
      }
    }
  }

  if path_str.starts_with("/tmp/runtime-") || path_str.starts_with("/tmp/.X11-unix") {
    return true;
  }

  false
}

pub fn validate_path(path: &str) -> Result<String, AppError> {
  PathValidator::validate(path).map(|p| p.to_string_lossy().into_owned())
}
