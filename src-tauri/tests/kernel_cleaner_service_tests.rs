use cleanux_lib::models::{DataValue, ResponseStatus};
use cleanux_lib::services::kernel_cleaner_service::{
  BootSpaceInfo, InitramfsInfo, KernelCleanerService, KernelInfo,
};

#[test]
fn test_kernel_info_serialization() {
  let kernel_info = KernelInfo {
    version: "5.4.0- generic".to_string(),
    path: "/lib/modules/5.4.0- generic".to_string(),
    size: 102400,
    is_current: true,
  };

  let json = serde_json::to_string(&kernel_info).unwrap();
  assert!(json.contains("5.4.0- generic"));
  assert!(json.contains("102400"));
  assert!(json.contains("\"is_current\":true"));
}

#[test]
fn test_initramfs_info_serialization() {
  let initramfs_info = InitramfsInfo {
    version: "5.4.0- generic".to_string(),
    path: "/boot/initrd.img-5.4.0- generic".to_string(),
    size: 51200,
  };

  let json = serde_json::to_string(&initramfs_info).unwrap();
  assert!(json.contains("5.4.0- generic"));
  assert!(json.contains("51200"));
}

#[test]
fn test_boot_space_info_serialization() {
  let boot_space_info = BootSpaceInfo {
    total: 1000000,
    used: 500000,
    available: 500000,
    usage_percent: 50.0,
  };

  let json = serde_json::to_string(&boot_space_info).unwrap();
  assert!(json.contains("1000000"));
  assert!(json.contains("500000"));
  assert!(json.contains("50.0"));
}

#[test]
fn test_get_current_kernel() {
  let service = KernelCleanerService;
  let result = service.get_current_kernel();
  assert!(!result.is_empty());
}

#[test]
fn test_get_installed_kernels() {
  let service = KernelCleanerService;
  let kernels = service.get_installed_kernels();
  assert!(!kernels.is_empty());
  for kernel in &kernels {
    assert!(!kernel.version.is_empty());
    assert!(!kernel.path.is_empty());
  }
}

#[test]
fn test_get_old_kernels() {
  let service = KernelCleanerService;
  let old_kernels = service.get_old_kernels();
  for kernel in &old_kernels {
    assert!(!kernel.is_current);
  }
}

#[test]
fn test_get_old_kernels_size() {
  let service = KernelCleanerService;
  let size = service.get_old_kernels_size();
  assert!(size >= 0);
}

#[test]
fn test_get_old_initramfs() {
  let service = KernelCleanerService;
  let initramfs_files = service.get_old_initramfs();
  for initramfs in &initramfs_files {
    assert!(!initramfs.version.is_empty());
    assert!(!initramfs.path.is_empty());
  }
}

#[test]
fn test_get_boot_space_info() {
  let service = KernelCleanerService;
  let boot_space = service.get_boot_space_info();
  assert!(boot_space.total >= 0);
  assert!(boot_space.usage_percent >= 0.0);
  assert!(boot_space.usage_percent <= 100.0);
}

#[test]
fn test_remove_kernel_validation_current() {
  let service = KernelCleanerService;
  let current = service.get_current_kernel();
  let result = service.remove_kernel(&current);
  assert!(result.is_err());
  let response = result.unwrap_err();
  assert_eq!(response.status, ResponseStatus::Error);
  assert!(response.message.contains("currently running kernel"));
}

#[test]
fn test_remove_kernel_validation_not_found() {
  let service = KernelCleanerService;
  let result = service.remove_kernel("nonexistent-kernel-version-12345");
  assert!(result.is_err());
  let response = result.unwrap_err();
  assert_eq!(response.status, ResponseStatus::Error);
  assert!(response.message.contains("not found"));
}

#[test]
fn test_remove_initramfs_nonexistent() {
  let service = KernelCleanerService;
  let result = service.remove_initramfs("nonexistent-initramfs-version-12345");
  assert!(result.is_ok());
  let response = result.unwrap();
  assert_eq!(response.status, ResponseStatus::Success);
}

#[test]
fn test_update_grub_returns_result() {
  let service = KernelCleanerService;
  let result = service.update_grub();
  match result {
    Ok(response) => {
      assert_eq!(response.status, ResponseStatus::Success);
    }
    Err(response) => {
      assert_eq!(response.status, ResponseStatus::Error);
    }
  }
}
