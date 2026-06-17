/* helpers */
use crate::utils::common_paths::CommonPath;
use crate::utils::{
  calculate_dir_size, data_string, remove_dir_contents, service_method_full, success_response,
};
/* models */
use crate::models::{AppError, Response};
/* sys lib */
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

type CleanerResult<T> = Result<T, AppError>;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum JunkCategory {
  Browser,
  Thumbnails,
  Applications,
  System,
  Logs,
}

#[derive(Debug, Clone, serde::Serialize)]
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
  service_method_full!(get_junk_summary => get_junk_summary_inner);

  fn get_junk_summary_inner(&self) -> CleanerResult<Response<Value>> {
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
              "file_count": item.file_count,
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
          "total_size": browser_size,
          "file_count": browser_count,
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
          "total_size": thumbnails_size,
          "file_count": thumbnails_count,
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
          "total_size": applications_size,
          "file_count": applications_count,
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
          "total_size": system_size,
          "file_count": system_count,
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
          "total_size": logs_size,
          "file_count": logs_count,
          "description": "Rotated and old log files",
          "items": junk_items_to_json(&logs),
      }),
    );

    Ok(success_response(
      "Junk summary retrieved successfully",
      Value::Object(summary),
    ))
  }

  pub fn scan_browser_caches(&self) -> Result<Response<Value>, Response<Value>> {
    match self.scan_browser_caches_inner() {
      Ok(items) => {
        let size: u64 = items.iter().map(|i| i.size).sum();
        let count: u32 = items.iter().map(|i| i.file_count).sum();
        Ok(success_response(
          format!("Found {} browser cache items ({} bytes)", items.len(), size),
          serde_json::json!({
            "category": "Browser",
            "total_size": size,
            "file_count": count,
            "description": "Browser cache files (Firefox, Chrome, Brave, Edge)",
            "items": items,
          }),
        ))
      }
      Err(e) => Err(e.into_response()),
    }
  }

  fn scan_browser_caches_inner(&self) -> CleanerResult<Vec<JunkItem>> {
    let mut items = Vec::new();

    let browser_paths = [
      (CommonPath::MozillaCache, "Firefox", JunkCategory::Browser),
      (CommonPath::ChromeCache, "Chrome", JunkCategory::Browser),
      (CommonPath::BraveCache, "Brave", JunkCategory::Browser),
      (CommonPath::EdgeCache, "Edge", JunkCategory::Browser),
    ];

    for (common_path, name, category) in browser_paths {
      let path = common_path
        .path()
        .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;
      if path.exists() {
        let (size, file_count) = calculate_dir_size(&path).unwrap_or((0, 0));
        items.push(JunkItem {
          path: path.to_string_lossy().into_owned(),
          size,
          category,
          description: format!("{} browser cache", name),
          file_count: file_count as u32,
        });
      }
    }

    Ok(items)
  }

  pub fn scan_thumbnail_caches(&self) -> Result<Response<Value>, Response<Value>> {
    match self.scan_thumbnail_caches_inner() {
      Ok(items) => {
        let size: u64 = items.iter().map(|i| i.size).sum();
        let count: u32 = items.iter().map(|i| i.file_count).sum();
        Ok(success_response(
          format!(
            "Found {} thumbnail cache items ({} bytes)",
            items.len(),
            size
          ),
          serde_json::json!({
            "category": "Thumbnails",
            "total_size": size,
            "file_count": count,
            "description": "Image thumbnail cache",
            "items": items,
          }),
        ))
      }
      Err(e) => Err(e.into_response()),
    }
  }

  fn scan_thumbnail_caches_inner(&self) -> CleanerResult<Vec<JunkItem>> {
    let path = CommonPath::Thumbnails
      .path()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;

    if !path.exists() {
      return Ok(Vec::new());
    }

    let (size, file_count) = calculate_dir_size(&path).unwrap_or((0, 0));
    Ok(vec![JunkItem {
      path: path.to_string_lossy().into_owned(),
      size,
      category: JunkCategory::Thumbnails,
      description: "Image thumbnail cache".to_string(),
      file_count: file_count as u32,
    }])
  }

  pub fn scan_application_caches(&self) -> Result<Response<Value>, Response<Value>> {
    match self.scan_application_caches_inner() {
      Ok(items) => {
        let size: u64 = items.iter().map(|i| i.size).sum();
        let count: u32 = items.iter().map(|i| i.file_count).sum();
        Ok(success_response(
          format!(
            "Found {} application cache items ({} bytes)",
            items.len(),
            size
          ),
          serde_json::json!({
            "category": "Applications",
            "total_size": size,
            "file_count": count,
            "description": "Application caches (Flatpak, Snap, AppImage)",
            "items": items,
          }),
        ))
      }
      Err(e) => Err(e.into_response()),
    }
  }

  fn scan_application_caches_inner(&self) -> CleanerResult<Vec<JunkItem>> {
    let mut items = Vec::new();

    let app_cache_paths = [
      (
        CommonPath::FlatpakCache,
        "Flatpak",
        JunkCategory::Applications,
      ),
      (
        CommonPath::FlatpakAlt,
        "Flatpak (alt)",
        JunkCategory::Applications,
      ),
      (CommonPath::SnapCache, "Snap", JunkCategory::Applications),
      (
        CommonPath::SnapAlt,
        "Snap (alt)",
        JunkCategory::Applications,
      ),
    ];

    for (common_path, name, category) in app_cache_paths {
      let path = match common_path.path() {
        Some(p) => p,
        None => continue,
      };
      if path.exists() {
        let (size, file_count) = calculate_dir_size(&path).unwrap_or((0, 0));
        items.push(JunkItem {
          path: path.to_string_lossy().into_owned(),
          size,
          category,
          description: format!("{} application cache", name),
          file_count: file_count as u32,
        });
      }
    }

    if let Some(app_image_cache) = CommonPath::AppImageCache.path() {
      if app_image_cache.exists() {
        let (size, file_count) = calculate_dir_size(&app_image_cache).unwrap_or((0, 0));
        items.push(JunkItem {
          path: app_image_cache.to_string_lossy().into_owned(),
          size,
          category: JunkCategory::Applications,
          description: "AppImage cache".to_string(),
          file_count: file_count as u32,
        });
      }
    }

    Ok(items)
  }

  pub fn scan_system_temp(&self) -> Result<Response<Value>, Response<Value>> {
    match self.scan_system_temp_inner() {
      Ok(items) => {
        let size: u64 = items.iter().map(|i| i.size).sum();
        let count: u32 = items.iter().map(|i| i.file_count).sum();
        Ok(success_response(
          format!("Found {} system temp items ({} bytes)", items.len(), size),
          serde_json::json!({
            "category": "System",
            "total_size": size,
            "file_count": count,
            "description": "System temporary files (/tmp, /var/tmp)",
            "items": items,
          }),
        ))
      }
      Err(e) => Err(e.into_response()),
    }
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
        let (size, file_count) = calculate_dir_size(path).unwrap_or((0, 0));
        items.push(JunkItem {
          path: path_str.to_string(),
          size,
          category: JunkCategory::System,
          description: description.to_string(),
          file_count: file_count as u32,
        });
      }
    }

    Ok(items)
  }

  pub fn scan_log_rotations(&self) -> Result<Response<Value>, Response<Value>> {
    match self.scan_log_rotations_inner() {
      Ok(items) => {
        let size: u64 = items.iter().map(|i| i.size).sum();
        let count: u32 = items.iter().map(|i| i.file_count).sum();
        Ok(success_response(
          format!("Found {} log rotation items ({} bytes)", items.len(), size),
          serde_json::json!({
            "category": "Logs",
            "total_size": size,
            "file_count": count,
            "description": "Rotated and old log files",
            "items": items,
          }),
        ))
      }
      Err(e) => Err(e.into_response()),
    }
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
          if let Some(filename) = path.file_name() {
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

  pub fn clean_junk_category(&self, category: String) -> Result<Response<Value>, Response<Value>> {
    self
      .clean_junk_category_inner(category)
      .map_err(|e| e.into_response())
  }

  fn clean_junk_category_inner(&self, category: String) -> CleanerResult<Response<Value>> {
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

  fn clean_paths(
    &self,
    paths: &[(PathBuf, &str)],
    description: &str,
  ) -> CleanerResult<Response<Value>> {
    let mut cleaned_count = 0u32;
    let mut errors: Vec<String> = Vec::new();

    for (path, name) in paths {
      if path.exists() {
        match remove_dir_contents(path) {
          Ok(count) => cleaned_count += count as u32,
          Err(e) => errors.push(format!("{}: {}", name, e)),
        }
      }
    }

    if errors.is_empty() {
      Ok(success_response(
        format!("Cleaned {} {}", cleaned_count, description),
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

  fn clean_browser_caches(&self) -> CleanerResult<Response<Value>> {
    let browser_paths = [
      CommonPath::MozillaCache,
      CommonPath::ChromeCache,
      CommonPath::BraveCache,
      CommonPath::EdgeCache,
    ];

    let mut cleaned_count = 0u32;
    let mut errors: Vec<String> = Vec::new();

    for common_path in browser_paths {
      if let Some(path) = common_path.path() {
        if path.exists() {
          match remove_dir_contents(&path) {
            Ok(count) => cleaned_count += count as u32,
            Err(e) => errors.push(format!("{}: {}", path.display(), e)),
          }
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

  fn clean_thumbnail_caches(&self) -> CleanerResult<Response<Value>> {
    let path = CommonPath::Thumbnails
      .path()
      .ok_or_else(|| AppError::InvalidPath("Home directory not found".to_string()))?;

    if !path.exists() {
      return Ok(success_response(
        "Thumbnail cache already clean",
        data_string("0"),
      ));
    }

    match remove_dir_contents(&path) {
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

  fn clean_application_caches(&self) -> CleanerResult<Response<Value>> {
    let paths = [
      (CommonPath::FlatpakCache, "Flatpak"),
      (CommonPath::FlatpakAlt, "Flatpak alt"),
      (CommonPath::SnapCache, "Snap"),
      (CommonPath::SnapAlt, "Snap alt"),
      (CommonPath::AppImageCache, "AppImage"),
    ];

    let mut cleaned_count = 0u32;
    let mut errors: Vec<String> = Vec::new();

    for (common_path, name) in paths {
      if let Some(path) = common_path.path() {
        if path.exists() {
          match remove_dir_contents(&path) {
            Ok(count) => {
              cleaned_count += count as u32;
            }
            Err(e) => errors.push(format!("{}: {}", name, e)),
          }
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

  fn clean_system_temp(&self) -> CleanerResult<Response<Value>> {
    let temp_paths: Vec<(PathBuf, &str)> = ["/tmp", "/var/tmp"]
      .iter()
      .map(|p| (PathBuf::from(p), *p))
      .collect();

    self.clean_paths(&temp_paths, "temporary directories")
  }

  fn clean_log_rotations(&self) -> CleanerResult<Response<Value>> {
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
}
