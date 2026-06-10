/* helpers */
use crate::helpers::{data_string, success_response};
/* models */
use crate::models::{AppError, DataValue, ResponseModel};
/* sys lib */
use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime};

const LOG_CACHE_TTL_SECS: u64 = 45;

#[derive(Debug, Clone)]
struct CachedAnalysis {
  severity: LogSeverity,
  count: u32,
  mtime: SystemTime,
  timestamp: Instant,
}

struct LogAnalysisCache {
  data: Mutex<HashMap<PathBuf, CachedAnalysis>>,
}

impl LogAnalysisCache {
  fn new() -> Self {
    Self {
      data: Mutex::new(HashMap::new()),
    }
  }

  fn get(&self, path: &Path, mtime: SystemTime) -> Option<(LogSeverity, u32)> {
    let guard = self.data.lock().ok()?;
    let cached = guard.get(path)?;
    if cached.mtime == mtime && cached.timestamp.elapsed() < Duration::from_secs(LOG_CACHE_TTL_SECS)
    {
      Some((cached.severity.clone(), cached.count))
    } else {
      None
    }
  }

  fn set(&self, path: PathBuf, mtime: SystemTime, severity: LogSeverity, count: u32) {
    if let Ok(mut guard) = self.data.lock() {
      guard.insert(
        path,
        CachedAnalysis {
          severity,
          count,
          mtime,
          timestamp: Instant::now(),
        },
      );
    }
  }

  fn invalidate(&self, path: &Path) {
    if let Ok(mut guard) = self.data.lock() {
      guard.remove(path);
    }
  }

  fn clear(&self) {
    if let Ok(mut guard) = self.data.lock() {
      guard.clear();
    }
  }
}

static LOG_CACHE: std::sync::OnceLock<LogAnalysisCache> = std::sync::OnceLock::new();

fn get_log_cache() -> &'static LogAnalysisCache {
  LOG_CACHE.get_or_init(LogAnalysisCache::new)
}

pub struct LogAnalyzerService;

#[derive(Debug, Clone)]
pub struct LogEntry {
  pub path: String,
  pub severity: LogSeverity,
  pub count: u32,
  pub last_modified: String,
  pub size: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogSeverity {
  Error,
  Warning,
  Info,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogCategory {
  System,
  Application,
  Security,
  Hardware,
}

type AnalyzerResult<T> = Result<T, AppError>;

impl LogAnalyzerService {
  pub fn get_log_summary(&self) -> Result<ResponseModel, ResponseModel> {
    self.get_log_summary_inner().map_err(|e| e.into_response())
  }

  fn get_log_summary_inner(&self) -> AnalyzerResult<ResponseModel> {
    let entries = self.analyze_logs();
    let mut summary = serde_json::Map::new();

    let mut system_logs = Vec::new();
    let mut application_logs = Vec::new();
    let mut security_logs = Vec::new();
    let mut hardware_logs = Vec::new();

    for entry in entries {
      let category = self.categorize_log(&entry.path);
      match category {
        LogCategory::System => system_logs.push(entry),
        LogCategory::Application => application_logs.push(entry),
        LogCategory::Security => security_logs.push(entry),
        LogCategory::Hardware => hardware_logs.push(entry),
      }
    }

    let system_size: u64 = system_logs.iter().map(|e| e.size).sum();
    let application_size: u64 = application_logs.iter().map(|e| e.size).sum();
    let security_size: u64 = security_logs.iter().map(|e| e.size).sum();
    let hardware_size: u64 = hardware_logs.iter().map(|e| e.size).sum();

    fn log_entry_to_json(entry: &LogEntry) -> serde_json::Value {
      serde_json::json!({
        "path": entry.path,
        "severity": match entry.severity {
          LogSeverity::Error => "error",
          LogSeverity::Warning => "warning",
          LogSeverity::Info => "info",
        },
        "count": entry.count,
        "lastModified": entry.last_modified,
        "size": entry.size,
      })
    }

    summary.insert(
      "system".to_string(),
      serde_json::json!({
        "category": "System",
        "count": system_logs.len(),
        "size": system_size,
        "entries": system_logs.iter().map(log_entry_to_json).collect::<Vec<_>>(),
      }),
    );

    summary.insert(
      "application".to_string(),
      serde_json::json!({
        "category": "Application",
        "count": application_logs.len(),
        "size": application_size,
        "entries": application_logs.iter().map(log_entry_to_json).collect::<Vec<_>>(),
      }),
    );

    summary.insert(
      "security".to_string(),
      serde_json::json!({
        "category": "Security",
        "count": security_logs.len(),
        "size": security_size,
        "entries": security_logs.iter().map(log_entry_to_json).collect::<Vec<_>>(),
      }),
    );

    summary.insert(
      "hardware".to_string(),
      serde_json::json!({
        "category": "Hardware",
        "count": hardware_logs.len(),
        "size": hardware_size,
        "entries": hardware_logs.iter().map(log_entry_to_json).collect::<Vec<_>>(),
      }),
    );

    Ok(success_response(
      "Log summary retrieved successfully",
      DataValue::Object(serde_json::Value::Object(summary)),
    ))
  }

  pub fn get_log_entries(&self, category: String) -> Result<ResponseModel, ResponseModel> {
    self
      .get_log_entries_inner(category)
      .map_err(|e| e.into_response())
  }

  fn get_log_entries_inner(&self, category: String) -> AnalyzerResult<ResponseModel> {
    let all_entries = self.analyze_logs();
    let target_category = match category.to_lowercase().as_str() {
      "system" => LogCategory::System,
      "application" => LogCategory::Application,
      "security" => LogCategory::Security,
      "hardware" => LogCategory::Hardware,
      _ => {
        return Err(AppError::message(format!("Invalid category: {}", category)));
      }
    };

    let filtered: Vec<LogEntry> = all_entries
      .into_iter()
      .filter(|e| self.categorize_log(&e.path) == target_category)
      .collect();

    let entries_json: Vec<serde_json::Value> = filtered
      .iter()
      .map(|e| {
        serde_json::json!({
          "path": e.path,
          "severity": match e.severity {
            LogSeverity::Error => "error",
            LogSeverity::Warning => "warning",
            LogSeverity::Info => "info",
          },
          "count": e.count,
          "lastModified": e.last_modified,
          "size": e.size,
        })
      })
      .collect();

    Ok(success_response(
      format!("{} log entries retrieved", entries_json.len()),
      DataValue::Array(entries_json),
    ))
  }

  pub fn clean_old_logs(&self, days: u32) -> Result<ResponseModel, ResponseModel> {
    self
      .clean_old_logs_inner(days)
      .map_err(|e| e.into_response())
  }

  fn clean_old_logs_inner(&self, days: u32) -> AnalyzerResult<ResponseModel> {
    let log_dir = Path::new("/var/log");
    if !log_dir.exists() {
      return Err(AppError::message("Log directory not found"));
    }

    let cutoff_time = SystemTime::now()
      .checked_sub(std::time::Duration::from_secs(days as u64 * 86400))
      .unwrap_or(SystemTime::UNIX_EPOCH);

    let mut cleaned_count = 0u32;
    let mut errors: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(log_dir) {
      for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
          if let Ok(metadata) = fs::metadata(&path) {
            if let Ok(modified) = metadata.modified() {
              if modified < cutoff_time {
                match fs::remove_file(&path) {
                  Ok(_) => cleaned_count += 1,
                  Err(e) => errors.push(format!("{}: {}", path.display(), e)),
                }
              }
            }
          }
        }
      }
    }

    if errors.is_empty() {
      Ok(success_response(
        format!("Cleaned {} old log files", cleaned_count),
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

  fn analyze_logs(&self) -> Vec<LogEntry> {
    let log_dir = Path::new("/var/log");
    let mut entries = Vec::new();

    let log_files = [
      ("syslog", LogCategory::System),
      ("auth.log", LogCategory::Security),
      ("kern.log", LogCategory::System),
      ("application.log", LogCategory::Application),
      ("dmesg", LogCategory::Hardware),
      ("kmsg", LogCategory::Hardware),
      ("boot.log", LogCategory::System),
      ("dpkg.log", LogCategory::System),
      ("alternatives.log", LogCategory::System),
    ];

    for (filename, _category) in log_files.iter() {
      let path = log_dir.join(filename);
      if path.exists() {
        if let Ok(metadata) = fs::metadata(&path) {
          let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
          let modified_datetime: DateTime<Local> = modified.into();
          let (severity, count) = self.analyze_logs_with_cache(&path, modified);

          entries.push(LogEntry {
            path: path.to_string_lossy().into_owned(),
            severity,
            count,
            last_modified: modified_datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            size: metadata.len(),
          });
        }
      }
    }

    entries
  }

  fn analyze_logs_with_cache(&self, path: &Path, mtime: SystemTime) -> (LogSeverity, u32) {
    if let Some(result) = get_log_cache().get(path, mtime) {
      return result;
    }

    let content = match fs::read_to_string(path) {
      Ok(c) => c,
      Err(_) => return (LogSeverity::Info, 0),
    };

    let severity = Self::determine_severity(&content);
    let count = Self::count_entries_for_severity(&content, &severity);

    get_log_cache().set(path.to_path_buf(), mtime, severity.clone(), count);

    (severity, count)
  }

  fn determine_severity(content: &str) -> LogSeverity {
    let content_lower = content.to_lowercase();
    if content_lower.contains("error")
      || content_lower.contains("fail")
      || content_lower.contains("critical")
    {
      LogSeverity::Error
    } else if content_lower.contains("warn") || content_lower.contains("notice") {
      LogSeverity::Warning
    } else {
      LogSeverity::Info
    }
  }

  fn count_entries_for_severity(content: &str, severity: &LogSeverity) -> u32 {
    match severity {
      LogSeverity::Error => {
        let mut count = 0u32;
        let mut search_start = 0;
        while let Some(pos) = content[search_start..].find("error") {
          count += 1;
          search_start += pos + 5;
        }
        let mut search_start = 0;
        while let Some(pos) = content[search_start..].find("fail") {
          count += 1;
          search_start += pos + 4;
        }
        let mut search_start = 0;
        while let Some(pos) = content[search_start..].find("critical") {
          count += 1;
          search_start += pos + 8;
        }
        count
      }
      LogSeverity::Warning => {
        let mut count = 0u32;
        let mut search_start = 0;
        while let Some(pos) = content[search_start..].find("warn") {
          count += 1;
          search_start += pos + 4;
        }
        let mut search_start = 0;
        while let Some(pos) = content[search_start..].find("notice") {
          count += 1;
          search_start += pos + 6;
        }
        count
      }
      LogSeverity::Info => {
        let total_lines = content.lines().count() as u32;
        let mut error_warn_count = 0u32;
        let mut search_start = 0;
        while let Some(pos) = content[search_start..].find("error") {
          error_warn_count += 1;
          search_start += pos + 5;
        }
        let mut search_start = 0;
        while let Some(pos) = content[search_start..].find("fail") {
          error_warn_count += 1;
          search_start += pos + 4;
        }
        let mut search_start = 0;
        while let Some(pos) = content[search_start..].find("warn") {
          error_warn_count += 1;
          search_start += pos + 4;
        }
        let mut search_start = 0;
        while let Some(pos) = content[search_start..].find("notice") {
          error_warn_count += 1;
          search_start += pos + 6;
        }
        total_lines.saturating_sub(error_warn_count)
      }
    }
  }

  fn categorize_log(&self, path: &str) -> LogCategory {
    let path_lower = path.to_lowercase();
    if path_lower.contains("auth") || path_lower.contains("secure") {
      LogCategory::Security
    } else if path_lower.contains("kern")
      || path_lower.contains("dmesg")
      || path_lower.contains("kmsg")
    {
      LogCategory::Hardware
    } else if path_lower.contains("application") {
      LogCategory::Application
    } else {
      LogCategory::System
    }
  }

  pub fn clear_log_cache() {
    get_log_cache().clear();
  }
}
