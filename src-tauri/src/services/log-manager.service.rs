/* helpers */
use crate::utils::{
  data_string, format_size, get_dir_size, stderr_string, stdout_string, success_response,
};
/* models */
use crate::models::{AppError, Response};
/* sys lib */
use chrono::{DateTime, Local};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

pub struct LogManagerService;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JournalInfo {
  pub size_bytes: u64,
  pub size_human: String,
  pub oldest_entry: Option<String>,
  pub newest_entry: Option<String>,
  pub is_active: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RotatedLogInfo {
  pub path: String,
  pub size_bytes: u64,
  pub size_human: String,
  pub modified: String,
  pub compression_ratio: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogrotateConfig {
  pub path: String,
  pub enabled: bool,
  pub schedule: Option<String>,
  pub max_size: Option<String>,
  pub max_age: Option<String>,
  pub compress: bool,
  pub rotate_count: Option<u32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogrotateAnalysis {
  pub total_configs: usize,
  pub enabled_configs: usize,
  pub configs: Vec<LogrotateConfig>,
  pub potential_savings_mb: u64,
  pub issues: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VarLogUsage {
  pub total_bytes: u64,
  pub total_human: String,
  pub file_count: usize,
  pub directory_count: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogFileInfo {
  pub path: String,
  pub size_bytes: u64,
  pub size_human: String,
  pub modified: String,
  pub file_type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogManagerSummary {
  pub journal_size_bytes: u64,
  pub journal_size_human: String,
  pub rotated_logs_size_bytes: u64,
  pub rotated_logs_size_human: String,
  pub var_log_size_bytes: u64,
  pub var_log_size_human: String,
  pub logrotate_configs_count: usize,
  pub potential_savings_mb: u64,
}

impl LogManagerService {
  fn get_journal_size_inner() -> Result<u64, AppError> {
    let journal_path = Path::new("/var/log/journal");
    if !journal_path.exists() {
      return Ok(0);
    }

    let mut total_size = 0u64;
    if let Ok(entries) = fs::read_dir(journal_path) {
      for entry in entries.flatten() {
        if entry.path().is_dir() {
          total_size += get_dir_size(&entry.path());
        }
      }
    }

    Ok(total_size)
  }

  fn get_journal_oldest_entry() -> Option<String> {
    let output = Command::new("journalctl")
      .args(["--list-boots", "-q"])
      .output()
      .ok()?;

    let stdout = stdout_string(&output);
    stdout.lines().last().map(|s| s.to_string())
  }

  fn get_journal_newest_entry() -> Option<String> {
    let output = Command::new("journalctl")
      .args(["--list-boots", "-q"])
      .output()
      .ok()?;

    let stdout = stdout_string(&output);
    stdout.lines().next().map(|s| s.to_string())
  }

  fn is_journal_active() -> bool {
    Path::new("/var/log/journal").exists()
  }

  fn extract_logrotate_value(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
      let line = line.trim();
      if line.starts_with(key) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
          return Some(parts[1].to_string());
        }
      }
    }
    None
  }

  fn extract_schedule(content: &str) -> Option<String> {
    let schedules = vec!["daily", "weekly", "monthly"];
    for line in content.lines() {
      let line = line.trim();
      for schedule in &schedules {
        if line == *schedule || line.starts_with(*schedule) {
          return Some(schedule.to_string());
        }
      }
    }
    None
  }

  fn extract_rotate_count(content: &str) -> Option<u32> {
    for line in content.lines() {
      let line = line.trim();
      if line.starts_with("rotate") {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
          return parts[1].parse().ok();
        }
      }
    }
    None
  }

  pub fn get_journal_size() -> u64 {
    Self::get_journal_size_inner().unwrap_or(0)
  }

  pub fn get_journal_usage() -> JournalInfo {
    let size_bytes = Self::get_journal_size();
    JournalInfo {
      size_bytes,
      size_human: format_size(size_bytes),
      oldest_entry: Self::get_journal_oldest_entry(),
      newest_entry: Self::get_journal_newest_entry(),
      is_active: Self::is_journal_active(),
    }
  }

  fn prepare_journal_vacuum() {
    let _ = Command::new("journalctl")
      .args(["--rotate", "--vacuum-time=1s"])
      .output();

    let _ = Command::new("journalctl").args(["--flush"]).output();
  }

  pub fn vacuum_journal(
    size_mb: u32,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    Self::prepare_journal_vacuum();

    let vacuum_output = Command::new("journalctl")
      .args(&[format!("--vacuum-size={}M", size_mb)])
      .output();

    match vacuum_output {
      Ok(output) => {
        if output.status.success() {
          let before = Self::get_journal_size_inner().unwrap_or(0);
          Ok(success_response(
            format!("Journal vacuumed to {} MB", size_mb),
            data_string(format!("before:{}", before)),
          ))
        } else {
          let stderr = stderr_string(&output);
          Err(AppError::message(format!("Failed to vacuum journal: {}", stderr)).into_response())
        }
      }
      Err(e) => {
        Err(AppError::message(format!("Failed to execute journalctl: {}", e)).into_response())
      }
    }
  }

  pub fn vacuum_journal_by_days(
    days: u32,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    Self::prepare_journal_vacuum();

    let vacuum_output = Command::new("journalctl")
      .args(&[format!("--vacuum-time={}d", days)])
      .output();

    match vacuum_output {
      Ok(output) => {
        if output.status.success() {
          let before = Self::get_journal_size_inner().unwrap_or(0);
          Ok(success_response(
            format!("Journal vacuumed to {} days", days),
            data_string(format!("before:{}", before)),
          ))
        } else {
          let stderr = stderr_string(&output);
          Err(AppError::message(format!("Failed to vacuum journal: {}", stderr)).into_response())
        }
      }
      Err(e) => {
        Err(AppError::message(format!("Failed to execute journalctl: {}", e)).into_response())
      }
    }
  }

  pub fn get_rotated_logs_size() -> u64 {
    Self::get_rotated_logs_size_inner().unwrap_or(0)
  }

  fn get_rotated_logs_size_inner() -> Result<u64, AppError> {
    let log_dirs = [
      Path::new("/var/log"),
      Path::new("/var/log/apache2"),
      Path::new("/var/log/nginx"),
      Path::new("/var/log/apache"),
    ];

    let patterns = [".gz", ".old", ".1", ".2", ".bz2", ".xz", ".lz4"];
    let mut total_size = 0u64;

    for dir in log_dirs.iter() {
      if dir.exists() {
        if let Ok(entries) = fs::read_dir(dir) {
          for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
              if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if patterns.iter().any(|p| ext_str.contains(p))
                  || path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.ends_with(".1") || n.ends_with(".old") || n.contains("rotate"))
                    .unwrap_or(false)
                {
                  if let Ok(meta) = fs::metadata(&path) {
                    total_size += meta.len();
                  }
                }
              }
            }
          }
        }
      }
    }

    Ok(total_size)
  }

  pub fn get_rotated_logs() -> Vec<RotatedLogInfo> {
    Self::get_rotated_logs_inner().unwrap_or_default()
  }

  fn get_rotated_logs_inner() -> Result<Vec<RotatedLogInfo>, AppError> {
    let log_dirs = [
      Path::new("/var/log"),
      Path::new("/var/log/apache2"),
      Path::new("/var/log/nginx"),
      Path::new("/var/log/apache"),
    ];

    let patterns = [".gz", ".old", ".1", ".2", ".bz2", ".xz", ".lz4"];
    let mut logs = Vec::new();

    for dir in log_dirs.iter() {
      if dir.exists() {
        if let Ok(entries) = fs::read_dir(dir) {
          for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
              let is_rotated = path
                .extension()
                .map(|e| {
                  let ext = e.to_string_lossy().to_lowercase();
                  patterns.iter().any(|p| ext.contains(p))
                })
                .unwrap_or(false);

              let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

              let is_old_log = filename.ends_with(".1")
                || filename.ends_with(".old")
                || filename.contains("rotate")
                || filename.ends_with(".2")
                || filename.ends_with(".gz");

              if is_rotated || is_old_log {
                if let Ok(meta) = fs::metadata(&path) {
                  let modified: DateTime<Local> =
                    meta.modified().unwrap_or(SystemTime::UNIX_EPOCH).into();

                  logs.push(RotatedLogInfo {
                    path: path.to_string_lossy().into_owned(),
                    size_bytes: meta.len(),
                    size_human: format_size(meta.len()),
                    modified: modified.format("%Y-%m-%d %H:%M:%S").to_string(),
                    compression_ratio: None,
                  });
                }
              }
            }
          }
        }
      }
    }

    logs.sort_by_key(|b| std::cmp::Reverse(b.size_bytes));
    Ok(logs)
  }

  pub fn clean_rotated_logs(
    days: u32,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    Self::clean_rotated_logs_inner(days).map_err(|e| e.into_response())
  }

  fn clean_rotated_logs_inner(days: u32) -> Result<Response<serde_json::Value>, AppError> {
    let cutoff = SystemTime::now()
      .checked_sub(std::time::Duration::from_secs(days as u64 * 86400))
      .unwrap_or(SystemTime::UNIX_EPOCH);

    let logs = Self::get_rotated_logs_inner()?;
    let mut cleaned_count = 0u32;
    let mut errors: Vec<String> = Vec::new();

    for log in logs {
      if let Ok(meta) = fs::metadata(&log.path) {
        if let Ok(modified) = meta.modified() {
          if modified < cutoff {
            match fs::remove_file(&log.path) {
              Ok(_) => cleaned_count += 1,
              Err(e) => errors.push(format!("{}: {}", log.path, e)),
            }
          }
        }
      }
    }

    if errors.is_empty() {
      Ok(success_response(
        format!(
          "Cleaned {} rotated log files older than {} days",
          cleaned_count, days
        ),
        data_string(cleaned_count.to_string()),
      ))
    } else {
      Err(AppError::message(format!(
        "Cleaned {} files, errors: {}",
        cleaned_count,
        errors.join("; ")
      )))
    }
  }

  pub fn get_logrotate_configs() -> Vec<LogrotateConfig> {
    Self::get_logrotate_configs_inner().unwrap_or_default()
  }

  fn get_logrotate_configs_inner() -> Result<Vec<LogrotateConfig>, AppError> {
    let mut configs = Vec::new();

    if Path::new("/etc/logrotate.conf").exists() {
      if let Ok(content) = fs::read_to_string("/etc/logrotate.conf") {
        let enabled = !content.contains("disabled") && !content.contains("#disabled");
        configs.push(LogrotateConfig {
          path: "/etc/logrotate.conf".to_string(),
          enabled,
          schedule: Some("daily".to_string()),
          max_size: Self::extract_logrotate_value(&content, "size"),
          max_age: Self::extract_logrotate_value(&content, "maxage"),
          compress: content.contains("compress"),
          rotate_count: Self::extract_rotate_count(&content),
        });
      }
    }

    if Path::new("/etc/logrotate.d/").exists() {
      if let Ok(entries) = fs::read_dir("/etc/logrotate.d/") {
        for entry in entries.flatten() {
          let path = entry.path();
          if path.is_file() {
            if let Some(name) = path
              .file_name()
              .and_then(|n| n.to_str())
              .map(|s| s.to_string())
            {
              if let Ok(content) = fs::read_to_string(&path) {
                let enabled = !content.contains("disabled") && !content.contains("#disabled");
                configs.push(LogrotateConfig {
                  path: format!("/etc/logrotate.d/{}", name),
                  enabled,
                  schedule: Self::extract_schedule(&content),
                  max_size: Self::extract_logrotate_value(&content, "size"),
                  max_age: Self::extract_logrotate_value(&content, "maxage"),
                  compress: content.contains("compress"),
                  rotate_count: Self::extract_rotate_count(&content),
                });
              }
            }
          }
        }
      }
    }

    Ok(configs)
  }

  pub fn analyze_logrotate() -> LogrotateAnalysis {
    let configs = Self::get_logrotate_configs();
    let total_configs = configs.len();
    let enabled_configs = configs.iter().filter(|c| c.enabled).count();

    let mut potential_savings = 0u64;
    let mut issues = Vec::new();

    for config in &configs {
      if !config.enabled {
        issues.push(format!("{}: config is disabled", config.path));
      }
      if config.max_size.is_none() && config.max_age.is_none() {
        issues.push(format!("{}: no size or age limit set", config.path));
      }
      if !config.compress {
        potential_savings += 50;
      }
    }

    LogrotateAnalysis {
      total_configs,
      enabled_configs,
      configs,
      potential_savings_mb: potential_savings,
      issues,
    }
  }

  pub fn get_var_log_usage() -> VarLogUsage {
    Self::get_var_log_usage_inner().unwrap_or(VarLogUsage {
      total_bytes: 0,
      total_human: "0 B".to_string(),
      file_count: 0,
      directory_count: 0,
    })
  }

  fn get_var_log_usage_inner() -> Result<VarLogUsage, AppError> {
    let log_path = Path::new("/var/log");
    let mut total_size = 0u64;
    let mut file_count = 0usize;
    let mut directory_count = 0usize;

    if log_path.exists() {
      Self::walk_dir(
        log_path,
        &mut total_size,
        &mut file_count,
        &mut directory_count,
      );
    }

    Ok(VarLogUsage {
      total_bytes: total_size,
      total_human: format_size(total_size),
      file_count,
      directory_count,
    })
  }

  fn walk_dir(path: &Path, total_size: &mut u64, file_count: &mut usize, dir_count: &mut usize) {
    if let Ok(entries) = fs::read_dir(path) {
      for entry in entries.flatten() {
        let entry_path = entry.path();
        if entry_path.is_dir() {
          *dir_count += 1;
          Self::walk_dir(&entry_path, total_size, file_count, dir_count);
        } else if entry_path.is_file() {
          *file_count += 1;
          if let Ok(meta) = fs::metadata(&entry_path) {
            *total_size += meta.len();
          }
        }
      }
    }
  }

  pub fn get_largest_log_files(limit: usize) -> Vec<LogFileInfo> {
    Self::get_largest_log_files_inner(limit).unwrap_or_default()
  }

  fn get_largest_log_files_inner(limit: usize) -> Result<Vec<LogFileInfo>, AppError> {
    let log_path = Path::new("/var/log");
    let mut files: Vec<LogFileInfo> = Vec::new();

    if log_path.exists() {
      Self::collect_log_files(log_path, &mut files)?;
    }

    files.sort_by_key(|b| std::cmp::Reverse(b.size_bytes));
    files.truncate(limit);

    Ok(files)
  }

  fn collect_log_files(path: &Path, files: &mut Vec<LogFileInfo>) -> Result<(), AppError> {
    if let Ok(entries) = fs::read_dir(path) {
      for entry in entries.flatten() {
        let entry_path = entry.path();
        if entry_path.is_file() {
          if let Ok(meta) = fs::metadata(&entry_path) {
            let modified: DateTime<Local> =
              meta.modified().unwrap_or(SystemTime::UNIX_EPOCH).into();

            let _filename = entry_path
              .file_name()
              .and_then(|n| n.to_str())
              .unwrap_or("")
              .to_string();

            let extension = entry_path
              .extension()
              .and_then(|e| e.to_str())
              .unwrap_or("")
              .to_string();

            files.push(LogFileInfo {
              path: entry_path.to_string_lossy().into_owned(),
              size_bytes: meta.len(),
              size_human: format_size(meta.len()),
              modified: modified.format("%Y-%m-%d %H:%M:%S").to_string(),
              file_type: if extension.is_empty() {
                "log".to_string()
              } else {
                extension
              },
            });
          }
        }
      }
    }
    Ok(())
  }

  pub fn get_log_manager_summary() -> LogManagerSummary {
    let journal_size = Self::get_journal_size();
    let rotated_size = Self::get_rotated_logs_size();
    let var_log = Self::get_var_log_usage();
    let analysis = Self::analyze_logrotate();

    LogManagerSummary {
      journal_size_bytes: journal_size,
      journal_size_human: format_size(journal_size),
      rotated_logs_size_bytes: rotated_size,
      rotated_logs_size_human: format_size(rotated_size),
      var_log_size_bytes: var_log.total_bytes,
      var_log_size_human: var_log.total_human,
      logrotate_configs_count: analysis.total_configs,
      potential_savings_mb: analysis.potential_savings_mb,
    }
  }
}
