//! RepairServiceTrait — domain trait for system repair operations.
//!
//! Defines the contract for filesystem repair, permission fixes, and memory reclamation.
//! Implementation lives in `infrastructure`.

use crate::error::Result;

/// Result of a filesystem check operation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FsckResult {
    pub path: String,
    pub errors_corrected: u32,
    pub errors_uncorrected: u32,
    pub exit_code: i32,
    pub raw_output: String,
}

/// Result of a permission repair operation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PermissionFixResult {
    pub path: String,
    pub dirs_fixed: u32,
    pub files_fixed: u32,
    pub errors: Vec<String>,
}

/// Result of memory reclamation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MemoryClearResult {
    pub bytes_freed: u64,
    pub pages_dropped: u64,
}

pub trait RepairServiceTrait: Send + Sync + Clone {
    /// Run a dry-run filesystem check (`fsck -n -N`) on the given path.
    fn check_filesystem(&mut self, path: &str) -> Result<FsckResult>;

    /// Fix common permission issues: 755 on directories, 644 on files
    /// recursively under `path`.
    fn fix_permissions(&mut self, path: &str) -> Result<PermissionFixResult>;

    /// Reclaim memory by dropping caches and triggering OOM reaping.
    fn clear_memory(&mut self) -> Result<MemoryClearResult>;
}
