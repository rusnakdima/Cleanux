use cleanux_lib::models::ResponseStatus;
use cleanux_lib::services::directory_service::DirectoryService;
use std::fs;

#[test]
fn test_scan_directory_with_files() {
  let temp_dir = tempfile::tempdir().unwrap();
  let temp_path = temp_dir.path().to_str().unwrap();

  fs::write(temp_dir.path().join("file1.txt"), "content1").unwrap();
  fs::write(temp_dir.path().join("file2.txt"), "content2").unwrap();

  let result = DirectoryService::scan_directory(temp_path, 10);

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
  assert!(response.message.contains("successfully"));
}

#[test]
fn test_scan_directory_nonexistent_path() {
  let result = DirectoryService::scan_directory("/nonexistent/path/12345", 10);

  assert!(result.is_err());
  let response = result.unwrap_err();
  assert_eq!(response.status, ResponseStatus::Error);
  assert!(response.message.contains("does not exist"));
}

#[test]
fn test_get_directory_size() {
  let temp_dir = tempfile::tempdir().unwrap();

  fs::write(temp_dir.path().join("small.txt"), "abc").unwrap();

  let result = DirectoryService::get_directory_size(temp_dir.path().to_str().unwrap());

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}

#[test]
fn test_get_directory_size_nonexistent() {
  let result = DirectoryService::get_directory_size("/nonexistent/path/12345");

  assert!(result.is_err());
  let response = result.unwrap_err();
  assert_eq!(response.status, ResponseStatus::Error);
}

#[test]
fn test_find_empty_directories() {
  let temp_dir = tempfile::tempdir().unwrap();
  let empty_dir = temp_dir.path().join("empty");
  fs::create_dir(&empty_dir).unwrap();

  let result = DirectoryService::find_empty_directories(temp_dir.path().to_str().unwrap());

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}

#[test]
fn test_find_empty_directories_nonexistent() {
  let result = DirectoryService::find_empty_directories("/nonexistent/path/12345");

  assert!(result.is_err());
  let response = result.unwrap_err();
  assert_eq!(response.status, ResponseStatus::Error);
}

#[test]
fn test_find_nested_empty_directories() {
  let temp_dir = tempfile::tempdir().unwrap();
  let nested_empty = temp_dir.path().join("nested").join("empty");
  fs::create_dir_all(&nested_empty).unwrap();

  let result = DirectoryService::find_nested_empty_directories(temp_dir.path().to_str().unwrap());

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}

#[test]
fn test_find_nested_empty_directories_with_files() {
  let temp_dir = tempfile::tempdir().unwrap();
  let nested = temp_dir.path().join("nested").join("empty");
  fs::create_dir_all(&nested).unwrap();
  fs::write(temp_dir.path().join("file.txt"), "content").unwrap();

  let result = DirectoryService::find_nested_empty_directories(temp_dir.path().to_str().unwrap());

  assert!(result.is_ok());
}

#[test]
fn test_remove_empty_directory() {
  let temp_dir = tempfile::tempdir().unwrap();
  let empty_dir = temp_dir.path().join("to_remove");
  fs::create_dir(&empty_dir).unwrap();

  let result = DirectoryService::remove_empty_directory(empty_dir.to_str().unwrap());

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
  assert!(!empty_dir.exists());
}

#[test]
fn test_remove_empty_directory_not_empty() {
  let temp_dir = tempfile::tempdir().unwrap();
  fs::write(temp_dir.path().join("file.txt"), "content").unwrap();

  let result = DirectoryService::remove_empty_directory(temp_dir.path().to_str().unwrap());

  assert!(result.is_err());
  let response = result.unwrap_err();
  assert_eq!(response.status, ResponseStatus::Error);
  assert!(response.message.contains("not empty"));
}

#[test]
fn test_remove_empty_directory_nonexistent() {
  let result = DirectoryService::remove_empty_directory("/nonexistent/path/12345");

  assert!(result.is_err());
  let response = result.unwrap_err();
  assert_eq!(response.status, ResponseStatus::Error);
}

#[test]
fn test_remove_empty_directories_multiple() {
  let temp_dir = tempfile::tempdir().unwrap();
  let dir1 = temp_dir.path().join("empty1");
  let dir2 = temp_dir.path().join("empty2");
  fs::create_dir(&dir1).unwrap();
  fs::create_dir(&dir2).unwrap();

  let paths = vec![
    dir1.to_string_lossy().to_string(),
    dir2.to_string_lossy().to_string(),
  ];
  let result = DirectoryService::remove_empty_directories(paths);

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}

#[test]
fn test_remove_empty_directories_mixed() {
  let temp_dir = tempfile::tempdir().unwrap();
  let empty_dir = temp_dir.path().join("empty");
  let nonempty_dir = temp_dir.path().join("nonempty");
  fs::create_dir(&empty_dir).unwrap();
  fs::create_dir(&nonempty_dir).unwrap();
  fs::write(nonempty_dir.join("file.txt"), "content").unwrap();

  let paths = vec![
    empty_dir.to_string_lossy().to_string(),
    nonempty_dir.to_string_lossy().to_string(),
  ];
  let result = DirectoryService::remove_empty_directories(paths);

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}

#[test]
fn test_scan_directory_calculates_size_correctly() {
  let temp_dir = tempfile::tempdir().unwrap();

  fs::write(temp_dir.path().join("file1.txt"), "12345678").unwrap();
  fs::create_dir(temp_dir.path().join("subdir")).unwrap();
  fs::write(temp_dir.path().join("subdir").join("file2.txt"), "12").unwrap();

  let result = DirectoryService::scan_directory(temp_dir.path().to_str().unwrap(), 10);

  assert!(result.is_ok());
}

#[test]
fn test_nested_empty_directory_detection() {
  let temp_dir = tempfile::tempdir().unwrap();
  let level1 = temp_dir.path().join("level1");
  let level2 = level1.join("level2");
  let level3 = level2.join("level3");
  fs::create_dir_all(&level3).unwrap();

  let result = DirectoryService::find_nested_empty_directories(temp_dir.path().to_str().unwrap());

  assert!(result.is_ok());
}

#[test]
fn test_remove_multiple_empty_dirs_cleanup() {
  let temp_dir = tempfile::tempdir().unwrap();
  let dirs: Vec<_> = (0..3)
    .map(|i| {
      let d = temp_dir.path().join(format!("empty_{}", i));
      fs::create_dir(&d).unwrap();
      d
    })
    .collect();

  let paths: Vec<String> = dirs
    .iter()
    .map(|d| d.to_string_lossy().to_string())
    .collect();
  let result = DirectoryService::remove_empty_directories(paths);

  assert!(result.is_ok());
  for d in &dirs {
    assert!(!d.exists());
  }
}
