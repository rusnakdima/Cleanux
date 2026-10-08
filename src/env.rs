//! Environment utilities for Cleanux.

use std::path::PathBuf;

/// Returns the app data directory for the given app name.
/// Creates the directory if it doesn't exist.
pub fn data_dir(app_name: &str) -> PathBuf {
  let base = dirs::data_dir()
    .unwrap_or_else(|| PathBuf::from("."))
    .join(app_name);
  std::fs::create_dir_all(&base).ok();
  base
}

/// Load theme preference (dark mode) from app config.
pub fn load_theme_pref(_app_name: &str) -> bool {
  // Default to dark mode for Cleanux
  true
}

/// Save theme preference (dark mode) to app config.
pub fn save_theme_pref(_app_name: &str, _dark: bool) -> Result<(), std::io::Error> {
  // No-op for now; theme state managed via signals
  Ok(())
}

/// Get a dynamically-assigned port for the MCP bridge.
/// Reads from DIOXUS_BRIDGE_PORT env var, falls back to random port.
pub fn dynamic_port() -> u16 {
  std::env::var("DIOXUS_BRIDGE_PORT")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or_else(|| {
      use std::time::{SystemTime, UNIX_EPOCH};
      let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
      // Use a port in the 9000-9999 range based on time
      9000 + (seed % 1000) as u16
    })
}
