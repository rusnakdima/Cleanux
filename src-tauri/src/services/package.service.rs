use crate::helpers::{calculate_directory_size, data_string, success_response};
use crate::models::{AppError, DataValue, ResponseModel};
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
pub struct OrphanedPackage {
  pub name: String,
  pub version: String,
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
  fn run_command(cmd: &str, args: &[&str]) -> std::result::Result<(bool, String, u64), AppError> {
    let output = Command::new(cmd)
      .args(args)
      .output()
      .map_err(|e| AppError::message(format!("Failed to run {} {}: {}", cmd, args.join(" "), e)))?;

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    Ok((
      output.status.success(),
      stderr,
      output.status.code().unwrap_or(-1) as u64,
    ))
  }

  fn get_command_output(cmd: &str, args: &[&str]) -> std::result::Result<String, AppError> {
    let output = Command::new(cmd)
      .args(args)
      .output()
      .map_err(|e| AppError::message(format!("Failed to run {} {}: {}", cmd, args.join(" "), e)))?;

    if output.status.success() {
      Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
      Err(AppError::message(
        String::from_utf8_lossy(&output.stderr).to_string(),
      ))
    }
  }

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
    let home = dirs::home_dir()?;
    let snap_path = home.join("snap");
    let path = Path::new(&snap_path);

    if !path.exists() {
      return None;
    }

    let size = calculate_directory_size(path);
    Some(PackageCacheInfo {
      name: "snap".to_string(),
      cachePath: snap_path.to_string_lossy().to_string(),
      size,
      description: "Snap package manager cache".to_string(),
    })
  }

  fn get_flatpak_cache_info() -> Option<PackageCacheInfo> {
    let home = dirs::home_dir()?;
    let flatpak_path = home.join(".local/share/flatpak/app");
    let path = Path::new(&flatpak_path);

    if !path.exists() {
      return None;
    }

    let size = calculate_directory_size(path);
    Some(PackageCacheInfo {
      name: "flatpak".to_string(),
      cachePath: flatpak_path.to_string_lossy().to_string(),
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

    let size = calculate_directory_size(path);
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
      let stderr = String::from_utf8_lossy(&output.stderr);
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
      let stderr = String::from_utf8_lossy(&output.stderr);
      return Err(AppError::message(format!(
        "Failed to list snaps: {}",
        stderr
      )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
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
      let stderr = String::from_utf8_lossy(&output.stderr);
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
      let stderr = String::from_utf8_lossy(&output.stderr);
      Err(AppError::message(format!(
        "Failed to clean YUM cache: {}",
        stderr
      )))
    }
  }

  pub fn get_apt_cache_size_internal(&self) -> u64 {
    let cache_path = Path::new("/var/cache/apt/archives/");
    if cache_path.exists() {
      calculate_directory_size(cache_path)
    } else {
      0
    }
  }

  pub fn apt_clean(&self) -> Result<ResponseModel, AppError> {
    let before_size = self.get_apt_cache_size_internal();
    let (success, stderr, _) = Self::run_command("apt-get", &["clean"])?;

    if success {
      let after_size = self.get_apt_cache_size_internal();
      let freed = before_size.saturating_sub(after_size);
      Ok(success_response(
        format!("APT cache cleaned. Freed {} bytes", freed),
        DataValue::Object(serde_json::json!({
            "command": "apt-get clean",
            "spaceFreed": freed,
            "message": "APT cache cleaned successfully"
        })),
      ))
    } else {
      let err_msg = format!("Failed to clean APT cache: {}", stderr);
      Err(AppError::message(err_msg))
    }
  }

  pub fn apt_autoremove(&self) -> Result<ResponseModel, AppError> {
    let (success, stderr, _) = Self::run_command("apt-get", &["autoremove", "-y"])?;

    if success {
      Ok(success_response(
        "APT autoremove completed successfully",
        DataValue::Object(serde_json::json!({
            "command": "apt-get autoremove -y",
            "spaceFreed": 0,
            "message": "APT autoremove completed successfully"
        })),
      ))
    } else {
      let err_msg = format!("Failed to run apt-get autoremove: {}", stderr);
      Err(AppError::message(err_msg))
    }
  }

  pub fn apt_autoclean(&self) -> Result<ResponseModel, AppError> {
    let before_size = self.get_apt_cache_size_internal();
    let (success, stderr, _) = Self::run_command("apt-get", &["autoclean"])?;

    if success {
      let after_size = self.get_apt_cache_size_internal();
      let freed = before_size.saturating_sub(after_size);
      Ok(success_response(
        format!("APT autoclean completed. Freed {} bytes", freed),
        DataValue::Object(serde_json::json!({
            "command": "apt-get autoclean",
            "spaceFreed": freed,
            "message": "APT autoclean completed successfully"
        })),
      ))
    } else {
      let err_msg = format!("Failed to run apt-get autoclean: {}", stderr);
      Err(AppError::message(err_msg))
    }
  }

  pub fn get_orphaned_packages(&self) -> Vec<OrphanedPackage> {
    let output = match Self::run_command("dpkg", &["--get-selections"]) {
      Ok((success, _, _)) if success => {
        Self::get_command_output("dpkg", &["--get-selections"]).unwrap_or_default()
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

  pub fn remove_orphaned_package(&self, name: &str) -> Result<ResponseModel, AppError> {
    let (success, stderr, _) = Self::run_command("dpkg", &["--remove", name])?;

    if success {
      Ok(success_response(
        format!("Removed orphaned package: {}", name),
        data_string(name),
      ))
    } else {
      let err_msg = format!("Failed to remove package {}: {}", name, stderr);
      Err(AppError::message(err_msg))
    }
  }

  pub fn get_partial_downloads(&self) -> Vec<String> {
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

  pub fn get_dnf_cache_size_internal(&self) -> u64 {
    let cache_path = Path::new("/var/cache/dnf/");
    if cache_path.exists() {
      calculate_directory_size(cache_path)
    } else {
      0
    }
  }

  pub fn dnf_clean_all(&self) -> Result<ResponseModel, AppError> {
    let before_size = self.get_dnf_cache_size_internal();
    let (success, stderr, _) = Self::run_command("dnf", &["clean", "all"])?;

    if success {
      let after_size = self.get_dnf_cache_size_internal();
      let freed = before_size.saturating_sub(after_size);
      Ok(success_response(
        format!("DNF cache cleaned. Freed {} bytes", freed),
        DataValue::Object(serde_json::json!({
            "command": "dnf clean all",
            "spaceFreed": freed,
            "message": "DNF cache cleaned successfully"
        })),
      ))
    } else {
      let err_msg = format!("Failed to clean DNF cache: {}", stderr);
      Err(AppError::message(err_msg))
    }
  }

  pub fn get_pacman_cache_size_internal(&self) -> u64 {
    let cache_path = Path::new("/var/cache/pacman/pkg/");
    if cache_path.exists() {
      calculate_directory_size(cache_path)
    } else {
      0
    }
  }

  pub fn pacman_clean(&self, keep_recent: u32) -> Result<ResponseModel, AppError> {
    let before_size = self.get_pacman_cache_size_internal();

    let output = Self::get_command_output(
      "sh",
      &[
        "-c",
        &format!(
          "ls -t /var/cache/pacman/pkg/ | tail -n +{}",
          keep_recent + 1
        ),
      ],
    )?;
    let packages: Vec<String> = output
      .lines()
      .filter(|s| !s.is_empty())
      .map(|s| s.to_string())
      .collect();

    let mut _freed: u64 = 0;
    for pkg in &packages {
      let path = Path::new("/var/cache/pacman/pkg/").join(pkg);
      if let Ok(metadata) = fs::metadata(&path) {
        _freed += metadata.len();
      }
      let _ = Self::run_command("rm", &["-f", &path.to_string_lossy()]);
    }

    let after_size = self.get_pacman_cache_size_internal();
    let actual_freed = before_size.saturating_sub(after_size);

    Ok(success_response(
      format!(
        "Pacman cache cleaned. Removed {} old packages. Freed {} bytes",
        packages.len(),
        actual_freed
      ),
      DataValue::Object(serde_json::json!({
          "command": format!("pacman cache clean (keep {})", keep_recent),
          "spaceFreed": actual_freed,
          "message": format!("Removed {} old packages", packages.len())
      })),
    ))
  }

  pub fn pacman_full_clean(&self) -> Result<ResponseModel, AppError> {
    let before_size = self.get_pacman_cache_size_internal();
    let (success, stderr, _) = Self::run_command("pacman", &["-Scc", "--noconfirm"])?;

    if success {
      let after_size = self.get_pacman_cache_size_internal();
      let freed = before_size.saturating_sub(after_size);
      Ok(success_response(
        format!("Pacman full cache clean completed. Freed {} bytes", freed),
        DataValue::Object(serde_json::json!({
            "command": "pacman -Scc --noconfirm",
            "spaceFreed": freed,
            "message": "Pacman full cache clean completed"
        })),
      ))
    } else {
      let err_msg = format!("Failed to run pacman -Scc: {}", stderr);
      Err(AppError::message(err_msg))
    }
  }

  pub fn get_zypper_cache_size_internal(&self) -> u64 {
    let cache_path = Path::new("/var/cache/zypp/");
    if cache_path.exists() {
      calculate_directory_size(cache_path)
    } else {
      0
    }
  }

  pub fn zypper_clean(&self) -> Result<ResponseModel, AppError> {
    let before_size = self.get_zypper_cache_size_internal();
    let (success, stderr, _) = Self::run_command("zypper", &["clean"])?;

    if success {
      let after_size = self.get_zypper_cache_size_internal();
      let freed = before_size.saturating_sub(after_size);
      Ok(success_response(
        format!("Zypper cache cleaned. Freed {} bytes", freed),
        DataValue::Object(serde_json::json!({
            "command": "zypper clean",
            "spaceFreed": freed,
            "message": "Zypper cache cleaned successfully"
        })),
      ))
    } else {
      let err_msg = format!("Failed to clean Zypper cache: {}", stderr);
      Err(AppError::message(err_msg))
    }
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
      if let Ok((success, stderr, _)) = Self::run_command("apt-get", &["clean"]) {
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
      if let Ok((success, stderr, _)) = Self::run_command("apt-get", &["autoclean"]) {
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
      if let Ok((success, stderr, _)) = Self::run_command("dnf", &["clean", "all"]) {
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
      if let Ok((success, stderr, _)) = Self::run_command("pacman", &["-Sc", "--noconfirm"]) {
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
      if let Ok((success, stderr, _)) = Self::run_command("zypper", &["clean"]) {
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