use std::sync::Mutex;
use sysinfo::System;

use crate::models::{AppError, Response, Status};
use serde_json::Value;

static PROCESS_SYSTEM: Mutex<Option<System>> = Mutex::new(None);

pub struct MemoryService;

#[derive(Debug, Clone, serde::Serialize)]
pub struct MemoryInfo {
  pub total: u64,
  pub used: u64,
  pub available: u64,
  pub cached: u64,
  pub buffers: u64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SwapInfo {
  pub total: u64,
  pub used: u64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProcessMemory {
  pub pid: u32,
  pub name: String,
  pub memory_mb: f64,
  pub cpu_percent: f32,
}

fn parse_meminfo_value(content: &str, key: &str) -> u64 {
  content
    .lines()
    .find(|line| line.starts_with(key))
    .and_then(|line| {
      line
        .split_whitespace()
        .nth(1)
        .and_then(|v| v.parse::<u64>().ok())
    })
    .unwrap_or(0)
    * 1024
}

fn parse_proc_swaps() -> SwapInfo {
  let content = std::fs::read_to_string("/proc/swaps").unwrap_or_default();
  let mut total: u64 = 0;
  let mut used: u64 = 0;

  for (idx, line) in content.lines().enumerate() {
    if idx == 0 {
      continue;
    }
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 4 {
      total += parts[2].parse::<u64>().unwrap_or(0) * 1024;
      used += parts[3].parse::<u64>().unwrap_or(0) * 1024;
    }
  }

  SwapInfo { total, used }
}

impl MemoryService {
  pub fn get_memory_info() -> Result<Response<Value>, Response<Value>> {
    let content = std::fs::read_to_string("/proc/meminfo")
      .map_err(|e| AppError::Message(format!("Failed to read meminfo: {}", e)).into_response())?;

    let total = parse_meminfo_value(&content, "MemTotal:");
    let free = parse_meminfo_value(&content, "MemFree:");
    let buffers = parse_meminfo_value(&content, "Buffers:");
    let cached = parse_meminfo_value(&content, "Cached:");
    let available = free + buffers + cached;
    let used = total.saturating_sub(free);

    let memory_info = MemoryInfo {
      total,
      used,
      available,
      cached,
      buffers,
    };

    Ok(Response {
      status: Status::Success,
      message: "Memory info retrieved".to_string(),
      data: serde_json::to_value(memory_info).map_err(|e| e.to_string())?,
    })
  }

  pub fn get_swap_info() -> Result<Response<Value>, Response<Value>> {
    let swap_info = parse_proc_swaps();

    Ok(Response {
      status: Status::Success,
      message: "Swap info retrieved".to_string(),
      data: serde_json::to_value(swap_info).map_err(|e| e.to_string())?,
    })
  }

  pub fn get_process_memory() -> Result<Response<Value>, Response<Value>> {
    {
      let mut sys = PROCESS_SYSTEM.lock().unwrap();
      if sys.is_none() {
        let mut s = System::new_all();
        s.refresh_all();
        *sys = Some(s);
      }
    }
    let mut sys = PROCESS_SYSTEM.lock().unwrap();
    let sys = sys.as_mut().unwrap();
    sys.refresh_all();

    let mut processes: Vec<ProcessMemory> = sys
      .processes()
      .iter()
      .map(|(pid, process)| ProcessMemory {
        pid: pid.as_u32(),
        name: process.name().to_string_lossy().into_owned(),
        memory_mb: process.memory() as f64 / 1024.0 / 1024.0,
        cpu_percent: process.cpu_usage(),
      })
      .collect();

    processes.sort_by(|a, b| {
      b.memory_mb
        .partial_cmp(&a.memory_mb)
        .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(Response {
      status: Status::Success,
      message: format!("Found {} processes", processes.len()),
      data: Value::Array(
        processes
          .into_iter()
          .map(serde_json::to_value)
          .filter_map(Result::ok)
          .collect(),
      ),
    })
  }

  pub fn optimize_memory() -> Result<Response<Value>, Response<Value>> {
    std::fs::write("/proc/sys/vm/drop_caches", "3")
      .map_err(|e| AppError::Message(format!("Failed to drop caches: {}", e)).into_response())?;

    Ok(Response {
      status: Status::Success,
      message: "Memory caches dropped successfully".to_string(),
      data: Value::Bool(true),
    })
  }
}
