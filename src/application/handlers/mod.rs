//! KAS handlers module for Cleanux
//!
//! KAS (Kernel Abstraction Service) handlers replace Tauri commands.
//! They provide type-safe operations for CRUD, system management, and business logic.

pub mod cleaner_handlers;
pub mod crud_handlers;
pub mod health_handlers;
pub mod storage_handlers;
pub mod system_handlers;

pub use cleaner_handlers::*;
pub use crud_handlers::*;
pub use health_handlers::*;
pub use storage_handlers::*;
pub use system_handlers::*;
