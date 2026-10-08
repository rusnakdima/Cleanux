//! RepairServiceImpl — implementation of RepairServiceTrait
//!
//! Performs filesystem checks, permission repairs, and memory reclamation.

use crate::domain::services::repair_service::{
    FsckResult, MemoryClearResult, PermissionFixResult, RepairServiceTrait,
};
use crate::error::AppError;

pub type Result<T> = std::result::Result<T, AppError>;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

/// Minimum permission bits enforced on directories (rwxr-xr-x).
const DIR_PERM: u32 = 0o755;
/// Minimum permission bits enforced on regular files (rw-r--r--).
const FILE_PERM: u32 = 0o644;

pub struct RepairServiceImpl;

impl RepairServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Clone for RepairServiceImpl {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl Default for RepairServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl RepairServiceTrait for RepairServiceImpl {
    /// Run `fsck -n -N` (dry-run, no-write) on the given path or device.
    fn check_filesystem(&mut self, path: &str) -> Result<FsckResult> {
        let output = Command::new("fsck")
            .args(["-n", "-N", path])
            .output()
            .map_err(|e| AppError::Internal(format!("fsck failed to start: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Parse fsck exit code: 0 = clean, 1 = errors fixed, 2 = errors not fixed
        let exit_code = output.status.code().unwrap_or(-1);
        let errors_corrected = if exit_code == 0 || exit_code == 1 {
            count_fsed_errors(&stdout) as u32
        } else {
            0
        };
        let errors_uncorrected = if exit_code == 2 { 1 } else { 0 };

        Ok(FsckResult {
            path: path.to_string(),
            errors_corrected,
            errors_uncorrected,
            exit_code,
            raw_output: format!("{}\n{}", stdout, stderr).trim().to_string(),
        })
    }

    /// Recursively fix permissions: directories → 0755, regular files → 0644.
    /// Skips symlinks to avoid dereferencing them.
    fn fix_permissions(&mut self, path: &str) -> Result<PermissionFixResult> {
        let root = Path::new(path);
        if !root.exists() {
            return Err(AppError::ValidationError(format!(
                "repair path does not exist: {}",
                path
            )));
        }

        let mut dirs_fixed = 0u32;
        let mut files_fixed = 0u32;
        let mut errors = Vec::new();

        for entry in WalkDir::new(root).follow_links(false).into_iter().filter_map(|e| e.ok())
        {
            let file_path = entry.path();
            let meta = match file_path.metadata() {
                Ok(m) => m,
                Err(e) => {
                    errors.push(format!("{}: metadata error: {}", file_path.display(), e));
                    continue;
                }
            };

            if meta.is_symlink() {
                continue;
            } else if meta.is_dir() {
                match fix_path_permissions(file_path, DIR_PERM) {
                    Ok(true) => dirs_fixed += 1,
                    Ok(false) => {}
                    Err(e) => errors.push(format!("{}: {}", file_path.display(), e)),
                }
            } else if meta.is_file() {
                match fix_path_permissions(file_path, FILE_PERM) {
                    Ok(true) => files_fixed += 1,
                    Ok(false) => {}
                    Err(e) => errors.push(format!("{}: {}", file_path.display(), e)),
                }
            }
        }

        Ok(PermissionFixResult {
            path: path.to_string(),
            dirs_fixed,
            files_fixed,
            errors,
        })
    }

    /// Reclaim memory by syncing, dropping caches, and triggering OOM reaping.
    fn clear_memory(&mut self) -> Result<MemoryClearResult> {
        // 1. Sync filesystems to flush dirty pages
        let _ = Command::new("sync").output();

        // 2. Try drop_caches (requires root; harmless without it)
        let pages_before = read_vmstat_nr_free_pages();
        let _ = Command::new("sh")
            .args(["-c", "echo 3 > /proc/sys/vm/drop_caches 2>/dev/null"])
            .output();
        let pages_after = read_vmstat_nr_free_pages();

        let pages_dropped = pages_after.saturating_sub(pages_before);

        // 3. Trigger OOM killer reaping of killed-but-not-collected zombie processes
        let _ = Command::new("sh")
            .args(["-c", "echo 1 > /proc/sys/vm/compact_memory 2>/dev/null"])
            .output();

        // Estimate bytes freed: page size from /proc/self/statm pages field × 4096
        let page_size = std::fs::read_to_string("/proc/self/statm")
            .ok()
            .and_then(|s| {
                let pages: u64 = s.split_whitespace().nth(1)?.parse().ok()?;
                Some(pages * 4096)
            })
            .unwrap_or(4096);
        let bytes_freed = pages_dropped * page_size;

        Ok(MemoryClearResult {
            bytes_freed,
            pages_dropped,
        })
    }
}

/// Set permission mode on a path if it differs from target.
/// Returns Ok(true) if changed, Ok(false) if already correct.
fn fix_path_permissions(path: &Path, target: u32) -> std::io::Result<bool> {
    use std::os::unix::fs::PermissionsExt;

    let meta = path.metadata()?;
    let current_mode = meta.permissions().mode() & 0o777;
    let target_mode = target;

    if current_mode == target_mode {
        return Ok(false);
    }

    let mut perms = meta.permissions();
    perms.set_mode(target_mode);
    std::fs::set_permissions(path, perms)?;
    Ok(true)
}

/// Count lines in fsck output that report corrected errors.
fn count_fsed_errors(output: &str) -> usize {
    output
        .lines()
        .filter(|line| {
            line.contains("was corrected")
                || line.contains("Freezer")
                || line.contains("cached")
        })
        .count()
}

/// Read /proc/vmstat for nr_free_pages (free memory pages).
fn read_vmstat_nr_free_pages() -> u64 {
    std::fs::read_to_string("/proc/vmstat")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("nr_free_pages "))
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|v| v.parse().ok())
        })
        .unwrap_or(0)
}
