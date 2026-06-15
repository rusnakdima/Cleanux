use crate::helpers::{
  calculate_dir_size, data_string, stderr_string, stdout_string, success_response,
};
use crate::models::{AppError, DataValue, ResponseModel};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct PackageCacheInfo {
  pub name: String,
  pub cache_path: String,
  pub size: u64,
  pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageManagerSummary {
  pub apt_available: bool,
  pub apt_cache_size: u64,
  pub apt_autoremove_size: u64,
  pub apt_orphaned_count: usize,
  pub apt_partial_downloads: usize,
  pub dnf_available: bool,
  pub dnf_cache_size: u64,
  pub pacman_available: bool,
  pub pacman_cache_size: u64,
  pub zypper_available: bool,
  pub zypper_cache_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanResult {
  pub command: String,
  pub space_freed: u64,
  pub message: String,
}

fn calculate_deb_size(path: &Path) -> u64 {
  fs::read_dir(path)
    .map(|entries| {
      entries
        .filter_map(|e| e.ok())
        .filter(|e| {
          e.path()
            .extension()
            .map(|ext| ext == "deb")
            .unwrap_or(false)
        })
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
    })
    .unwrap_or(0)
}

fn get_cache_info_configs() -> Vec<(&'static str, std::path::PathBuf, &'static str)> {
  let home = dirs::home_dir().unwrap_or_default();
  vec![
    (
      "apt",
      std::path::PathBuf::from("/var/cache/apt/archives/"),
      "APT package manager cache",
    ),
    ("snap", home.join("snap"), "Snap package manager cache"),
    (
      "flatpak",
      home.join(".local/share/flatpak/app"),
      "Flatpak application cache",
    ),
    (
      "yum",
      std::path::PathBuf::from("/var/cache/yum/"),
      "YUM package manager cache",
    ),
  ]
}

fn clean_snap() -> Result<ResponseModel, AppError> {
  let output = Command::new("snap")
    .args(["list", "--all"])
    .output()
    .map_err(|e| AppError::message(format!("Failed to run snap list: {}", e)))?;

  if !output.status.success() {
    let stderr = stderr_string(&output);
    return Err(AppError::message(format!(
      "Failed to list snaps: {}",
      stderr
    )));
  }

  let stdout = stdout_string(&output);
  let mut revision_counts: std::collections::HashMap<String, u32> =
    std::collections::HashMap::new();

  for line in stdout.lines().skip(1) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 4 {
      let name = parts[0];
      let revision = parts[1];
      let key = format!("{}-{}", name, revision);
      *revision_counts.entry(key).or_insert(0) += 1;
    }
  }

  let mut removed_count = 0;
  for (key, _) in revision_counts {
    if let Some(dash_pos) = key.rfind('-') {
      let name = &key[..dash_pos];
      let revision = &key[dash_pos + 1..];
      let remove_output = Command::new("snap")
        .args(["remove", name, "--revision", revision])
        .output();

      if let Ok(out) = remove_output {
        if out.status.success() {
          removed_count += 1;
        }
      }
    }
  }

  Ok(success_response(
    format!(
      "Snap cleanup completed, removed {} revisions",
      removed_count
    ),
    data_string(removed_count.to_string()),
  ))
}

fn clean_flatpak() -> Result<ResponseModel, AppError> {
  let output = Command::new("flatpak")
    .args(["uninstall", "--unused", "-y"])
    .output()
    .map_err(|e| AppError::message(format!("Failed to run flatpak uninstall: {}", e)))?;

  if output.status.success() {
    Ok(success_response(
      "Flatpak unused packages cleaned successfully",
      data_string("flatpak"),
    ))
  } else {
    let stderr = stderr_string(&output);
    Ok(crate::helpers::info_response(
      format!("Flatpak cleanup: {}", stderr),
      data_string("flatpak"),
    ))
  }
}

fn clean_yum() -> Result<ResponseModel, AppError> {
  let output = Command::new("yum")
    .args(["clean", "all"])
    .output()
    .map_err(|e| AppError::message(format!("Failed to run yum clean: {}", e)))?;

  if output.status.success() {
    Ok(success_response(
      "YUM cache cleaned successfully",
      data_string("yum"),
    ))
  } else {
    let stderr = stderr_string(&output);
    Err(AppError::message(format!(
      "Failed to clean YUM cache: {}",
      stderr
    )))
  }
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_package_cache_info() -> Result<ResponseModel, ResponseModel> {
  let mut cache_infos: Vec<PackageCacheInfo> = Vec::new();

  let configs = get_cache_info_configs();
  for (name, path, description) in configs {
    if path.exists() {
      let size = if name == "apt" {
        calculate_deb_size(&path)
      } else if path.exists() {
        calculate_dir_size(&path).map(|(size, _)| size).unwrap_or(0)
      } else {
        0
      };
      cache_infos.push(PackageCacheInfo {
        name: name.to_string(),
        cache_path: path.to_string_lossy().to_string(),
        size,
        description: description.to_string(),
      });
    }
  }

  let data: Vec<serde_json::Value> = cache_infos
    .iter()
    .map(|info| {
      serde_json::json!({
          "name": info.name,
          "cachePath": info.cache_path,
          "size": info.size,
          "description": info.description
      })
    })
    .collect();

  Ok(success_response(
    "Package cache info retrieved successfully",
    DataValue::Array(data),
  ))
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn clean_package_cache(manager: String) -> Result<ResponseModel, ResponseModel> {
  match manager.as_str() {
    "snap" => clean_snap().map_err(|e| e.into_response()),
    "flatpak" => clean_flatpak().map_err(|e| e.into_response()),
    "yum" => clean_yum().map_err(|e| e.into_response()),
    _ => Err(ResponseModel {
      status: crate::models::ResponseStatus::Error,
      message: format!("Unknown package manager: {}", manager),
      data: DataValue::Object(serde_json::json!({})),
    }),
  }
}
