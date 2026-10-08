//! Handlers module — split into command and query handlers.
//!
//! - `command_handlers`: all write/create/delete operations
//! - `query_handlers`: all read-only query operations

pub mod cleaner_handlers;
pub mod command_handlers;
pub mod query_handlers;
