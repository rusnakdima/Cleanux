use cleanux_lib::security::privilege::PrivilegeOperation;
use cleanux_lib::security::sanitize_path;

#[test]
fn test_privilege_operation_new() {
  let op = PrivilegeOperation::new("DELETE", "/tmp/test");
  assert_eq!(op.operation, "DELETE");
  assert_eq!(op.target, "/tmp/test");
  assert!(!op.timestamp.is_empty());
}

#[test]
fn test_privilege_operation_with_result() {
  let op = PrivilegeOperation::new("DELETE", "/tmp/test").with_result("SUCCESS");
  assert_eq!(op.result, "SUCCESS");
}

#[test]
fn test_sanitize_path_prevents_traversal_removal() {
  let result = sanitize_path("/home/user/../etc/passwd");
  assert!(
    !result.contains(".."),
    "path should not contain '..' sequences: {}",
    result
  );
  assert!(
    result.contains("/home"),
    "path should still contain /home: {}",
    result
  );
}

#[test]
fn test_sanitize_path_removes_null_bytes() {
  let result = sanitize_path("/home/user\0/../etc");
  assert!(!result.contains('\0'), "path should not contain null bytes");
}

#[test]
fn test_sanitize_path_handles_consecutive_dots() {
  let result = sanitize_path("/foo/....//....//bar");
  assert!(
    !result.contains(".."),
    "path should not contain '..' sequences: {}",
    result
  );
}

#[test]
fn test_sanitize_path_preserves_valid_paths() {
  assert_eq!(
    sanitize_path("/home/user/Documents"),
    "/home/user/Documents"
  );
  assert_eq!(sanitize_path("/tmp/cleanux"), "/tmp/cleanux");
}
