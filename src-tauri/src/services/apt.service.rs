use crate::utils::{calculate_dir_size, get_command_output, run_command};
use crate::models::AppError;
use crate::Response;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use serde_json::Value;
pub struct AptService;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrphanedPackage {
  pub name: String,
  pub version: String,
  pub description: String,
}
impl AptService {
  pub fn get_cache_size_internal() -> u64 {
    let cache_path = Path::new("/var/cache/apt/archives/");
    if cache_path.exists() {
      calculate_dir_size(cache_path)
        .map(|(size, _)| size)
        .unwrap_or(0)
    } else {
      0
    }
  }
  pub fn clean() -> Result<Response<Value>, AppError> {
    let before_size = Self::get_cache_size_internal();
    let (success, stderr, _) = run_command("apt-get", &["clean"])?;
    if success {
      let after_size = Self::get_cache_size_internal();
      let freed = before_size.saturating_sub(after_size);
      Ok(Response::success(
        serde_json::json!({
            "command": "apt-get clean",
            "spaceFreed": freed,
            "message": "APT cache cleaned successfully"
        }),
        Some(&format!("APT cache cleaned. Freed {} bytes", freed)),
      ))
    } else {
      let err_msg = format!("Failed to clean APT cache: {}", stderr);
      Err(AppError::Internal(err_msg))
    }
  }
  pub fn autoremove() -> Result<Response<Value>, AppError> {
    let (success, stderr, _) = run_command("apt-get", &["autoremove", "-y"])?;
    if success {
      Ok(Response::success(
        serde_json::json!({
            "command": "apt-get autoremove -y",
            "spaceFreed": 0,
            "message": "APT autoremove completed successfully"
        }),
        Some("APT autoremove completed successfully"),
      ))
    } else {
      let err_msg = format!("Failed to run apt-get autoremove: {}", stderr);
      Err(AppError::Internal(err_msg))
    }
  }
  pub fn autoclean() -> Result<Response<Value>, AppError> {
    let before_size = Self::get_cache_size_internal();
    let (success, stderr, _) = run_command("apt-get", &["autoclean"])?;
    if success {
      let after_size = Self::get_cache_size_internal();
      let freed = before_size.saturating_sub(after_size);
      Ok(Response::success(
        serde_json::json!({
            "command": "apt-get autoclean",
            "spaceFreed": freed,
            "message": "APT autoclean completed successfully"
        }),
        Some(&format!("APT autoclean completed. Freed {} bytes", freed)),
      ))
    } else {
      let err_msg = format!("Failed to run apt-get autoclean: {}", stderr);
      Err(AppError::Internal(err_msg))
    }
  }
  pub fn get_orphaned_packages() -> Vec<OrphanedPackage> {
    let output = match run_command("dpkg", &["--get-selections"]) {
      Ok((success, _, _)) if success => {
        get_command_output("dpkg", &["--get-selections"]).unwrap_or_default()
      }
      _ => return Vec::new(),
    };
    let marked_for_removal: std::collections::HashSet<String> = output
      .lines()
      .filter(|line| line.contains("deinstall"))
      .filter_map(|line| line.split_whitespace().next().map(String::from))
      .collect();
    if marked_for_removal.is_empty() {
      return Vec::new();
    }
    let mut orphans = Vec::new();
    for name in marked_for_removal {
      orphans.push(OrphanedPackage {
        name: name.clone(),
        version: String::new(),
        description: String::new(),
      });
    }
    orphans
  }
  pub fn remove_orphaned_package(name: &str) -> Result<Response<Value>, AppError> {
    let (success, stderr, _) = run_command("dpkg", &["--remove", name])?;
    if success {
      Ok(Response::success(
        serde_json::Value::String(name.to_string()),
        Some(&format!("Removed orphaned package: {}", name)),
      ))
    } else {
      let err_msg = format!("Failed to remove package {}: {}", name, stderr);
      Err(AppError::Internal(err_msg))
    }
  }
  pub fn get_partial_downloads() -> Vec<String> {
    let cache_path = Path::new("/var/cache/apt/archives/");
    if !cache_path.exists() {
      return Vec::new();
    }
    fs::read_dir(cache_path)
      .map(|entries| {
        entries
          .filter_map(|e| e.ok())
          .filter(|e| {
            e.path()
              .extension()
              .map(|ext| ext == "part")
              .unwrap_or(false)
          })
          .filter_map(|e| e.file_name().into_string().ok())
          .collect()
      })
      .unwrap_or_default()
  }
}
