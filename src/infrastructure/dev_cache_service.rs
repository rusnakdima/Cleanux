//! DevCacheService — scans and cleans developer tool caches.
//!
//! Wraps the dev_cache_scanner module with a typed service interface.
//! Handles npm, pip, Cargo, Go, Maven, and Gradle cache directories.

use crate::infrastructure::scanners::dev_cache_scanner::{self, DevCacheSummary};
use serde::{Deserialize, Serialize};

pub type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

/// Information about a single dev-tool cache.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevCacheInfo {
  pub tool: String,
  pub path: String,
  pub size_bytes: u64,
  pub item_count: u64,
}

/// Result of a dev cache cleaning operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevCacheCleanResult {
  pub tool: String,
  pub freed_bytes: u64,
  pub success: bool,
}

/// Format bytes to human-readable string.
fn _format_bytes(bytes: u64) -> String {
  const KB: u64 = 1024;
  const MB: u64 = KB * 1024;
  const GB: u64 = MB * 1024;
  if bytes >= GB {
    format!("{:.1} GB", bytes as f64 / GB as f64)
  } else if bytes >= MB {
    format!("{:.1} MB", bytes as f64 / MB as f64)
  } else if bytes >= KB {
    format!("{:.1} KB", bytes as f64 / KB as f64)
  } else {
    format!("{} B", bytes)
  }
}

/// Get summary of all dev caches.
pub async fn get_dev_cache_summary() -> Result<DevCacheSummary> {
  Ok(dev_cache_scanner::dev_cache_summary().await)
}

/// Get individual cache info for each tool.
pub async fn get_all_dev_cache_info() -> Result<Vec<DevCacheInfo>> {
  use dev_cache_scanner::dev_cache_summary;
  let summary = dev_cache_summary().await;

  let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/"));

  Ok(vec![
    DevCacheInfo {
      tool: "npm".into(),
      path: home.join(".npm").display().to_string(),
      size_bytes: summary.npm_bytes,
      item_count: 0,
    },
    DevCacheInfo {
      tool: "pip".into(),
      path: home.join(".cache/pip").display().to_string(),
      size_bytes: summary.pip_bytes,
      item_count: 0,
    },
    DevCacheInfo {
      tool: "cargo".into(),
      path: home.join(".cargo/registry/cache").display().to_string(),
      size_bytes: summary.cargo_bytes,
      item_count: 0,
    },
    DevCacheInfo {
      tool: "go".into(),
      path: home.join("go/pkg/mod").display().to_string(),
      size_bytes: summary.go_bytes,
      item_count: 0,
    },
    DevCacheInfo {
      tool: "maven".into(),
      path: home.join(".m2/repository").display().to_string(),
      size_bytes: summary.maven_bytes,
      item_count: 0,
    },
    DevCacheInfo {
      tool: "gradle".into(),
      path: home.join(".gradle/caches").display().to_string(),
      size_bytes: summary.gradle_bytes,
      item_count: 0,
    },
  ])
}

/// Clean npm cache.
pub async fn clean_npm() -> Result<DevCacheCleanResult> {
  let freed = dev_cache_scanner::clean_npm().await;
  Ok(DevCacheCleanResult {
    tool: "npm".into(),
    freed_bytes: freed,
    success: true,
  })
}

/// Clean pip cache.
pub async fn clean_pip() -> Result<DevCacheCleanResult> {
  let freed = dev_cache_scanner::clean_pip().await;
  Ok(DevCacheCleanResult {
    tool: "pip".into(),
    freed_bytes: freed,
    success: true,
  })
}

/// Clean Cargo cache.
pub async fn clean_cargo() -> Result<DevCacheCleanResult> {
  let freed = dev_cache_scanner::clean_cargo().await;
  Ok(DevCacheCleanResult {
    tool: "cargo".into(),
    freed_bytes: freed,
    success: true,
  })
}

/// Clean Go module cache.
pub async fn clean_go() -> Result<DevCacheCleanResult> {
  let freed = dev_cache_scanner::clean_go().await;
  Ok(DevCacheCleanResult {
    tool: "go".into(),
    freed_bytes: freed,
    success: true,
  })
}

/// Clean Maven repository.
pub async fn clean_maven() -> Result<DevCacheCleanResult> {
  let freed = dev_cache_scanner::clean_maven().await;
  Ok(DevCacheCleanResult {
    tool: "maven".into(),
    freed_bytes: freed,
    success: true,
  })
}

/// Clean Gradle caches.
pub async fn clean_gradle() -> Result<DevCacheCleanResult> {
  let freed = dev_cache_scanner::clean_gradle().await;
  Ok(DevCacheCleanResult {
    tool: "gradle".into(),
    freed_bytes: freed,
    success: true,
  })
}

/// Clean all dev caches.
pub async fn clean_all() -> Result<Vec<DevCacheCleanResult>> {
  let mut results = Vec::new();
  results.push(clean_npm().await?);
  results.push(clean_pip().await?);
  results.push(clean_cargo().await?);
  results.push(clean_go().await?);
  results.push(clean_maven().await?);
  results.push(clean_gradle().await?);
  Ok(results)
}
