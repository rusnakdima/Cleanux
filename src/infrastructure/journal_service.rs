//! JournalService — reads system logs via journalctl.

use crate::error::AppError;
use serde::{Deserialize, Serialize};

pub type Result<T, E = AppError> = std::result::Result<T, E>;

/// A single log entry from journalctl.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
  pub timestamp: String,
  pub level: String,
  pub message: String,
  pub source: String,
}

/// JournalService for reading system logs.
pub struct JournalService;

impl JournalService {
  /// Get recent log entries.
  pub async fn get_recent_logs(lines: usize) -> Result<Vec<LogEntry>> {
    let output = tokio::process::Command::new("journalctl")
      .args(["-n", &lines.to_string()])
      .args(["--output=short-iso"])
      .output()
      .await
      .map_err(|e| AppError::Internal(e.to_string().into()))?;

    if !output.status.success() {
      return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(Self::parse_journal_output(&stdout))
  }

  /// Search logs matching a query string.
  pub async fn search_logs(query: &str) -> Result<Vec<LogEntry>> {
    let output = tokio::process::Command::new("journalctl")
      .args(["--grep", query])
      .args(["--output=short-iso"])
      .output()
      .await
      .map_err(|e| AppError::Internal(e.to_string().into()))?;

    if !output.status.success() {
      return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(Self::parse_journal_output(&stdout))
  }

  /// Export logs in the specified format ("json" or "text").
  pub async fn export_logs(&self, format: &str) -> Result<String> {
    let output = tokio::process::Command::new("journalctl")
      .args(["-n", "1000"])
      .args(["--output=short-iso"])
      .output()
      .await
      .map_err(|e| AppError::Internal(e.to_string().into()))?;

    if !output.status.success() {
      return Ok(String::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    match format {
      "json" => {
        let entries = Self::parse_journal_output(&stdout);
        serde_json::to_string_pretty(&entries).map_err(|e| AppError::Internal(e.to_string().into()))
      }
      _ => Ok(stdout.to_string()),
    }
  }

  fn parse_journal_output(output: &str) -> Vec<LogEntry> {
    output
      .lines()
      .filter_map(|line| {
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        if parts.len() >= 3 {
          let timestamp = parts[0].to_string();
          let level = parts[1].replace(['[', ']'], "").to_lowercase();
          let message = parts[2..].join(" ");
          let source = "systemd".to_string();
          Some(LogEntry {
            timestamp,
            level,
            message,
            source,
          })
        } else {
          None
        }
      })
      .collect()
  }
}
