use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use log;

use crate::helpers::{
  get_dir_size, home_dir, models_into_data_array, stdout_string, success_response,
};
use crate::models::{AppError, DataValue, ResponseModel, ResponseStatus};

#[derive(Debug, Clone, serde::Serialize)]
pub struct AppResidue {
  pub path: String,
  pub app_name: String,
  pub size: u64,
  pub residue_type: ResidueType,
  pub detected_as_uninstalled: bool,
}

#[derive(Debug, Clone, serde::Serialize, PartialEq)]
pub enum ResidueType {
  Config,
  Data,
  Cache,
  Both,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct OrphanedConfig {
  pub path: String,
  pub package_name: String,
  pub config_type: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AppResidueSummary {
  pub total_configs: usize,
  pub total_data: usize,
  pub total_caches: usize,
  pub total_size: u64,
  pub found_uninstalled: usize,
}

pub struct AppResidueService;

impl AppResidueService {
  fn get_installed_apps() -> HashSet<String> {
    let mut installed: HashSet<String> = HashSet::new();

    if let Ok(output) = Command::new("dpkg").arg("-l").output() {
      if output.status.success() {
        let stdout = stdout_string(&output);
        for line in stdout.lines().skip(5) {
          let parts: Vec<&str> = line.split_whitespace().collect();
          if parts.len() >= 4 {
            let status = parts[0];
            let name = parts[3].to_lowercase();
            if status == "ii" {
              installed.insert(name.clone());
              if let Some(descr) = parts.get(4) {
                let alt_name = format!("{}-{}", name, *descr).to_lowercase();
                installed.insert(alt_name);
              }
            }
          }
        }
      }
    }

    if let Ok(output) = Command::new("snap").arg("list").output() {
      if output.status.success() {
        let stdout = stdout_string(&output);
        for line in stdout.lines().skip(1) {
          let parts: Vec<&str> = line.split_whitespace().collect();
          if !parts.is_empty() {
            installed.insert(parts[0].to_lowercase());
          }
        }
      }
    }

    if let Ok(output) = Command::new("flatpak").arg("list").output() {
      if output.status.success() {
        let stdout = stdout_string(&output);
        for line in stdout.lines() {
          let parts: Vec<&str> = line.split('\t').collect();
          if !parts.is_empty() {
            let name = parts[0].to_lowercase();
            installed.insert(name);
            if let Some(last) = parts.last() {
              let alt_name = last.replace('.', "-").to_lowercase();
              installed.insert(alt_name);
            }
          }
        }
      }
    }

    let common_apps = [
      "code",
      "visual-studio-code",
      "firefox",
      "chrome",
      "chromium",
      "brave",
      "opera",
      "vlc",
      "spotify",
      "discord",
      "slack",
      "teams",
      "zoom",
      "skype",
      "telegram",
      "whatsapp",
      "signal",
      "gimp",
      "inkscape",
      "blender",
      "audacity",
      "obs",
      "steam",
      "libreoffice",
      "openoffice",
      "thunderbird",
      "evolution",
      "nautilus",
      "dolphin",
      "konqueror",
      "nemo",
      "pcmanfm",
      "terminal",
      "gnome-terminal",
      "konsole",
      "xfce4-terminal",
      "file-roller",
      "ark",
      "p7zip",
      "gzip",
      "tar",
      "curl",
      "wget",
      "ssh",
      "git",
      "vim",
      "nano",
      "emacs",
      "atom",
      "sublime-text",
      "intellij",
      "pycharm",
      "webstorm",
      "clion",
      "goland",
      "rider",
      "android-studio",
      "eclipse",
      "netbeans",
      "mysql",
      "postgresql",
      "redis",
      "apache",
      "nginx",
      "docker",
      "kubernetes",
      "kubectl",
      "helm",
      "terraform",
      "ansible",
      "vagrant",
      "virtualbox",
    ];
    for app in common_apps {
      installed.insert(app.to_string());
    }

    installed
  }

  fn normalize_app_name(name: &str) -> String {
    name
      .to_lowercase()
      .replace(['.', '-', '_'], "-")
      .replace(|c: char| !c.is_alphanumeric() && c != '-', "")
  }

  fn matches_installed_app(app_name: &str, installed_apps: &HashSet<String>) -> bool {
    let normalized = Self::normalize_app_name(app_name);

    if installed_apps.contains(&normalized) {
      return true;
    }

    for installed in installed_apps {
      if normalized.contains(installed) || installed.contains(&normalized) {
        return true;
      }
      let parts: Vec<&str> = normalized.split('-').collect();
      if parts.len() > 1 {
        let short_name = parts[0];
        if normalized.starts_with(short_name) && installed.contains(short_name) {
          return true;
        }
      }
    }

    false
  }

  pub fn scan_user_configs(&self) -> Vec<AppResidue> {
    let config_dir = match home_dir() {
      Ok(h) => h.join(".config"),
      Err(_) => return Vec::new(),
    };

    if !config_dir.exists() {
      return Vec::new();
    }

    let installed = Self::get_installed_apps();
    let mut residues: Vec<AppResidue> = Vec::new();

    if let Ok(entries) = fs::read_dir(&config_dir) {
      for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let app_name = path
          .file_name()
          .map(|n| n.to_string_lossy().into_owned())
          .unwrap_or_default();

        if app_name.starts_with('.') || app_name.starts_with('#') {
          continue;
        }

        let is_installed = Self::matches_installed_app(&app_name, &installed);

        let size = get_dir_size(&path);
        if size == 0 && path.is_dir() {
          continue;
        }

        residues.push(AppResidue {
          path: path.to_string_lossy().into_owned(),
          app_name: app_name.clone(),
          size,
          residue_type: ResidueType::Config,
          detected_as_uninstalled: !is_installed,
        });
      }
    }

    residues.sort_by_key(|b| std::cmp::Reverse(b.size));
    residues
  }

  pub fn scan_user_data(&self) -> Vec<AppResidue> {
    let data_dir = match home_dir() {
      Ok(h) => h.join(".local/share"),
      Err(_) => return Vec::new(),
    };

    if !data_dir.exists() {
      return Vec::new();
    }

    let installed = Self::get_installed_apps();
    let mut residues: Vec<AppResidue> = Vec::new();

    let known_data_dirs = [
      "Trash",
      "Desktop",
      "Documents",
      "Downloads",
      "Music",
      "Pictures",
      "Videos",
    ];
    let config_residues = self.scan_user_configs();
    let config_names: HashSet<String> = config_residues
      .iter()
      .filter(|r| r.detected_as_uninstalled)
      .map(|r| Self::normalize_app_name(&r.app_name))
      .collect();

    if let Ok(entries) = fs::read_dir(&data_dir) {
      for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let app_name = path
          .file_name()
          .map(|n| n.to_string_lossy().into_owned())
          .unwrap_or_default();

        if known_data_dirs.contains(&app_name.as_str()) {
          continue;
        }

        if app_name.starts_with('.') {
          continue;
        }

        let normalized = Self::normalize_app_name(&app_name);
        let is_installed =
          config_names.contains(&normalized) || Self::matches_installed_app(&app_name, &installed);

        let size = get_dir_size(&path);
        if size == 0 && path.is_dir() {
          continue;
        }

        residues.push(AppResidue {
          path: path.to_string_lossy().into_owned(),
          app_name: app_name.clone(),
          size,
          residue_type: ResidueType::Data,
          detected_as_uninstalled: !is_installed,
        });
      }
    }

    residues.sort_by_key(|b| std::cmp::Reverse(b.size));
    residues
  }

  pub fn scan_user_caches(&self) -> Vec<AppResidue> {
    let cache_dir = match home_dir() {
      Ok(h) => h.join(".cache"),
      Err(_) => return Vec::new(),
    };

    if !cache_dir.exists() {
      return Vec::new();
    }

    let installed = Self::get_installed_apps();
    let mut residues: Vec<AppResidue> = Vec::new();

    if let Ok(entries) = fs::read_dir(&cache_dir) {
      for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let app_name = path
          .file_name()
          .map(|n| n.to_string_lossy().into_owned())
          .unwrap_or_default();

        if app_name == "fontconfig" || app_name == "icon-cache" || app_name == "plasma_icon_cache" {
          continue;
        }

        if app_name.starts_with('.') {
          continue;
        }

        let is_installed = Self::matches_installed_app(&app_name, &installed);

        let size = get_dir_size(&path);
        if size == 0 && path.is_dir() {
          continue;
        }

        residues.push(AppResidue {
          path: path.to_string_lossy().into_owned(),
          app_name: app_name.clone(),
          size,
          residue_type: ResidueType::Cache,
          detected_as_uninstalled: !is_installed,
        });
      }
    }

    residues.sort_by_key(|b| std::cmp::Reverse(b.size));
    residues
  }

  pub fn get_orphaned_configs(&self) -> Vec<OrphanedConfig> {
    let mut orphaned: Vec<OrphanedConfig> = Vec::new();

    if let Ok(output) = Command::new("dpkg").arg("-l").output() {
      if output.status.success() {
        let stdout = stdout_string(&output);
        for line in stdout.lines().skip(5) {
          let parts: Vec<&str> = line.split_whitespace().collect();
          if parts.len() >= 4 {
            let status = parts[0];
            if status == "rc" {
              let package_name = parts[3].to_string();
              orphaned.push(OrphanedConfig {
                path: format!("/etc/apt/triggers.cache/{}", package_name),
                package_name,
                config_type: "dpkg_orphaned".to_string(),
              });
            }
          }
        }
      }
    }

    let config_dir = PathBuf::from("/etc");
    if config_dir.exists() {
      if let Ok(entries) = fs::read_dir(&config_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
          let path = entry.path();
          if path.is_dir() {
            let pkg_name = path
              .file_name()
              .map(|n| n.to_string_lossy().into_owned())
              .unwrap_or_default();
            if !pkg_name.is_empty()
              && pkg_name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
            {
              let installed = Self::get_installed_apps();
              if !Self::matches_installed_app(&pkg_name, &installed) {
                orphaned.push(OrphanedConfig {
                  path: path.to_string_lossy().into_owned(),
                  package_name: pkg_name,
                  config_type: "etc_config".to_string(),
                });
              }
            }
          }
        }
      }
    }

    orphaned
  }

  pub fn scan_home_residues(&self) -> Vec<AppResidue> {
    let home = match home_dir() {
      Ok(h) => h,
      Err(_) => return Vec::new(),
    };

    let mut residues: Vec<AppResidue> = Vec::new();
    let installed = Self::get_installed_apps();

    let home_entries = [".config", ".local/share", ".cache"];
    for entry_name in home_entries {
      let entry_path = home.join(entry_name);
      if entry_path.exists() && entry_path.is_dir() {
        if let Ok(entries) = fs::read_dir(&entry_path) {
          for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            let app_name = path
              .file_name()
              .map(|n| n.to_string_lossy().into_owned())
              .unwrap_or_default();

            if app_name.starts_with('.') {
              continue;
            }

            let is_installed = Self::matches_installed_app(&app_name, &installed);
            let size = get_dir_size(&path);

            if size == 0 && path.is_dir() {
              continue;
            }

            residues.push(AppResidue {
              path: path.to_string_lossy().into_owned(),
              app_name,
              size,
              residue_type: ResidueType::Both,
              detected_as_uninstalled: !is_installed,
            });
          }
        }
      }
    }

    residues.sort_by_key(|b| std::cmp::Reverse(b.size));
    residues
  }

  pub fn clean_residue(&self, path: &str) -> Result<ResponseModel, ResponseModel> {
    log::info!("Attempting to clean residue at: {}", path);
    let path_obj = Path::new(path);

    if !path_obj.exists() {
      log::error!("Path does not exist: {}", path);
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Path does not exist: {}", path),
        data: DataValue::Bool(false),
      });
    }

    let result = if path_obj.is_dir() {
      fs::remove_dir_all(path)
    } else {
      fs::remove_file(path)
    };

    match result {
      Ok(_) => {
        log::info!("Successfully removed residue: {}", path);
        Ok(ResponseModel {
          status: ResponseStatus::Success,
          message: format!("Removed residue: {}", path),
          data: DataValue::Bool(true),
        })
      }
      Err(e) => {
        log::error!("Failed to remove residue {}: {}", path, e);
        Err(ResponseModel {
          status: ResponseStatus::Error,
          message: format!("Failed to remove {}: {}", path, e),
          data: DataValue::Bool(false),
        })
      }
    }
  }

  pub fn clean_multiple(&self, paths: Vec<String>) -> Result<ResponseModel, ResponseModel> {
    log::info!("Attempting to clean {} residue items", paths.len());
    let mut removed = 0;
    let mut failed: Vec<String> = Vec::new();

    for path in paths {
      let path_obj = Path::new(&path);
      if !path_obj.exists() {
        log::warn!("Path does not exist, skipping: {}", path);
        continue;
      }

      let result = if path_obj.is_dir() {
        fs::remove_dir_all(&path)
      } else {
        fs::remove_file(&path)
      };

      match result {
        Ok(_) => {
          log::debug!("Removed: {}", path);
          removed += 1
        }
        Err(e) => {
          log::error!("Failed to remove {}: {}", path, e);
          failed.push(format!("{}: {}", path, e))
        }
      }
    }

    let result = serde_json::json!({
        "removed": removed,
        "failed": failed
    });

    if failed.is_empty() {
      log::info!("Successfully cleaned all {} residue items", removed);
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Cleaned {} residue items", removed),
        data: DataValue::Object(result),
      })
    } else {
      log::warn!("Cleaned {} items, {} failed", removed, failed.len());
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Cleaned {} items, {} failed", removed, failed.len()),
        data: DataValue::Object(result),
      })
    }
  }

  pub fn get_residue_summary(&self) -> AppResidueSummary {
    let configs = self.scan_user_configs();
    let data = self.scan_user_data();
    let caches = self.scan_user_caches();

    let total_configs = configs.len();
    let total_data = data.len();
    let total_caches = caches.len();
    let found_uninstalled = configs
      .iter()
      .chain(data.iter())
      .chain(caches.iter())
      .filter(|r| r.detected_as_uninstalled)
      .count();
    let total_size: u64 = configs
      .iter()
      .chain(data.iter())
      .chain(caches.iter())
      .map(|r| r.size)
      .sum();

    AppResidueSummary {
      total_configs,
      total_data,
      total_caches,
      total_size,
      found_uninstalled,
    }
  }

  pub fn scan_user_configs_response(&self) -> Result<ResponseModel, ResponseModel> {
    let residues = self.scan_user_configs();
    let count = residues.len();
    let data = models_into_data_array(residues).map_err(|e| AppError::Serde(e).into_response())?;
    Ok(success_response(
      format!("Found {} config residues", count),
      data,
    ))
  }

  pub fn scan_user_data_response(&self) -> Result<ResponseModel, ResponseModel> {
    let residues = self.scan_user_data();
    let count = residues.len();
    let data = models_into_data_array(residues).map_err(|e| AppError::Serde(e).into_response())?;
    Ok(success_response(
      format!("Found {} data residues", count),
      data,
    ))
  }

  pub fn scan_user_caches_response(&self) -> Result<ResponseModel, ResponseModel> {
    let residues = self.scan_user_caches();
    let count = residues.len();
    let data = models_into_data_array(residues).map_err(|e| AppError::Serde(e).into_response())?;
    Ok(success_response(
      format!("Found {} cache residues", count),
      data,
    ))
  }

  pub fn scan_home_residues_response(&self) -> Result<ResponseModel, ResponseModel> {
    let residues = self.scan_home_residues();
    let count = residues.len();
    let data = models_into_data_array(residues).map_err(|e| AppError::Serde(e).into_response())?;
    Ok(success_response(
      format!("Found {} home residues", count),
      data,
    ))
  }

  pub fn get_orphaned_configs_response(&self) -> Result<ResponseModel, ResponseModel> {
    let orphaned = self.get_orphaned_configs();
    let count = orphaned.len();
    let data = models_into_data_array(orphaned).map_err(|e| AppError::Serde(e).into_response())?;
    Ok(success_response(
      format!("Found {} orphaned configs", count),
      data,
    ))
  }
}
