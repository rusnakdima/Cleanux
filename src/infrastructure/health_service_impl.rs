//! HealthServiceImpl — implementation of HealthServiceTrait
//!
//! Takes system health snapshots using the sysinfo crate and CPU temperature
//! from sys_utils. Compares two snapshots by loading from DocumentService.

use crate::application::health_service::HealthServiceTrait;
use crate::domain::entities::cleaning_report::ComparisonDetails;
use crate::domain::entities::health_snapshot::{
  DiskHealth, HealthSnapshot, MemoryHealth, PackageHealth, TemperatureHealth,
};
use crate::domain::SnapshotComparison;
use crate::error::AppError;
use crate::error::Result;
use crate::global_state::health_snapshot_service;
use crate::infrastructure::sys_utils;
use chrono::Utc;
use sysinfo::{Disks, System};
use tokio::runtime::Runtime;

pub struct HealthServiceImpl;

impl HealthServiceImpl {
  pub fn new() -> Self {
    Self
  }
}

impl Default for HealthServiceImpl {
  fn default() -> Self {
    Self::new()
  }
}

impl HealthServiceTrait for HealthServiceImpl {
  /// Take a current health snapshot.
  fn take_snapshot(&mut self) -> Result<HealthSnapshot> {
    let sys = System::new_all();

    // CPU usage
    let _cpu_percent = sys.global_cpu_usage();

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
    let disks = Disks::new_with_refreshed_list();
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

    // CPU temperature (blocking async call)
    let rt = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()
      .map_err(|e| AppError::Internal(format!("failed to build runtime: {}", e)))?;

    let cpu_temp = rt.block_on(sys_utils::get_cpu_temp()).unwrap_or(f64::NAN);

    let max_temp = cpu_temp;

    // GPU temp placeholder (not available on Linux without proprietary drivers)
    let _gpu_temp: Option<f64> = None;

    // Overall score: 100 - average of normalized pressure metrics
    let overall_score: f64 = 100.0 - (max_temp.max(memory_percent) + disk_percent) / 3.0;
    let overall_score = overall_score.clamp(0.0, 100.0);

    Ok(HealthSnapshot {
      id: Some(uuid::Uuid::new_v4().to_string()),
      timestamp: Utc::now(),
      overall_score,
      disk: DiskHealth {
        total_bytes: total_disk,
        used_bytes: used_disk,
        available_bytes: total_disk.saturating_sub(used_disk),
        health_percentage: 100.0 - disk_percent,
      },
      memory: MemoryHealth {
        total_bytes: total_mem,
        used_bytes: used_mem,
        available_bytes: available_mem,
        health_percentage: 100.0 - memory_percent,
      },
      temperature: TemperatureHealth {
        cpu_temp: Some(cpu_temp),
        gpu_temp: _gpu_temp,
        max_temp,
        health_status: if cpu_temp < 60.0 {
          "good".to_string()
        } else if cpu_temp < 80.0 {
          "warm".to_string()
        } else {
          "hot".to_string()
        },
      },
      system_packages: PackageHealth {
        total_packages: 0,
        out_of_date: 0,
        orphan_count: 0,
      },
    })
  }

  /// Compare two snapshots by ID.
  fn compare_snapshots(&self, before_id: &str, after_id: &str) -> Result<SnapshotComparison> {
    let service = health_snapshot_service();

    let rt =
      Runtime::new().map_err(|e| AppError::Internal(format!("failed to build runtime: {}", e)))?;
    let before = rt
      .block_on(service.get(before_id))
      .map_err(|e| AppError::from(format!("failed to load before snapshot: {}", e)))?;
    let after = rt
      .block_on(service.get(after_id))
      .map_err(|e| AppError::from(format!("failed to load after snapshot: {}", e)))?;

    let (before, after) = match (before, after) {
      (Some(b), Some(a)) => (b, a),
      _ => {
        return Err(AppError::NotFound("one or both snapshots not found".into()));
      }
    };

    let health_improvement = after.overall_score - before.overall_score;

    Ok(SnapshotComparison {
      before_id: before_id.to_string(),
      after_id: after_id.to_string(),
      space_reclaimed: before.disk.used_bytes.saturating_sub(after.disk.used_bytes),
      items_cleaned: 0,
      health_improvement,
      details: ComparisonDetails {
        cache_change: 0,
        trash_change: 0,
        log_change: 0,
        large_file_change: 0,
        disk_change_percent: Some(0.0),
        memory_change_percent: Some(0.0),
        temperature_change: Some(0.0),
        package_change: Some(0),
      },
    })
  }
}
