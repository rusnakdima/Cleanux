//! Structured Logging Plugin handlers for Cleanux
//!
//! Provides log export and query functionality.

use crate::error::AppError;
use crate::response::Response;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Mutex;

/// A single structured log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
  pub timestamp: String,
  pub level: String,
  pub message: String,
  pub target: Option<String>,
}

/// Log export format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogExport {
  pub format: String,
  pub entries: Vec<LogEntry>,
  pub total_count: usize,
}

/// Log query filter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogFilter {
  pub level: Option<String>,
  pub since: Option<String>,
  pub until: Option<String>,
  pub target: Option<String>,
  pub limit: Option<usize>,
}

/// In-memory log buffer (last 1000 entries)
static LOG_BUFFER: std::sync::LazyLock<Mutex<VecDeque<LogEntry>>> =
  std::sync::LazyLock::new(|| Mutex::new(VecDeque::new()));

/// Record a log entry into the in-memory buffer
fn record_log(level: &str, message: &str, target: Option<String>) {
  let entry = LogEntry {
    timestamp: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
    level: level.to_string(),
    message: message.to_string(),
    target,
  };

  if let Ok(mut buffer) = LOG_BUFFER.lock() {
    if buffer.len() >= 1000 {
      buffer.pop_front();
    }
    buffer.push_back(entry);
  }
}

/// Export logs in structured format (JSON)
pub async fn logs_export(filter: LogFilter) -> Result<Response<LogExport>, AppError> {
  let buffer = LOG_BUFFER
    .lock()
    .map_err(|e| AppError::Lock(e.to_string()))?;

  let entries: Vec<LogEntry> = buffer
    .iter()
    .filter(|e| {
      if let Some(level) = &filter.level {
        if &e.level != level {
          return false;
        }
      }
      if let Some(since) = &filter.since {
        if e.timestamp < *since {
          return false;
        }
      }
      if let Some(until) = &filter.until {
        if e.timestamp > *until {
          return false;
        }
      }
      if let Some(target) = &filter.target {
        if e.target.as_deref() != Some(target) {
          return false;
        }
      }
      true
    })
    .cloned()
    .collect();

  let total_count = entries.len();
  let limit = filter.limit.unwrap_or(entries.len());
  let entries: Vec<LogEntry> = entries.into_iter().rev().take(limit).collect();

  Ok(Response::success(
    LogExport {
      format: "json".to_string(),
      entries,
      total_count,
    },
    Some("Logs exported"),
  ))
}

/// Query logs with optional filters
pub async fn logs_query(filter: LogFilter) -> Result<Response<Vec<LogEntry>>, AppError> {
  let buffer = LOG_BUFFER
    .lock()
    .map_err(|e| AppError::Lock(e.to_string()))?;

  let entries: Vec<LogEntry> = buffer
    .iter()
    .filter(|e| {
      if let Some(level) = &filter.level {
        if !level.split(',').any(|l| e.level == l.trim()) {
          return false;
        }
      }
      if let Some(since) = &filter.since {
        if e.timestamp < *since {
          return false;
        }
      }
      if let Some(until) = &filter.until {
        if e.timestamp > *until {
          return false;
        }
      }
      if let Some(target) = &filter.target {
        if e.target.as_deref() != Some(target) {
          return false;
        }
      }
      true
    })
    .cloned()
    .collect();

  let total_count = entries.len();
  let limit = filter.limit.unwrap_or(entries.len());
  let entries: Vec<LogEntry> = entries.into_iter().rev().take(limit).collect();

  Ok(Response::success(
    entries,
    Some(&format!("{} entries found", total_count)),
  ))
}

/// Record a debug log entry
pub fn log_debug(message: &str, target: Option<String>) {
  record_log("DEBUG", message, target);
}

/// Record an info log entry
pub fn log_info(message: &str, target: Option<String>) {
  record_log("INFO", message, target);
}

/// Record a warn log entry
pub fn log_warn(message: &str, target: Option<String>) {
  record_log("WARN", message, target);
}

/// Record an error log entry
pub fn log_error(message: &str, target: Option<String>) {
  record_log("ERROR", message, target);
}
