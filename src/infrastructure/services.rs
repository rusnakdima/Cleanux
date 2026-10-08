//! Infrastructure service implementations for Cleanux.
//!
//! Local stub implementations — these were previously re-exported from `dioxus_shared`
//! but that path no longer exists after the sdui consolidation.

use thiserror::Error;

// ---------------------------------------------------------------------------
// System Info Service
// ---------------------------------------------------------------------------

#[derive(Error, Debug, Clone)]
pub enum SystemInfoError {
  #[error("Failed to access system info: {0}")]
  AccessError(String),
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
  pub name: String,
  pub total: u64,
  pub used: u64,
  pub available: u64,
}

#[derive(Debug, Clone)]
pub struct MemoryInfo {
  pub total: u64,
  pub used: u64,
  pub available: u64,
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
  pub pid: u32,
  pub name: String,
  pub cpu_usage: f32,
  pub memory: u64,
}

/// Stub SystemInfoService implementation.
#[derive(Clone, Default)]
pub struct SystemInfoServiceImpl;

impl SystemInfoServiceImpl {
  pub fn new() -> Self {
    Self
  }

  pub fn uptime(&self) -> Result<u64, SystemInfoError> {
    Ok(0)
  }

  pub fn cpu_usage(&self) -> Result<f32, SystemInfoError> {
    Ok(0.0)
  }

  pub fn memory_info(&self) -> Result<MemoryInfo, SystemInfoError> {
    Ok(MemoryInfo {
      total: 0,
      used: 0,
      available: 0,
    })
  }

  pub fn disk_info(&self) -> Result<Vec<DiskInfo>, SystemInfoError> {
    Ok(vec![])
  }

  pub fn processes(&self) -> Result<Vec<ProcessInfo>, SystemInfoError> {
    Ok(vec![])
  }

  pub fn hostname(&self) -> Result<String, SystemInfoError> {
    Ok("cleanux".to_string())
  }

  pub fn platform(&self) -> Result<String, SystemInfoError> {
    Ok(std::env::consts::OS.to_string())
  }
}

// ---------------------------------------------------------------------------
// Clipboard Service
// ---------------------------------------------------------------------------

#[derive(Clone, Default)]
pub struct ClipboardServiceImpl;

impl ClipboardServiceImpl {
  pub fn new() -> Self {
    Self
  }
}

// ---------------------------------------------------------------------------
// Notification Service (stub implementation)
// ---------------------------------------------------------------------------

/// Stub notification service implementation.
#[derive(Clone, Default)]
pub struct NotificationServiceImpl;

impl NotificationServiceImpl {
  pub fn new() -> Self {
    Self
  }

  pub fn notify(&self, _title: &str, _body: &str) -> Result<(), String> {
    Ok(())
  }

  pub fn notify_info(&self, _title: &str, _body: &str) -> Result<(), String> {
    Ok(())
  }

  pub fn notify_warning(&self, _title: &str, _body: &str) -> Result<(), String> {
    Ok(())
  }

  pub fn notify_error(&self, _title: &str, _body: &str) -> Result<(), String> {
    Ok(())
  }
}
