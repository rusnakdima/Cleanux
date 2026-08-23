//! Application services and handlers for Cleanux
//!
//! Orchestrates domain logic and infrastructure.
//! Contains KAS handlers and business services.

pub mod automation_service;
pub mod cleaning_service;
pub mod health_service;
pub mod routine_service;

// KAS handlers module
pub mod kas;

// Re-export for convenience
pub use automation_service::*;
pub use cleaning_service::*;
pub use health_service::*;
pub use routine_service::*;
