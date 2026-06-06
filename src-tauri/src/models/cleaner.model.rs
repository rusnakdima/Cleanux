/* models */
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CleaningProfile {
  pub name: String,
  pub description: String,
  pub created_at: DateTime<Utc>,
  pub paths: Vec<String>,
  pub exclude_patterns: Vec<String>,
  pub clean_cache: bool,
  pub clean_trash: bool,
  pub clean_logs: bool,
  pub min_large_file_size: u64,
}

impl CleaningProfile {
  pub fn new(name: String) -> Self {
    Self {
      name,
      description: String::new(),
      created_at: Utc::now(),
      paths: Vec::new(),
      exclude_patterns: Vec::new(),
      clean_cache: true,
      clean_trash: true,
      clean_logs: false,
      min_large_file_size: 100 * 1024 * 1024,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheFileModel {
  pub path: String,
  pub size: u64,
  pub modified: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LargeFileModel {
  pub name: String,
  pub path: String,
  pub size: u64,
  pub modified: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogFileModel {
  pub path: String,
  pub size: u64,
  pub modified: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct TrashFileModel {
  pub name: String,
  pub path: String,
  pub size: u64,
  pub deletedDate: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct ScanSummaryModel {
  pub fileCount: usize,
  pub totalSize: u64,
}
