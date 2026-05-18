use std::fs;
use std::path::{Path, PathBuf};

use crate::models::{DataValue, ResponseModel, ResponseStatus};

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
  if let Some(home) = dirs::home_dir() {
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
      name = line.strip_prefix("Name=").unwrap().to_string();
    } else if line.starts_with("Exec=") {
      command = line.strip_prefix("Exec=").unwrap().to_string();
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
    path: path.to_string_lossy().to_string(),
    command,
    enabled,
  })
}

#[allow(non_snake_case)]
impl StartupService {
  pub fn get_startup_items() -> Result<ResponseModel, ResponseModel> {
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

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: format!("Found {} startup items", items.len()),
      data: DataValue::Array(
        items
          .into_iter()
          .map(serde_json::to_value)
          .filter_map(|r| r.ok())
          .collect(),
      ),
    })
  }

  pub fn disable_startup_item(path: &str) -> Result<ResponseModel, ResponseModel> {
    let path_buf = PathBuf::from(path);

    if !is_in_autostart_dir(&path_buf) {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Path is not in autostart directory: {}", path),
        data: DataValue::String("".to_string()),
      });
    }

    if !path_buf.exists() {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("File not found: {}", path),
        data: DataValue::String("".to_string()),
      });
    }

    if !path_buf
      .extension()
      .map(|e| e == "desktop")
      .unwrap_or(false)
    {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Not a .desktop file: {}", path),
        data: DataValue::String("".to_string()),
      });
    }

    let disabled_path = PathBuf::from(format!("{}.disabled", path));
    fs::rename(&path_buf, &disabled_path).map_err(|e| ResponseModel {
      status: ResponseStatus::Error,
      message: format!("Failed to disable startup item: {}", e),
      data: DataValue::String("".to_string()),
    })?;

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: format!("Disabled startup item: {}", path),
      data: DataValue::String(disabled_path.to_string_lossy().to_string()),
    })
  }

  pub fn enable_startup_item(path: &str) -> Result<ResponseModel, ResponseModel> {
    let path_buf = PathBuf::from(path);

    if !is_in_autostart_dir(&path_buf) {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Path is not in autostart directory: {}", path),
        data: DataValue::String("".to_string()),
      });
    }

    let enabled_path = if path.ends_with(".disabled") {
      PathBuf::from(path.strip_suffix(".disabled").unwrap())
    } else {
      path_buf.clone()
    };

    if !enabled_path.exists() {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Original file not found: {}", enabled_path.display()),
        data: DataValue::String("".to_string()),
      });
    }

    if !enabled_path
      .extension()
      .map(|e| e == "desktop")
      .unwrap_or(false)
    {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Not a .desktop file: {}", enabled_path.display()),
        data: DataValue::String("".to_string()),
      });
    }

    fs::rename(&path_buf, &enabled_path).map_err(|e| ResponseModel {
      status: ResponseStatus::Error,
      message: format!("Failed to enable startup item: {}", e),
      data: DataValue::String("".to_string()),
    })?;

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: format!("Enabled startup item: {}", enabled_path.display()),
      data: DataValue::String(enabled_path.to_string_lossy().to_string()),
    })
  }
}
