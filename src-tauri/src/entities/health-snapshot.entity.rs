use chrono::{DateTime, Utc};
use nosql_orm::Model;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Model)]
#[table_name("health_snapshots")]
#[index("timestamp", 1)]
pub struct HealthSnapshotEntity {
  pub id: Option<String>,
  #[timestamp]
  pub timestamp: DateTime<Utc>,
  pub health_score: f64,
  pub cache_size: u64,
  pub trash_size: u64,
  pub log_size: u64,
  pub large_files_count: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthTrendEntity {
  pub trend: String,
  pub change_percent: f64,
  pub days_analyzed: u32,
}
