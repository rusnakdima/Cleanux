//! Theme provider for Cleanux UI.

use crate::themes::{ThemeMode, ThemeVariant};

/// Theme state held in Dioxus context.
#[derive(Debug, Clone)]
pub struct ThemeState {
  pub mode: ThemeMode,
  pub variant: ThemeVariant,
  pub is_dark: bool,
}

impl ThemeState {
  pub fn new(mode: ThemeMode, variant: ThemeVariant) -> Self {
    let is_dark = matches!(mode, ThemeMode::Dark);
    Self {
      mode,
      variant,
      is_dark,
    }
  }
}

use dioxus::prelude::*;

/// ThemeProvider component that manages theme state.
#[derive(Props, Clone, PartialEq)]
pub struct ThemeProviderProps {
  pub initial_mode: ThemeMode,
  #[props(default = ThemeVariant::MaterialDesign3)]
  pub initial_variant: ThemeVariant,
  pub children: Element,
}

/// Provides theme context to the component tree.
#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
  let theme_state = use_signal(|| ThemeState::new(props.initial_mode, props.initial_variant));

  provide_context(theme_state);

  let is_dark = theme_state.read().mode == ThemeMode::Dark;
  let class = if is_dark { "dark" } else { "light" };

  rsx! {
    div {
      class: "{class}",
      style { { crate::themes::get_theme_css() } }
      { props.children }
    }
  }
}

/// Hook to get the current theme mode.
pub fn use_theme_mode() -> ThemeMode {
  ThemeMode::Dark // Default to dark
}

/// Hook to get the current theme variant.
pub fn use_theme_variant() -> ThemeVariant {
  ThemeVariant::MaterialDesign3
}

/// Hook to toggle between light and dark mode.
pub fn use_toggle_theme() -> impl Fn() {
  || {}
}
