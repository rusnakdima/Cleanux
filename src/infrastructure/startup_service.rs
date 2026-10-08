//! Startup service — manage system startup applications.

use std::path::PathBuf;

use tokio::process::Command;

use crate::error::AppError;
use crate::response::Response;

pub type Result<T, E = AppError> = std::result::Result<T, E>;

// ---------------------------------------------------------------------------
// Data Types
// ---------------------------------------------------------------------------

/// Startup item from systemd, XDG, or cron.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartupItem {
  pub id: String,
  pub name: String,
  pub command: String,
  pub enabled: bool,
  pub source: String,
  pub path: Option<String>,
}

// ---------------------------------------------------------------------------
// Startup Item Management
// ---------------------------------------------------------------------------

/// List all startup items from systemd, XDG autostart, and cron.
pub async fn list_startup_items() -> Result<Response<Vec<StartupItem>>, AppError> {
  let mut items = Vec::new();

  // Collect from each source
  items.extend(list_systemd_units().await?);
  items.extend(list_xdg_autostart().await?);
  items.extend(list_cron_reboot().await?);

  Ok(Response::success(items, Some("Startup items retrieved")))
}

/// Disable a startup item by ID.
/// ID format: "systemd:<unit>", "xdg:<file>", "cron:<line>"
pub async fn disable_startup_item(id: &str) -> Result<Response<bool>, AppError> {
  let parts: Vec<&str> = id.splitn(2, ':').collect();
  if parts.len() != 2 {
    return Err(AppError::ValidationError(format!(
      "invalid startup item id: {}",
      id
    )));
  }

  let (source, identifier) = (parts[0], parts[1]);

  match source {
    "systemd" => disable_systemd_unit(identifier).await?,
    "xdg" => disable_xdg_autostart(identifier).await?,
    "cron" => disable_cron_reboot(identifier).await?,
    _ => {
      return Err(AppError::ValidationError(format!(
        "unknown startup source: {}",
        source
      )));
    }
  }

  Ok(Response::success(true, Some("Startup item disabled")))
}

/// Enable a startup item by ID.
pub async fn enable_startup_item(id: &str) -> Result<Response<bool>, AppError> {
  let parts: Vec<&str> = id.splitn(2, ':').collect();
  if parts.len() != 2 {
    return Err(AppError::ValidationError(format!(
      "invalid startup item id: {}",
      id
    )));
  }

  let (source, identifier) = (parts[0], parts[1]);

  match source {
    "systemd" => enable_systemd_unit(identifier).await?,
    "xdg" => enable_xdg_autostart(identifier).await?,
    "cron" => enable_cron_reboot(identifier).await?,
    _ => {
      return Err(AppError::ValidationError(format!(
        "unknown startup source: {}",
        source
      )));
    }
  }

  Ok(Response::success(true, Some("Startup item enabled")))
}

// ---------------------------------------------------------------------------
// Systemd Units
// ---------------------------------------------------------------------------

async fn list_systemd_units() -> Result<Vec<StartupItem>, AppError> {
  let output = Command::new("systemctl")
    .args(["list-unit-files", "--no-pager", "--full", "--no-legend"])
    .output()
    .await
    .map_err(|e| AppError::Internal(format!("systemctl failed: {}", e)))?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(AppError::Internal(format!(
      "systemctl list-unit-files failed: {}",
      stderr
    )));
  }

  let stdout = String::from_utf8_lossy(&output.stdout);
  let mut items = Vec::new();

  for line in stdout.lines() {
    let line = line.trim();
    if line.is_empty() {
      continue;
    }

    // Format: <unit> <state> [<mode>]
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
      continue;
    }

    let unit = parts[0];
    let state = parts[1];
    let enabled = state == "enabled" || state == "enabled-runtime";

    // Get unit description
    let description = get_systemd_description(unit).await;

    items.push(StartupItem {
      id: format!("systemd:{}", unit),
      name: description.unwrap_or_else(|| unit.to_string()),
      command: unit.to_string(),
      enabled,
      source: "systemd".to_string(),
      path: Some(unit.to_string()),
    });
  }

  Ok(items)
}

async fn get_systemd_description(unit: &str) -> Option<String> {
  let output = Command::new("systemctl")
    .args([
      "show",
      unit,
      "--property=Description",
      "--value",
      "--no-pager",
    ])
    .output()
    .await
    .ok()?;

  if output.status.success() {
    let desc = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if !desc.is_empty() {
      return Some(desc);
    }
  }
  None
}

async fn disable_systemd_unit(unit: &str) -> Result<(), AppError> {
  let output = Command::new("systemctl")
    .args(["disable", unit, "--no-pager"])
    .output()
    .await
    .map_err(|e| AppError::Internal(format!("systemctl disable failed: {}", e)))?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(AppError::Internal(format!(
      "systemctl disable failed for {}: {}",
      unit, stderr
    )));
  }

  // Also mask to prevent re-enabling on reboot
  let _ = Command::new("systemctl")
    .args(["mask", unit, "--no-pager"])
    .output()
    .await;

  Ok(())
}

async fn enable_systemd_unit(unit: &str) -> Result<(), AppError> {
  // Unmask first if masked
  let _ = Command::new("systemctl")
    .args(["unmask", unit, "--no-pager"])
    .output()
    .await;

  let output = Command::new("systemctl")
    .args(["enable", unit, "--no-pager"])
    .output()
    .await
    .map_err(|e| AppError::Internal(format!("systemctl enable failed: {}", e)))?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(AppError::Internal(format!(
      "systemctl enable failed for {}: {}",
      unit, stderr
    )));
  }

  Ok(())
}

// ---------------------------------------------------------------------------
// XDG Autostart
// ---------------------------------------------------------------------------

fn xdg_autostart_dirs() -> Vec<PathBuf> {
  let mut dirs = Vec::new();

  // User autostart
  if let Some(home) = dirs::home_dir() {
    dirs.push(home.join(".config/autostart"));
  }

  // System-wide autostart
  dirs.push(PathBuf::from("/etc/xdg/autostart"));
  dirs.push(PathBuf::from("/usr/share/autostart"));

  dirs
}

async fn list_xdg_autostart() -> Result<Vec<StartupItem>, AppError> {
  let mut items = Vec::new();

  for dir in xdg_autostart_dirs() {
    if !dir.exists() {
      continue;
    }

    let entries = match std::fs::read_dir(&dir) {
      Ok(e) => e,
      Err(_) => continue,
    };

    for entry in entries.flatten() {
      let path = entry.path();
      if path.extension().map(|e| e == "desktop").unwrap_or(false) {
        if let Some(item) = parse_desktop_file(&path).await {
          items.push(item);
        }
      }
    }
  }

  Ok(items)
}

async fn parse_desktop_file(path: &PathBuf) -> Option<StartupItem> {
  let content = std::fs::read_to_string(path).ok()?;
  let filename = path.file_name()?.to_string_lossy().to_string();
  let is_system = path.starts_with("/etc") || path.starts_with("/usr");

  let mut name = filename.clone();
  let mut command = String::new();
  let mut enabled = true;
  let mut hidden = false;

  for line in content.lines() {
    let line = line.trim();
    if line.starts_with("Name=") {
      name = line[5..].to_string();
    } else if line.starts_with("Exec=") {
      command = line[5..].to_string();
    } else if line.starts_with("Hidden=") {
      hidden = line[7..].to_lowercase() == "true";
    } else if line.starts_with("X-GNOME-Autostart-enabled=") {
      enabled = line[26..].to_lowercase() != "false";
    } else if line.starts_with("AutostartCondition=") {
      // GNOME specific, treat as enabled if not explicitly disabled
    }
  }

  if hidden {
    return None;
  }

  // XDG items are "disabled" if Hidden=true or if not explicitly enabled
  let enabled = enabled && !hidden;

  Some(StartupItem {
    id: format!("xdg:{}", filename),
    name,
    command,
    enabled,
    source: if is_system {
      "xdg-system".to_string()
    } else {
      "xdg".to_string()
    },
    path: Some(path.to_string_lossy().to_string()),
  })
}

async fn disable_xdg_autostart(file: &str) -> Result<(), AppError> {
  let path = find_xdg_file(file)
    .ok_or_else(|| AppError::NotFound(format!("autostart file not found: {}", file)))?;

  // Mark as hidden
  let content = std::fs::read_to_string(&path)
    .map_err(|e| AppError::Internal(format!("failed to read {}: {}", path.display(), e)))?;

  let new_content = if content.lines().any(|l| l.trim().starts_with("Hidden=")) {
    content
      .lines()
      .map(|l| {
        if l.trim().starts_with("Hidden=") {
          "Hidden=true".to_string()
        } else {
          l.to_string()
        }
      })
      .collect::<Vec<_>>()
      .join("\n")
  } else {
    format!("{}\nHidden=true", content.trim_end())
  };

  std::fs::write(&path, new_content)
    .map_err(|e| AppError::Internal(format!("failed to write {}: {}", path.display(), e)))?;

  Ok(())
}

async fn enable_xdg_autostart(file: &str) -> Result<(), AppError> {
  let path = find_xdg_file(file)
    .ok_or_else(|| AppError::NotFound(format!("autostart file not found: {}", file)))?;

  let content = std::fs::read_to_string(&path)
    .map_err(|e| AppError::Internal(format!("failed to read {}: {}", path.display(), e)))?;

  let new_content: String;
  if content.lines().any(|l| l.trim().starts_with("Hidden=")) {
    new_content = content
      .lines()
      .map(|l| {
        if l.trim().starts_with("Hidden=") {
          "Hidden=false".to_string()
        } else {
          l.to_string()
        }
      })
      .collect::<Vec<_>>()
      .join("\n");
  } else {
    // Remove Hidden= if present
    new_content = content
      .lines()
      .filter(|l| !l.trim().starts_with("Hidden="))
      .collect::<Vec<_>>()
      .join("\n");
  }

  std::fs::write(&path, new_content)
    .map_err(|e| AppError::Internal(format!("failed to write {}: {}", path.display(), e)))?;

  Ok(())
}

fn find_xdg_file(file: &str) -> Option<PathBuf> {
  for dir in xdg_autostart_dirs() {
    let path = dir.join(file);
    if path.exists() {
      return Some(path);
    }
  }
  None
}

// ---------------------------------------------------------------------------
// Cron @reboot
// ---------------------------------------------------------------------------

async fn list_cron_reboot() -> Result<Vec<StartupItem>, AppError> {
  let output = Command::new("crontab")
    .args(["-l"])
    .output()
    .await
    .map_err(|e| AppError::Internal(format!("crontab -l failed: {}", e)))?;

  if !output.status.success() {
    // No crontab or error - this is fine
    return Ok(Vec::new());
  }

  let stdout = String::from_utf8_lossy(&output.stdout);
  let mut items = Vec::new();

  for line in stdout.lines() {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
      continue;
    }

    // Look for @reboot entries
    if line.starts_with("@reboot") {
      let parts: Vec<&str> = line.splitn(2, ' ').collect();
      if parts.len() >= 2 {
        let command = parts[1..].join(" ");
        let name = extract_command_name(&command);
        let id = format!("cron:{}", line.replace(' ', "_").replace('/', "_"));

        items.push(StartupItem {
          id,
          name,
          command,
          enabled: true,
          source: "cron".to_string(),
          path: None,
        });
      }
    }
  }

  Ok(items)
}

fn extract_command_name(command: &str) -> String {
  // Extract executable name from command
  let parts: Vec<&str> = command.split_whitespace().collect();
  if let Some(first) = parts.first() {
    let name = std::path::Path::new(first)
      .file_name()
      .map(|n| n.to_string_lossy().to_string())
      .unwrap_or_else(|| first.to_string());

    if name.starts_with('/') {
      return name;
    }

    // Check for arguments
    if parts.len() > 1 {
      return format!("{} ( {})", name, &command[first.len()..].trim());
    }
    return name;
  }
  command.to_string()
}

async fn disable_cron_reboot(line: &str) -> Result<(), AppError> {
  // Comment out the @reboot line
  let output = Command::new("crontab")
    .args(["-l"])
    .output()
    .await
    .map_err(|e| AppError::Internal(format!("crontab -l failed: {}", e)))?;

  let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
  let current: Vec<String> = if output.status.success() {
    stdout_str.lines().map(|s| s.to_string()).collect()
  } else {
    Vec::new()
  };

  let original_line = line.replace('_', " ");
  let new_crontab: String = current
    .iter()
    .map(|l| {
      if l.contains(&original_line) && l.starts_with("@reboot") {
        format!("# DISABLED: {}", l)
      } else {
        l.to_string()
      }
    })
    .collect::<Vec<_>>()
    .join("\n");

  let mut child = Command::new("crontab")
    .args(["-"])
    .stdin(std::process::Stdio::piped())
    .spawn()
    .map_err(|e| AppError::Internal(format!("failed to spawn crontab: {}", e)))?;

  if let Some(mut stdin) = child.stdin.take() {
    tokio::io::AsyncWriteExt::write_all(&mut stdin, new_crontab.as_bytes())
      .await
      .map_err(|e| AppError::Internal(format!("failed to write crontab: {}", e)))?;
  }

  let status = child
    .wait()
    .await
    .map_err(|e| AppError::Internal(format!("crontab wait failed: {}", e)))?;

  if !status.success() {
    return Err(AppError::Internal("crontab update failed".to_string()));
  }

  Ok(())
}

async fn enable_cron_reboot(line: &str) -> Result<(), AppError> {
  // Uncomment the @reboot line
  let output = Command::new("crontab")
    .args(["-l"])
    .output()
    .await
    .map_err(|e| AppError::Internal(format!("crontab -l failed: {}", e)))?;

  let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
  let current: Vec<String> = if output.status.success() {
    stdout_str.lines().map(|s| s.to_string()).collect()
  } else {
    Vec::new()
  };

  let original_line = line.replace('_', " ");
  let new_crontab: String = current
    .iter()
    .map(|l| {
      if l.contains(&original_line) && l.contains("# DISABLED: @reboot") {
        l.replace("# DISABLED: ", "")
      } else {
        l.to_string()
      }
    })
    .collect::<Vec<_>>()
    .join("\n");

  let mut child = Command::new("crontab")
    .args(["-"])
    .stdin(std::process::Stdio::piped())
    .spawn()
    .map_err(|e| AppError::Internal(format!("failed to spawn crontab: {}", e)))?;

  if let Some(mut stdin) = child.stdin.take() {
    tokio::io::AsyncWriteExt::write_all(&mut stdin, new_crontab.as_bytes())
      .await
      .map_err(|e| AppError::Internal(format!("failed to write crontab: {}", e)))?;
  }

  let status = child
    .wait()
    .await
    .map_err(|e| AppError::Internal(format!("crontab wait failed: {}", e)))?;

  if !status.success() {
    return Err(AppError::Internal("crontab update failed".to_string()));
  }

  Ok(())
}
