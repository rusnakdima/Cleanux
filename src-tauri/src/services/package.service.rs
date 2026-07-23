use crate::models::AppError;
use crate::utils::{calculate_dir_size, stderr_string, stdout_string};
use crate::Response;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;
pub struct PackageService;
#[derive(Debug, Clone)]
pub struct PackageCacheInfo {
  pub name: String,
  pub cache_path: String,
  pub size: u64,
  pub description: String,
}
impl PackageService {
  pub fn get_package_cache_info() -> Result<Response<Value>, Response<Value>> {
    Self::get_package_cache_info_inner().map_err(|e| e.into_response())
  }
  fn get_package_cache_info_inner() -> Result<Response<Value>, AppError> {
    let mut cache_infos: Vec<PackageCacheInfo> = Vec::new();
    let configs = Self::get_cache_info_configs();
    for (name, path, description) in configs {
      if path.exists() {
        let size = if name == "apt" {
          Self::calculate_deb_size(&path)
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
    Ok(Response::success(
      Value::Array(data),
      Some("Package cache info retrieved successfully"),
    ))
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
  pub fn clean_package_cache(manager: &str) -> Result<Response<Value>, Response<Value>> {
    Self::clean_package_cache_inner(manager).map_err(|e| e.into_response())
  }
  fn clean_package_cache_inner(manager: &str) -> Result<Response<Value>, AppError> {
    match manager {
      "snap" => Self::clean_snap(),
      "flatpak" => Self::clean_flatpak(),
      "yum" => Self::clean_yum(),
      _ => Err(AppError::Internal(format!(
        "Unknown package manager: {}",
        manager
      ))),
    }
  }
  fn clean_snap() -> Result<Response<Value>, AppError> {
    let output = Command::new("snap")
      .args(["list", "--all"])
      .output()
      .map_err(|e| AppError::Internal(format!("Failed to run snap list: {}", e)))?;
    if !output.status.success() {
      let stderr = stderr_string(&output);
      return Err(AppError::Internal(format!(
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
    Ok(Response::success(
      serde_json::Value::String(removed_count.to_string()),
      Some(&format!(
        "Snap cleanup completed, removed {} revisions",
        removed_count
      )),
    ))
  }
  fn clean_flatpak() -> Result<Response<Value>, AppError> {
    let output = Command::new("flatpak")
      .args(["uninstall", "--unused", "-y"])
      .output()
      .map_err(|e| AppError::Internal(format!("Failed to run flatpak uninstall: {}", e)))?;
    if output.status.success() {
      Ok(Response::success(
        serde_json::Value::String("flatpak".to_string()),
        Some("Flatpak unused packages cleaned successfully"),
      ))
    } else {
      let stderr = stderr_string(&output);
      Ok(Response::error_with_status(
        tauri_shared::response::Status::Info,
        format!("Flatpak cleanup: {}", stderr),
      ))
    }
  }
  fn clean_yum() -> Result<Response<Value>, AppError> {
    let output = Command::new("yum")
      .args(["clean", "all"])
      .output()
      .map_err(|e| AppError::Internal(format!("Failed to run yum clean: {}", e)))?;
    if output.status.success() {
      Ok(Response::success(
        serde_json::Value::String("yum".to_string()),
        Some("YUM cache cleaned successfully"),
      ))
    } else {
      let stderr = stderr_string(&output);
      Err(AppError::Internal(format!(
        "Failed to clean YUM cache: {}",
        stderr
      )))
    }
  }
}
