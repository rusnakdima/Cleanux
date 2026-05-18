/* helpers */
use crate::helpers::{data_string, success_response};
/* models */
use crate::models::{AppError, DataValue, ResponseModel};
/* sys lib */
use std::fs;
use std::path::{Path, PathBuf};

type CleanerResult<T> = Result<T, AppError>;

#[derive(Debug, Clone, PartialEq)]
pub enum JunkCategory {
  Browser,
  Thumbnails,
  Applications,
  System,
  Logs,
}

impl JunkCategory {
  fn as_str(&self) -> &'static str {
    match self {
      JunkCategory::Browser => "browser",
      JunkCategory::Thumbnails => "thumbnails",
      JunkCategory::Applications => "applications",
      JunkCategory::System => "system",
      JunkCategory::Logs => "logs",
    }
  }
}

#[derive(Debug, Clone)]
pub struct JunkItem {
  pub path: String,
  pub size: u64,
  pub category: JunkCategory,
  pub description: String,
  pub file_count: u32,
}

#[derive(Debug, Clone)]
pub struct JunkSummary {
  pub category: String,
  pub total_size: u64,
  pub file_count: u32,
  pub description: String,
  pub items: Vec<JunkItem>,
}

pub struct JunkCleanerService;

impl JunkCleanerService {
  pub fn get_junk_summary(&self) -> Result<ResponseModel, ResponseModel> {
    self.get_junk_summary_inner().map_err(|e| e.into_response())
  }

  fn get_junk_summary_inner(&self) -> CleanerResult<ResponseModel> {
    let mut summary = serde_json::Map::new();

    let browser = self.scan_browser_caches_inner()?;
    let thumbnails = self.scan_thumbnail_caches_inner()?;
    let applications = self.scan_application_caches_inner()?;
    let system = self.scan_system_temp_inner()?;
    let logs = self.scan_log_rotations_inner()?;

    fn junk_items_to_json(items: &[JunkItem]) -> serde_json::Value {
      serde_json::json!(items
        .iter()
        .map(|item| {
          serde_json::json!({
              "path": item.path,
              "size": item.size,
              "category": match item.category {
                  JunkCategory::Browser => "browser",
                  JunkCategory::Thumbnails => "thumbnails",
                  JunkCategory::Applications => "applications",
                  JunkCategory::System => "system",
                  JunkCategory::Logs => "logs",
              },
              "description": item.description,
              "fileCount": item.file_count,
          })
        })
        .collect::<Vec<_>>())
    }

    let browser_size: u64 = browser.iter().map(|i| i.size).sum();
    let browser_count: u32 = browser.iter().map(|i| i.file_count).sum();
    summary.insert(
      "browser".to_string(),
      serde_json::json!({
          "category": "Browser",
          "totalSize": browser_size,
          "fileCount": browser_count,
          "description": "Browser cache files (Firefox, Chrome, Brave, Edge)",
          "items": junk_items_to_json(&browser),
      }),
    );

    let thumbnails_size: u64 = thumbnails.iter().map(|i| i.size).sum();
    let thumbnails_count: u32 = thumbnails.iter().map(|i| i.file_count).sum();
    summary.insert(
      "thumbnails".to_string(),
      serde_json::json!({
          "category": "Thumbnails",
          "totalSize": thumbnails_size,
          "fileCount": thumbnails_count,
          "description": "Image thumbnail cache",
          "items": junk_items_to_json(&thumbnails),
      }),
    );

    let applications_size: u64 = applications.iter().map(|i| i.size).sum();
    let applications_count: u32 = applications.iter().map(|i| i.file_count).sum();
    summary.insert(
      "applications".to_string(),
      serde_json::json!({
          "category": "Applications",
          "totalSize": applications_size,
          "fileCount": applications_count,
          "description": "Application caches (Flatpak, Snap, AppImage)",
          "items": junk_items_to_json(&applications),
      }),
    );

    let system_size: u64 = system.iter().map(|i| i.size).sum();
    let system_count: u32 = system.iter().map(|i| i.file_count).sum();
    summary.insert(
      "system".to_string(),
      serde_json::json!({
          "category": "System",
          "totalSize": system_size,
          "fileCount": system_count,
          "description": "System temporary files (/tmp, /var/tmp)",
          "items": junk_items_to_json(&system),
      }),
    );

    let logs_size: u64 = logs.iter().map(|i| i.size).sum();
    let logs_count: u32 = logs.iter().map(|i| i.file_count).sum();
    summary.insert(
      "logs".to_string(),
      serde_json::json!({
          "category": "Logs",
          "totalSize": logs_size,
          "fileCount": logs_count,
          "description": "Rotated and old log files",
          "items": junk_items_to_json(&logs),
      }),
    );

    Ok(success_response(
      "Junk summary retrieved successfully",
      DataValue::Object(serde_json::Value::Object(summary)),
    ))
  }

  pub fn scan_browser_caches(&self) -> Result<ResponseModel, ResponseModel> {
    self
      .scan_browser_caches_inner()
      .map_err(|e| e.into_response())
  }

  fn scan_browser_caches_inner(&self) -> CleanerResult<Vec<JunkItem>> {
    let mut items = Vec::new();
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;

    let browser_paths = [
      (
        home.join(".cache/mozilla"),
        "Firefox",
        JunkCategory::Browser,
      ),
      (
        home.join(".cache/google-chrome"),
        "Chrome",
        JunkCategory::Browser,
      ),
      (home.join(".cache/Brave"), "Brave", JunkCategory::Browser),
      (
        home.join(".cache/microsoft-edge"),
        "Edge",
        JunkCategory::Browser,
      ),
    ];

    for (path, name, category) in browser_paths {
      if path.exists() {
        let (size, file_count) = Self::calculate_dir_size(&path);
        items.push(JunkItem {
          path: path.to_string_lossy().to_string(),
          size,
          category,
          description: format!("{} browser cache", name),
          file_count,
        });
      }
    }

    Ok(items)
  }

  pub fn scan_thumbnail_caches(&self) -> Result<ResponseModel, ResponseModel> {
    self
      .scan_thumbnail_caches_inner()
      .map_err(|e| e.into_response())
  }

  fn scan_thumbnail_caches_inner(&self) -> CleanerResult<Vec<JunkItem>> {
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let path = home.join(".cache/thumbnails");

    if !path.exists() {
      return Ok(Vec::new());
    }

    let (size, file_count) = Self::calculate_dir_size(&path);
    Ok(vec![JunkItem {
      path: path.to_string_lossy().to_string(),
      size,
      category: JunkCategory::Thumbnails,
      description: "Image thumbnail cache".to_string(),
      file_count,
    }])
  }

  pub fn scan_application_caches(&self) -> Result<ResponseModel, ResponseModel> {
    self
      .scan_application_caches_inner()
      .map_err(|e| e.into_response())
  }

  fn scan_application_caches_inner(&self) -> CleanerResult<Vec<JunkItem>> {
    let mut items = Vec::new();
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;

    let app_cache_paths = [
      (
        home.join(".cache/flatpak"),
        "Flatpak",
        JunkCategory::Applications,
      ),
      (
        home.join(".var/app"),
        "Flatpak (alt)",
        JunkCategory::Applications,
      ),
      (home.join("snap"), "Snap", JunkCategory::Applications),
      (
        home.join(".cache/snap"),
        "Snap (alt)",
        JunkCategory::Applications,
      ),
    ];

    for (path, name, category) in app_cache_paths {
      if path.exists() {
        let (size, file_count) = Self::calculate_dir_size(&path);
        items.push(JunkItem {
          path: path.to_string_lossy().to_string(),
          size,
          category,
          description: format!("{} application cache", name),
          file_count,
        });
      }
    }

    let app_image_cache = home.join(".cache/appimage");
    if app_image_cache.exists() {
      let (size, file_count) = Self::calculate_dir_size(&app_image_cache);
      items.push(JunkItem {
        path: app_image_cache.to_string_lossy().to_string(),
        size,
        category: JunkCategory::Applications,
        description: "AppImage cache".to_string(),
        file_count,
      });
    }

    Ok(items)
  }

  pub fn scan_system_temp(&self) -> Result<ResponseModel, ResponseModel> {
    self.scan_system_temp_inner().map_err(|e| e.into_response())
  }

  fn scan_system_temp_inner(&self) -> CleanerResult<Vec<JunkItem>> {
    let mut items = Vec::new();

    let temp_paths = [
      ("/tmp", "User temporary files"),
      ("/var/tmp", "System temporary files"),
    ];

    for (path_str, description) in temp_paths {
      let path = Path::new(path_str);
      if path.exists() {
        let (size, file_count) = Self::calculate_dir_size(path);
        items.push(JunkItem {
          path: path_str.to_string(),
          size,
          category: JunkCategory::System,
          description: description.to_string(),
          file_count,
        });
      }
    }

    Ok(items)
  }

  pub fn scan_log_rotations(&self) -> Result<ResponseModel, ResponseModel> {
    self
      .scan_log_rotations_inner()
      .map_err(|e| e.into_response())
  }

  fn scan_log_rotations_inner(&self) -> CleanerResult<Vec<JunkItem>> {
    let mut items = Vec::new();
    let log_dir = Path::new("/var/log");

    if !log_dir.exists() {
      return Ok(Vec::new());
    }

    let mut total_size = 0u64;
    let mut total_count = 0u32;

    if let Ok(entries) = fs::read_dir(log_dir) {
      for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
          if let Some(ext) = path.extension() {
            if ext == "gz" {
              if let Ok(metadata) = fs::metadata(&path) {
                total_size += metadata.len();
                total_count += 1;
              }
            }
          }
          if let Ok(filename) = path.file_name() {
            let name = filename.to_string_lossy();
            if name.ends_with(".old")
              || name.ends_with(".bak")
              || name.ends_with(".1")
              || name.ends_with(".2")
            {
              if let Ok(metadata) = fs::metadata(&path) {
                total_size += metadata.len();
                total_count += 1;
              }
            }
          }
        }
      }
    }

    if total_count > 0 {
      items.push(JunkItem {
        path: "/var/log".to_string(),
        size: total_size,
        category: JunkCategory::Logs,
        description: "Rotated and archived log files".to_string(),
        file_count: total_count,
      });
    }

    Ok(items)
  }

  pub fn clean_junk_category(&self, category: String) -> Result<ResponseModel, ResponseModel> {
    self
      .clean_junk_category_inner(category)
      .map_err(|e| e.into_response())
  }

  fn clean_junk_category_inner(&self, category: String) -> CleanerResult<ResponseModel> {
    let cat = match category.to_lowercase().as_str() {
      "browser" => JunkCategory::Browser,
      "thumbnails" => JunkCategory::Thumbnails,
      "applications" => JunkCategory::Applications,
      "system" => JunkCategory::System,
      "logs" => JunkCategory::Logs,
      _ => {
        return Err(AppError::message(format!("Invalid category: {}", category)));
      }
    };

    match cat {
      JunkCategory::Browser => self.clean_browser_caches(),
      JunkCategory::Thumbnails => self.clean_thumbnail_caches(),
      JunkCategory::Applications => self.clean_application_caches(),
      JunkCategory::System => self.clean_system_temp(),
      JunkCategory::Logs => self.clean_log_rotations(),
    }
  }

  fn clean_browser_caches(&self) -> CleanerResult<ResponseModel> {
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let browser_paths = [
      home.join(".cache/mozilla"),
      home.join(".cache/google-chrome"),
      home.join(".cache/Brave"),
      home.join(".cache/microsoft-edge"),
    ];

    let mut cleaned_count = 0u32;
    let mut errors: Vec<String> = Vec::new();

    for path in browser_paths {
      if path.exists() {
        match Self::remove_dir_contents(&path) {
          Ok(count) => cleaned_count += count,
          Err(e) => errors.push(format!("{}: {}", path.display(), e)),
        }
      }
    }

    if errors.is_empty() {
      Ok(success_response(
        format!("Cleaned {} browser cache directories", cleaned_count),
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

  fn clean_thumbnail_caches(&self) -> CleanerResult<ResponseModel> {
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let path = home.join(".cache/thumbnails");

    if !path.exists() {
      return Ok(success_response(
        "Thumbnail cache already clean",
        data_string("0"),
      ));
    }

    match Self::remove_dir_contents(&path) {
      Ok(count) => Ok(success_response(
        format!("Cleaned thumbnail cache ({} items)", count),
        data_string(count.to_string()),
      )),
      Err(e) => Err(AppError::message(format!(
        "Failed to clean thumbnail cache: {}",
        e
      ))),
    }
  }

  fn clean_application_caches(&self) -> CleanerResult<ResponseModel> {
    let home = dirs::home_dir()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
    let paths = [
      (home.join(".cache/flatpak"), "Flatpak"),
      (home.join(".var/app"), "Flatpak alt"),
      (home.join("snap"), "Snap"),
      (home.join(".cache/snap"), "Snap alt"),
      (home.join(".cache/appimage"), "AppImage"),
    ];

    let mut cleaned_count = 0u32;
    let mut errors: Vec<String> = Vec::new();

    for (path, name) in paths {
      if path.exists() {
        match Self::remove_dir_contents(&path) {
          Ok(count) => {
            cleaned_count += count;
          }
          Err(e) => errors.push(format!("{}: {}", name, e)),
        }
      }
    }

    if errors.is_empty() {
      Ok(success_response(
        format!("Cleaned {} application cache directories", cleaned_count),
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

  fn clean_system_temp(&self) -> CleanerResult<ResponseModel> {
    let temp_paths = ["/tmp", "/var/tmp"];
    let mut cleaned_count = 0u32;
    let mut errors: Vec<String> = Vec::new();

    for path_str in temp_paths {
      let path = Path::new(path_str);
      if path.exists() {
        match Self::remove_dir_contents(path) {
          Ok(count) => cleaned_count += count,
          Err(e) => errors.push(format!("{}: {}", path_str, e)),
        }
      }
    }

    if errors.is_empty() {
      Ok(success_response(
        format!("Cleaned {} temporary directories", cleaned_count),
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

  fn clean_log_rotations(&self) -> CleanerResult<ResponseModel> {
    let log_dir = Path::new("/var/log");
    if !log_dir.exists() {
      return Ok(success_response(
        "Log directory not found",
        data_string("0"),
      ));
    }

    let mut cleaned_count = 0u32;
    let mut errors: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(log_dir) {
      for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
          let should_delete = path.extension().map(|e| e == "gz").unwrap_or(false)
            || path
              .file_name()
              .map(|name| {
                let name = name.to_string_lossy();
                name.ends_with(".old")
                  || name.ends_with(".bak")
                  || name.ends_with(".1")
                  || name.ends_with(".2")
              })
              .unwrap_or(false);

          if should_delete {
            match fs::remove_file(&path) {
              Ok(_) => cleaned_count += 1,
              Err(e) => errors.push(format!("{}: {}", path.display(), e)),
            }
          }
        }
      }
    }

    if errors.is_empty() {
      Ok(success_response(
        format!("Cleaned {} rotated log files", cleaned_count),
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
