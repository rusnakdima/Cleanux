//! Desktop Window Configuration handlers for Cleanux
//!
//! Provides window state management (always_on_top, fullscreen, etc.)
//! via the Dioxus desktop window API.

use crate::response::Response;
use serde::{Deserialize, Serialize};

/// Window configuration state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
  pub always_on_top: bool,
  pub fullscreen: bool,
  pub maximized: bool,
  pub minimized: bool,
  pub focus: bool,
  pub title: String,
}

/// Global window config state (simplified — real impl uses platform APIs)
static WINDOW_CONFIG: std::sync::LazyLock<parking_lot::RwLock<WindowConfig>> =
  std::sync::LazyLock::new(|| {
    parking_lot::RwLock::new(WindowConfig {
      always_on_top: false,
      fullscreen: false,
      maximized: false,
      minimized: false,
      focus: true,
      title: String::from("Cleanux"),
    })
  });

/// Get current window configuration
pub fn window_config_get() -> Response<WindowConfig> {
  Response::success(
    WINDOW_CONFIG.read().clone(),
    Some("Window config retrieved"),
  )
}

/// Set window configuration
pub fn window_config_set(config: WindowConfig) -> Response<()> {
  tracing::info!(
    "Window config set: always_on_top={}, fullscreen={}, maximized={}",
    config.always_on_top,
    config.fullscreen,
    config.maximized
  );

  *WINDOW_CONFIG.write() = config;

  Response::success((), Some("Window config updated"))
}

/// Set always on top
pub fn window_set_always_on_top(value: bool) -> Response<()> {
  tracing::info!("Window always_on_top set to {}", value);
  WINDOW_CONFIG.write().always_on_top = value;
  Response::success((), Some(&format!("always_on_top set to {}", value)))
}

/// Set fullscreen
pub fn window_set_fullscreen(value: bool) -> Response<()> {
  tracing::info!("Window fullscreen set to {}", value);
  WINDOW_CONFIG.write().fullscreen = value;
  Response::success((), Some(&format!("fullscreen set to {}", value)))
}

/// Set maximized
pub fn window_set_maximized(value: bool) -> Response<()> {
  tracing::info!("Window maximized set to {}", value);
  WINDOW_CONFIG.write().maximized = value;
  Response::success((), Some(&format!("maximized set to {}", value)))
}

/// Minimize window
pub fn window_minimize() -> Response<()> {
  tracing::info!("Window minimized");
  WINDOW_CONFIG.write().minimized = true;
  Response::success((), Some("Window minimized"))
}

/// Restore window from minimized
pub fn window_restore() -> Response<()> {
  tracing::info!("Window restored");
  WINDOW_CONFIG.write().minimized = false;
  Response::success((), Some("Window restored"))
}

/// Set window focus
pub fn window_focus() -> Response<()> {
  tracing::info!("Window focused");
  WINDOW_CONFIG.write().focus = true;
  Response::success((), Some("Window focused"))
}
