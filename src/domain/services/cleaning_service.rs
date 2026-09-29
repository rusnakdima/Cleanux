//! CleaningService — domain trait for cleaning operations.
//!
//! Defines the contract for scanning and cleaning junk categories.
//! Implementations live in `infrastructure`.

use crate::domain::CleaningReport;
use crate::domain::entities::cleaning_profile::CleaningProfile;
use dioxus_shared::Result;

/// Scans for junk items across configured paths.
pub trait CleaningService: Send + Sync {
    /// Scan all configured junk categories and return items found.
    fn scan(&self) -> Result<Vec<ScanResult>>;

    /// Clean the given junk items and produce a report.
    fn clean(&self, items: Vec<JunkItem>) -> Result<CleaningReport>;

    /// Apply a named cleaning profile by ID.
    fn apply_profile(&self, profile_id: &str) -> Result<CleaningReport>;
}

/// Run an automation routine by ID and record execution history.
pub trait RoutineService: Send + Sync {
    /// List all saved routines.
    fn list_routines(&self) -> Result<Vec<crate::domain::entities::automation_recipe::AutomationRecipe>>;

    /// Get a routine by ID.
    fn get_routine(&self, id: &str) -> Result<Option<crate::domain::entities::automation_recipe::AutomationRecipe>>;

    /// Execute a routine and record history.
    fn run_routine(&self, id: &str) -> Result<crate::domain::entities::execution_history::ExecutionHistory>;

    /// Get execution history.
    fn get_history(&self) -> Result<Vec<crate::domain::entities::execution_history::ExecutionHistory>>;
}

/// System health monitoring and snapshot comparison.
pub trait HealthService: Send + Sync {
    /// Take a current health snapshot.
    fn take_snapshot(&self) -> Result<crate::domain::HealthSnapshot>;

    /// Compare two snapshots by ID.
    fn compare_snapshots(&self, before_id: &str, after_id: &str) -> Result<crate::domain::SnapshotComparison>;
}

// Supporting types used by the traits
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScanResult {
    pub category: String,
    pub items: Vec<JunkItem>,
    pub total_size: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JunkItem {
    pub path: String,
    pub category: String,
    pub size: u64,
    pub modified: String,
}
