use cleanux_lib::services::kernel_cleaner_service::KernelInfo;
#[test]
fn test_kernel_info_empty_version() {
  let kernel = KernelInfo {
    version: String::new(),
    path: String::new(),
    size: 0,
    is_current: false,
  };
  assert!(kernel.version.is_empty());
  assert!(!kernel.is_current);
}
#[test]
fn test_kernel_info_is_current() {
  let kernel = KernelInfo {
    version: "5.4.0".to_string(),
    path: "/lib/modules/5.4.0".to_string(),
    size: 100,
    is_current: true,
  };
  assert!(kernel.is_current);
}
#[test]
fn test_kernel_info_not_current() {
  let kernel = KernelInfo {
    version: "5.4.0".to_string(),
    path: "/lib/modules/5.4.0".to_string(),
    size: 100,
    is_current: false,
  };
  assert!(!kernel.is_current);
}
