use cleanux_lib::models::{CacheFileModel, PaginatedData, ResponseStatus};
use cleanux_lib::services::cache_cleaning_service::CacheCleaningService;
use std::fs;
#[test]
fn test_get_cache_files_empty() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  let cache_dir = home.join(".cache");
  fs::create_dir_all(&cache_dir).unwrap();
  let service = CacheCleaningService;
  let result = service.getCacheFiles(None, None);
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}
#[test]
fn test_get_cache_files_with_items() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  let cache_dir = home.join(".cache");
  fs::create_dir_all(&cache_dir).unwrap();
  fs::write(cache_dir.join("file1.txt"), "test content 1").unwrap();
  fs::write(cache_dir.join("file2.txt"), "test content 2").unwrap();
  let service = CacheCleaningService;
  let result = service.getCacheFiles(None, None);
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}
#[test]
fn test_get_cache_files_with_limit_and_offset() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  let cache_dir = home.join(".cache");
  fs::create_dir_all(&cache_dir).unwrap();
  for i in 0..5 {
    fs::write(
      cache_dir.join(format!("file{}.txt", i)),
      format!("content {}", i),
    )
    .unwrap();
  }
  let service = CacheCleaningService;
  let result = service.getCacheFiles(Some(2), Some(0));
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}
#[test]
fn test_clear_selected_cache_files_empty() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  let service = CacheCleaningService;
  let result = service.clearSelectedCacheFiles(vec![]);
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}
#[test]
fn test_clear_selected_cache_files_nonexistent() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  let service = CacheCleaningService;
  let result = service.clearSelectedCacheFiles(vec!["/nonexistent/path/file.txt".to_string()]);
  assert!(result.is_err());
}
#[test]
fn test_clear_selected_cache_files_success() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  let cache_dir = home.join(".cache");
  fs::create_dir_all(&cache_dir).unwrap();
  let file_path = cache_dir.join("to_delete.txt");
  fs::write(&file_path, "test content").unwrap();
  let service = CacheCleaningService;
  let result = service.clearSelectedCacheFiles(vec![file_path.to_string_lossy().to_string()]);
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
  assert!(!file_path.exists());
}
#[test]
fn test_clear_selected_cache_files_multiple() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  let cache_dir = home.join(".cache");
  fs::create_dir_all(&cache_dir).unwrap();
  let file1 = cache_dir.join("file1.txt");
  let file2 = cache_dir.join("file2.txt");
  fs::write(&file1, "content 1").unwrap();
  fs::write(&file2, "content 2").unwrap();
  let service = CacheCleaningService;
  let result = service.clearSelectedCacheFiles(vec![
    file1.to_string_lossy().to_string(),
    file2.to_string_lossy().to_string(),
  ]);
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
  assert!(!file1.exists());
  assert!(!file2.exists());
}
#[test]
fn test_clear_cache_empty() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  let cache_dir = home.join(".cache");
  fs::create_dir_all(&cache_dir).unwrap();
  let service = CacheCleaningService;
  let result = service.clearCache();
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}
#[test]
fn test_clear_cache_with_files() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  let cache_dir = home.join(".cache");
  fs::create_dir_all(&cache_dir).unwrap();
  fs::write(cache_dir.join("file1.txt"), "test content").unwrap();
  fs::write(cache_dir.join("file2.txt"), "test content").unwrap();
  let service = CacheCleaningService;
  let result = service.clearCache();
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
  assert!(cache_dir.exists());
}
#[test]
fn test_cache_file_model_serialization() {
  let model = CacheFileModel {
    path: "/home/user/.cache/file.txt".to_string(),
    size: 1024,
    modified: "2024-01-15 10:30:00".to_string(),
  };
  let json = serde_json::to_string(&model).unwrap();
  assert!(json.contains("file.txt"));
  assert!(json.contains("path"));
  assert!(json.contains("size"));
  assert!(json.contains("modified"));
  assert!(json.contains("1024"));
}
#[test]
fn test_cache_file_model_deserialization() {
  let json =
    r#"{"path":"/home/user/.cache/test.txt","size":2048,"modified":"2024-01-15 10:30:00"}"#;
  let model: CacheFileModel = serde_json::from_str(json).unwrap();
  assert_eq!(model.path, "/home/user/.cache/test.txt");
  assert_eq!(model.size, 2048);
  assert_eq!(model.modified, "2024-01-15 10:30:00");
}
#[test]
fn test_cache_file_model_clone() {
  let model = CacheFileModel {
    path: "/cache/test.txt".to_string(),
    size: 512,
    modified: "2024-01-15 10:30:00".to_string(),
  };
  let cloned = model.clone();
  assert_eq!(cloned.path, model.path);
  assert_eq!(cloned.size, model.size);
  assert_eq!(cloned.modified, model.modified);
}
#[test]
fn test_paginated_data_serialization() {
  let paginated = PaginatedData::new(
    vec![
      CacheFileModel {
        path: "/cache/file1.txt".to_string(),
        size: 100,
        modified: "2024-01-15 10:30:00".to_string(),
      },
      CacheFileModel {
        path: "/cache/file2.txt".to_string(),
        size: 200,
        modified: "2024-01-15 11:00:00".to_string(),
      },
    ],
    true,
    10,
  );
  let json = serde_json::to_string(&paginated).unwrap();
  assert!(json.contains("data"));
  assert!(json.contains("has_more"));
  assert!(json.contains("total"));
  assert!(json.contains("100"));
  assert!(json.contains("200"));
  assert!(json.contains("10"));
}
#[test]
fn test_paginated_data_deserialization() {
  let json = r#"{"data":[{"path":"/cache/file.txt","size":512,"modified":"2024-01-15 10:30:00"}],"has_more":false,"total":1}"#;
  let paginated: PaginatedData<CacheFileModel> = serde_json::from_str(json).unwrap();
  assert_eq!(paginated.total, 1);
  assert!(!paginated.has_more);
  assert_eq!(paginated.data.len(), 1);
  assert_eq!(paginated.data[0].path, "/cache/file.txt");
}
#[test]
fn test_scan_nested_cache_directories() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  let cache_dir = home.join(".cache");
  fs::create_dir_all(&cache_dir).unwrap();
  fs::write(cache_dir.join("root_file.txt"), "root content").unwrap();
  let sub_dir = cache_dir.join("subdir");
  fs::create_dir_all(&sub_dir).unwrap();
  fs::write(sub_dir.join("nested_file.txt"), "nested content").unwrap();
  let service = CacheCleaningService;
  let result = service.getCacheFiles(None, None);
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}
#[test]
fn test_clear_cache_directory_not_exist() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  let cache_dir = home.join(".cache");
  if cache_dir.exists() {
    fs::remove_dir_all(&cache_dir).unwrap();
  }
  fs::create_dir_all(&cache_dir).unwrap();
  let service = CacheCleaningService;
  let result = service.clearCache();
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
  assert!(cache_dir.exists());
}
