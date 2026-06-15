use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSnapshotEntity {
  pub id: Option<String>,
  pub timestamp: String,
  pub health_score: f64,
  pub cache_size: u64,
  pub trash_size: u64,
  pub log_size: u64,
  pub large_files_count: i64,
}

impl HealthSnapshotEntity {
  pub fn new(
    timestamp: String,
    health_score: f64,
    cache_size: u64,
    trash_size: u64,
    log_size: u64,
    large_files_count: i64,
  ) -> Self {
    Self {
      id: None,
      timestamp,
      health_score,
      cache_size,
      trash_size,
      log_size,
      large_files_count,
    }
  }
}
