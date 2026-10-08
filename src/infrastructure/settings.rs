//! Infrastructure layer — settings persistence via JSON file storage.
//!
//! Provides a local `SettingsStorage<CleanuxSettings>` backed by a JSON file
//! in the app data directory.

use crate::domain::settings::CleanuxSettings;
use crate::error::AppError;
use crate::global_state::{add_excluded_folder, remove_excluded_folder, update_setting};
use std::path::PathBuf;
use std::sync::{Arc, LazyLock};

/// Canonical settings storage — loaded once, flushed on every save.
pub static SETTINGS_STORAGE: LazyLock<Arc<SettingsStorage<CleanuxSettings>>> =
  LazyLock::new(|| Arc::new(SettingsStorage::new("cleanux")));

/// Settings persistence backed by a JSON file in the app data directory.
pub struct SettingsStorage<T> {
  app_name: String,
  _phantom: std::marker::PhantomData<T>,
}

impl<T: serde::de::DeserializeOwned + serde::Serialize + Default> SettingsStorage<T> {
  /// Create a new settings storage for the given app name.
  pub fn new(app_name: &str) -> Self {
    Self {
      app_name: app_name.to_string(),
      _phantom: std::marker::PhantomData,
    }
  }

  /// Path to the settings file.
  fn path(&self) -> PathBuf {
    let base = dirs::data_dir()
      .unwrap_or_else(|| PathBuf::from("."))
      .join(&self.app_name);
    std::fs::create_dir_all(&base).ok();
    base.join("settings.json")
  }

  /// Load settings from disk.
  pub fn load(&self) -> T {
    let path = self.path();
    if path.exists() {
      match std::fs::read_to_string(&path) {
        Ok(content) => match serde_json::from_str(&content) {
          Ok(settings) => return settings,
          Err(e) => eprintln!("[settings] failed to parse settings: {e}"),
        },
        Err(e) => eprintln!("[settings] failed to read settings file: {e}"),
      }
    }
    T::default()
  }

  /// Save settings to disk.
  pub fn save(&self, settings: &T) -> Result<(), AppError> {
    let path = self.path();
    let content = serde_json::to_string_pretty(settings)
      .map_err(|e| AppError::Serialization(format!("settings serialization failed: {e}")))?;
    std::fs::write(&path, content)
      .map_err(|e| AppError::Io(format!("failed to write settings: {e}")))?;
    Ok(())
  }
}

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
