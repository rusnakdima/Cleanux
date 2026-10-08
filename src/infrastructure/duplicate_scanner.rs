//! DuplicateScannerService — finds duplicate files by SHA-256 hash.
//!
//! Scans a directory tree, computes SHA-256 for regular files,
//! and groups files by hash to identify duplicates.

use crate::error::AppError;

pub type Result<T> = std::result::Result<T, AppError>;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// A group of duplicate files with the same content hash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
  /// Short SHA-256 hash (first 12 hex chars).
  pub hash: String,
  /// Total size of files in this group.
  pub total_size: u64,
  /// Number of duplicate files.
  pub count: usize,
  /// List of file paths in this group.
  pub files: Vec<String>,
  /// Size of a single file (all are equal).
  pub file_size: u64,
  /// Space wasted (total_size - file_size).
  pub wasted: u64,
}

impl DuplicateGroup {
  pub fn new(hash: String, first: String, size: u64) -> Self {
    Self {
      hash,
      total_size: size,
      count: 1,
      files: vec![first],
      file_size: size,
      wasted: 0,
    }
  }

  pub fn add_file(&mut self, path: String) {
    self.files.push(path);
    self.count += 1;
    self.total_size += self.file_size;
    self.wasted = self.total_size - self.file_size;
  }
}

/// Summary of duplicate scan results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateScanResult {
  pub groups: Vec<DuplicateGroup>,
  pub total_groups: usize,
  pub total_wasted: u64,
  pub total_duplicates: usize,
  pub files_scanned: usize,
}

impl DuplicateScanResult {
  pub fn new() -> Self {
    Self {
      groups: Vec::new(),
      total_groups: 0,
      total_wasted: 0,
      total_duplicates: 0,
      files_scanned: 0,
    }
  }
}

/// Compute SHA-256 hash of a file.
fn compute_file_hash(path: &Path) -> Result<(String, u64)> {
  let file = File::open(path)
    .map_err(|e| AppError::Internal(format!("failed to open {}: {}", path.display(), e)))?;
  let file_size = file
    .metadata()
    .map_err(|e| AppError::Internal(format!("failed to metadata {}: {}", path.display(), e)))?
    .len();
  let mut reader = BufReader::with_capacity(8192, file);
  let mut hasher = Sha256::new();
  let mut buffer = [0u8; 8192];

  loop {
    let bytes_read = reader
      .read(&mut buffer)
      .map_err(|e| AppError::Internal(format!("failed to read {}: {}", path.display(), e)))?;
    if bytes_read == 0 {
      break;
    }
    hasher.update(&buffer[..bytes_read]);
  }

  let hash = hex::encode(hasher.finalize());
  Ok((hash, file_size))
}

/// Recursively scan directory for regular files.
fn scan_files_recursive(dir: &Path) -> Result<Vec<(String, u64)>> {
  let mut files = Vec::new();
  if !dir.exists() {
    return Ok(files);
  }

  let entries = std::fs::read_dir(dir)
    .map_err(|e| AppError::Internal(format!("failed to read dir {}: {}", dir.display(), e)))?;

  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      // Skip /proc, /sys, /dev, /run to avoid issues
      let skip_dirs = ["/proc", "/sys", "/dev", "/run", "/snap", "/mnt"];
      if let Some(name) = path.to_str() {
        if skip_dirs.iter().any(|&s| name.starts_with(s)) {
          continue;
        }
      }
      files.extend(scan_files_recursive(&path)?);
    } else if path.is_file() {
      if let Ok(meta) = path.metadata() {
        if meta.len() > 0 {
          if let Some(path_str) = path.to_str() {
            files.push((path_str.to_string(), meta.len()));
          }
        }
      }
    }
  }

  Ok(files)
}

/// Find duplicate files in a directory tree using SHA-256 hashing.
pub async fn find_duplicates(path: &str) -> Result<DuplicateScanResult> {
  let path = Path::new(path).to_path_buf();

  // First pass: collect all files
  let files: Vec<(String, u64)> = tokio::task::spawn_blocking({
    let path = path.clone();
    move || scan_files_recursive(&path)
  })
  .await
  .map_err(|e| AppError::Internal(format!("task join error: {}", e)))??;

  let files_scanned = files.len();
  if files_scanned == 0 {
    return Ok(DuplicateScanResult::new());
  }

  // Group by file size first (optimization: only hash files with same size)
  let mut size_groups: HashMap<u64, Vec<String>> = HashMap::new();
  for (path_str, size) in &files {
    size_groups.entry(*size).or_default().push(path_str.clone());
  }

  // Filter to only sizes with potential duplicates (>1 file of same size)
  let potential_dupes: Vec<(String, u64)> = size_groups
    .into_iter()
    .filter(|(_, v)| v.len() > 1)
    .flat_map(|(_, paths)| {
      paths
        .into_iter()
        .map(|p| {
          let size = files
            .iter()
            .find(|(sp, _)| sp == &p)
            .map(|(_, s)| *s)
            .unwrap_or(0);
          (p, size)
        })
        .collect::<Vec<_>>()
    })
    .collect();

  // Compute hashes for potential duplicates
  let mut hash_map: HashMap<String, DuplicateGroup> = HashMap::new();

  for (path_str, size) in potential_dupes {
    let path = Path::new(&path_str).to_path_buf();
    let hash_result = tokio::task::spawn_blocking(move || compute_file_hash(&path)).await;
    match hash_result {
      Ok(Ok((hash, _))) => {
        let entry = hash_map
          .entry(hash.clone())
          .or_insert_with(|| DuplicateGroup::new(hash.clone(), path_str.clone(), size));
        if entry.count == 1 && entry.files[0] != path_str {
          entry.add_file(path_str);
        } else if entry.count > 1 {
          entry.add_file(path_str);
        }
      }
      Ok(Err(e)) => {
        // Skip files we can't hash
        tracing::warn!("skipping {}: {}", path_str, e);
      }
      Err(e) => {
        tracing::warn!("task error hashing {}: {}", path_str, e);
      }
    }
  }

  // Build result
  let mut result = DuplicateScanResult::new();
  result.files_scanned = files_scanned;

  let actual_groups: Vec<DuplicateGroup> = hash_map.into_values().filter(|g| g.count > 1).collect();

  for group in actual_groups {
    result.total_groups += 1;
    result.total_duplicates += group.count - 1;
    result.total_wasted += group.wasted;
    result.groups.push(group);
  }

  // Sort by wasted space descending
  result.groups.sort_by(|a, b| b.wasted.cmp(&a.wasted));

  Ok(result)
}

/// Synchronous wrapper for bridge invocation.
pub fn find_duplicates_sync(path: &str) -> Result<DuplicateScanResult> {
  let rt = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .map_err(|e| AppError::Internal(format!("failed to build runtime: {}", e)))?;
  rt.block_on(find_duplicates(path))
}
