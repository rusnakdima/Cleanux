/* helpers */
use crate::helpers::constants_helper::LARGE_FILE_THRESHOLD_BYTES;
use crate::helpers::validation_helper::{is_allowed_path, validate_path};
/* models */
use crate::models::{AppError, CacheFileModel, LargeFileModel, LogFileModel, TrashFileModel};
/* sys lib */
use chrono::{DateTime, Local};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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

pub fn collect_cache_file_models(
  cache_dir: PathBuf,
  skip: Option<usize>,
  limit: Option<usize>,
) -> (Vec<CacheFileModel>, bool, usize) {
  let all_files: Vec<CacheFileModel> = collect_file_models(cache_dir.as_path(), 4, 10000, |path| {
    let metadata = fs::metadata(path).ok()?;
    Some(CacheFileModel {
      path: path.to_string_lossy().to_string(),
      size: metadata.len(),
      modified: modified_string(&metadata),
    })
  });

  let total = all_files.len();
  let skip_count = skip.unwrap_or(0);
  let limit_count = limit.unwrap_or(usize::MAX);

  let has_more = if skip_count >= total {
    false
  } else {
    skip_count + limit_count < total
  };

  let result: Vec<CacheFileModel> = all_files
    .into_iter()
    .skip(skip_count)
    .take(limit_count)
    .collect();

  (result, has_more, total)
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

/// Scan configured user folders for files above threshold with pagination support.
pub fn scan_large_file_models(
  home: &Path,
  max_depth: usize,
  per_dir_cap: usize,
  sort_truncate: Option<usize>,
  skip: Option<usize>,
  limit: Option<usize>,
) -> (Vec<LargeFileModel>, bool, usize) {
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

  files.sort_by_key(|b| std::cmp::Reverse(b.size));

  let total = files.len();

  if let Some(max) = sort_truncate {
    if files.len() > max {
      files.truncate(max);
    }
  }

  let skip_count = skip.unwrap_or(0);
  let limit_count = limit.unwrap_or(usize::MAX);

  let has_more = if skip_count >= total {
    false
  } else {
    skip_count + limit_count < total
  };

  let result: Vec<LargeFileModel> = files
    .into_iter()
    .skip(skip_count)
    .take(limit_count)
    .collect();

  (result, has_more, total)
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

pub fn calculate_dir_size(path: &Path) -> Result<(u64, u64), AppError> {
  let mut total_size = 0u64;
  let mut file_count = 0u64;

  if !path.exists() {
    return Ok((0, 0));
  }

  for entry in fs::read_dir(path)? {
    let entry = entry?;
    let metadata = entry.metadata()?;
    if entry.path().is_file() {
      total_size += metadata.len();
      file_count += 1;
    } else if entry.path().is_dir() {
      let (size, count) = calculate_dir_size(&entry.path())?;
      total_size += size;
      file_count += count;
    }
  }

  Ok((total_size, file_count))
}

pub fn remove_dir_contents(path: &Path) -> Result<u64, AppError> {
  if !path.exists() {
    return Ok(0);
  }

  let mut count = 0u64;
  for entry in fs::read_dir(path)? {
    let entry = entry?;
    let entry_path = entry.path();
    if entry_path.is_dir() {
      fs::remove_dir_all(&entry_path)?;
      count += 1;
    } else if entry_path.is_file() {
      fs::remove_file(&entry_path)?;
      count += 1;
    }
  }
  Ok(count)
}

pub fn get_dir_size(path: &Path) -> u64 {
  calculate_dir_size(path).map(|(size, _)| size).unwrap_or(0)
}

pub fn format_size(bytes: u64) -> String {
  const KB: u64 = 1024;
  const MB: u64 = KB * 1024;
  const GB: u64 = MB * 1024;

  if bytes >= GB {
    format!("{:.2} GB", bytes as f64 / GB as f64)
  } else if bytes >= MB {
    format!("{:.2} MB", bytes as f64 / MB as f64)
  } else if bytes >= KB {
    format!("{:.2} KB", bytes as f64 / KB as f64)
  } else {
    format!("{} B", bytes)
  }
}

use crate::helpers::response_helper::data_string;
use crate::helpers::response_helper::success_response;
use crate::models::ResponseModel;

pub fn clean_cache_dir(path: &Path, name: &str) -> Result<ResponseModel, AppError> {
  if !path.exists() {
    return Ok(success_response(
      format!("{} cache already clean", name),
      data_string("0"),
    ));
  }

  match remove_dir_contents(path) {
    Ok(count) => Ok(success_response(
      format!("Cleaned {} cache ({} items)", name, count),
      data_string(count.to_string()),
    )),
    Err(e) => Err(AppError::message(format!(
      "Failed to clean {} cache: {}",
      name, e
    ))),
  }
}
