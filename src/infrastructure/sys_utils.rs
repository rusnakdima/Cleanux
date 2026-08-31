//! System utilities for Cleanux
//!
//! Provides cross-platform helpers for file scanning, CPU temp, etc.

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Get home directory with cross-platform fallback
pub fn home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
}

/// Get total size of all files in a directory (recursive)
pub async fn get_dir_size(path: &Path) -> Result<u64, String> {
    if !path.exists() {
        return Ok(0);
    }

    let mut size = 0u64;
    let mut stack = vec![path.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let mut entries = match tokio::fs::read_dir(&dir).await {
            Ok(v) => v,
            Err(_) => continue,
        };

        while let Ok(Some(entry)) = entries.next_entry().await {
            let entry_path = entry.path();
            if let Ok(metadata) = entry.metadata().await {
                if metadata.is_file() {
                    size += metadata.len();
                } else if metadata.is_dir() {
                    stack.push(entry_path);
                }
            }
        }
    }

    Ok(size)
}

/// Walk directory, collect (path_string, size) for files under max_depth
pub async fn walkdir_size(
    dir: &Path,
    results: &mut Vec<(String, u64)>,
    max_depth: usize,
) -> Result<(), String> {
    if !dir.exists() {
        return Ok(());
    }

    for entry in WalkDir::new(dir)
        .max_depth(max_depth)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Ok(metadata) = entry.metadata() {
                results.push((path.to_string_lossy().to_string(), metadata.len()));
            }
        }
    }

    Ok(())
}

/// Delete files older than max_age_days in path. Returns bytes freed.
pub async fn delete_old_files(path: &Path, max_age_days: u32) -> Result<u64, String> {
    if !path.exists() {
        return Ok(0);
    }

    let use_cutoff = max_age_days > 0;
    let cutoff = Utc::now() - chrono::Duration::days(max_age_days as i64);

    let mut freed = 0u64;
    let mut stack = vec![path.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let mut entries = match tokio::fs::read_dir(&dir).await {
            Ok(v) => v,
            Err(_) => continue,
        };

        while let Ok(Some(entry)) = entries.next_entry().await {
            let entry_path = entry.path();
            if let Ok(metadata) = entry.metadata().await {
                if metadata.is_dir() {
                    stack.push(entry_path);
                } else if metadata.is_file() {
                    // Check modification time
                    if let Ok(modified) = metadata.modified() {
                        let modified: DateTime<Utc> = modified.into();
                        if !use_cutoff || modified < cutoff {
                            let size = metadata.len();
                            if tokio::fs::remove_file(&entry_path).await.is_ok() {
                                freed += size;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(freed)
}

/// Read CPU temperature from /sys/class/thermal/
pub async fn get_cpu_temp() -> Result<f64, String> {
    let mut temps = Vec::new();

    // Try thermal zones
    for entry in WalkDir::new("/sys/class/thermal")
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path
            .file_name()
            .is_some_and(|n| n.to_string_lossy().starts_with("thermal_zone"))
        {
            let temp_file = path.join("temp");
            if let Ok(content) = tokio::fs::read_to_string(&temp_file).await {
                if let Ok(temp_milli) = content.trim().parse::<u32>() {
                    temps.push(temp_milli as f64 / 1000.0);
                }
            }
        }
    }

    if temps.is_empty() {
        return Ok(f64::NAN);
    }

    Ok(temps.iter().sum::<f64>() / temps.len() as f64)
}

/// Find duplicate files by hashing
pub async fn find_duplicates_size(root: &Path, min_size: u64) -> Result<u64, String> {
    if !root.exists() {
        return Ok(0);
    }

    // Phase 1: collect files by size
    let mut size_groups: std::collections::HashMap<u64, Vec<PathBuf>> =
        std::collections::HashMap::new();

    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Ok(metadata) = entry.metadata() {
                let size = metadata.len();
                if size >= min_size {
                    size_groups
                        .entry(size)
                        .or_default()
                        .push(path.to_path_buf());
                }
            }
        }
    }

    // Phase 2: hash files with same size
    let mut hash_groups: std::collections::HashMap<String, Vec<PathBuf>> =
        std::collections::HashMap::new();

    for (_, paths) in size_groups {
        if paths.len() < 2 {
            continue;
        }

        let mut file_hashes: std::collections::HashMap<String, PathBuf> =
            std::collections::HashMap::new();

        for path in paths {
            if let Ok(content) = tokio::fs::read(&path).await {
                let mut hasher = Sha256::new();
                hasher.update(&content);
                let hash = format!("{:x}", hasher.finalize());
                file_hashes.insert(hash, path);
            }
        }

        // Keep only hashes with duplicates
        for (hash, path) in file_hashes {
            let count = hash_groups.entry(hash.clone()).or_default().len();
            if count == 0 {
                // First file with this hash
                hash_groups.insert(hash, vec![path]);
            } else {
                hash_groups.get_mut(&hash).unwrap().push(path);
            }
        }
    }

    // Count wasted space (size * (count - 1) for each group)
    let mut wasted = 0u64;
    for paths in hash_groups.values() {
        if paths.len() > 1 {
            // Get size from first path
            if let Ok(metadata) = tokio::fs::metadata(&paths[0]).await {
                let size = metadata.len();
                wasted += size * (paths.len() - 1) as u64;
            }
        }
    }

    Ok(wasted)
}

/// Get size of log directory
pub async fn get_log_dir_size() -> Result<u64, String> {
    let log_dir = PathBuf::from("/var/log");
    if !log_dir.exists() {
        return Ok(0);
    }

    let mut size = 0u64;

    for entry in WalkDir::new(&log_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            // Skip rotated logs
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            if name.ends_with(".gz") || name.ends_with(".1") || name.ends_with(".old") {
                continue;
            }
            if let Ok(metadata) = entry.metadata() {
                size += metadata.len();
            }
        }
    }

    Ok(size)
}
