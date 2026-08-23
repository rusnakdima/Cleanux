//! CleaningService - orchestrates cleaning operations

use crate::domain::CleaningReport;

pub trait CleaningServiceTrait {
    fn scan_for_junk(&mut self) -> Result<Vec<JunkItem>, String>;
    fn clean_junk(&mut self, items: Vec<JunkItem>) -> Result<CleaningReport, String>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JunkItem {
    pub path: String,
    pub category: JunkCategory,
    pub size: u64,
    pub modified: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum JunkCategory {
    Cache,
    Trash,
    Logs,
    LargeFiles,
    Duplicates,
    SystemTemp,
    BrowserCache,
    AppCache,
}
