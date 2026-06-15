use std::fmt;

#[derive(Clone, Debug)]
pub enum AppError {
  NotFound(String),
  ValidationError(String),
  Duplicate(String),
  Unauthorized,
  Forbidden,
  Internal(String),
  Database(String),
  Network(String),
  Io(String),
  PermissionDenied(String),
  InvalidPath(String),
  PathOutsideAllowed(String),
  ServiceNotFound(String),
  ProcessNotFound(u32),
  BackupFailed(String),
  Unknown(String),
  Message(String),
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::NotFound(msg) => write!(f, "Not found: {}", msg),
      Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
      Self::Duplicate(msg) => write!(f, "Duplicate: {}", msg),
      Self::Unauthorized => write!(f, "Unauthorized"),
      Self::Forbidden => write!(f, "Forbidden"),
      Self::Internal(msg) => write!(f, "Internal error: {}", msg),
      Self::Database(msg) => write!(f, "Database error: {}", msg),
      Self::Network(msg) => write!(f, "Network error: {}", msg),
      Self::Io(msg) => write!(f, "IO error: {}", msg),
      Self::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
      Self::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
      Self::PathOutsideAllowed(msg) => write!(f, "Path outside allowed directories: {}", msg),
      Self::ServiceNotFound(msg) => write!(f, "Service not found: {}", msg),
      Self::ProcessNotFound(pid) => write!(f, "Process not found: {}", pid),
      Self::BackupFailed(msg) => write!(f, "Backup failed: {}", msg),
      Self::Unknown(msg) => write!(f, "Unknown error: {}", msg),
      Self::Message(msg) => write!(f, "{}", msg),
    }
  }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
  fn from(err: std::io::Error) -> Self {
    Self::Io(err.to_string())
  }
}

impl From<serde_json::Error> for AppError {
  fn from(err: serde_json::Error) -> Self {
    Self::Database(err.to_string())
  }
}

impl AppError {
  pub fn message(text: impl Into<String>) -> Self {
    Self::Message(text.into())
  }

  pub fn not_found(entity: &str) -> Self {
    Self::NotFound(entity.into())
  }

  pub fn invalid_path(path: &str) -> Self {
    Self::InvalidPath(path.into())
  }

  pub fn permission_denied(path: &str) -> Self {
    Self::PermissionDenied(path.into())
  }

  pub fn into_response(self) -> crate::models::ResponseModel {
    crate::models::ResponseModel {
      status: crate::models::ResponseStatus::Error,
      message: self.to_string(),
      data: crate::models::DataValue::String(String::new()),
    }
  }
}
