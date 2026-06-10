/* helpers */
use crate::helpers::{
  calculate_dir_size, data_string, home_dir, info_response, stderr_string, stdout_string,
  success_response,
};
/* models */
use crate::models::{AppError, DataValue, ResponseModel};
/* sys lib */
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct PackageManagerService;

#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct PackageCacheInfo {
  pub name: String,
  pub cachePath: String,
  pub size: u64,
  pub description: String,
}

impl PackageManagerService {
  pub fn get_package_cache_info() -> Result<ResponseModel, ResponseModel> {
    Self::get_package_cache_info_inner().map_err(|e| e.into_response())
  }

  fn get_package_cache_info_inner() -> Result<ResponseModel, AppError> {
    let mut cache_infos: Vec<PackageCacheInfo> = Vec::new();

    if let Some(info) = Self::get_apt_cache_info() {
      cache_infos.push(info);
    }
    if let Some(info) = Self::get_snap_cache_info() {
      cache_infos.push(info);
    }
    if let Some(info) = Self::get_flatpak_cache_info() {
      cache_infos.push(info);
    }
    if let Some(info) = Self::get_yum_cache_info() {
      cache_infos.push(info);
    }

    let data: Vec<serde_json::Value> = cache_infos
      .iter()
      .map(|info| {
        serde_json::json!({
          "name": info.name,
          "cachePath": info.cachePath,
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

  fn get_apt_cache_info() -> Option<PackageCacheInfo> {
    let cache_path = "/var/cache/apt/archives/";
    let path = Path::new(cache_path);

    if !path.exists() {
      return None;
    }

    let size = Self::calculate_deb_size(path);
    Some(PackageCacheInfo {
      name: "apt".to_string(),
      cachePath: cache_path.to_string(),
      size,
      description: "APT package manager cache".to_string(),
    })
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

  fn get_snap_cache_info() -> Option<PackageCacheInfo> {
    let home = home_dir().ok()?;
    let snap_path = home.join("snap");
    let path = Path::new(&snap_path);

    if !path.exists() {
      return None;
    }

    let size = calculate_dir_size(path).map(|(size, _)| size).unwrap_or(0);
    Some(PackageCacheInfo {
      name: "snap".to_string(),
      cachePath: snap_path.to_string_lossy().into_owned(),
      size,
      description: "Snap package manager cache".to_string(),
    })
  }

  fn get_flatpak_cache_info() -> Option<PackageCacheInfo> {
    let home = home_dir().ok()?;
    let flatpak_path = home.join(".local/share/flatpak/app");
    let path = Path::new(&flatpak_path);

    if !path.exists() {
      return None;
    }

    let size = calculate_dir_size(path).map(|(size, _)| size).unwrap_or(0);
    Some(PackageCacheInfo {
      name: "flatpak".to_string(),
      cachePath: flatpak_path.to_string_lossy().into_owned(),
      size,
      description: "Flatpak application cache".to_string(),
    })
  }

  fn get_yum_cache_info() -> Option<PackageCacheInfo> {
    let cache_path = "/var/cache/yum/";
    let path = Path::new(cache_path);

    if !path.exists() {
      return None;
    }

    let size = calculate_dir_size(path).map(|(size, _)| size).unwrap_or(0);
    Some(PackageCacheInfo {
      name: "yum".to_string(),
      cachePath: cache_path.to_string(),
      size,
      description: "YUM package manager cache".to_string(),
    })
  }

  pub fn clean_package_cache(manager: &str) -> Result<ResponseModel, ResponseModel> {
    Self::clean_package_cache_inner(manager).map_err(|e| e.into_response())
  }

  fn clean_package_cache_inner(manager: &str) -> Result<ResponseModel, AppError> {
    match manager {
      "apt" => Self::clean_apt(),
      "snap" => Self::clean_snap(),
      "flatpak" => Self::clean_flatpak(),
      "yum" => Self::clean_yum(),
      _ => Err(AppError::message(format!(
        "Unknown package manager: {}",
        manager
      ))),
    }
  }

  fn clean_apt() -> Result<ResponseModel, AppError> {
    let output = Command::new("apt-get")
      .args(["clean"])
      .output()
      .map_err(|e| AppError::message(format!("Failed to run apt-get clean: {}", e)))?;

    if output.status.success() {
      Ok(success_response(
        "APT cache cleaned successfully",
        data_string("apt"),
      ))
    } else {
      let stderr = stderr_string(&output);
      Err(AppError::message(format!(
        "Failed to clean APT cache: {}",
        stderr
      )))
    }
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
      Ok(info_response(
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
}
