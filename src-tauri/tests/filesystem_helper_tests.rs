use cleanux_lib::helpers::filesystem_helper::{
  calculate_dir_size, format_size, get_dir_size, remove_dir_contents,
};
use cleanux_lib::helpers::validation_helper::{is_allowed_path, validate_path};
use std::fs;
use std::path::PathBuf;

#[test]
fn test_format_size_bytes() {
  assert_eq!(format_size(0), "0 B");
  assert_eq!(format_size(512), "512 B");
  assert_eq!(format_size(1023), "1023 B");
}

#[test]
fn test_format_size_kilobytes() {
  assert_eq!(format_size(1024), "1.00 KB");
  assert_eq!(format_size(1536), "1.50 KB");
  assert_eq!(format_size(10240), "10.00 KB");
}

#[test]
fn test_format_size_megabytes() {
  assert_eq!(format_size(1024 * 1024), "1.00 MB");
  assert_eq!(format_size(1024 * 1024 * 5), "5.00 MB");
  assert_eq!(format_size(1024 * 1024 / 2), "0.50 MB");
}

#[test]
fn test_format_size_gigabytes() {
  assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
  assert_eq!(format_size(1024 * 1024 * 1024 * 2), "2.00 GB");
}

#[test]
fn test_calculate_dir_size_empty_dir() {
  let temp_dir = tempfile::tempdir().unwrap();
  let result = calculate_dir_size(temp_dir.path());
  assert!(result.is_ok());
  let (size, count) = result.unwrap();
  assert_eq!(size, 0);
  assert_eq!(count, 0);
}

#[test]
fn test_calculate_dir_size_with_files() {
  let temp_dir = tempfile::tempdir().unwrap();
  let file_path = temp_dir.path().join("test.txt");
  fs::write(&file_path, "hello world").unwrap();

  let result = calculate_dir_size(temp_dir.path());
  assert!(result.is_ok());
  let (size, count) = result.unwrap();
  assert_eq!(size, 11);
  assert_eq!(count, 1);
}

#[test]
fn test_calculate_dir_size_nested() {
  let temp_dir = tempfile::tempdir().unwrap();
  let sub_dir = temp_dir.path().join("subdir");
  fs::create_dir(&sub_dir).unwrap();
  fs::write(temp_dir.path().join("file1.txt"), "hello").unwrap();
  fs::write(sub_dir.join("file2.txt"), "world").unwrap();

  let result = calculate_dir_size(temp_dir.path());
  assert!(result.is_ok());
  let (size, count) = result.unwrap();
  assert_eq!(size, 10);
  assert_eq!(count, 2);
}

#[test]
fn test_calculate_dir_size_nonexistent() {
  let result = calculate_dir_size(PathBuf::from("/nonexistent/path").as_path());
  assert!(result.is_ok());
  let (size, count) = result.unwrap();
  assert_eq!(size, 0);
  assert_eq!(count, 0);
}

#[test]
fn test_get_dir_size() {
  let temp_dir = tempfile::tempdir().unwrap();
  fs::write(temp_dir.path().join("test.txt"), "hello").unwrap();

  let size = get_dir_size(temp_dir.path());
  assert_eq!(size, 5);
}

#[test]
fn test_remove_dir_contents() {
  let temp_dir = tempfile::tempdir().unwrap();
  let file1 = temp_dir.path().join("file1.txt");
  let file2 = temp_dir.path().join("file2.txt");
  fs::write(&file1, "hello").unwrap();
  fs::write(&file2, "world").unwrap();

  let result = remove_dir_contents(temp_dir.path());
  assert!(result.is_ok());
  let count = result.unwrap();
  assert_eq!(count, 2);

  assert!(!file1.exists());
  assert!(!file2.exists());
  assert!(temp_dir.path().exists());
}

#[test]
fn test_remove_dir_contents_empty_dir() {
  let temp_dir = tempfile::tempdir().unwrap();

  let result = remove_dir_contents(temp_dir.path());
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_remove_dir_contents_nonexistent() {
  let result = remove_dir_contents(PathBuf::from("/nonexistent/path").as_path());
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_validate_path_invalid() {
  let result = validate_path("/nonexistent/path/12345");
  assert!(result.is_err());
}

#[test]
fn test_validate_path_valid() {
  let result = validate_path("/tmp");
  if result.is_ok() {
    let path = result.unwrap();
    assert!(path.is_absolute());
  }
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
fn test_is_allowed_path_within_home() {
  let home = PathBuf::from("/home/testuser");
  let allowed = PathBuf::from("/home/testuser/.cache");
  assert!(is_allowed_path(&allowed, &home));
}

#[test]
fn test_is_allowed_path_tmp() {
  let home = PathBuf::from("/home/testuser");
  let tmp = PathBuf::from("/tmp/test");
  assert!(is_allowed_path(&tmp, &home));
}

#[test]
fn test_is_allowed_path_var_cache() {
  let home = PathBuf::from("/home/testuser");
  let cache = PathBuf::from("/var/cache/apt");
  assert!(is_allowed_path(&cache, &home));
}

#[test]
fn test_is_allowed_path_blocked() {
  let home = PathBuf::from("/home/testuser");
  let blocked = PathBuf::from("/etc/passwd");
  assert!(!is_allowed_path(&blocked, &home));
}

#[test]
fn test_is_allowed_path_var_log() {
  let home = PathBuf::from("/home/testuser");
  let var_log = PathBuf::from("/var/log");
  assert!(is_allowed_path(&var_log, &home));
}

#[test]
fn test_validate_path_current_dir() {
  let result = validate_path(".");
  if result.is_ok() {
    let path = result.unwrap();
    assert!(path.exists());
  }
}
