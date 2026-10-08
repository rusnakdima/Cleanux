//! Domain service traits for Cleanux.
//!
//! Thin traits defining the interface between application handlers and
//! infrastructure implementations.

pub mod cleaning_service;
pub mod repair_service;

pub use cleaning_service::{CleaningService, HealthService, RoutineService};
pub use repair_service::RepairServiceTrait;
