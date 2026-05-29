use cleanux_lib::helpers::validation_helper::{is_allowed_path, validate_path};
use std::path::PathBuf;

#[test]
fn test_validate_path_nonexistent() {
  let result = validate_path("/nonexistent/path/12345");
  assert!(result.is_err());
}

#[test]
fn test_validate_path_trailing_symlink() {
  let result = validate_path("/proc/self");
  if result.is_ok() {
    let path = result.unwrap();
    assert!(path.is_absolute());
  }
}

#[test]
fn test_is_allowed_path_home() {
  let home = PathBuf::from("/home/testuser");
  let allowed_path = PathBuf::from("/home/testuser/.cache");
  assert!(is_allowed_path(&allowed_path, &home));
}

#[test]
fn test_is_allowed_path_tmp() {
  let home = PathBuf::from("/home/testuser");
  let allowed_path = PathBuf::from("/tmp/test");
  assert!(is_allowed_path(&allowed_path, &home));
}

#[test]
fn test_is_allowed_path_var_cache() {
  let home = PathBuf::from("/home/testuser");
  let allowed_path = PathBuf::from("/var/cache/apt");
  assert!(is_allowed_path(&allowed_path, &home));
}

#[test]
fn test_is_allowed_path_blocked() {
  let home = PathBuf::from("/home/testuser");
  let blocked_path = PathBuf::from("/etc/passwd");
  assert!(!is_allowed_path(&blocked_path, &home));
}

#[test]
fn test_validate_path_current_dir() {
  let result = validate_path(".");
  if result.is_ok() {
    let path = result.unwrap();
    assert!(path.exists());
  }
}
