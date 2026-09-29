//! Canonical service implementations from `dioxus-shared`.
//!
//! Adopts the single canonical implementation for features repeating in >2 apps:
//! - `SystemInfoServiceImpl` for CPU/memory/disk info
//! - `NotificationServiceImpl` for desktop notifications
//! - `ClipboardServiceImpl` for system clipboard

pub use dioxus_shared::services::SystemInfoServiceImpl;

pub use dioxus_shared::services::notification::NotificationServiceImpl;
pub use dioxus_shared::core::media::clipboard::ClipboardServiceImpl;
