/* helpers */
use crate::utils::{data_string, service_method_full, success_response};
/* models */
use crate::models::{AppError, Response};
/* services::junk */
use crate::services::junk::{
  app_cache_scanner::ApplicationCacheScanner, browser_scanner::BrowserCacheScanner,
  log_rotation_scanner::LogRotationScanner, system_temp_scanner::SystemTempScanner,
  thumbnail_scanner::ThumbnailCacheScanner, JunkCategory, JunkItem,
};
/* sys lib */
use serde_json::Value;
type CleanerResult<T> = Result<T, AppError>;
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
    let browser = BrowserCacheScanner::scan()?;
    let thumbnails = ThumbnailCacheScanner::scan()?;
    let applications = ApplicationCacheScanner::scan()?;
    let system = SystemTempScanner::scan()?;
    let logs = LogRotationScanner::scan()?;
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
    match BrowserCacheScanner::scan() {
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
  pub fn scan_thumbnail_caches(&self) -> Result<Response<Value>, Response<Value>> {
    match ThumbnailCacheScanner::scan() {
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
  pub fn scan_application_caches(&self) -> Result<Response<Value>, Response<Value>> {
    match ApplicationCacheScanner::scan() {
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
  pub fn scan_system_temp(&self) -> Result<Response<Value>, Response<Value>> {
    match SystemTempScanner::scan() {
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
  pub fn scan_log_rotations(&self) -> Result<Response<Value>, Response<Value>> {
    match LogRotationScanner::scan() {
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
  fn clean_browser_caches(&self) -> CleanerResult<Response<Value>> {
    match BrowserCacheScanner::clean() {
      Ok(count) => Ok(success_response(
        format!("Cleaned {} browser cache directories", count),
        data_string(count.to_string()),
      )),
      Err(e) => Err(AppError::message(format!(
        "Failed to clean browser caches: {}",
        e
      ))),
    }
  }
  fn clean_thumbnail_caches(&self) -> CleanerResult<Response<Value>> {
    match ThumbnailCacheScanner::clean() {
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
    match ApplicationCacheScanner::clean() {
      Ok(count) => Ok(success_response(
        format!("Cleaned {} application cache directories", count),
        data_string(count.to_string()),
      )),
      Err(e) => Err(AppError::message(format!(
        "Failed to clean application caches: {}",
        e
      ))),
    }
  }
  fn clean_system_temp(&self) -> CleanerResult<Response<Value>> {
    match SystemTempScanner::clean() {
      Ok(count) => Ok(success_response(
        format!("Cleaned {} temporary directories", count),
        data_string(count.to_string()),
      )),
      Err(e) => Err(AppError::message(format!(
        "Failed to clean system temp: {}",
        e
      ))),
    }
  }
  fn clean_log_rotations(&self) -> CleanerResult<Response<Value>> {
    match LogRotationScanner::clean() {
      Ok(count) => Ok(success_response(
        format!("Cleaned {} rotated log files", count),
        data_string(count.to_string()),
      )),
      Err(e) => Err(AppError::message(format!(
        "Failed to clean log rotations: {}",
        e
      ))),
    }
  }
}
