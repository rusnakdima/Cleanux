//! Cleanux - Schema-Driven UI Dioxus Application
//!
//! This app uses schema-driven UI via `dioxus-shared::DynamicPage`.
//! Architecture:
//! - `domain/` - Cleaning entities (CleaningProfile, CleaningReport, etc.)
//! - `application/` - Services and KAS handlers
//! - `infrastructure/` - Storage, system commands, package managers, MCP server
//! - `presentation/` - Dioxus UI pages and components

pub mod application;
pub mod app;
pub mod bridge;
pub mod domain;
pub mod global_state;
pub mod infrastructure;
pub mod presentation;

// Re-export commonly used types
#[allow(ambiguous_glob_reexports)]
pub use application::*;
#[allow(ambiguous_glob_reexports)]
pub use dioxus::prelude::*;
#[allow(ambiguous_glob_reexports)]
pub use domain::*;

pub use dioxus_shared::schema::load_schema;
pub use presentation::sdui::SduiRoot;
