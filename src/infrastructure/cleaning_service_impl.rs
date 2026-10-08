//! CleaningServiceImpl — real implementation of CleaningServiceTrait
//!
//! Scans and cleans junk categories by delegating to the existing handlers
//! in `crate::application::handlers` which use `sysinfo`, `walkdir`, and `tokio::fs`.

use crate::application::cleaning_service::{CleaningServiceTrait, JunkCategory, JunkItem};
use crate::application::handlers::cleaner_handlers;
use crate::domain::CleaningReport;
use chrono::Utc;
use std::time::Instant;

/// Default minimum file size (10 MB) for duplicate/large-file detection.
const DEFAULT_MIN_DUP_SIZE: u64 = 10 * 1024 * 1024;

pub struct CleaningServiceImpl;

impl CleaningServiceImpl {
  pub fn new() -> Self {
    Self
  }
}

impl Default for CleaningServiceImpl {
  fn default() -> Self {
    Self::new()
  }
}

/// Convert a category string to JunkCategory enum.
#[allow(dead_code)]
fn junk_category_from_str(s: &str) -> Option<JunkCategory> {
  match s.to_lowercase().as_str() {
    "cache" => Some(JunkCategory::Cache),
    "trash" => Some(JunkCategory::Trash),
    "logs" => Some(JunkCategory::Logs),
    "largefiles" | "large_files" => Some(JunkCategory::LargeFiles),
    "duplicates" => Some(JunkCategory::Duplicates),
    "systemtemp" | "system_temp" => Some(JunkCategory::SystemTemp),
    "browsercache" => Some(JunkCategory::BrowserCache),
    "appcache" => Some(JunkCategory::AppCache),
    _ => None,
  }
}

/// Convert a handler Value to JunkItem.
fn value_to_junk_item(value: &serde_json::Value, category: JunkCategory) -> Option<JunkItem> {
  let path = value.get("path")?.as_str()?.to_string();
  let size = value.get("size")?.as_u64()?;
  let modified = value
    .get("modified")
    .and_then(|v| v.as_str())
    .unwrap_or("")
    .to_string();

  Some(JunkItem {
    path,
    category,
    size,
    modified,
  })
}

impl CleaningServiceTrait for CleaningServiceImpl {
  /// Scan for junk across all supported categories (cache, trash, logs).
  fn scan_for_junk(&mut self) -> Result<Vec<JunkItem>, String> {
    let rt = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()
      .map_err(|e| format!("failed to build runtime: {}", e))?;

    let mut items = Vec::new();

    // Cache scan
    if let Ok(resp) = rt.block_on(cleaner_handlers::scan_cache()) {
      if let Some(data) = resp.data {
        for value in data.iter() {
          if let Some(item) = value_to_junk_item(value, JunkCategory::Cache) {
            items.push(item);
          }
        }
      }
    }

    // Trash scan
    if let Ok(resp) = rt.block_on(cleaner_handlers::scan_trash()) {
      if let Some(data) = resp.data {
        for value in data.iter() {
          if let Some(item) = value_to_junk_item(value, JunkCategory::Trash) {
            items.push(item);
          }
        }
      }
    }

    // Logs scan
    if let Ok(resp) = rt.block_on(cleaner_handlers::scan_logs()) {
      if let Some(data) = resp.data {
        for value in data.iter() {
          if let Some(item) = value_to_junk_item(value, JunkCategory::Logs) {
            items.push(item);
          }
        }
      }
    }

    Ok(items)
  }

  /// Clean the given junk items and produce a CleaningReport.
  fn clean_junk(&mut self, items: Vec<JunkItem>) -> Result<CleaningReport, String> {
    let rt = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()
      .map_err(|e| format!("failed to build runtime: {}", e))?;

    let start = Instant::now();
    let mut space_reclaimed: u64 = 0;
    let mut cache_count: i64 = 0;
    let mut trash_count: i64 = 0;
    let mut logs_count: i64 = 0;
    let mut large_files_count: i64 = 0;
    let mut duplicates_count: i64 = 0;

    for item in &items {
      let category_str = match item.category {
        JunkCategory::Cache | JunkCategory::BrowserCache | JunkCategory::AppCache => "cache",
        JunkCategory::Trash => "trash",
        JunkCategory::Logs => "logs",
        JunkCategory::LargeFiles => "cache", // reuse cache path logic
        JunkCategory::Duplicates => "cache",
        JunkCategory::SystemTemp => "cache",
      };

      if let Ok(resp) = rt.block_on(cleaner_handlers::clean_junk_category(category_str)) {
        if let Some(freed) = resp.data {
          space_reclaimed += freed;
        }
      }

      // Count per category
      match item.category {
        JunkCategory::Cache | JunkCategory::BrowserCache | JunkCategory::AppCache => {
          cache_count += 1
        }
        JunkCategory::Trash => trash_count += 1,
        JunkCategory::Logs => logs_count += 1,
        JunkCategory::LargeFiles => large_files_count += 1,
        JunkCategory::Duplicates => duplicates_count += 1,
        JunkCategory::SystemTemp => cache_count += 1,
      }
    }

    let duration = start.elapsed().as_secs_f64();

    Ok(CleaningReport {
      id: Some(uuid::Uuid::new_v4().to_string()),
      date: Utc::now().to_rfc3339(),
      items_cleaned: items.len() as i64,
      space_reclaimed,
      duration,
      categories: ReportCategories {
        cache: cache_count,
        trash: trash_count,
        logs: logs_count,
        large_files: large_files_count,
        duplicates: duplicates_count,
      },
    })
  }
}

/// Extended operations beyond the base trait — used by handlers for category-specific scans.
impl CleaningServiceImpl {
  /// Scan for duplicate files.
  pub fn scan_duplicates(&mut self) -> Result<Vec<JunkItem>, String> {
    let rt = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()
      .map_err(|e| format!("failed to build runtime: {}", e))?;

    let mut results: Vec<(String, u64)> = Vec::new();
    let home = crate::infrastructure::sys_utils::home_dir();

    rt.block_on(async {
      crate::infrastructure::sys_utils::walkdir_size(&home, &mut results, 6).await
    })
    .ok();

    let mut items: Vec<JunkItem> = results
      .into_iter()
      .filter(|(_, size)| *size >= DEFAULT_MIN_DUP_SIZE)
      .map(|(path, size)| JunkItem {
        path,
        category: JunkCategory::Duplicates,
        size,
        modified: String::new(),
      })
      .collect();

    items.sort_by_key(|a| std::cmp::Reverse(a.size));
    Ok(items)
  }

  /// Scan for system temp files.
  pub fn scan_system_temp(&mut self) -> Result<Vec<JunkItem>, String> {
    let rt = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()
      .map_err(|e| format!("failed to build runtime: {}", e))?;

    let tmp_paths = vec!["/tmp", "/var/tmp"];
    let mut all_items = Vec::new();

    for tmp in tmp_paths {
      let path = std::path::PathBuf::from(tmp);
      if !path.exists() {
        continue;
      }
      let mut results: Vec<(String, u64)> = Vec::new();
      rt.block_on(async {
        crate::infrastructure::sys_utils::walkdir_size(&path, &mut results, 4).await
      })
      .ok();

      for (path_str, size) in results {
        all_items.push(JunkItem {
          path: path_str,
          category: JunkCategory::SystemTemp,
          size,
          modified: String::new(),
        });
      }
    }

    Ok(all_items)
  }

  /// Get a summary of all junk categories (sizes only, no item listing).
  pub fn get_junk_summary(&mut self) -> Result<JunkSummary, String> {
    let rt = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()
      .map_err(|e| format!("failed to build runtime: {}", e))?;

    let summary = rt.block_on(cleaner_handlers::get_junk_summary())?;
    let data = summary.data.unwrap_or(cleaner_handlers::JunkSummary {
      cache: 0,
      trash: 0,
      logs: 0,
      large_files: 0,
      total: 0,
    });

    Ok(JunkSummary {
      cache: data.cache,
      trash: data.trash,
      logs: data.logs,
      large_files: data.large_files,
      total: data.total,
    })
  }

  /// Clean a specific category by name.
  pub fn clean_junk_category(&mut self, category: &str) -> Result<u64, String> {
    let rt = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()
      .map_err(|e| format!("failed to build runtime: {}", e))?;

    let freed = rt.block_on(cleaner_handlers::clean_junk_category(category))?;
    freed.data.ok_or_else(|| "no data in response".to_string())
  }
}

/// Re-export summary type used by the UI.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct JunkSummary {
  pub cache: u64,
  pub trash: u64,
  pub logs: u64,
  pub large_files: u64,
  pub total: u64,
}

use crate::domain::entities::cleaning_report::ReportCategories;
