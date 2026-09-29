//! Infrastructure layer — settings persistence via `SettingsStorage`.
//!
//! Wraps `dioxus_shared::services::SettingsStorage<CleanuxSettings>`.
//! The static `SETTINGS` in `global_state` is backed by this storage.

use crate::domain::settings::CleanuxSettings;
use crate::global_state::{add_excluded_folder, remove_excluded_folder, update_setting};
use dioxus_shared::services::SettingsStorage;
use dioxus_shared::AppError;
use std::sync::{Arc, LazyLock};

/// Canonical settings storage — loaded once, flushed on every save.
pub static SETTINGS_STORAGE: LazyLock<Arc<SettingsStorage<CleanuxSettings>>> =
    LazyLock::new(|| Arc::new(SettingsStorage::new("cleanux")));

/// Load settings from disk into a `CleanuxSettings` struct.
pub fn load_settings() -> CleanuxSettings {
    SETTINGS_STORAGE.load()
}

/// Save settings to disk. Returns `Ok(())` on success or an `AppError`.
pub fn save_settings(settings: &CleanuxSettings) -> Result<(), AppError> {
    SETTINGS_STORAGE.save(settings)
}

/// Apply a key/value patch to the current settings and persist.
///
/// Mirrors the old `global_state::update_setting` behaviour for UI pages
/// that still update field-by-field.  For full replacement use `save_settings`.
pub fn apply_setting_patch(key: &str, val: serde_json::Value) {
    update_setting(key, val);
    let current = load_settings();
    let _ = save_settings(&current);
}

/// Add an excluded folder and persist.
pub fn add_excluded(path: String) {
    add_excluded_folder(path);
    let current = load_settings();
    let _ = save_settings(&current);
}

/// Remove an excluded folder by index and persist.
pub fn remove_excluded(index: usize) {
    remove_excluded_folder(index);
    let current = load_settings();
    let _ = save_settings(&current);
}
