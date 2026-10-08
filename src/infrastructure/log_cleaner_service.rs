//! Log cleaner service
//!
//! Provides log scanning, analysis, and cleaning functionality.

use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Log entry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
  pub path: PathBuf,
  pub size_bytes: u64,
  pub modified_secs: u64,
}

/// Log size summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogSizes {
  pub total_bytes: u64,
  pub journal_bytes: u64,
  pub syslog_bytes: u64,
  pub apt_bytes: u64,
  pub dnf_bytes: u64,
  pub journal_count: u64,
}

/// Result of cleaning operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanResult {
  pub files_removed: u64,
  pub bytes_freed: u64,
  pub output: String,
}

pub struct LogCleanerService;

impl LogCleanerService {
  pub fn new() -> Self {
    Self
  }

  /// Scans for log files and returns their details.
  pub async fn scan_logs(&self) -> Result<Vec<LogEntry>> {
    let log_dirs = vec!["/var/log"];

    let mut entries = Vec::new();

    for dir in log_dirs {
      let path = PathBuf::from(dir);
      if !path.exists() {
        continue;
      }

      let mut cmd = tokio::process::Command::new("find");
      cmd
        .arg(dir)
        .args(["-type", "f", "-name", "*.log", "-o", "-name", "*.log.*"])
        .args(["-printf", "%p %s %T+\n"]);

      let output = cmd
        .output()
        .await
        .map_err(|e| AppError::System(e.to_string()))?;

      if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
          let parts: Vec<&str> = line.splitn(3, ' ').collect();
          if parts.len() >= 3 {
            let path = PathBuf::from(parts[0]);
            let size_bytes = parts[1].parse::<u64>().unwrap_or(0);
            let modified_secs = parts[2].parse::<u64>().unwrap_or(0);

            entries.push(LogEntry {
              path,
              size_bytes,
              modified_secs,
            });
          }
        }
      }
    }

    Ok(entries)
  }

  /// Gets sizes of all log categories.
  pub async fn get_log_sizes(&self) -> Result<LogSizes> {
    let mut total_bytes: u64 = 0;
    let mut journal_bytes: u64 = 0;
    let mut syslog_bytes: u64 = 0;
    let mut apt_bytes: u64 = 0;
    let mut dnf_bytes: u64 = 0;
    let mut journal_count: u64 = 0;

    // Get journald size
    let journal_output = tokio::process::Command::new("du")
      .args(["-sb", "/var/log/journal"])
      .output()
      .await
      .map_err(|e| AppError::System(e.to_string()))?;

    if journal_output.status.success() {
      let stdout = String::from_utf8_lossy(&journal_output.stdout);
      if let Some(size) = stdout
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<u64>().ok())
      {
        journal_bytes = size;
        total_bytes += size;
      }
    }

    // Count journal files
    let journal_count_output = tokio::process::Command::new("find")
      .args(["/var/log/journal", "-type", "f"])
      .output()
      .await
      .map_err(|e| AppError::System(e.to_string()))?;

    if journal_count_output.status.success() {
      let stdout = String::from_utf8_lossy(&journal_count_output.stdout);
      journal_count = stdout.lines().count() as u64;
    }

    // Get syslog size
    let syslog_output = tokio::process::Command::new("du")
      .args(["-sb", "/var/log/syslog"])
      .output()
      .await
      .map_err(|e| AppError::System(e.to_string()))?;

    if syslog_output.status.success() {
      let stdout = String::from_utf8_lossy(&syslog_output.stdout);
      if let Some(size) = stdout
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<u64>().ok())
      {
        syslog_bytes = size;
        total_bytes += size;
      }
    }

    // Get apt history size
    let apt_output = tokio::process::Command::new("du")
      .args(["-sb", "/var/log/apt"])
      .output()
      .await
      .map_err(|e| AppError::System(e.to_string()))?;

    if apt_output.status.success() {
      let stdout = String::from_utf8_lossy(&apt_output.stdout);
      if let Some(size) = stdout
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<u64>().ok())
      {
        apt_bytes = size;
        total_bytes += size;
      }
    }

    // Get dnf log size
    let dnf_output = tokio::process::Command::new("du")
      .args(["-sb", "/var/log/dnf"])
      .output()
      .await
      .map_err(|e| AppError::System(e.to_string()))?;

    if dnf_output.status.success() {
      let stdout = String::from_utf8_lossy(&dnf_output.stdout);
      if let Some(size) = stdout
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<u64>().ok())
      {
        dnf_bytes = size;
        total_bytes += size;
      }
    }

    Ok(LogSizes {
      total_bytes,
      journal_bytes,
      syslog_bytes,
      apt_bytes,
      dnf_bytes,
      journal_count,
    })
  }

  /// Cleans old log files (rotated/compressed logs older than 7 days).
  pub async fn clean_old_logs(&self, days: u32) -> Result<CleanResult> {
    let mut files_removed: u64 = 0;
    let mut bytes_freed: u64 = 0;

    // Find and remove old rotated logs
    let output = tokio::process::Command::new("find")
      .args([
        "/var/log",
        "-type",
        "f",
        "(",
        "-name",
        "*.log.*",
        "-o",
        "-name",
        "*.log-[0-9]*",
        "-o",
        "-name",
        "*.gz",
        "-o",
        "-name",
        "*.bz2",
        "-o",
        "-name",
        "*.xz",
        ")",
        "-mtime",
        &format!("+{}", days),
      ])
      .output()
      .await
      .map_err(|e| AppError::System(e.to_string()))?;

    if output.status.success() {
      let stdout = String::from_utf8_lossy(&output.stdout);
      let files: Vec<&str> = stdout.lines().collect();

      for file in &files {
        // Get file size before removal
        let size_output = tokio::process::Command::new("stat")
          .args(["-c", "%s", file])
          .output()
          .await;

        if let Ok(size_out) = size_output {
          if let Ok(size_str) = String::from_utf8(size_out.stdout) {
            if let Ok(size) = size_str.trim().parse::<u64>() {
              bytes_freed += size;
            }
          }
        }

        // Remove the file
        let remove_result = tokio::process::Command::new("rm")
          .arg("-f")
          .arg(file)
          .output()
          .await;

        if remove_result.is_ok() {
          files_removed += 1;
        }
      }
    }

    Ok(CleanResult {
      files_removed,
      bytes_freed,
      output: format!(
        "Removed {} files, freed {} bytes",
        files_removed, bytes_freed
      ),
    })
  }

  /// Clears the systemd journal.
  pub async fn clear_journal(&self, vacuum_size: Option<&str>) -> Result<CleanResult> {
    let mut args = vec!["--no-pager", "--flush", "journalctl"];

    let (_action, output_str) = if let Some(size) = vacuum_size {
      args.push("--vacuum-size");
      args.push(size);
      ("vacuum", format!("Journal vacuumed to {}", size))
    } else {
      args.push("--rotate");
      ("rotate", "Journal rotated".to_string())
    };

    let output = tokio::process::Command::new("sudo")
      .args(args)
      .output()
      .await
      .map_err(|e| {
        if e.to_string().contains("Permission denied")
          || e.to_string().contains("Operation not permitted")
        {
          AppError::PermissionDenied("Root privileges required to clear journal".to_string())
        } else {
          AppError::System(e.to_string())
        }
      })?;

    if output.status.success() {
      Ok(CleanResult {
        files_removed: 1,
        bytes_freed: 0,
        output: output_str,
      })
    } else {
      let stderr = String::from_utf8_lossy(&output.stderr);
      Err(AppError::System(stderr.to_string()))
    }
  }
}

impl Default for LogCleanerService {
  fn default() -> Self {
    Self::new()
  }
}
