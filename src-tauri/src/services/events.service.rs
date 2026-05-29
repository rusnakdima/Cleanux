use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize)]
pub struct ScanProgress {
  pub phase: String,
  pub current: u64,
  pub total: u64,
  pub current_path: Option<String>,
  pub percentage: f32,
  pub items_found: u64,
  pub bytes_scanned: u64,
}

impl ScanProgress {
  pub fn new(phase: &str, current: u64, total: u64) -> Self {
    let percentage = if total > 0 {
      (current as f32 / total as f32) * 100.0
    } else {
      0.0
    };
    Self {
      phase: phase.to_string(),
      current,
      total,
      current_path: None,
      percentage,
      items_found: 0,
      bytes_scanned: 0,
    }
  }

  pub fn with_path(mut self, path: &str) -> Self {
    self.current_path = Some(path.to_string());
    self
  }

  pub fn with_items(mut self, items: u64) -> Self {
    self.items_found = items;
    self
  }

  pub fn with_bytes(mut self, bytes: u64) -> Self {
    self.bytes_scanned = bytes;
    self
  }

  pub fn percentage(&self) -> f32 {
    if self.total > 0 {
      (self.current as f32 / self.total as f32) * 100.0
    } else {
      0.0
    }
  }
}

#[derive(Clone, Serialize)]
pub struct HealthAlert {
  pub health_score: f64,
  pub message: String,
}

pub struct EventsService;

impl EventsService {
  pub fn emit_scan_progress(app: &AppHandle, progress: &ScanProgress) -> Result<(), String> {
    app.emit("scan-progress", progress)
      .map_err(|e| format!("Failed to emit scan-progress event: {}", e))
  }

  pub fn emit_scan_started(app: &AppHandle, scan_type: &str) -> Result<(), String> {
    let progress = ScanProgress {
      phase: "started".to_string(),
      current: 0,
      total: 0,
      current_path: Some(scan_type.to_string()),
      percentage: 0.0,
      items_found: 0,
      bytes_scanned: 0,
    };
    app.emit("scan-progress", progress)
      .map_err(|e| format!("Failed to emit scan-started event: {}", e))
  }

  pub fn emit_scan_completed(app: &AppHandle, items_found: u64, bytes_scanned: u64) -> Result<(), String> {
    let progress = ScanProgress {
      phase: "completed".to_string(),
      current: 100,
      total: 100,
      current_path: None,
      percentage: 100.0,
      items_found,
      bytes_scanned,
    };
    app.emit("scan-progress", progress)
      .map_err(|e| format!("Failed to emit scan-completed event: {}", e))
  }

  pub fn emit_scan_error(app: &AppHandle, error: &str) -> Result<(), String> {
    let progress = ScanProgress {
      phase: "error".to_string(),
      current: 0,
      total: 0,
      current_path: Some(error.to_string()),
      percentage: 0.0,
      items_found: 0,
      bytes_scanned: 0,
    };
    app.emit("scan-progress", progress)
      .map_err(|e| format!("Failed to emit scan-error event: {}", e))
  }

  pub fn emit_health_alert(app: &AppHandle, health_score: f64, message: &str) -> Result<(), String> {
    let alert = HealthAlert {
      health_score,
      message: message.to_string(),
    };
    app.emit("health-alert", alert)
      .map_err(|e| format!("Failed to emit health-alert event: {}", e))
  }

  pub fn emit_cleaning_progress(app: &AppHandle, category: &str, current: u64, total: u64) -> Result<(), String> {
    let progress = ScanProgress {
      phase: format!("cleaning_{}", category),
      current,
      total,
      current_path: None,
      percentage: if total > 0 { (current as f32 / total as f32) * 100.0 } else { 0.0 },
      items_found: current,
      bytes_scanned: 0,
    };
    app.emit("cleaning-progress", progress)
      .map_err(|e| format!("Failed to emit cleaning-progress event: {}", e))
  }
}
