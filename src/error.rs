//! Error types for Cleanux application.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
  RequestFailed(String),
  Lock(String),
  AlgorithmNotFound(String),
  Serialization(String),
  Initialization(String),
  Config(String),
  /// System-level errors (syscalls, journal, process management).
  System(String),
  /// AI service errors (Ollama, Vosk, transcription, TTS).
  Ai(String),
}

impl std::fmt::Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
      AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
      AppError::Duplicate(msg) => write!(f, "Duplicate: {}", msg),
      AppError::Unauthorized => write!(f, "Unauthorized"),
      AppError::Forbidden => write!(f, "Forbidden"),
      AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
      AppError::Database(msg) => write!(f, "Database error: {}", msg),
      AppError::Network(msg) => write!(f, "Network error: {}", msg),
      AppError::Io(msg) => write!(f, "I/O error: {}", msg),
      AppError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
      AppError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
      AppError::RequestFailed(msg) => write!(f, "Request failed: {}", msg),
      AppError::Lock(msg) => write!(f, "Lock error: {}", msg),
      AppError::AlgorithmNotFound(msg) => write!(f, "Algorithm not found: {}", msg),
      AppError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
      AppError::Initialization(msg) => write!(f, "Initialization error: {}", msg),
      AppError::Config(msg) => write!(f, "Config error: {}", msg),
      AppError::System(msg) => write!(f, "System error: {}", msg),
      AppError::Ai(msg) => write!(f, "AI error: {}", msg),
    }
  }
}

impl std::error::Error for AppError {}

impl From<String> for AppError {
  fn from(s: String) -> Self {
    AppError::Internal(s)
  }
}

impl From<&str> for AppError {
  fn from(s: &str) -> Self {
    AppError::Internal(s.to_string())
  }
}

impl From<std::io::Error> for AppError {
  fn from(e: std::io::Error) -> Self {
    AppError::Io(format!("{}", e))
  }
}

impl<T> From<std::sync::PoisonError<T>> for AppError {
  fn from(e: std::sync::PoisonError<T>) -> Self {
    AppError::Lock(format!("{}", e))
  }
}

impl AppError {
  pub fn internal(msg: impl Into<String>) -> Self {
    AppError::Internal(msg.into())
  }

  pub fn not_found(msg: impl Into<String>) -> Self {
    AppError::NotFound(msg.into())
  }

  pub fn validation(msg: impl Into<String>) -> Self {
    AppError::ValidationError(msg.into())
  }

  pub fn is_not_found(&self) -> bool {
    matches!(self, AppError::NotFound(_))
  }
}

/// Result type alias for Cleanux.
pub type Result<T> = std::result::Result<T, AppError>;
