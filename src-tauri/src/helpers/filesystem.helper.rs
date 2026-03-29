/* models */
use crate::models::{CacheFileModel, LargeFileModel, LogFileModel, TrashFileModel};
/* sys lib */
use chrono::{DateTime, Local};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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

pub fn collect_cache_file_models(cache_dir: PathBuf) -> Vec<CacheFileModel> {
  WalkDir::new(cache_dir)
    .max_depth(4)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file())
    .take(1000)
    .collect::<Vec<_>>()
    .into_par_iter()
    .filter_map(|entry| {
      let path = entry.path();
      let metadata = fs::metadata(path).ok()?;
      Some(CacheFileModel {
        path: path.to_string_lossy().to_string(),
        size: metadata.len(),
        modified: modified_string(&metadata),
      })
    })
    .collect()
}

pub fn collect_trash_file_models(trash_dir: &Path) -> Vec<TrashFileModel> {
  let mut trash_files = Vec::new();
  if let Ok(entries) = fs::read_dir(trash_dir) {
    for entry in entries.flatten() {
      let path = entry.path();
      if let Ok(metadata) = fs::metadata(&path) {
        let deleted_date: DateTime<Local> = metadata
          .modified()
          .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
          .into();
        trash_files.push(TrashFileModel {
          name: path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
          path: path.to_string_lossy().to_string(),
          size: metadata.len(),
          deletedDate: deleted_date.format("%Y-%m-%d %H:%M:%S").to_string(),
        });
      }
    }
  }
  trash_files
}

pub fn collect_log_file_models(log_dir: &Path, max_depth: usize, take: usize) -> Vec<LogFileModel> {
  WalkDir::new(log_dir)
    .max_depth(max_depth)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file())
    .take(take)
    .collect::<Vec<_>>()
    .into_par_iter()
    .filter_map(|entry| {
      let path = entry.path();
      let metadata = fs::metadata(path).ok()?;
      Some(LogFileModel {
        path: path.to_string_lossy().to_string(),
        size: metadata.len(),
        modified: modified_string(&metadata),
      })
    })
    .collect()
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
  for path in paths {
    if let Err(e) = fs::remove_file(&path) {
      errors.push(format!("{}: {}", path, e));
    } else {
      cleared += 1;
    }
  }
  BulkRemoveOutcome { cleared, errors }
}
