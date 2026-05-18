/* helpers */
use crate::helpers::{data_string, success_response};
/* models */
use crate::models::{AppError, ResponseModel};
/* sys lib */
use std::fs;
use std::path::{Path, PathBuf};

type DevCacheResult<T> = Result<T, AppError>;

#[derive(Debug, Clone)]
pub struct DevCacheItem {
  pub name: String,
  pub cache_path: String,
  pub size: u64,
  pub description: String,
}

#[derive(Debug, Clone)]
pub struct DevCacheSummary {
  pub npm: DevCacheItem,
  pub pip: DevCacheItem,
  pub cargo: DevCacheItem,
  pub go: DevCacheItem,
  pub maven: DevCacheItem,
  pub gradle: DevCacheItem,
}

pub struct DevCacheService;

impl DevCacheService {
  pub fn get_all_dev_caches(&self) -> ResponseModel {
    self
      .get_all_dev_caches_inner()
      .map_err(|e| e.into_response())
  }

  fn get_all_dev_caches_inner(&self) -> DevCacheResult<ResponseModel> {
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;

    let npm = self.scan_npm_cache_inner(&home);
    let pip = self.scan_pip_cache_inner(&home);
    let cargo = self.scan_cargo_cache_inner(&home);
    let go = self.scan_go_cache_inner(&home);
    let maven = self.scan_maven_cache_inner(&home);
    let gradle = self.scan_gradle_cache_inner(&home);

    let mut summary = serde_json::Map::new();
    summary.insert("npm".to_string(), serde_json::json!(npm));
    summary.insert("pip".to_string(), serde_json::json!(pip));
    summary.insert("cargo".to_string(), serde_json::json!(cargo));
    summary.insert("go".to_string(), serde_json::json!(go));
    summary.insert("maven".to_string(), serde_json::json!(maven));
    summary.insert("gradle".to_string(), serde_json::json!(gradle));

    Ok(success_response(
      "Dev cache summary retrieved successfully",
      serde_json::Value::Object(summary).into(),
    ))
  }

  pub fn get_npm_cache_size(&self) -> u64 {
    let home = dirs::home_dir().unwrap_or_default();
    self.scan_npm_cache_inner(&home).size
  }

  pub fn get_pip_cache_size(&self) -> u64 {
    let home = dirs::home_dir().unwrap_or_default();
    self.scan_pip_cache_inner(&home).size
  }

  pub fn get_cargo_cache_size(&self) -> u64 {
    let home = dirs::home_dir().unwrap_or_default();
    self.scan_cargo_cache_inner(&home).size
  }

  pub fn get_go_cache_size(&self) -> u64 {
    let home = dirs::home_dir().unwrap_or_default();
    self.scan_go_cache_inner(&home).size
  }

  pub fn get_maven_cache_size(&self) -> u64 {
    let home = dirs::home_dir().unwrap_or_default();
    self.scan_maven_cache_inner(&home).size
  }

  pub fn get_gradle_cache_size(&self) -> u64 {
    let home = dirs::home_dir().unwrap_or_default();
    self.scan_gradle_cache_inner(&home).size
  }

  fn scan_npm_cache_inner(&self, home: &Path) -> DevCacheItem {
    let paths = [home.join(".npm"), PathBuf::from("/tmp/npm-*")];

    let mut total_size = 0u64;
    let mut cache_path = String::new();

    for path in &paths {
      if path.exists() {
        let (size, _) = Self::calculate_dir_size(path);
        if size > 0 {
          total_size += size;
          if cache_path.is_empty() {
            cache_path = path.to_string_lossy().to_string();
          }
        }
      }
    }

    if cache_path.is_empty() {
      cache_path = "~/.npm".to_string();
    }

    DevCacheItem {
      name: "npm".to_string(),
      cache_path,
      size: total_size,
      description: "Node.js package manager cache".to_string(),
    }
  }

  fn scan_pip_cache_inner(&self, home: &Path) -> DevCacheItem {
    let pip_cache_path = home.join(".cache/pip");
    let root_pip_cache = PathBuf::from("/root/.cache/pip");

    let mut total_size = 0u64;
    let mut cache_path = String::new();

    if pip_cache_path.exists() {
      let (size, _) = Self::calculate_dir_size(&pip_cache_path);
      total_size += size;
      cache_path = pip_cache_path.to_string_lossy().to_string();
    }
    if root_pip_cache.exists() && root_pip_cache != pip_cache_path {
      let (size, _) = Self::calculate_dir_size(&root_pip_cache);
      total_size += size;
      if cache_path.is_empty() {
        cache_path = root_pip_cache.to_string_lossy().to_string();
      }
    }

    if cache_path.is_empty() {
      cache_path = "~/.cache/pip".to_string();
    }

    DevCacheItem {
      name: "pip".to_string(),
      cache_path,
      size: total_size,
      description: "Python package manager cache".to_string(),
    }
  }

  fn scan_cargo_cache_inner(&self, home: &Path) -> DevCacheItem {
    let registry = home.join(".cargo/registry");
    let git = home.join(".cargo/git");

    let mut total_size = 0u64;
    let mut cache_path = String::new();

    if registry.exists() {
      let (size, _) = Self::calculate_dir_size(&registry);
      total_size += size;
      cache_path = registry.to_string_lossy().to_string();
    }
    if git.exists() {
      let (size, _) = Self::calculate_dir_size(&git);
      total_size += size;
      if cache_path.is_empty() {
        cache_path = git.to_string_lossy().to_string();
      }
    }

    if cache_path.is_empty() {
      cache_path = "~/.cargo/registry".to_string();
    }

    DevCacheItem {
      name: "cargo".to_string(),
      cache_path,
      size: total_size,
      description: "Rust package manager cache".to_string(),
    }
  }

  fn scan_go_cache_inner(&self, home: &Path) -> DevCacheItem {
    let go_path = home.join("go/pkg/mod");

    let mut total_size = 0u64;
    let cache_path = if go_path.exists() {
      let (size, _) = Self::calculate_dir_size(&go_path);
      total_size = size;
      go_path.to_string_lossy().to_string()
    } else {
      "~/go/pkg/mod".to_string()
    };

    DevCacheItem {
      name: "go".to_string(),
      cache_path,
      size: total_size,
      description: "Go module cache".to_string(),
    }
  }

  fn scan_maven_cache_inner(&self, home: &Path) -> DevCacheItem {
    let maven_path = home.join(".m2/repository");

    let mut total_size = 0u64;
    let cache_path = if maven_path.exists() {
      let (size, _) = Self::calculate_dir_size(&maven_path);
      total_size = size;
      maven_path.to_string_lossy().to_string()
    } else {
      "~/.m2/repository".to_string()
    };

    DevCacheItem {
      name: "maven".to_string(),
      cache_path,
      size: total_size,
      description: "Maven repository cache".to_string(),
    }
  }

  fn scan_gradle_cache_inner(&self, home: &Path) -> DevCacheItem {
    let gradle_path = home.join(".gradle/caches");

    let mut total_size = 0u64;
    let cache_path = if gradle_path.exists() {
      let (size, _) = Self::calculate_dir_size(&gradle_path);
      total_size = size;
      gradle_path.to_string_lossy().to_string()
    } else {
      "~/.gradle/caches".to_string()
    };

    DevCacheItem {
      name: "gradle".to_string(),
      cache_path,
      size: total_size,
      description: "Gradle build cache".to_string(),
    }
  }

  pub fn clean_npm_cache(&self) -> ResponseModel {
    self.clean_npm_cache_inner().map_err(|e| e.into_response())
  }

  fn clean_npm_cache_inner(&self) -> DevCacheResult<ResponseModel> {
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let npm_path = home.join(".npm");

    if !npm_path.exists() {
      return Ok(success_response(
        "npm cache already clean",
        data_string("0"),
      ));
    }

    match Self::remove_dir_contents(&npm_path) {
      Ok(count) => Ok(success_response(
        format!("Cleaned npm cache ({} items)", count),
        data_string(count.to_string()),
      )),
      Err(e) => Err(AppError::message(format!(
        "Failed to clean npm cache: {}",
        e
      ))),
    }
  }

  pub fn clean_pip_cache(&self) -> ResponseModel {
    self.clean_pip_cache_inner().map_err(|e| e.into_response())
  }

  fn clean_pip_cache_inner(&self) -> DevCacheResult<ResponseModel> {
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let pip_path = home.join(".cache/pip");

    if !pip_path.exists() {
      return Ok(success_response(
        "pip cache already clean",
        data_string("0"),
      ));
    }

    match Self::remove_dir_contents(&pip_path) {
      Ok(count) => Ok(success_response(
        format!("Cleaned pip cache ({} items)", count),
        data_string(count.to_string()),
      )),
      Err(e) => Err(AppError::message(format!(
        "Failed to clean pip cache: {}",
        e
      ))),
    }
  }

  pub fn clean_cargo_cache(&self) -> ResponseModel {
    self
      .clean_cargo_cache_inner()
      .map_err(|e| e.into_response())
  }

  fn clean_cargo_cache_inner(&self) -> DevCacheResult<ResponseModel> {
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let registry = home.join(".cargo/registry");
    let git = home.join(".cargo/git");

    let mut cleaned_count = 0u32;
    let mut errors: Vec<String> = Vec::new();

    if registry.exists() {
      match Self::remove_dir_contents(&registry) {
        Ok(count) => cleaned_count += count,
        Err(e) => errors.push(format!("registry: {}", e)),
      }
    }
    if git.exists() {
      match Self::remove_dir_contents(&git) {
        Ok(count) => cleaned_count += count,
        Err(e) => errors.push(format!("git: {}", e)),
      }
    }

    if errors.is_empty() {
      Ok(success_response(
        format!("Cleaned cargo cache ({} items)", cleaned_count),
        data_string(cleaned_count.to_string()),
      ))
    } else {
      Err(AppError::message(format!(
        "Cleaned {}, errors: {}",
        cleaned_count,
        errors.join("; ")
      )))
    }
  }

  pub fn clean_go_cache(&self) -> ResponseModel {
    self.clean_go_cache_inner().map_err(|e| e.into_response())
  }

  fn clean_go_cache_inner(&self) -> DevCacheResult<ResponseModel> {
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let go_path = home.join("go/pkg/mod");

    if !go_path.exists() {
      return Ok(success_response("Go cache already clean", data_string("0")));
    }

    match Self::remove_dir_contents(&go_path) {
      Ok(count) => Ok(success_response(
        format!("Cleaned Go cache ({} items)", count),
        data_string(count.to_string()),
      )),
      Err(e) => Err(AppError::message(format!(
        "Failed to clean Go cache: {}",
        e
      ))),
    }
  }

  pub fn clean_maven_cache(&self) -> ResponseModel {
    self
      .clean_maven_cache_inner()
      .map_err(|e| e.into_response())
  }

  fn clean_maven_cache_inner(&self) -> DevCacheResult<ResponseModel> {
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let maven_path = home.join(".m2/repository");

    if !maven_path.exists() {
      return Ok(success_response(
        "Maven cache already clean",
        data_string("0"),
      ));
    }

    match Self::remove_dir_contents(&maven_path) {
      Ok(count) => Ok(success_response(
        format!("Cleaned Maven cache ({} items)", count),
        data_string(count.to_string()),
      )),
      Err(e) => Err(AppError::message(format!(
        "Failed to clean Maven cache: {}",
        e
      ))),
    }
  }

  pub fn clean_gradle_cache(&self) -> ResponseModel {
    self
      .clean_gradle_cache_inner()
      .map_err(|e| e.into_response())
  }

  fn clean_gradle_cache_inner(&self) -> DevCacheResult<ResponseModel> {
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let gradle_path = home.join(".gradle/caches");

    if !gradle_path.exists() {
      return Ok(success_response(
        "Gradle cache already clean",
        data_string("0"),
      ));
    }

    match Self::remove_dir_contents(&gradle_path) {
      Ok(count) => Ok(success_response(
        format!("Cleaned Gradle cache ({} items)", count),
        data_string(count.to_string()),
      )),
      Err(e) => Err(AppError::message(format!(
        "Failed to clean Gradle cache: {}",
        e
      ))),
    }
  }

  pub fn clean_all_dev_caches(&self) -> ResponseModel {
    self
      .clean_all_dev_caches_inner()
      .map_err(|e| e.into_response())
  }

  fn clean_all_dev_caches_inner(&self) -> DevCacheResult<ResponseModel> {
    let mut cleaned_total = 0u32;
    let mut errors: Vec<String> = Vec::new();

    if let Ok(response) = self.clean_npm_cache_inner() {
      if let Some(count) = Self::extract_count_from_response(&response) {
        cleaned_total += count;
      }
    } else {
      errors.push("npm".to_string());
    }

    if let Ok(response) = self.clean_pip_cache_inner() {
      if let Some(count) = Self::extract_count_from_response(&response) {
        cleaned_total += count;
      }
    } else {
      errors.push("pip".to_string());
    }

    if let Ok(response) = self.clean_cargo_cache_inner() {
      if let Some(count) = Self::extract_count_from_response(&response) {
        cleaned_total += count;
      }
    } else {
      errors.push("cargo".to_string());
    }

    if let Ok(response) = self.clean_go_cache_inner() {
      if let Some(count) = Self::extract_count_from_response(&response) {
        cleaned_total += count;
      }
    } else {
      errors.push("go".to_string());
    }

    if let Ok(response) = self.clean_maven_cache_inner() {
      if let Some(count) = Self::extract_count_from_response(&response) {
        cleaned_total += count;
      }
    } else {
      errors.push("maven".to_string());
    }

    if let Ok(response) = self.clean_gradle_cache_inner() {
      if let Some(count) = Self::extract_count_from_response(&response) {
        cleaned_total += count;
      }
    } else {
      errors.push("gradle".to_string());
    }

    if errors.is_empty() {
      Ok(success_response(
        format!("Cleaned all dev caches ({} items)", cleaned_total),
        data_string(cleaned_total.to_string()),
      ))
    } else {
      Err(AppError::message(format!(
        "Cleaned {}, failed: {}",
        cleaned_total,
        errors.join(", ")
      )))
    }
  }

  fn extract_count_from_response(response: &ResponseModel) -> Option<u32> {
    if let crate::models::DataValue::String(s) = &response.data {
      s.parse().ok()
    } else {
      None
    }
  }

  fn calculate_dir_size(path: &Path) -> (u64, u32) {
    let mut total_size = 0u64;
    let mut file_count = 0u32;

    if let Ok(entries) = fs::read_dir(path) {
      for entry in entries.flatten() {
        if entry.path().is_file() {
          if let Ok(metadata) = fs::metadata(&entry.path()) {
            total_size += metadata.len();
            file_count += 1;
          }
        } else if entry.path().is_dir() {
          let (size, count) = Self::calculate_dir_size(&entry.path());
          total_size += size;
          file_count += count;
        }
      }
    }

    (total_size, file_count)
  }

  fn remove_dir_contents(path: &Path) -> Result<u32, String> {
    let mut count = 0u32;

    if let Ok(entries) = fs::read_dir(path) {
      for entry in entries.flatten() {
        let entry_path = entry.path();
        if entry_path.is_dir() {
          if let Err(e) = fs::remove_dir_all(&entry_path) {
            return Err(e.to_string());
          }
          count += 1;
        } else if entry_path.is_file() {
          if let Err(e) = fs::remove_file(&entry_path) {
            return Err(e.to_string());
          }
          count += 1;
        }
      }
    }

    Ok(count)
  }
}
