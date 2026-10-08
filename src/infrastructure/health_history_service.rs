//! Health history service for Cleanux
//!
//! Records health snapshots to persistent storage and provides history queries
//! and snapshot comparison.

use crate::domain::entities::cleaning_report::ComparisonDetails;
use crate::domain::entities::health_snapshot::HealthSnapshot;
use crate::domain::SnapshotComparison;
use crate::error::AppError;
use crate::error::Result;
use crate::global_state::health_snapshot_service;
use chrono::Utc;
use tokio::runtime::Runtime;

/// Service for managing health snapshot history.
pub struct HealthHistoryService {
  runtime: Runtime,
}

impl HealthHistoryService {
  pub fn new() -> Self {
    let runtime = Runtime::new().expect("failed to create tokio runtime");
    Self { runtime }
  }
}

impl Default for HealthHistoryService {
  fn default() -> Self {
    Self::new()
  }
}

impl HealthHistoryService {
  /// Record a snapshot of the current health state and persist it.
  pub fn record_snapshot(&self) -> Result<HealthSnapshot> {
    let sys = sysinfo::System::new_all();

    // Memory
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let available_mem = total_mem.saturating_sub(used_mem);
    let memory_percent = if total_mem > 0 {
      (used_mem as f64 / total_mem as f64) * 100.0
    } else {
      0.0
    };

    // Disk
    let disks = sysinfo::Disks::new_with_refreshed_list();
    let mut total_disk: u64 = 0;
    let mut used_disk: u64 = 0;
    for disk in disks.list() {
      total_disk += disk.total_space();
      used_disk += disk.total_space().saturating_sub(disk.available_space());
    }
    let disk_percent = if total_disk > 0 {
      (used_disk as f64 / total_disk as f64) * 100.0
    } else {
      0.0
    };

    // CPU temperature
    let cpu_temp = self
      .runtime
      .block_on(crate::infrastructure::sys_utils::get_cpu_temp())
      .unwrap_or(f64::NAN);

    let max_temp = cpu_temp;

    // Overall score
    let overall_score: f64 = 100.0 - (max_temp.max(memory_percent) + disk_percent) / 3.0;
    let overall_score = overall_score.clamp(0.0, 100.0);

    let snapshot = HealthSnapshot {
      id: Some(uuid::Uuid::new_v4().to_string()),
      timestamp: Utc::now(),
      overall_score,
      disk: crate::domain::entities::health_snapshot::DiskHealth {
        total_bytes: total_disk,
        used_bytes: used_disk,
        available_bytes: total_disk.saturating_sub(used_disk),
        health_percentage: 100.0 - disk_percent,
      },
      memory: crate::domain::entities::health_snapshot::MemoryHealth {
        total_bytes: total_mem,
        used_bytes: used_mem,
        available_bytes: available_mem,
        health_percentage: 100.0 - memory_percent,
      },
      temperature: crate::domain::entities::health_snapshot::TemperatureHealth {
        cpu_temp: Some(cpu_temp),
        gpu_temp: None,
        max_temp,
        health_status: if cpu_temp < 60.0 {
          "good".to_string()
        } else if cpu_temp < 80.0 {
          "warm".to_string()
        } else {
          "hot".to_string()
        },
      },
      system_packages: crate::domain::entities::health_snapshot::PackageHealth {
        total_packages: 0,
        out_of_date: 0,
        orphan_count: 0,
      },
    };

    // Persist
    if let Some(id) = &snapshot.id {
      self
        .runtime
        .block_on(health_snapshot_service().save(id, snapshot.clone()))
        .map_err(|e| AppError::Internal(format!("failed to persist health snapshot: {}", e)))?;
    }

    Ok(snapshot)
  }

  /// Get health snapshots from the last `days` days.
  pub fn get_history(&self, days: u32) -> Vec<HealthSnapshot> {
    let cutoff = Utc::now() - chrono::Duration::days(days as i64);

    // JsonDocService get_all would need a list method; iterate via storage
    // Return all snapshots filtered by date range
    let all = self.runtime.block_on(async {
      let storage_path = std::env::var("APPDATA")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
      let snapshots_dir = std::path::Path::new(&storage_path)
        .join("cleanux")
        .join("health_snapshots");
      let mut results = Vec::new();

      if let Ok(entries) = std::fs::read_dir(&snapshots_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
          let path = entry.path();
          if path.extension().is_some_and(|e| e == "json") {
            if let Ok(content) = std::fs::read_to_string(&path) {
              if let Ok(snapshot) = serde_json::from_str::<HealthSnapshot>(&content) {
                if snapshot.timestamp > cutoff {
                  results.push(snapshot);
                }
              }
            }
          }
        }
      }
      results
    });

    all
  }

  /// Compare two snapshots by their IDs.
  pub fn compare_snapshots(&self, id1: &str, id2: &str) -> Result<SnapshotComparison> {
    let before = self
      .runtime
      .block_on(health_snapshot_service().get(id1))
      .map_err(|e| AppError::Internal(format!("failed to load before snapshot: {}", e)))?;
    let after = self
      .runtime
      .block_on(health_snapshot_service().get(id2))
      .map_err(|e| AppError::Internal(format!("failed to load after snapshot: {}", e)))?;

    let (before, after) = match (before, after) {
      (Some(b), Some(a)) => (b, a),
      _ => {
        return Err(AppError::NotFound("one or both snapshots not found".into()));
      }
    };

    let health_improvement = after.overall_score - before.overall_score;

    Ok(SnapshotComparison {
      before_id: id1.to_string(),
      after_id: id2.to_string(),
      space_reclaimed: before.disk.used_bytes.saturating_sub(after.disk.used_bytes),
      items_cleaned: 0,
      health_improvement,
      details: ComparisonDetails {
        cache_change: 0,
        trash_change: 0,
        log_change: 0,
        large_file_change: 0,
        package_change: Some(0),
        temperature_change: Some(0.0),
        disk_change_percent: Some(0.0),
        memory_change_percent: Some(0.0),
      },
    })
  }
}
