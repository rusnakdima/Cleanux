//! Infrastructure layer for Cleanux
//!
//! External integrations: MCP server
pub mod cleaning_service_impl;
pub mod json_storage;
pub mod mcp;
pub mod memory_service;
pub mod sys_utils;

// Re-export for convenience
#[allow(ambiguous_glob_reexports)]
pub use cleaning_service_impl::*;
