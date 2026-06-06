/* sys lib */
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use sysinfo::System;

use crate::models::{DataValue, ResponseModel, ResponseStatus};

static MONITORING_ACTIVE: AtomicBool = AtomicBool::new(false);
static SYSTEM: Mutex<Option<System>> = Mutex::new(None);

pub struct MonitorService;

#[derive(Debug, Clone, serde::Serialize)]
pub struct SystemStats {
  pub cpu_usage: f32,
  pub memory_used: u64,
  pub memory_total: u64,
  pub memory_usage_percent: f32,
  pub disk_used: u64,
  pub disk_total: u64,
  pub disk_usage_percent: f32,
}

impl MonitorService {
  pub fn get_system_stats() -> Result<ResponseModel, ResponseModel> {
    {
      let mut sys = SYSTEM.lock().unwrap();
      if sys.is_none() {
        let mut s = System::new_all();
        s.refresh_all();
        *sys = Some(s);
      }
    }
    let mut sys = SYSTEM.lock().unwrap();
    let sys = sys.as_mut().unwrap();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_usage();

    let memory_used = sys.used_memory();
    let memory_total = sys.total_memory();
    let memory_usage_percent = if memory_total > 0 {
      (memory_used as f32 / memory_total as f32) * 100.0
    } else {
      0.0
    };

    let disks = sysinfo::Disks::new_with_refreshed_list();
    let mut disk_used: u64 = 0;
    let mut disk_total: u64 = 0;
    for disk in disks.list() {
      disk_total += disk.total_space();
      disk_used += disk.total_space() - disk.available_space();
    }
    let disk_usage_percent = if disk_total > 0 {
      (disk_used as f32 / disk_total as f32) * 100.0
    } else {
      0.0
    };

    let stats = SystemStats {
      cpu_usage,
      memory_used,
      memory_total,
      memory_usage_percent,
      disk_used,
      disk_total,
      disk_usage_percent,
    };

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "System stats retrieved successfully".to_string(),
      data: DataValue::Object(serde_json::to_value(stats).map_err(|e| e.to_string())?),
    })
  }

  pub fn start_monitoring() -> Result<ResponseModel, ResponseModel> {
    MONITORING_ACTIVE.store(true, Ordering::SeqCst);
    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "Monitoring started".to_string(),
      data: DataValue::Bool(true),
    })
  }

  pub fn stop_monitoring() -> Result<ResponseModel, ResponseModel> {
    MONITORING_ACTIVE.store(false, Ordering::SeqCst);
    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "Monitoring stopped".to_string(),
      data: DataValue::Bool(false),
    })
  }

  pub fn is_monitoring() -> bool {
    MONITORING_ACTIVE.load(Ordering::SeqCst)
  }
}
