use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
  #[error("IO error: {0}")]
  Io(#[from] std::io::Error),

  #[error("Serialization error: {0}")]
  Serde(#[from] serde_json::Error),

  #[error("Permission denied: {0}")]
  PermissionDenied(String),

  #[error("Invalid path: {0}")]
  InvalidPath(String),

  #[error("Path outside allowed directories: {0}")]
  PathOutsideAllowed(String),

  #[error("Service not found: {0}")]
  ServiceNotFound(String),

  #[error("Process not found: {0}")]
  ProcessNotFound(u32),

  #[error("Backup failed: {0}")]
  BackupFailed(String),

  #[error("Unknown error: {0}")]
  Unknown(String),

  #[error("{0}")]
  Message(String),
}

impl AppError {
  pub fn message(text: impl Into<String>) -> Self {
    AppError::Message(text.into())
  }

  pub fn into_response(self) -> crate::models::ResponseModel {
    crate::models::ResponseModel {
      status: crate::models::ResponseStatus::Error,
      message: self.to_string(),
      data: crate::models::DataValue::String(String::new()),
    }
  }
}
