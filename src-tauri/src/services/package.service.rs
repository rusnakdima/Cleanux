use crate::helpers::{
  calculate_dir_size, data_string, run_command, stderr_string, stdout_string, success_response,
};
use crate::models::{AppError, DataValue, ResponseModel};
use crate::services::apt_service::{AptService, OrphanedPackage};
use crate::services::dnf_service::DnfService;
use crate::services::pacman_service::PacmanService;
use crate::services::zypper_service::ZypperService;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct PackageService;

#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct PackageCacheInfo {
  pub name: String,
  pub cachePath: String,
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

impl PackageService {
  pub fn get_package_cache_info() -> Result<ResponseModel, ResponseModel> {
    Self::get_package_cache_info_inner().map_err(|e| e.into_response())
  }

  fn get_package_cache_info_inner() -> Result<ResponseModel, AppError> {
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
          cachePath: path.to_string_lossy().to_string(),
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

  #[allow(dead_code)]
  fn get_snap_cache_info() -> Option<PackageCacheInfo> {
    let home = crate::helpers::home_dir().ok()?;
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

  #[allow(dead_code)]
  fn get_flatpak_cache_info() -> Option<PackageCacheInfo> {
    let home = crate::helpers::home_dir().ok()?;
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

  #[allow(dead_code)]
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
      "apt" => AptService::clean(),
      "snap" => Self::clean_snap(),
      "flatpak" => Self::clean_flatpak(),
      "yum" => Self::clean_yum(),
      _ => Err(AppError::message(format!(
        "Unknown package manager: {}",
        manager
      ))),
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

  pub fn get_apt_cache_size_internal(&self) -> u64 {
    AptService::get_cache_size_internal()
  }

  pub fn apt_clean(&self) -> Result<ResponseModel, AppError> {
    AptService::clean()
  }

  pub fn apt_autoremove(&self) -> Result<ResponseModel, AppError> {
    AptService::autoremove()
  }

  pub fn apt_autoclean(&self) -> Result<ResponseModel, AppError> {
    AptService::autoclean()
  }

  pub fn get_orphaned_packages(&self) -> Vec<OrphanedPackage> {
    AptService::get_orphaned_packages()
  }

  pub fn remove_orphaned_package(&self, name: &str) -> Result<ResponseModel, AppError> {
    AptService::remove_orphaned_package(name)
  }

  pub fn get_partial_downloads(&self) -> Vec<String> {
    AptService::get_partial_downloads()
  }

  pub fn get_dnf_cache_size_internal(&self) -> u64 {
    DnfService::get_cache_size_internal()
  }

  pub fn dnf_clean_all(&self) -> Result<ResponseModel, AppError> {
    DnfService::clean_all()
  }

  pub fn get_pacman_cache_size_internal(&self) -> u64 {
    PacmanService::get_cache_size_internal()
  }

  pub fn pacman_clean(&self, keep_recent: u32) -> Result<ResponseModel, AppError> {
    PacmanService::clean(keep_recent)
  }

  pub fn pacman_full_clean(&self) -> Result<ResponseModel, AppError> {
    PacmanService::full_clean()
  }

  pub fn get_zypper_cache_size_internal(&self) -> u64 {
    ZypperService::get_cache_size_internal()
  }

  pub fn zypper_clean(&self) -> Result<ResponseModel, AppError> {
    ZypperService::clean()
  }

  pub fn get_package_summary(&self) -> PackageManagerSummary {
    let apt_orphans = self.get_orphaned_packages();
    let apt_partials = self.get_partial_downloads();

    PackageManagerSummary {
      apt_available: Path::new("/var/cache/apt/archives/").exists(),
      apt_cache_size: self.get_apt_cache_size_internal(),
      apt_autoremove_size: 0,
      apt_orphaned_count: apt_orphans.len(),
      apt_partial_downloads: apt_partials.len(),
      dnf_available: Path::new("/var/cache/dnf/").exists() || Path::new("/var/cache/yum/").exists(),
      dnf_cache_size: self.get_dnf_cache_size_internal(),
      pacman_available: Path::new("/var/cache/pacman/pkg/").exists(),
      pacman_cache_size: self.get_pacman_cache_size_internal(),
      zypper_available: Path::new("/var/cache/zypp/").exists(),
      zypper_cache_size: self.get_zypper_cache_size_internal(),
    }
  }

  pub fn deep_clean_all(&self) -> Result<ResponseModel, AppError> {
    let mut results: Vec<CleanResult> = Vec::new();

    if Path::new("/var/cache/apt/archives/").exists() {
      let before_size = self.get_apt_cache_size_internal();
      if let Ok((success, stderr, _)) = run_command("apt-get", &["clean"]) {
        if success {
          let freed = before_size.saturating_sub(self.get_apt_cache_size_internal());
          results.push(CleanResult {
            command: "apt-get clean".to_string(),
            space_freed: freed,
            message: "APT cache cleaned".to_string(),
          });
        } else {
          results.push(CleanResult {
            command: "apt-get clean".to_string(),
            space_freed: 0,
            message: format!("Failed: {}", stderr),
          });
        }
      }
      if let Ok((success, stderr, _)) = run_command("apt-get", &["autoclean"]) {
        if !success {
          results.push(CleanResult {
            command: "apt-get autoclean".to_string(),
            space_freed: 0,
            message: format!("Failed: {}", stderr),
          });
        }
      }
    }

    if Path::new("/var/cache/dnf/").exists() || Path::new("/var/cache/yum/").exists() {
      let before_size = self.get_dnf_cache_size_internal();
      if let Ok((success, stderr, _)) = run_command("dnf", &["clean", "all"]) {
        if success {
          let freed = before_size.saturating_sub(self.get_dnf_cache_size_internal());
          results.push(CleanResult {
            command: "dnf clean all".to_string(),
            space_freed: freed,
            message: "DNF cache cleaned".to_string(),
          });
        } else {
          results.push(CleanResult {
            command: "dnf clean all".to_string(),
            space_freed: 0,
            message: format!("Failed: {}", stderr),
          });
        }
      }
    }

    if Path::new("/var/cache/pacman/pkg/").exists() {
      let before_size = self.get_pacman_cache_size_internal();
      if let Ok((success, stderr, _)) = run_command("pacman", &["-Sc", "--noconfirm"]) {
        if success {
          let freed = before_size.saturating_sub(self.get_pacman_cache_size_internal());
          results.push(CleanResult {
            command: "pacman -Sc".to_string(),
            space_freed: freed,
            message: "Pacman cache cleaned".to_string(),
          });
        } else {
          results.push(CleanResult {
            command: "pacman -Sc".to_string(),
            space_freed: 0,
            message: format!("Failed: {}", stderr),
          });
        }
      }
    }

    if Path::new("/var/cache/zypp/").exists() {
      let before_size = self.get_zypper_cache_size_internal();
      if let Ok((success, stderr, _)) = run_command("zypper", &["clean"]) {
        if success {
          let freed = before_size.saturating_sub(self.get_zypper_cache_size_internal());
          results.push(CleanResult {
            command: "zypper clean".to_string(),
            space_freed: freed,
            message: "Zypper cache cleaned".to_string(),
          });
        } else {
          results.push(CleanResult {
            command: "zypper clean".to_string(),
            space_freed: 0,
            message: format!("Failed: {}", stderr),
          });
        }
      }
    }

    let total_freed: u64 = results.iter().map(|r| r.space_freed).sum();

    Ok(success_response(
      format!(
        "Deep clean completed. Total space freed: {} bytes",
        total_freed
      ),
      DataValue::Object(serde_json::json!({
          "totalSpaceFreed": total_freed,
          "results": results
      })),
    ))
  }
}
