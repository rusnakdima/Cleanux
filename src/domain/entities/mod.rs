//! Cleanux domain entities
//!
//! Core business entities: CleaningProfile, CleaningReport, AutomationRecipe, HealthSnapshot, ExecutionHistory

pub mod automation_recipe;
pub mod cleaning_profile;
pub mod cleaning_report;
pub mod execution_history;
pub mod health_snapshot;

// Re-export for convenience
pub use automation_recipe::*;
pub use cleaning_profile::*;
pub use cleaning_report::*;
pub use execution_history::*;
pub use health_snapshot::*;
