//! Memory optimization service
//!
//! Provides memory information and cache dropping functionality.

use std::process::Command;

/// Errors that can occur during memory operations
#[derive(Debug, Clone)]
pub enum MemoryError {
    /// Permission denied - requires root/sudo
    PermissionDenied,
    /// Failed to execute command
    Execution(String),
    /// Failed to read information
    Read(String),
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::PermissionDenied => {
                write!(f, "Permission denied: requires root privileges")
            }
            MemoryError::Execution(msg) => write!(f, "Execution error: {}", msg),
            MemoryError::Read(msg) => write!(f, "Read error: {}", msg),
        }
    }
}

impl std::error::Error for MemoryError {}

/// Result of dropping caches
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryResult {
    pub caches_cleared: bool,
    pub output: String,
}

/// Detailed memory information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryInfo {
    pub total_kb: u64,
    pub free_kb: u64,
    pub available_kb: u64,
    pub buffers_kb: u64,
    pub cached_kb: u64,
    pub swap_cached_kb: u64,
    pub used_kb: u64,
    pub usage_percent: f64,
}

pub struct MemoryService;

impl MemoryService {
    pub fn new() -> Self {
        Self
    }

    /// Drops filesystem caches by writing to /proc/sys/vm/drop_caches
    /// Requires root privileges. Uses sync + tee approach.
    pub async fn drop_caches(&self) -> Result<MemoryResult, MemoryError> {
        let output = Command::new("sh")
            .args(["-c", "sync && echo 3 | sudo tee /proc/sys/vm/drop_caches"])
            .output()
            .map_err(|e| MemoryError::Execution(e.to_string()))?;

        if output.status.success() {
            Ok(MemoryResult {
                caches_cleared: true,
                output: String::from_utf8_lossy(&output.stdout).to_string(),
            })
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("Permission denied") || stderr.contains("Operation not permitted") {
                Err(MemoryError::PermissionDenied)
            } else {
                Err(MemoryError::Execution(stderr.to_string()))
            }
        }
    }

    /// Gets detailed memory information from /proc/meminfo
    pub async fn get_memory_info(&self) -> Result<MemoryInfo, MemoryError> {
        let content = tokio::fs::read_to_string("/proc/meminfo")
            .await
            .map_err(|e| MemoryError::Read(e.to_string()))?;

        let parse_value = |key: &str| -> u64 {
            content
                .lines()
                .find(|line| line.starts_with(key))
                .and_then(|line| {
                    line.split_whitespace()
                        .nth(1)
                        .and_then(|v| v.parse::<u64>().ok())
                })
                .unwrap_or(0)
        };

        let total_kb = parse_value("MemTotal:");
        let free_kb = parse_value("MemFree:");
        let available_kb = parse_value("MemAvailable:");
        let buffers_kb = parse_value("Buffers:");
        let cached_kb = parse_value("Cached:");
        let swap_cached_kb = parse_value("SwapCached:");

        let used_kb = total_kb.saturating_sub(available_kb);
        let usage_percent = if total_kb > 0 {
            (used_kb as f64 / total_kb as f64) * 100.0
        } else {
            0.0
        };

        Ok(MemoryInfo {
            total_kb,
            free_kb,
            available_kb,
            buffers_kb,
            cached_kb,
            swap_cached_kb,
            used_kb,
            usage_percent,
        })
    }
}

impl Default for MemoryService {
    fn default() -> Self {
        Self::new()
    }
}
