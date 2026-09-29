//! Domain entities for Cleanux
//!
//! Core business entities: CleaningProfile, CleaningReport, AutomationRecipe, HealthSnapshot

pub mod entities;
pub mod settings;

// Re-export all entities from entities/ subdirectory
pub use entities::*;
