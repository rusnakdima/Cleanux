use cleanux_lib::errors::AppError;

#[test]
fn test_app_error_io() {
  let error = AppError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
  assert!(error.to_string().contains("IO error"));
}

#[test]
fn test_app_error_invalid_path() {
  let error = AppError::InvalidPath("/bad/path".to_string());
  assert!(error.to_string().contains("/bad/path"));
}

#[test]
fn test_app_error_permission_denied() {
  let error = AppError::PermissionDenied("access denied".to_string());
  assert!(error.to_string().contains("Permission denied"));
}

#[test]
fn test_app_error_process_not_found() {
  let error = AppError::ProcessNotFound(12345);
  assert!(error.to_string().contains("12345"));
}

#[test]
fn test_app_error_service_not_found() {
  let error = AppError::ServiceNotFound("nginx.service".to_string());
  assert!(error.to_string().contains("nginx.service"));
}

#[test]
fn test_app_error_unknown() {
  let error = AppError::Unknown("something went wrong".to_string());
  assert!(error.to_string().contains("something went wrong"));
}

#[test]
fn test_app_error_message() {
  let error = AppError::message("custom error message");
  assert_eq!(error.to_string(), "custom error message");
}
