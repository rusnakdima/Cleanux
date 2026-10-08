//! Theme system for Cleanux.

/// Theme mode (Light/Dark/System)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
  Light,
  Dark,
  #[default]
  System,
}

/// Theme variant (Material Design 3, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ThemeVariant {
  #[default]
  MaterialDesign3,
}

/// Generate theme CSS variables string.
pub fn get_theme_css() -> &'static str {
  // Material Design 3 dark theme - embedded CSS
  r#"
:root {
  --md-primary: #6750a4;
  --md-on-primary: #ffffff;
  --md-primary-container: #eaddff;
  --md-on-primary-container: #21005d;
  --md-secondary: #625b71;
  --md-on-secondary: #ffffff;
  --md-secondary-container: #e8def8;
  --md-on-secondary-container: #1d192b;
  --md-surface: #1c1b1f;
  --md-on-surface: #e6e1e5;
  --md-surface-variant: #49454f;
  --md-on-surface-variant: #cac4d0;
  --md-outline: #938f99;
  --md-background: #1c1b1f;
  --md-on-background: #e6e1e5;
  --md-error: #f2b8b5;
  --md-on-error: #601410;
}
.dark {
  --md-surface: #1c1b1f;
  --md-on-surface: #e6e1e5;
  --md-background: #1c1b1f;
  --md-on-background: #e6e1e5;
}
.light {
  --md-primary: #6750a4;
  --md-on-primary: #ffffff;
  --md-surface: #ffFBFE;
  --md-on-surface: #1c1b1f;
  --md-background: #ffFBFE;
  --md-on-background: #1c1b1f;
}
"#
}

/// Load theme preference from storage.
pub fn load_theme_pref(app_name: &str) -> bool {
  let config_path = dirs::config_dir()
    .unwrap_or_else(|| std::path::PathBuf::from("."))
    .join(app_name)
    .join("theme.json");

  if let Ok(content) = std::fs::read_to_string(&config_path) {
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
      return val.get("dark").and_then(|v| v.as_bool()).unwrap_or(true);
    }
  }
  true // Default to dark
}

/// Save theme preference to storage.
pub fn save_theme_pref(app_name: &str, dark: bool) -> Result<(), std::io::Error> {
  let config_dir = dirs::config_dir()
    .unwrap_or_else(|| std::path::PathBuf::from("."))
    .join(app_name);

  std::fs::create_dir_all(&config_dir)?;

  let config_path = config_dir.join("theme.json");
  let content = serde_json::json!({ "dark": dark });
  std::fs::write(&config_path, serde_json::to_string_pretty(&content)?)?;
  Ok(())
}
