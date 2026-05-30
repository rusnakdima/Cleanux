/* helpers */
use crate::helpers::{
  collect_log_file_models, data_empty_string, data_string, format_size, get_dir_size,
  models_into_data_array, pkexec_rm_paths, stderr_message, success_response,
};
/* models */
use crate::models::{AppError, DataValue, LogFileModel, ResponseModel};
/* sys lib */
use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
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
