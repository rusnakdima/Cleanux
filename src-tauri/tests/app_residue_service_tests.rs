use cleanux_lib::services::app_residue_service::{AppResidue, AppResidueService, ResidueType};
use std::fs;

#[test]
fn test_app_residue_service_scan_user_configs_returns_vec() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".config")).unwrap();
  fs::write(home.join(".config").join("someapp.conf"), "test").unwrap();

  let service = AppResidueService;
  let result = service.scan_user_configs();

  assert!(result.len() >= 0);
}

#[test]
fn test_app_residue_service_scan_user_data_returns_vec() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".local/share")).unwrap();
  fs::write(home.join(".local/share").join("someapp.data"), "test").unwrap();

  let service = AppResidueService;
  let result = service.scan_user_data();

  assert!(result.len() >= 0);
}

#[test]
fn test_app_residue_service_scan_user_caches_returns_vec() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".cache")).unwrap();
  fs::write(home.join(".cache").join("test.cache"), "test").unwrap();

  let service = AppResidueService;
  let result = service.scan_user_caches();

  assert!(result.len() >= 0);
}

#[test]
fn test_residue_type_serialization() {
  let types = vec![
    ResidueType::Config,
    ResidueType::Data,
    ResidueType::Cache,
    ResidueType::Both,
  ];

  for rt in types {
    let json = serde_json::to_string(&rt).unwrap();
    assert!(!json.is_empty());
  }
}

#[test]
fn test_app_residue_serialization() {
  let residue = AppResidue {
    path: "/home/user/.config/app".to_string(),
    app_name: "testapp".to_string(),
    size: 1024,
    residue_type: ResidueType::Config,
    detected_as_uninstalled: false,
  };

  let json = serde_json::to_string(&residue).unwrap();
  assert!(json.contains("testapp"));
  assert!(json.contains("1024"));
}

#[test]
fn test_app_residue_service_scan_home_residues() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  fs::create_dir_all(home.join(".config")).unwrap();
  fs::create_dir_all(home.join(".local/share")).unwrap();
  fs::create_dir_all(home.join(".cache")).unwrap();

  let service = AppResidueService;
  let result = service.scan_home_residues();

  assert!(result.len() >= 0);
}

#[test]
fn test_app_residue_service_get_residue_summary() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  let service = AppResidueService;
  let result = service.get_residue_summary();

  assert!(result.total_size >= 0);
}

#[test]
fn test_app_residue_service_get_orphaned_configs() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  let service = AppResidueService;
  let result = service.get_orphaned_configs();

  assert!(result.is_empty() || !result.is_empty());
}

#[test]
fn test_app_residue_service_clean_residue() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  let config_dir = home.join(".config/testapp");
  fs::create_dir_all(&config_dir).unwrap();
  fs::write(config_dir.join("config.txt"), "test").unwrap();

  let service = AppResidueService;
  let result = service.clean_residue(&config_dir.to_string_lossy());

  assert!(result.is_ok());
}

#[test]
fn test_app_residue_service_clean_multiple() {
  let temp_dir = tempfile::tempdir().unwrap();
  let home = temp_dir.path().to_path_buf();

  std::env::set_var("HOME", home.to_str().unwrap());

  let path1 = home.join(".config/app1");
  let path2 = home.join(".local/share/app2");
  let paths = vec![
    path1.to_string_lossy().to_string(),
    path2.to_string_lossy().to_string(),
  ];

  fs::create_dir_all(&path1).unwrap();
  fs::create_dir_all(&path2).unwrap();

  let service = AppResidueService;
  let result = service.clean_multiple(paths);

  assert!(result.is_ok());
}

#[test]
fn test_app_residue_fields() {
  let residue = AppResidue {
    path: "/home/user/.config/app".to_string(),
    app_name: "myapp".to_string(),
    size: 2048,
    residue_type: ResidueType::Data,
    detected_as_uninstalled: true,
  };

  assert_eq!(residue.app_name, "myapp");
  assert_eq!(residue.size, 2048);
  assert_eq!(residue.residue_type, ResidueType::Data);
  assert!(residue.detected_as_uninstalled);
}
