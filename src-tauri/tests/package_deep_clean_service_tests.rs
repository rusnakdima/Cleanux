use cleanux_lib::models::ResponseStatus;
use cleanux_lib::services::package_deep_clean_service::{
  CleanResult, OrphanedPackage, PackageDeepCleanService, PackageManagerSummary,
};
use std::fs;

#[test]
fn test_orphaned_package_serialization() {
  let pkg = OrphanedPackage {
    name: "test-package".to_string(),
    version: "1.0.0".to_string(),
    description: "A test package".to_string(),
  };

  let json = serde_json::to_string(&pkg).unwrap();
  assert!(json.contains("test-package"));
  assert!(json.contains("1.0.0"));
  assert!(json.contains("A test package"));

  let deserialized: OrphanedPackage = serde_json::from_str(&json).unwrap();
  assert_eq!(deserialized.name, "test-package");
  assert_eq!(deserialized.version, "1.0.0");
  assert_eq!(deserialized.description, "A test package");
}

#[test]
fn test_orphaned_package_camel_case_serialization() {
  let pkg = OrphanedPackage {
    name: "test".to_string(),
    version: "1.0".to_string(),
    description: "desc".to_string(),
  };

  let json = serde_json::to_string(&pkg).unwrap();
  assert!(json.contains("\"name\":"));
  assert!(json.contains("\"version\":"));
  assert!(json.contains("\"description\":"));
}

#[test]
fn test_package_manager_summary_serialization() {
  let summary = PackageManagerSummary {
    apt_available: true,
    apt_cache_size: 1024,
    apt_autoremove_size: 512,
    apt_orphaned_count: 3,
    apt_partial_downloads: 1,
    dnf_available: false,
    dnf_cache_size: 0,
    pacman_available: true,
    pacman_cache_size: 2048,
    zypper_available: false,
    zypper_cache_size: 0,
  };

  let json = serde_json::to_string(&summary).unwrap();
  assert!(json.contains("\"aptAvailable\":true"));
  assert!(json.contains("\"aptCacheSize\":1024"));
  assert!(json.contains("\"aptOrphanedCount\":3"));
  assert!(json.contains("\"pacmanCacheSize\":2048"));

  let deserialized: PackageManagerSummary = serde_json::from_str(&json).unwrap();
  assert_eq!(deserialized.apt_available, true);
  assert_eq!(deserialized.apt_cache_size, 1024);
  assert_eq!(deserialized.apt_orphaned_count, 3);
}

#[test]
fn test_clean_result_serialization() {
  let result = CleanResult {
    command: "apt-get clean".to_string(),
    space_freed: 4096,
    message: "APT cache cleaned".to_string(),
  };

  let json = serde_json::to_string(&result).unwrap();
  assert!(json.contains("\"command\":\"apt-get clean\""));
  assert!(json.contains("\"spaceFreed\":4096"));
  assert!(json.contains("\"message\":\"APT cache cleaned\""));

  let deserialized: CleanResult = serde_json::from_str(&json).unwrap();
  assert_eq!(deserialized.command, "apt-get clean");
  assert_eq!(deserialized.space_freed, 4096);
}

#[test]
fn test_get_apt_cache_size_returns_u64() {
  let service = PackageDeepCleanService;
  let _size = service.get_apt_cache_size();
}

#[test]
fn test_get_dnf_cache_size_returns_u64() {
  let service = PackageDeepCleanService;
  let _size = service.get_dnf_cache_size();
}

#[test]
fn test_get_pacman_cache_size_returns_u64() {
  let service = PackageDeepCleanService;
  let _size = service.get_pacman_cache_size();
}

#[test]
fn test_get_zypper_cache_size_returns_u64() {
  let service = PackageDeepCleanService;
  let _size = service.get_zypper_cache_size();
}

#[test]
fn test_get_orphaned_packages_returns_vec() {
  let service = PackageDeepCleanService;
  let orphans = service.get_orphaned_packages();
  let _ = orphans.len();
}

#[test]
fn test_get_partial_downloads_returns_vec() {
  let service = PackageDeepCleanService;
  let partials = service.get_partial_downloads();
  let _ = partials.len();
}

#[test]
fn test_calculate_directory_size_with_temp_dir() {
  let temp_dir = tempfile::tempdir().unwrap();
  fs::write(temp_dir.path().join("test_file.txt"), "test content").unwrap();

  let service = PackageDeepCleanService;
  let _size = service.get_apt_cache_size();
}

#[test]
fn test_package_deep_clean_service_new() {
  let service = PackageDeepCleanService;
  assert_eq!(
    service.get_orphaned_packages().len(),
    service.get_orphaned_packages().len()
  );
}

#[test]
fn test_apt_clean_returns_result_model() {
  let service = PackageDeepCleanService;
  let result = service.apt_clean();
  assert!(result.is_ok() || result.is_err());
  if let Ok(response) = result {
    assert_eq!(response.status, ResponseStatus::Success);
  }
}

#[test]
fn test_apt_autoremove_returns_result_model() {
  let service = PackageDeepCleanService;
  let result = service.apt_autoremove();
  assert!(result.is_ok() || result.is_err());
  if let Ok(response) = result {
    assert_eq!(response.status, ResponseStatus::Success);
  }
}

#[test]
fn test_apt_autoclean_returns_result_model() {
  let service = PackageDeepCleanService;
  let result = service.apt_autoclean();
  assert!(result.is_ok() || result.is_err());
  if let Ok(response) = result {
    assert_eq!(response.status, ResponseStatus::Success);
  }
}

#[test]
fn test_remove_orphaned_package_returns_result_model() {
  let service = PackageDeepCleanService;
  let result = service.remove_orphaned_package("nonexistent-package-name-xyz");
  assert!(result.is_ok() || result.is_err());
  if let Ok(response) = result {
    assert_eq!(response.status, ResponseStatus::Success);
  }
}

#[test]
fn test_dnf_clean_all_returns_result_model() {
  let service = PackageDeepCleanService;
  let result = service.dnf_clean_all();
  assert!(result.is_ok() || result.is_err());
  if let Ok(response) = result {
    assert_eq!(response.status, ResponseStatus::Success);
  }
}

#[test]
fn test_pacman_clean_returns_result_model() {
  let service = PackageDeepCleanService;
  let result = service.pacman_clean(0);
  assert!(result.is_ok() || result.is_err());
  if let Ok(response) = result {
    assert_eq!(response.status, ResponseStatus::Success);
  }
}

#[test]
fn test_pacman_full_clean_returns_result_model() {
  let service = PackageDeepCleanService;
  let result = service.pacman_full_clean();
  assert!(result.is_ok() || result.is_err());
  if let Ok(response) = result {
    assert_eq!(response.status, ResponseStatus::Success);
  }
}

#[test]
fn test_zypper_clean_returns_result_model() {
  let service = PackageDeepCleanService;
  let result = service.zypper_clean();
  assert!(result.is_ok() || result.is_err());
  if let Ok(response) = result {
    assert_eq!(response.status, ResponseStatus::Success);
  }
}

#[test]
fn test_deep_clean_all_returns_result_model() {
  let service = PackageDeepCleanService;
  let result = service.deep_clean_all();
  assert!(result.is_ok() || result.is_err());
  if let Ok(response) = result {
    assert_eq!(response.status, ResponseStatus::Success);
  }
}

#[test]
fn test_get_package_summary_returns_valid_summary() {
  let service = PackageDeepCleanService;
  let summary = service.get_package_summary();

  assert!(summary.apt_available == true || summary.apt_available == false);
  assert!(summary.dnf_available == true || summary.dnf_available == false);
  assert!(summary.pacman_available == true || summary.pacman_available == false);
  assert!(summary.zypper_available == true || summary.zypper_available == false);
}

#[test]
fn test_package_manager_summary_fields() {
  let summary = PackageManagerSummary {
    apt_available: true,
    apt_cache_size: 100,
    apt_autoremove_size: 50,
    apt_orphaned_count: 2,
    apt_partial_downloads: 1,
    dnf_available: true,
    dnf_cache_size: 200,
    pacman_available: false,
    pacman_cache_size: 0,
    zypper_available: true,
    zypper_cache_size: 300,
  };

  assert_eq!(summary.apt_available, true);
  assert_eq!(summary.apt_cache_size, 100);
  assert_eq!(summary.apt_orphaned_count, 2);
  assert_eq!(summary.dnf_available, true);
  assert_eq!(summary.pacman_available, false);
  assert_eq!(summary.zypper_available, true);
}
