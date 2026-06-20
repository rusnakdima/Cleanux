use cleanux_lib::services::junk_cleaner_service::{JunkCategory, JunkCleanerService, JunkItem};
use std::fs;
#[test]
fn test_junk_cleaner_service_scan_browser_caches() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  fs::create_dir_all(home.join(".cache/mozilla")).unwrap();
  fs::write(
    home.join(".cache/mozilla").join("cache_file.txt"),
    "mozilla cache",
  )
  .unwrap();
  fs::create_dir_all(home.join(".cache/google-chrome")).unwrap();
  fs::write(
    home.join(".cache/google-chrome").join("cache_file.txt"),
    "chrome cache",
  )
  .unwrap();
  let service = JunkCleanerService;
  let result = service.scan_browser_caches();
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}
#[test]
fn test_junk_cleaner_service_scan_thumbnail_caches() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  fs::create_dir_all(home.join(".cache/thumbnails")).unwrap();
  fs::write(
    home.join(".cache/thumbnails").join("thumb.db"),
    "thumbnail data",
  )
  .unwrap();
  let service = JunkCleanerService;
  let result = service.scan_thumbnail_caches();
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}
#[test]
fn test_junk_cleaner_service_scan_application_caches() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  fs::create_dir_all(home.join(".cache/flatpak")).unwrap();
  fs::write(
    home.join(".cache/flatpak").join("flatpak_file.txt"),
    "flatpak cache",
  )
  .unwrap();
  fs::create_dir_all(home.join("snap")).unwrap();
  fs::write(home.join("snap").join("snap_file.txt"), "snap cache").unwrap();
  let service = JunkCleanerService;
  let result = service.scan_application_caches();
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}
#[test]
fn test_junk_cleaner_service_scan_system_temp() {
  let temp_dir = tempfile::tempdir().unwrap();
  let temp_test = temp_dir.path().join("tmp_test_item");
  fs::write(&temp_test, "temp content").unwrap();
  let service = JunkCleanerService;
  let result = service.scan_system_temp();
  assert!(result.is_ok());
}
#[test]
fn test_junk_cleaner_service_clean_browser_caches() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  fs::create_dir_all(home.join(".cache/mozilla")).unwrap();
  fs::write(home.join(".cache/mozilla").join("test.txt"), "test").unwrap();
  let service = JunkCleanerService;
  let result = service.clean_junk_category("browser".to_string());
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}
#[test]
fn test_junk_cleaner_service_clean_thumbnail_caches() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  fs::create_dir_all(home.join(".cache/thumbnails")).unwrap();
  fs::write(home.join(".cache/thumbnails").join("test.txt"), "test").unwrap();
  let service = JunkCleanerService;
  let result = service.clean_junk_category("thumbnails".to_string());
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}
#[test]
fn test_junk_cleaner_service_clean_application_caches() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  fs::create_dir_all(home.join(".cache/flatpak")).unwrap();
  fs::write(home.join(".cache/flatpak").join("test.txt"), "test").unwrap();
  let service = JunkCleanerService;
  let result = service.clean_junk_category("applications".to_string());
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}
#[test]
fn test_junk_cleaner_service_get_junk_summary() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();
  std::env::set_var("HOME", home.to_str().unwrap());
  fs::create_dir_all(home.join(".cache/mozilla")).unwrap();
  fs::write(home.join(".cache/mozilla").join("test.txt"), "test").unwrap();
  let service = JunkCleanerService;
  let result = service.get_junk_summary();
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}
#[test]
fn test_junk_category_serialization() {
  let categories = vec![
    JunkCategory::Browser,
    JunkCategory::Thumbnails,
    JunkCategory::Applications,
    JunkCategory::System,
    JunkCategory::Logs,
  ];
  for cat in categories {
    let json = serde_json::to_string(&cat).unwrap();
    assert!(!json.is_empty());
  }
}
#[test]
fn test_junk_item_serialization() {
  let item = JunkItem {
    path: "/test/path".to_string(),
    size: 1024,
    category: JunkCategory::Browser,
    description: "Test item".to_string(),
    file_count: 5,
  };
  let json = serde_json::to_string(&item).unwrap();
  assert!(json.contains("path"));
  assert!(json.contains("size"));
  assert!(json.contains("Browser"));
}
#[test]
fn test_junk_cleaner_invalid_category() {
  let service = JunkCleanerService;
  let result = service.clean_junk_category("invalid_category".to_string());
  assert!(result.is_err());
}
