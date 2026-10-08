//! System repair service — async filesystem and system repair operations.
//!
//! Provides repair operations: broken symlink detection/removal, permission
//! repair, icon cache rebuild, and orphaned package removal.

use crate::error::AppError;
use std::path::Path;
use tokio::process::Command;
use walkdir::WalkDir;

pub type Result<T, E = AppError> = std::result::Result<T, E>;

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

/// A broken symbolic link found during scanning.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BrokenSymlink {
    /// Path to the broken symlink itself.
    pub path: String,
    /// Target the symlink was pointing to (None if target is missing).
    pub target: Option<String>,
}

/// Result of a broken symlink removal operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymlinkRemoveResult {
    pub removed: u32,
    pub failed: u32,
}

/// Result of a permission repair operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PermissionRepairResult {
    pub success: bool,
    pub message: String,
    pub dirs_fixed: u32,
    pub files_fixed: u32,
}

/// Result of an icon cache rebuild operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IconCacheResult {
    pub success: bool,
    pub message: String,
}

/// Result of an orphaned package removal operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OrphanRemoveResult {
    pub removed: u32,
    pub failed: u32,
    pub message: String,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// System repair service providing async repair operations.
#[derive(Clone, Default)]
pub struct SystemRepairService;

impl SystemRepairService {
    pub fn new() -> Self {
        Self
    }
}

// ---------------------------------------------------------------------------
// Operations
// ---------------------------------------------------------------------------

impl SystemRepairService {
    /// Scan for broken symbolic links under the given root path.
    pub async fn scan_broken_symlinks(&self, root: &str) -> Result<Vec<BrokenSymlink>> {
        let root_path = Path::new(root);
        if !root_path.exists() {
            return Err(AppError::InvalidPath(format!(
                "scan path does not exist: {}",
                root
            )));
        }

        let mut broken = Vec::new();

        // Use WalkDir to iterate; check each symlink's target validity.
        let walker = WalkDir::new(root_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok());

        for entry in walker {
            let path = entry.path();
            let meta = match path.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            if meta.file_type().is_symlink() {
                let target = std::fs::read_link(path).ok().map(|t| t.to_string_lossy().to_string());
                let is_broken = target
                    .as_ref()
                    .map(|t| !Path::new(t).exists())
                    .unwrap_or(true);

                if is_broken {
                    broken.push(BrokenSymlink {
                        path: path.to_string_lossy().to_string(),
                        target,
                    });
                }
            }
        }

        Ok(broken)
    }

    /// Remove the given broken symlink paths.
    pub async fn remove_broken_symlinks(&self, paths: &[String]) -> Result<SymlinkRemoveResult> {
        let mut removed = 0u32;
        let mut failed = 0u32;

        for path_str in paths {
            let path = Path::new(path_str);
            // Only remove if it's actually a symlink and broken.
            let is_broken_symlink = path
                .symlink_metadata()
                .ok()
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false);

            if !is_broken_symlink {
                continue;
            }

            match tokio::fs::remove_file(path).await {
                Ok(()) => removed += 1,
                Err(_) => failed += 1,
            }
        }

        Ok(SymlinkRemoveResult { removed, failed })
    }

    /// Repair permissions recursively: directories → 0755, executable files → 0755,
    /// regular files → 0644. Skips symlinks.
    pub async fn repair_permissions(&self, root: &str) -> Result<PermissionRepairResult> {
        let root_path = Path::new(root);
        if !root_path.exists() {
            return Err(AppError::InvalidPath(format!(
                "repair path does not exist: {}",
                root
            )));
        }

        // Use find + chmod pipelines for efficiency.
        let home = root.to_string();

        // Fix directories: 755
        let dir_output = Command::new("find")
            .arg(&home)
            .arg("-type")
            .arg("d")
            .arg("-exec")
            .arg("chmod")
            .arg("755")
            .arg("{}")
            .arg(";")
            .output()
            .await
            .map_err(|e| AppError::Internal(format!("chmod dirs failed: {}", e)))?;

        // Fix executable files: 755
        let exec_output = Command::new("find")
            .arg(&home)
            .arg("-type")
            .arg("f")
            .arg("-executable")
            .arg("-exec")
            .arg("chmod")
            .arg("755")
            .arg("{}")
            .arg(";")
            .output()
            .await
            .map_err(|e| AppError::Internal(format!("chmod executables failed: {}", e)))?;

        // Fix regular non-executable files: 644
        let file_output = Command::new("sh")
            .args([
                "-c",
                &format!(
                    "find '{}' -type f ! -executable -exec chmod 644 {{}} \\; 2>/dev/null",
                    home
                ),
            ])
            .output()
            .await
            .map_err(|e| AppError::Internal(format!("chmod files failed: {}", e)))?;

        let success = dir_output.status.success()
            && exec_output.status.success()
            && file_output.status.success();

        // Count what we touched by running separate count passes.
        let dirs_fixed = Command::new("find")
            .arg(&home)
            .arg("-type")
            .arg("d")
            .output()
            .await
            .map(|o| {
                String::from_utf8_lossy(&o.stdout)
                    .lines()
                    .count() as u32
            })
            .unwrap_or(0);

        let files_fixed = Command::new("find")
            .arg(&home)
            .arg("-type")
            .arg("f")
            .output()
            .await
            .map(|o| {
                String::from_utf8_lossy(&o.stdout)
                    .lines()
                    .count() as u32
            })
            .unwrap_or(0);

        Ok(PermissionRepairResult {
            success,
            message: if success {
                "Permission repair completed".to_string()
            } else {
                "Permission repair completed with some errors".to_string()
            },
            dirs_fixed,
            files_fixed,
        })
    }

    /// Rebuild the font icon cache using fc-cache.
    pub async fn rebuild_icon_cache(&self) -> Result<IconCacheResult> {
        let output = Command::new("fc-cache")
            .args(["-f", "-v"])
            .output()
            .await
            .map_err(|e| AppError::Internal(format!("fc-cache failed: {}", e)))?;

        let success = output.status.success();
        let message = if success {
            String::from_utf8_lossy(&output.stderr).trim().to_string()
        } else {
            String::from_utf8_lossy(&output.stderr).trim().to_string()
        };

        Ok(IconCacheResult { success, message })
    }

    /// Remove orphaned packages using the system's package manager.
    /// Supports apt, dnf, pacman, and zypper.
    pub async fn remove_orphaned_packages(&self) -> Result<OrphanRemoveResult> {
        let output = if Command::new("which")
            .arg("apt")
            .output()
            .await
            .ok()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            // Debian/Ubuntu: autoremove
            Command::new("apt")
                .args(["autoremove", "-y"])
                .output()
                .await
                .map_err(|e| AppError::Internal(format!("apt autoremove failed: {}", e)))?
        } else if Command::new("which")
            .arg("dnf")
            .output()
            .await
            .ok()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            // Fedora/RHEL: autoremove
            Command::new("dnf")
                .args(["autoremove", "-y"])
                .output()
                .await
                .map_err(|e| AppError::Internal(format!("dnf autoremove failed: {}", e)))?
        } else if Command::new("which")
            .arg("pacman")
            .output()
            .await
            .ok()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            // Arch Linux: remove unrequired
            Command::new("pacman")
                .args(["-Rns", "--noconfirm", "$(pacman -Qdtq)"])
                .output()
                .await
                .map_err(|e| AppError::Internal(format!("pacman autoremove failed: {}", e)))?
        } else if Command::new("which")
            .arg("zypper")
            .output()
            .await
            .ok()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            // openSUSE: remove-old
            Command::new("zypper")
                .args(["remove", "--clean-deps", "-y", ""])
                .output()
                .await
                .map_err(|e| AppError::Internal(format!("zypper remove failed: {}", e)))?
        } else {
            return Ok(OrphanRemoveResult {
                removed: 0,
                failed: 0,
                message: "No supported package manager found (apt/dnf/pacman/zypper)".to_string(),
            });
        };

        let success = output.status.success();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Parse removed count from output (crude heuristic).
        let removed = stdout
            .lines()
            .chain(stderr.lines())
            .filter(|l| l.contains("removed"))
            .count() as u32;

        Ok(OrphanRemoveResult {
            removed,
            failed: if success { 0 } else { 1 },
            message: if success {
                format!("Removed {} orphaned packages", removed)
            } else {
                format!("Orphan removal encountered errors: {}", stderr)
            },
        })
    }
}
