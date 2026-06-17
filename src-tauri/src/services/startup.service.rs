use std::fs;
use std::path::{Path, PathBuf};

use crate::models::{Response, Status};
use crate::utils::home_dir;

pub struct StartupService;

#[derive(Debug, serde::Serialize)]
pub struct StartupItem {
  name: String,
  path: String,
  command: String,
  enabled: bool,
}

fn get_autostart_dirs() -> Vec<PathBuf> {
  let mut dirs = Vec::new();
  if let Ok(home) = home_dir() {
    dirs.push(home.join(".config/autostart"));
  }
  dirs.push(PathBuf::from("/etc/xdg/autostart"));
  dirs
}

fn is_in_autostart_dir(path: &Path) -> bool {
  let autostart_dirs = get_autostart_dirs();
  autostart_dirs.iter().any(|dir| path.starts_with(dir))
}

fn parse_desktop_file(path: &PathBuf) -> Option<StartupItem> {
  let content = fs::read_to_string(path).ok()?;
  let mut name = String::new();
  let mut command = String::new();
  let mut enabled = true;

  for line in content.lines() {
    let line = line.trim();
    if line.starts_with("Name=") {
      name = line
        .strip_prefix("Name=")
        .map(String::from)
        .unwrap_or_default();
    } else if line.starts_with("Exec=") {
      command = line
        .strip_prefix("Exec=")
        .map(String::from)
        .unwrap_or_default();
    } else if line.starts_with("Hidden=true") || line.starts_with("Hidden=false") {
      enabled = line != "Hidden=true";
    } else if line.starts_with("X-GNOME-Autostart-enabled=") {
      enabled = line != "X-GNOME-Autostart-enabled=false";
    }
  }

  if name.is_empty() {
    return None;
  }

  Some(StartupItem {
    name,
    path: path.to_string_lossy().into_owned(),
    command,
    enabled,
  })
}

#[allow(non_snake_case)]
impl StartupService {
  pub fn get_startup_items() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    let mut items = Vec::new();

    for dir in get_autostart_dirs() {
      if dir.exists() {
        if let Ok(entries) = fs::read_dir(&dir) {
          for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().map(|e| e == "desktop").unwrap_or(false) {
              if let Some(item) = parse_desktop_file(&path) {
                items.push(item);
              }
            }
          }
        }
      }
    }

    Ok(Response::success(
      format!("Found {} startup items", items.len()),
      serde_json::to_value(items).unwrap_or(serde_json::Value::Null),
    ))
  }

  pub fn disable_startup_item(
    path: &str,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    let path_buf = PathBuf::from(path);

    if !is_in_autostart_dir(&path_buf) {
      return Err(Response::error(
        Status::Error,
        format!("Path is not in autostart directory: {}", path),
      ));
    }

    if !path_buf.exists() {
      return Err(Response::error(
        Status::Error,
        format!("File not found: {}", path),
      ));
    }

    if !path_buf
      .extension()
      .map(|e| e == "desktop")
      .unwrap_or(false)
    {
      return Err(Response::error(
        Status::Error,
        format!("Not a .desktop file: {}", path),
      ));
    }

    let disabled_path = PathBuf::from(format!("{}.disabled", path));
    fs::rename(&path_buf, &disabled_path).map_err(|e| {
      Response::error(
        Status::Error,
        format!("Failed to disable startup item: {}", e),
      )
    })?;

    Ok(Response::success(
      format!("Disabled startup item: {}", path),
      serde_json::Value::String(disabled_path.to_string_lossy().into_owned()),
    ))
  }

  pub fn enable_startup_item(
    path: &str,
  ) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    let path_buf = PathBuf::from(path);

    if !is_in_autostart_dir(&path_buf) {
      return Err(Response::error(
        Status::Error,
        format!("Path is not in autostart directory: {}", path),
      ));
    }

    let enabled_path = if path.ends_with(".disabled") {
      PathBuf::from(
        path
          .strip_suffix(".disabled")
          .map(String::from)
          .unwrap_or_default(),
      )
    } else {
      path_buf.clone()
    };

    if !enabled_path.exists() {
      return Err(Response::error(
        Status::Error,
        format!("Original file not found: {}", enabled_path.display()),
      ));
    }

    if !enabled_path
      .extension()
      .map(|e| e == "desktop")
      .unwrap_or(false)
    {
      return Err(Response::error(
        Status::Error,
        format!("Not a .desktop file: {}", enabled_path.display()),
      ));
    }

    fs::rename(&path_buf, &enabled_path).map_err(|e| {
      Response::error(
        Status::Error,
        format!("Failed to enable startup item: {}", e),
      )
    })?;

    Ok(Response::success(
      format!("Enabled startup item: {}", enabled_path.display()),
      serde_json::Value::String(enabled_path.to_string_lossy().into_owned()),
    ))
  }
}
