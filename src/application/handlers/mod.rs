//! KAS handlers module for Cleanux
//!
//! KAS (Kernel Abstraction Service) handlers replace Tauri commands.
//! They provide type-safe operations for CRUD, system management, and business logic.

pub mod allowlist_handlers;
pub mod automation_handlers;
pub mod cleaner_handlers;
pub mod crud_handlers;
pub mod dashboard_handlers;
pub mod health_handlers;
pub mod journal_handlers;
pub mod logging_handlers;
pub mod monitoring_handlers;
pub mod opener_handlers;
pub mod settings_handlers;
pub mod shell_pages_handlers;
pub mod storage_handlers;
pub mod system_handlers;
pub mod window_handlers;

pub use allowlist_handlers::*;
pub use automation_handlers::*;
pub use cleaner_handlers::*;
pub use crud_handlers::*;
pub use health_handlers::*;
pub use journal_handlers::*;
pub use logging_handlers::*;
pub use monitoring_handlers::*;
pub use opener_handlers::*;
pub use settings_handlers::*;
pub use shell_pages_handlers::*;
pub use storage_handlers::*;
pub use system_handlers::*;
pub use window_handlers::*;
