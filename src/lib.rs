//! Cleanux - Schema-Driven UI Dioxus Application
//!
//! This app uses schema-driven UI via `dioxus-shared::DynamicPage`.
//! Architecture:
//! - `domain/` - Cleaning entities (CleaningProfile, CleaningReport, etc.)
//! - `application/` - Services and KAS handlers
//! - `infrastructure/` - Storage, system commands, package managers
//! - `presentation/` - Dioxus UI pages and components
//! - `mcp/` - MCP server for bridge communication

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod mcp;
pub mod presentation;

// Re-export commonly used types
#[allow(ambiguous_glob_reexports)]
pub use application::*;
#[allow(ambiguous_glob_reexports)]
pub use dioxus::prelude::*;
#[allow(ambiguous_glob_reexports)]
pub use domain::*;

// Import pages for routing
#[allow(ambiguous_glob_reexports)]
use presentation::pages::*;

/// Application routes
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/dashboard")]
    Dashboard {},
    #[route("/cleaner")]
    Cleaner {},
    #[route("/storage")]
    Storage {},
    #[route("/system")]
    System {},
    #[route("/automation")]
    Automation {},
    #[route("/logs")]
    LogManager {},
    #[route("/settings")]
    Settings {},
    #[route("/profiles")]
    Profiles {},
    #[route("/reports")]
    Reports {},
}
