//! Domain settings for Cleanux.
//!
//! The single source-of-truth settings struct for the application.
//! Persisted via `crate::infrastructure::settings::SettingsStorage` in the infrastructure layer.

use serde::{Deserialize, Serialize};

/// Cleanux application settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CleanuxSettings {
  pub auto_clean: bool,
  pub auto_clean_freq: String,
  pub notify_on_complete: bool,
  pub secure_delete: bool,
  pub excluded_folders: Vec<String>,
  pub max_file_age: u32,
  pub large_file_threshold: u32,
  pub start_with_system: bool,
  pub minimize_to_tray: bool,
}

impl CleanuxSettings {
  pub fn new() -> Self {
    let home = dirs::home_dir()
      .map(|p| p.display().to_string())
      .unwrap_or_else(|| "/home/user".into());
    Self {
      auto_clean: true,
      auto_clean_freq: "daily".into(),
      notify_on_complete: true,
      secure_delete: false,
      excluded_folders: vec![
        format!("{home}/Documents/Important"),
        format!("{home}/Projects"),
      ],
      max_file_age: 30,
      large_file_threshold: 100,
      start_with_system: true,
      minimize_to_tray: true,
    }
  }
}
