use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::helpers::{home_dir, stderr_string, stdout_string};
use crate::models::{DataValue, ResponseModel, ResponseStatus};
use walkdir::WalkDir;

#[derive(Debug, Clone, serde::Serialize)]
pub struct RepairItem {
  pub path: String,
  pub issue_type: String,
  pub severity: String,
  pub description: Option<String>,
}

pub struct RepairService;

fn get_common_dirs() -> Vec<PathBuf> {
  let mut dirs = Vec::new();
  if let Ok(home) = home_dir() {
    dirs.push(home.join(".config"));
    dirs.push(home.join(".local/share"));
    dirs.push(home.join(".local/lib"));
  }
  dirs.push(PathBuf::from("/usr/local/lib"));
  dirs.push(PathBuf::from("/usr/lib"));
  dirs
}

#[allow(non_snake_case)]
impl RepairService {
  pub fn find_broken_symlinks() -> Result<ResponseModel, ResponseModel> {
    let mut broken_links: Vec<RepairItem> = Vec::new();

    for dir in get_common_dirs() {
      if !dir.exists() {
        continue;
      }

      for entry in WalkDir::new(&dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
      {
        let path = entry.path();
        if path.is_symlink() {
          let target = fs::read_link(path);
          if let Ok(target_path) = target {
            if !target_path.exists() {
              broken_links.push(RepairItem {
                path: path.to_string_lossy().into_owned(),
                issue_type: "broken_symlink".to_string(),
                severity: "warning".to_string(),
                description: Some(format!(
                  "Broken symlink points to non-existent target: {}",
                  target_path.display()
                )),
              });
            }
          }
        }
      }
    }

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: format!("Found {} broken symlinks", broken_links.len()),
      data: DataValue::Array(
        broken_links
          .into_iter()
          .map(serde_json::to_value)
          .filter_map(|r| r.ok())
          .collect(),
      ),
    })
  }

  pub fn find_orphaned_packages() -> Result<ResponseModel, ResponseModel> {
    let mut orphaned: Vec<RepairItem> = Vec::new();

    if let Ok(output) = Command::new("dpkg").arg("-l").output() {
      if output.status.success() {
        let stdout = stdout_string(&output);
        let lines: Vec<&str> = stdout.lines().collect();
        let package_names: std::collections::HashSet<String> = lines
          .iter()
          .skip(5)
          .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
              let status = parts[0];
              let name = parts[3];
              if status == "rc" {
                Some(name.to_string())
              } else {
                None
              }
            } else {
              None
            }
          })
          .collect();

        for pkg in package_names {
          orphaned.push(RepairItem {
            path: pkg.clone(),
            issue_type: "orphaned_package".to_string(),
            severity: "info".to_string(),
            description: Some(format!(
              "Package {} is marked as removed but not fully purged",
              pkg
            )),
          });
        }
      }
    }

    if let Ok(output) = Command::new("rpm")
      .args(["-qa", "--qf", "%{NAME}\n"])
      .output()
    {
      if output.status.success() {
        let stdout = stdout_string(&output);
        for line in stdout.lines() {
          if !line.is_empty() {
            orphaned.push(RepairItem {
              path: line.to_string(),
              issue_type: "orphaned_package".to_string(),
              severity: "info".to_string(),
              description: Some(format!(
                "RPM package {} may have leftover files after removal",
                line
              )),
            });
          }
        }
      }
    }

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: format!("Found {} orphaned packages", orphaned.len()),
      data: DataValue::Array(
        orphaned
          .into_iter()
          .map(serde_json::to_value)
          .filter_map(|r| r.ok())
          .collect(),
      ),
    })
  }

  pub fn clean_font_cache() -> Result<ResponseModel, ResponseModel> {
    let font_cache_path = home_dir()
      .map(|h| h.join(".cache/fontconfig"))
      .ok()
      .unwrap_or_else(|| PathBuf::from(".cache/fontconfig"));

    if !font_cache_path.exists() {
      return Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: "Font cache directory does not exist".to_string(),
        data: DataValue::Array(vec![]),
      });
    }

    let mut removed_count = 0;
    let mut failed: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(&font_cache_path) {
      for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let file_name = path
          .file_name()
          .map(|n| n.to_string_lossy().into_owned())
          .unwrap_or_default();

        if file_name.ends_with(".cache") || file_name == "fonts.cache-db" {
          match fs::remove_file(&path) {
            Ok(_) => removed_count += 1,
            Err(e) => failed.push(format!("{}: {}", path.display(), e)),
          }
        }
      }
    }

    let result = serde_json::json!({
        "removed": removed_count,
        "failed": failed
    });

    if failed.is_empty() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Cleaned {} font cache files", removed_count),
        data: DataValue::Object(result),
      })
    } else {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Cleaned {} files, {} failed", removed_count, failed.len()),
        data: DataValue::Object(result),
      })
    }
  }

  pub fn clean_icon_cache() -> Result<ResponseModel, ResponseModel> {
    let icon_cache_path = home_dir()
      .map(|h| h.join(".cache/icon-cache"))
      .ok()
      .unwrap_or_else(|| PathBuf::from(".cache/icon-cache"));

    let icon_cache_path_kde = home_dir()
      .map(|h| h.join(".cache/plasma_icon_cache"))
      .ok()
      .unwrap_or_else(|| PathBuf::from(".cache/plasma_icon_cache"));

    let mut removed_count = 0;
    let mut failed: Vec<String> = Vec::new();
    let mut cleaned_paths: Vec<String> = Vec::new();

    for cache_path in &[icon_cache_path, icon_cache_path_kde] {
      if cache_path.exists() {
        if let Ok(entries) = fs::read_dir(cache_path) {
          for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            match fs::remove_file(&path) {
              Ok(_) => {
                removed_count += 1;
                cleaned_paths.push(path.to_string_lossy().into_owned());
              }
              Err(e) => failed.push(format!("{}: {}", path.display(), e)),
            }
          }
        }
        if let Ok(()) = fs::remove_dir(cache_path) {
          cleaned_paths.push(cache_path.to_string_lossy().into_owned());
        }
      }
    }

    let result = serde_json::json!({
        "removed": removed_count,
        "failed": failed,
        "cleaned_paths": cleaned_paths
    });

    if failed.is_empty() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Cleaned {} icon cache files", removed_count),
        data: DataValue::Object(result),
      })
    } else {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Cleaned {} files, {} failed", removed_count, failed.len()),
        data: DataValue::Object(result),
      })
    }
  }

  pub fn repair_permissions() -> Result<ResponseModel, ResponseModel> {
    let mut repaired_count = 0;
    let mut failed: Vec<String> = Vec::new();

    let common_paths = vec![
      home_dir().map(|h| h.join(".config")),
      home_dir().map(|h| h.join(".local/share")),
      home_dir().map(|h| h.join(".cache")),
    ];

    for path in common_paths.into_iter().flatten() {
      if path.exists() {
        let perms_path = path.to_string_lossy().into_owned();
        let output = Command::new("pkexec")
          .args(["chmod", "755", &perms_path])
          .output();

        match output {
          Ok(result) => {
            if result.status.success() {
              repaired_count += 1;
            } else {
              failed.push(format!("{}: permission fix failed", perms_path));
            }
          }
          Err(e) => {
            failed.push(format!("{}: {}", perms_path, e));
          }
        }
      }
    }

    let result = serde_json::json!({
        "repaired": repaired_count,
        "failed": failed
    });

    if failed.is_empty() {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Repaired permissions for {} items", repaired_count),
        data: DataValue::Object(result),
      })
    } else {
      Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Repaired {} items, {} failed", repaired_count, failed.len()),
        data: DataValue::Object(result),
      })
    }
  }

  pub fn remove_broken_symlink(path: &str) -> Result<ResponseModel, ResponseModel> {
    let symlink_path = Path::new(path);

    if !symlink_path.exists() {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Path does not exist: {}", path),
        data: DataValue::Bool(false),
      });
    }

    if !symlink_path.is_symlink() {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Path is not a symlink: {}", path),
        data: DataValue::Bool(false),
      });
    }

    match fs::remove_file(path) {
      Ok(_) => Ok(ResponseModel {
        status: ResponseStatus::Success,
        message: format!("Removed broken symlink: {}", path),
        data: DataValue::Bool(true),
      }),
      Err(e) => Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Failed to remove symlink: {} - {}", path, e),
        data: DataValue::Bool(false),
      }),
    }
  }

  pub fn remove_orphaned_package(path: &str) -> Result<ResponseModel, ResponseModel> {
    let is_valid_package_name = path
      .chars()
      .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' || c == '+');
    if !is_valid_package_name || path.is_empty() {
      return Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Invalid package name format: {}", path),
        data: DataValue::Bool(false),
      });
    }

    let output = Command::new("dpkg").args(["--purge", path]).output();

    match output {
      Ok(result) => {
        if result.status.success() {
          Ok(ResponseModel {
            status: ResponseStatus::Success,
            message: format!("Purged package: {}", path),
            data: DataValue::Bool(true),
          })
        } else {
          let stderr = stderr_string(&result);
          Err(ResponseModel {
            status: ResponseStatus::Error,
            message: format!("Failed to purge package {}: {}", path, stderr),
            data: DataValue::Bool(false),
          })
        }
      }
      Err(e) => Err(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Failed to execute dpkg: {}", e),
        data: DataValue::Bool(false),
      }),
    }
  }
}
