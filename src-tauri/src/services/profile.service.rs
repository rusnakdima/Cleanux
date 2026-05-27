/* models */
use crate::models::{AppError, CleaningProfile, DataValue, ResponseModel};
/* helpers */
use crate::helpers::{data_empty_string, data_string, success_response};
/* sys lib */
use std::fs;
use std::path::PathBuf;

type ProfileResult<T> = Result<T, AppError>;

pub struct ProfileService;

impl ProfileService {
  fn get_profiles_dir() -> ProfileResult<PathBuf> {
    let config_dir =
      dirs::config_dir().ok_or_else(|| AppError::message("Config directory not found"))?;
    Ok(config_dir.join("cleanux").join("profiles"))
  }

  fn ensure_profiles_dir() -> ProfileResult<PathBuf> {
    let dir = Self::get_profiles_dir()?;
    if !dir.exists() {
      fs::create_dir_all(&dir)
        .map_err(|e| AppError::message(format!("Failed to create profiles directory: {}", e)))?;
    }
    Ok(dir)
  }

  fn get_profile_path(name: &str) -> ProfileResult<PathBuf> {
    let dir = Self::ensure_profiles_dir()?;
    let safe_name = name.replace('/', "_").replace('\\', "_").replace("..", "_");
    Ok(dir.join(format!("{}.json", safe_name)))
  }

  pub fn save_profile(profile: CleaningProfile) -> Result<ResponseModel, ResponseModel> {
    Self::save_profile_inner(profile).map_err(|e| e.into_response())
  }

  fn save_profile_inner(profile: CleaningProfile) -> ProfileResult<ResponseModel> {
    let path = Self::get_profile_path(&profile.name)?;
    let json = serde_json::to_string_pretty(&profile)
      .map_err(|e| AppError::message(format!("Failed to serialize profile: {}", e)))?;
    fs::write(&path, json)
      .map_err(|e| AppError::message(format!("Failed to save profile: {}", e)))?;
    Ok(success_response(
      format!("Profile '{}' saved successfully", profile.name),
      data_empty_string(),
    ))
  }

  pub fn load_profile(name: &str) -> Result<ResponseModel, ResponseModel> {
    Self::load_profile_inner(name).map_err(|e| e.into_response())
  }

  fn load_profile_inner(name: &str) -> ProfileResult<ResponseModel> {
    let path = Self::get_profile_path(name)?;
    if !path.exists() {
      return Err(AppError::message("Profile not found"));
    }
    let json = fs::read_to_string(&path)
      .map_err(|e| AppError::message(format!("Failed to read profile: {}", e)))?;
    let profile: CleaningProfile = serde_json::from_str(&json)
      .map_err(|e| AppError::message(format!("Failed to parse profile: {}", e)))?;
    Ok(success_response(
      format!("Profile '{}' loaded successfully", name),
      DataValue::Object(serde_json::to_value(&profile).unwrap_or(serde_json::Value::Null)),
    ))
  }

  pub fn list_profiles() -> Result<ResponseModel, ResponseModel> {
    Self::list_profiles_inner().map_err(|e| e.into_response())
  }

  fn list_profiles_inner() -> ProfileResult<ResponseModel> {
    let dir = Self::ensure_profiles_dir()?;
    let mut profiles: Vec<serde_json::Value> = Vec::new();

    if dir.exists() {
      let entries = fs::read_dir(&dir)
        .map_err(|e| AppError::message(format!("Failed to read profiles directory: {}", e)))?;
      for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
          if let Ok(json) = fs::read_to_string(&path) {
            if let Ok(profile) = serde_json::from_str::<CleaningProfile>(&json) {
              profiles.push(serde_json::json!({
                  "name": profile.name,
                  "description": profile.description,
                  "created_at": profile.created_at,
                  "paths": profile.paths,
                  "exclude_patterns": profile.exclude_patterns,
                  "clean_cache": profile.clean_cache,
                  "clean_trash": profile.clean_trash,
                  "clean_logs": profile.clean_logs,
                  "min_large_file_size": profile.min_large_file_size,
              }));
            }
          }
        }
      }
    }

    Ok(success_response(
      format!("Found {} profiles", profiles.len()),
      DataValue::Array(profiles),
    ))
  }

  pub fn delete_profile(name: &str) -> Result<ResponseModel, ResponseModel> {
    Self::delete_profile_inner(name).map_err(|e| e.into_response())
  }

  fn delete_profile_inner(name: &str) -> ProfileResult<ResponseModel> {
    let path = Self::get_profile_path(name)?;
    if !path.exists() {
      return Err(AppError::message("Profile not found"));
    }
    fs::remove_file(&path)
      .map_err(|e| AppError::message(format!("Failed to delete profile: {}", e)))?;
    Ok(success_response(
      format!("Profile '{}' deleted successfully", name),
      data_empty_string(),
    ))
  }

  pub fn apply_profile(&self, name: &str) -> Result<ResponseModel, ResponseModel> {
    Self::apply_profile_inner(name).map_err(|e| e.into_response())
  }

  fn apply_profile_inner(name: &str) -> ProfileResult<ResponseModel> {
    let path = Self::get_profile_path(name)?;
    if !path.exists() {
      return Err(AppError::message("Profile not found"));
    }
    let json = fs::read_to_string(&path)
      .map_err(|e| AppError::message(format!("Failed to read profile: {}", e)))?;
    let profile: CleaningProfile = serde_json::from_str(&json)
      .map_err(|e| AppError::message(format!("Failed to parse profile: {}", e)))?;

    let mut results: Vec<String> = Vec::new();

    if profile.clean_cache {
      if let Some(cache_dir) = dirs::cache_dir() {
        if cache_dir.exists() {
          match fs::remove_dir_all(&cache_dir) {
            Ok(_) => {
              let _ = fs::create_dir_all(&cache_dir);
              results.push("Cache cleared".to_string());
            }
            Err(e) => results.push(format!("Cache clear failed: {}", e)),
          }
        }
      }
    }

    if profile.clean_trash {
      if let Some(home) = dirs::home_dir() {
        let trash_dir = home.join(".local/share/Trash/files");
        if trash_dir.exists() {
          if let Ok(entries) = fs::read_dir(&trash_dir) {
            for entry in entries.flatten() {
              let path = entry.path();
              if path.is_file() {
                if let Err(e) = fs::remove_file(&path) {
                  results.push(format!("Trash clear partial: {}", e));
                  break;
                }
              }
            }
            results.push("Trash cleared".to_string());
          }
        }
      }
    }

    if profile.clean_logs {
      let log_dir = PathBuf::from("/var/log");
      if log_dir.exists() {
        results.push("Log cleaning requires elevated permissions".to_string());
      }
    }

    if profile.min_large_file_size > 0 {
      if let Some(_home) = dirs::home_dir() {
        results.push(format!(
          "Large file cleaning with threshold {} bytes",
          profile.min_large_file_size
        ));
      }
    }

    Ok(success_response(
      format!("Profile '{}' applied: {}", name, results.join(", ")),
      data_string(results.len().to_string()),
    ))
  }
}
