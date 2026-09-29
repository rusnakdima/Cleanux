//! Infrastructure layer for Cleanux
//!
//! External integrations: MCP server
pub mod automation_service_impl;
pub mod cleaning_service_impl;
pub mod commands;
pub mod health_service_impl;
pub mod history;
pub mod json_storage;
pub mod mcp;
pub mod memory_service;
pub mod monitoring_service;
pub mod package_managers;
pub mod scanners;
pub mod scheduler_service;
pub mod services;
pub mod settings;
pub mod sys_utils;

// Re-export for convenience
#[allow(ambiguous_glob_reexports)]
pub use cleaning_service_impl::*;
