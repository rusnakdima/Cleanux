//! CleaningProfile domain entity

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleaningProfile {
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

impl Default for CleaningProfile {
    fn default() -> Self {
        Self {
            id: None,
            name: String::new(),
            description: String::new(),
            created_at: Utc::now(),
            paths: Vec::new(),
            exclude_patterns: Vec::new(),
            clean_cache: true,
            clean_trash: true,
            clean_logs: true,
            min_large_file_size: 100 * 1024 * 1024, // 100MB
        }
    }
}
