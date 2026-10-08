//! Cleanux - Standalone DDD Dioxus Desktop Application
//!
//! A system cleanup application with hardcoded UI (no SDUI dependency).
//! Architecture:
//! - `domain/` - Cleaning entities (CleaningProfile, CleaningReport, etc.)
//! - `application/` - Services and KAS handlers
//! - `infrastructure/` - Storage, system commands, package managers, MCP server
//! - `presentation/` - Dioxus UI pages and components

pub mod allowlist;
pub mod app;
pub mod application;
pub mod bridge;
pub mod bridge_state;
pub mod domain;
pub mod env;
pub mod error;
pub mod global_state;
pub mod infrastructure;
pub mod mcp_bridge;
pub mod presentation;
pub mod provider;
pub mod response;
pub mod themes;

// Local re-exports for compatibility
pub use error::{AppError, Result};
pub use response::Response;
pub use themes::{get_theme_css, load_theme_pref, ThemeMode, ThemeVariant};

/// Headless test root — mirrors the App component but without the desktop
/// launch plumbing. Used by `dioxus-vdom-inspector` to drive the VirtualDom
/// without a window.
#[cfg(feature = "headless")]
pub fn root_app_for_test() -> dioxus::prelude::Element {
  app::App()
}
