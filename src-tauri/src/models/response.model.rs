/* sys lib */
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
  Success,
  Info,
  Warning,
  Error,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum DataValue {
  String(String),
  Number(f64),
  Bool(bool),
  Array(Vec<serde_json::Value>),
  Object(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseModel {
  pub status: ResponseStatus,
  pub message: String,
  pub data: DataValue,
}

impl From<Box<dyn std::error::Error + Send + Sync>> for ResponseModel {
  fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
    ResponseModel {
      status: ResponseStatus::Error,
      message: error.to_string(),
      data: DataValue::String("".to_string()),
    }
  }
}

impl From<String> for ResponseModel {
  fn from(error: String) -> Self {
    ResponseModel {
      status: ResponseStatus::Error,
      message: error,
      data: DataValue::String("".to_string()),
    }
  }
}

impl From<&str> for ResponseModel {
  fn from(error: &str) -> Self {
    ResponseModel {
      status: ResponseStatus::Error,
      message: error.to_string(),
      data: DataValue::String("".to_string()),
    }
  }
}

impl From<AppError> for ResponseModel {
  fn from(error: AppError) -> Self {
    error.into_response()
  }
}

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

  pub fn into_response(self) -> ResponseModel {
    ResponseModel {
      status: ResponseStatus::Error,
      message: self.to_string(),
      data: DataValue::String(String::new()),
    }
  }
}
