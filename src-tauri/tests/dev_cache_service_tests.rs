use cleanux_lib::services::dev_cache_service::{DevCacheItem, DevCacheService};
use std::fs;

#[test]
fn test_dev_cache_service_get_all_caches() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".npm")).unwrap();
  fs::write(home.join(".npm").join("package.json"), "test").unwrap();

  fs::create_dir_all(home.join(".cache/pip")).unwrap();
  fs::write(home.join(".cache/pip").join("package.txt"), "test").unwrap();

  let service = DevCacheService;
  let result = service.get_all_dev_caches();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}

#[test]
fn test_dev_cache_service_clean_npm_cache() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".npm")).unwrap();
  fs::write(home.join(".npm").join("test.txt"), "test").unwrap();

  let service = DevCacheService;
  let result = service.clean_npm_cache();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}

#[test]
fn test_dev_cache_service_clean_pip_cache() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".cache/pip")).unwrap();
  fs::write(home.join(".cache/pip").join("test.txt"), "test").unwrap();

  let service = DevCacheService;
  let result = service.clean_pip_cache();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}

#[test]
fn test_dev_cache_service_clean_cargo_cache() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".cargo/registry")).unwrap();
  fs::write(home.join(".cargo/registry").join("test.txt"), "test").unwrap();

  let service = DevCacheService;
  let result = service.clean_cargo_cache();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}

#[test]
fn test_dev_cache_service_clean_go_cache() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join("go/pkg/mod")).unwrap();
  fs::write(home.join("go/pkg/mod").join("test.txt"), "test").unwrap();

  let service = DevCacheService;
  let result = service.clean_go_cache();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}

#[test]
fn test_dev_cache_service_clean_maven_cache() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".m2/repository")).unwrap();
  fs::write(home.join(".m2/repository").join("test.txt"), "test").unwrap();

  let service = DevCacheService;
  let result = service.clean_maven_cache();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}

#[test]
fn test_dev_cache_service_clean_gradle_cache() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".gradle/caches")).unwrap();
  fs::write(home.join(".gradle/caches").join("test.txt"), "test").unwrap();

  let service = DevCacheService;
  let result = service.clean_gradle_cache();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}

#[test]
fn test_dev_cache_service_clean_all() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".npm")).unwrap();
  fs::create_dir_all(home.join(".cache/pip")).unwrap();
  fs::create_dir_all(home.join(".cargo/registry")).unwrap();
  fs::create_dir_all(home.join("go/pkg/mod")).unwrap();
  fs::create_dir_all(home.join(".m2/repository")).unwrap();
  fs::create_dir_all(home.join(".gradle/caches")).unwrap();

  let service = DevCacheService;
  let result = service.clean_all_dev_caches();

  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(
    response.status,
    cleanux_lib::models::ResponseStatus::Success
  );
}

#[test]
fn test_dev_cache_service_scan_npm_empty() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  let service = DevCacheService;
  let result = service.get_all_dev_caches();

  assert!(result.is_ok());
}

#[test]
fn test_dev_cache_item_serialization() {
  let item = DevCacheItem {
    name: "npm".to_string(),
    cache_path: "/home/user/.npm".to_string(),
    size: 1024,
    description: "npm cache".to_string(),
  };

  let json = serde_json::to_string(&item).unwrap();
  assert!(json.contains("npm"));
  assert!(json.contains("1024"));
}
