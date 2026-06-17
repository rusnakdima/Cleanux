use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use log;

use crate::models::{AppError, Response, Status};
use crate::services::app_residue::AppDetector;
use crate::utils::{calculate_dir_size, home_dir, models_into_data_array, success_response};
use serde_json::Value;

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
  pub fn scan_user_configs(&self) -> Vec<AppResidue> {
    let config_dir = match home_dir() {
      Ok(h) => h.join(".config"),
      Err(_) => return Vec::new(),
    };

    if !config_dir.exists() {
      return Vec::new();
    }

    let installed = AppDetector::get_installed_apps();
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

        let is_installed = AppDetector::matches_installed_app(&app_name, &installed);

        let size = calculate_dir_size(&path).map(|(s, _)| s).unwrap_or(0);
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

    let installed = AppDetector::get_installed_apps();
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
      .map(|r| AppDetector::normalize_app_name(&r.app_name))
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

        let normalized = AppDetector::normalize_app_name(&app_name);
        let is_installed = config_names.contains(&normalized)
          || AppDetector::matches_installed_app(&app_name, &installed);

        let size = calculate_dir_size(&path).map(|(s, _)| s).unwrap_or(0);
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

    let installed = AppDetector::get_installed_apps();
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

        let is_installed = AppDetector::matches_installed_app(&app_name, &installed);

        let size = calculate_dir_size(&path).map(|(s, _)| s).unwrap_or(0);
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
    use crate::utils::stdout_string;

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
              let installed = AppDetector::get_installed_apps();
              if !AppDetector::matches_installed_app(&pkg_name, &installed) {
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
    let installed = AppDetector::get_installed_apps();

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

            let is_installed = AppDetector::matches_installed_app(&app_name, &installed);
            let size = calculate_dir_size(&path).map(|(s, _)| s).unwrap_or(0);

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

  pub fn clean_residue(&self, path: &str) -> Result<Response<Value>, Response<Value>> {
    log::info!("Attempting to clean residue at: {}", path);
    let path_obj = Path::new(path);

    if !path_obj.exists() {
      log::error!("Path does not exist: {}", path);
      return Err(Response {
        status: Status::Error,
        message: format!("Path does not exist: {}", path),
        data: Value::Bool(false),
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
        Ok(Response {
          status: Status::Success,
          message: format!("Removed residue: {}", path),
          data: Value::Bool(true),
        })
      }
      Err(e) => {
        log::error!("Failed to remove residue {}: {}", path, e);
        Err(Response {
          status: Status::Error,
          message: format!("Failed to remove {}: {}", path, e),
          data: Value::Bool(false),
        })
      }
    }
  }

  pub fn clean_multiple(&self, paths: Vec<String>) -> Result<Response<Value>, Response<Value>> {
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
      Ok(Response {
        status: Status::Success,
        message: format!("Cleaned {} residue items", removed),
        data: result,
      })
    } else {
      log::warn!("Cleaned {} items, {} failed", removed, failed.len());
      Ok(Response {
        status: Status::Success,
        message: format!("Cleaned {} items, {} failed", removed, failed.len()),
        data: result,
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

  pub fn scan_user_configs_response(&self) -> Result<Response<Value>, Response<Value>> {
    let residues = self.scan_user_configs();
    let count = residues.len();
    let data = models_into_data_array(residues).map_err(|e| AppError::from(e).into_response())?;
    Ok(success_response(
      format!("Found {} config residues", count),
      data,
    ))
  }

  pub fn scan_user_data_response(&self) -> Result<Response<Value>, Response<Value>> {
    let residues = self.scan_user_data();
    let count = residues.len();
    let data = models_into_data_array(residues).map_err(|e| AppError::from(e).into_response())?;
    Ok(success_response(
      format!("Found {} data residues", count),
      data,
    ))
  }

  pub fn scan_user_caches_response(&self) -> Result<Response<Value>, Response<Value>> {
    let residues = self.scan_user_caches();
    let count = residues.len();
    let data = models_into_data_array(residues).map_err(|e| AppError::from(e).into_response())?;
    Ok(success_response(
      format!("Found {} cache residues", count),
      data,
    ))
  }

  pub fn scan_home_residues_response(&self) -> Result<Response<Value>, Response<Value>> {
    let residues = self.scan_home_residues();
    let count = residues.len();
    let data = models_into_data_array(residues).map_err(|e| AppError::from(e).into_response())?;
    Ok(success_response(
      format!("Found {} home residues", count),
      data,
    ))
  }

  pub fn get_orphaned_configs_response(&self) -> Result<Response<Value>, Response<Value>> {
    let orphaned = self.get_orphaned_configs();
    let count = orphaned.len();
    let data = models_into_data_array(orphaned).map_err(|e| AppError::from(e).into_response())?;
    Ok(success_response(
      format!("Found {} orphaned configs", count),
      data,
    ))
  }
}
