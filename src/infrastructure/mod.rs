//! Infrastructure layer for Cleanux
//!
//! External integrations: storage, system commands, package managers, MCP server

pub mod json_storage;
pub mod mcp;
pub mod package_managers;
pub mod storage;
pub mod system;

pub use package_managers::*;
pub use system::*;
