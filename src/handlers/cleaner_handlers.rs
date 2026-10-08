//! Cleaner handlers compatibility shim.
//!
//! Re-exports cleaner functions from command_handlers and query_handlers
//! so existing infrastructure code can continue using `crate::handlers::cleaner_handlers::*`.

// Re-export query functions
pub use crate::handlers::query_handlers::{
    find_broken_symlinks, find_orphaned_packages, get_category_sizes, get_container_summary,
    get_junk_summary, get_startup_items, scan_cache, scan_cache_categories, scan_logs, scan_trash,
    JunkSummary,
};

// Re-export command functions
pub use crate::handlers::command_handlers::{
    clean_junk_category, disable_startup_item, docker_system_prune, set_startup_item_enabled,
    start_quick_clean,
};
