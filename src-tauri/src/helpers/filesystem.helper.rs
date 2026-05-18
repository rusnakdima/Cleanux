/* models */
use crate::models::{CacheFileModel, LargeFileModel, LogFileModel, TrashFileModel};
/* sys lib */
use chrono::{DateTime, Local};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::helpers::validation_helper::{is_allowed_path, validate_path};

pub const LARGE_FILE_THRESHOLD_BYTES: u64 = 100 * 1024 * 1024;

pub fn home_scan_dirs(home: &Path) -> Vec<PathBuf> {
  vec![
    home.join("Downloads"),
    home.join("Documents"),
    home.join("Videos"),
    home.join("Pictures"),
    home.join("Desktop"),
  ]
}

pub fn modified_string(metadata: &std::fs::Metadata) -> String {
  let modified: DateTime<Local> = metadata
    .modified()
    .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    .into();
  modified.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn collect_file_models<T, F>(
  root: &Path,
  max_depth: u32,
  take_count: usize,
  filter: F,
) -> Vec<T>
where
  F: Fn(&Path) -> Option<T> + Send + Sync + 'static,
  T: Send,
{
  WalkDir::new(root)
    .max_depth(max_depth as usize)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file())
    .take(take_count)
    .collect::<Vec<_>>()
    .into_par_iter()
    .filter_map(|entry| filter(entry.path()))
    .collect()
}

pub fn collect_cache_file_models(cache_dir: PathBuf) -> Vec<CacheFileModel> {
  collect_file_models(cache_dir.as_path(), 4, 1000, |path| {
    let metadata = fs::metadata(path).ok()?;
    Some(CacheFileModel {
      path: path.to_string_lossy().to_string(),
      size: metadata.len(),
      modified: modified_string(&metadata),
    })
  })
}

pub fn collect_trash_file_models(trash_dir: &Path) -> Vec<TrashFileModel> {
  collect_file_models(trash_dir, 1, 10000, |path| {
    let metadata = fs::metadata(path).ok()?;
    let deleted_date: DateTime<Local> = metadata
      .modified()
      .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
      .into();
    Some(TrashFileModel {
      name: path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string(),
      path: path.to_string_lossy().to_string(),
      size: metadata.len(),
      deletedDate: deleted_date.format("%Y-%m-%d %H:%M:%S").to_string(),
    })
  })
}

pub fn collect_log_file_models(log_dir: &Path, max_depth: usize, take: usize) -> Vec<LogFileModel> {
  collect_file_models(log_dir, max_depth as u32, take, |path| {
    let metadata = fs::metadata(path).ok()?;
    Some(LogFileModel {
      path: path.to_string_lossy().to_string(),
      size: metadata.len(),
      modified: modified_string(&metadata),
    })
  })
}

/// Scan configured user folders for files above threshold (same rules as original getLargeFiles).
pub fn scan_large_file_models(
  home: &Path,
  max_depth: usize,
  per_dir_cap: usize,
  sort_truncate: Option<usize>,
) -> Vec<LargeFileModel> {
  let dirs = home_scan_dirs(home);
  let mut files: Vec<LargeFileModel> = dirs
    .into_par_iter()
    .map(|dir| {
      if !dir.exists() {
        return Vec::new();
      }
      WalkDir::new(dir)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|entry| {
          let metadata = entry.metadata().ok()?;
          if metadata.len() > LARGE_FILE_THRESHOLD_BYTES {
            Some(LargeFileModel {
              name: entry.file_name().to_string_lossy().to_string(),
              path: entry.path().to_string_lossy().to_string(),
              size: metadata.len(),
              modified: modified_string(&metadata),
            })
          } else {
            None
          }
        })
        .take(per_dir_cap)
        .collect::<Vec<_>>()
    })
    .flatten()
    .collect();

  files.sort_by(|a, b| b.size.cmp(&a.size));
  if let Some(max) = sort_truncate {
    if files.len() > max {
      files.truncate(max);
    }
  }
  files
}

pub struct BulkRemoveOutcome {
  pub cleared: usize,
  pub errors: Vec<String>,
}

pub fn remove_paths_with_errors(paths: Vec<String>) -> BulkRemoveOutcome {
  let mut cleared = 0usize;
  let mut errors = Vec::new();

  let home = match dirs::home_dir() {
    Some(h) => h,
    None => {
      return BulkRemoveOutcome {
        cleared: 0,
        errors: vec!["Home directory not found".to_string()],
      };
    }
  };

  for path in paths {
    let canonical = match validate_path(&path) {
      Ok(p) => p,
      Err(e) => {
        errors.push(format!("{}: {}", path, e));
        continue;
      }
    };

    if !is_allowed_path(&canonical, &home) {
      errors.push(format!("Path not within allowed directories: {}", path));
      continue;
    }

    if let Err(e) = fs::remove_file(&canonical) {
      errors.push(format!("{}: {}", path, e));
    } else {
      cleared += 1;
    }
  }
  BulkRemoveOutcome { cleared, errors }
}
