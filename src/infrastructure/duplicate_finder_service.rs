//! DuplicateFinderService — async duplicate file operations using system commands.
//!
//! Provides `find_duplicates` and `delete_duplicates` using `tokio::process::Command`
//! for async execution.

use crate::error::AppError;
use crate::infrastructure::duplicate_scanner;

pub type Result<T, E = AppError> = std::result::Result<T, E>;

pub use duplicate_scanner::{DuplicateGroup, DuplicateScanResult};

/// Find duplicate files in a directory tree using SHA-256 hashing.
pub async fn find_duplicates(path: &str) -> Result<DuplicateScanResult> {
    duplicate_scanner::find_duplicates(path)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
}

/// Delete the specified duplicate files using `rm` command.
/// Returns the number of successfully deleted files.
pub async fn delete_duplicates(paths: Vec<String>) -> Result<usize> {
    let mut deleted = 0;
    for path in paths {
        let output = tokio::process::Command::new("rm")
            .arg("-f")
            .arg(&path)
            .output()
            .await
            .map_err(|e| AppError::Io(format!("failed to execute rm for {}: {}", path, e)))?;

        if output.status.success() {
            deleted += 1;
            tracing::info!("deleted duplicate: {}", path);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            tracing::warn!("failed to delete {}: {}", path, stderr);
        }
    }
    Ok(deleted)
}

/// Delete a single duplicate file using `rm` command.
pub async fn delete_duplicate(path: &str) -> Result<()> {
    let output = tokio::process::Command::new("rm")
        .arg("-f")
        .arg(path)
        .output()
        .await
        .map_err(|e| AppError::Io(format!("failed to execute rm: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Io(format!("failed to delete {}: {}", path, stderr)));
    }

    Ok(())
}
