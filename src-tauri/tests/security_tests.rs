use cleanux_lib::security::privilege::{requires_confirmation, PrivilegeOperation};

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
fn test_requires_confirmation_dangerous_commands() {
  assert!(requires_confirmation("rm -rf /"));
  assert!(requires_confirmation("rm -rf /home/*"));
  assert!(requires_confirmation("dd"));
  assert!(requires_confirmation("mkfs"));
}

#[test]
fn test_requires_confirmation_safe_commands() {
  assert!(!requires_confirmation("ls"));
  assert!(!requires_confirmation("rm /tmp/test"));
  assert!(!requires_confirmation("cat /etc/passwd"));
}
