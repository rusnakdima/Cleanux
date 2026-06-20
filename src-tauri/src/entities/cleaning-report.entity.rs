use nosql_orm::Model;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Model)]
#[table_name("cleaning_reports")]
#[index("date", 1)]
pub struct CleaningReportEntity {
  pub id: Option<String>,
  pub date: String,
  pub items_cleaned: i64,
  pub space_reclaimed: u64,
  pub duration: f64,
  pub categories: ReportCategories,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportCategories {
  pub cache: i64,
  pub trash: i64,
  pub logs: i64,
  pub large_files: i64,
  pub duplicates: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotComparison {
  pub before_id: String,
  pub after_id: String,
  pub space_reclaimed: u64,
  pub items_cleaned: i64,
  pub health_improvement: f64,
  pub details: ComparisonDetails,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonDetails {
  pub cache_change: i64,
  pub trash_change: i64,
  pub log_change: i64,
  pub large_file_change: i64,
}
