use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
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
    match err.kind() {
      std::io::ErrorKind::NotFound => Self::NotFound(err.to_string()),
      std::io::ErrorKind::PermissionDenied => Self::PermissionDenied(err.to_string()),
      _ => Self::Io(err.to_string()),
    }
  }
}
impl From<serde_json::Error> for AppError {
  fn from(err: serde_json::Error) -> Self {
    Self::ValidationError(err.to_string())
  }
}
impl From<nosql_orm::error::OrmError> for AppError {
  fn from(err: nosql_orm::error::OrmError) -> Self {
    use nosql_orm::error::OrmError;
    match err {
      OrmError::NotFound(_) => Self::NotFound("Entity".into()),
      OrmError::Duplicate(_) => Self::Duplicate("Entity".into()),
      _ => Self::Database(err.to_string()),
    }
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
  pub fn into_response(self) -> crate::models::Response<serde_json::Value> {
    use crate::models::{Response, Status};
    match self {
      Self::NotFound(msg) => Response::error(Status::NotFound, format!("Not found: {}", msg)),
      Self::ValidationError(msg) => Response::error(
        Status::ValidationError,
        format!("Validation error: {}", msg),
      ),
      Self::Duplicate(msg) => Response::error(Status::Error, format!("Duplicate: {}", msg)),
      Self::Unauthorized => Response::error(Status::Unauthorized, "Unauthorized"),
      Self::Forbidden => Response::error(Status::Forbidden, "Forbidden"),
      Self::Internal(msg) => Response::error(Status::Error, format!("Internal error: {}", msg)),
      Self::Database(msg) => Response::error(Status::Error, format!("Database error: {}", msg)),
      Self::Network(msg) => Response::error(Status::Error, format!("Network error: {}", msg)),
      Self::Io(msg) => Response::error(Status::Error, format!("IO error: {}", msg)),
      Self::PermissionDenied(msg) => {
        Response::error(Status::Error, format!("Permission denied: {}", msg))
      }
      Self::InvalidPath(msg) => Response::error(Status::Error, format!("Invalid path: {}", msg)),
      Self::PathOutsideAllowed(msg) => Response::error(
        Status::Error,
        format!("Path outside allowed directories: {}", msg),
      ),
      Self::ServiceNotFound(msg) => {
        Response::error(Status::Error, format!("Service not found: {}", msg))
      }
      Self::ProcessNotFound(pid) => {
        Response::error(Status::Error, format!("Process not found: {}", pid))
      }
      Self::BackupFailed(msg) => Response::error(Status::Error, format!("Backup failed: {}", msg)),
      Self::Unknown(msg) => Response::error(Status::Error, format!("Unknown error: {}", msg)),
      Self::Message(msg) => Response::error(Status::Error, msg),
    }
  }
}
