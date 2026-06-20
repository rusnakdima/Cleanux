use chrono::{DateTime, Utc};
use nosql_orm::Model;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Model)]
#[table_name("cleaning_profiles")]
#[index("name", 1)]
pub struct CleaningProfileEntity {
  pub id: Option<String>,
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
impl From<crate::models::CleaningProfile> for CleaningProfileEntity {
  fn from(profile: crate::models::CleaningProfile) -> Self {
    Self {
      id: None,
      name: profile.name,
      description: profile.description,
      created_at: profile.created_at,
      paths: profile.paths,
      exclude_patterns: profile.exclude_patterns,
      clean_cache: profile.clean_cache,
      clean_trash: profile.clean_trash,
      clean_logs: profile.clean_logs,
      min_large_file_size: profile.min_large_file_size,
    }
  }
}
