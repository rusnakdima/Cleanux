/* sys lib */
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct CacheFileModel {
  pub path: String,
  pub size: u64,
  pub modified: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct TrashFileModel {
  pub name: String,
  pub path: String,
  pub size: u64,
  pub deletedDate: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct LogFileModel {
  pub path: String,
  pub size: u64,
  pub modified: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct LargeFileModel {
  pub name: String,
  pub path: String,
  pub size: u64,
  pub modified: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct ScanSummaryModel {
  pub totalSize: u64,
  pub fileCount: usize,
}
