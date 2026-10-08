//! ProcessService — system process management using sysinfo.

use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};

use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

use crate::response::Response;

pub type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

/// Process entry for KAS responses
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessEntry {
  pub pid: u32,
  pub name: String,
  pub cpu_percent: f64,
  pub memory_bytes: u64,
  pub user: String,
  pub state: String,
}

/// Singleton System for process enumeration
static SYSTEM: LazyLock<Mutex<System>> = LazyLock::new(|| Mutex::new(System::new_all()));

/// Get all running processes
pub fn get_processes() -> Result<Vec<ProcessEntry>> {
  let mut sys = SYSTEM
    .lock()
    .map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
  sys.refresh_processes_specifics(
    ProcessesToUpdate::All,
    true,
    ProcessRefreshKind::everything(),
  );

  let mut processes = Vec::new();
  for (pid, process) in sys.processes() {
    let status = match process.status() {
      sysinfo::ProcessStatus::Run => "Running",
      sysinfo::ProcessStatus::Sleep => "Sleeping",
      sysinfo::ProcessStatus::Stop => "Stopped",
      sysinfo::ProcessStatus::Zombie => "Zombie",
      sysinfo::ProcessStatus::Idle => "Idle",
      _ => "Unknown",
    };

    let user = process
      .user_id()
      .map(|uid| format!("{:?}", uid))
      .unwrap_or_default();

    processes.push(ProcessEntry {
      pid: pid.as_u32(),
      name: process.name().to_string_lossy().to_string(),
      cpu_percent: process.cpu_usage() as f64,
      memory_bytes: process.memory() * 1024,
      user,
      state: status.to_string(),
    });
  }

  processes.sort_by(|a, b| {
    b.cpu_percent
      .partial_cmp(&a.cpu_percent)
      .unwrap_or(std::cmp::Ordering::Equal)
  });
  Ok(processes)
}

/// Kill a single process by PID (SIGTERM)
pub fn kill_process(pid: u32) -> Result<()> {
  let sys = SYSTEM
    .lock()
    .map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
  if let Some(process) = sys.process(Pid::from_u32(pid)) {
    if process.kill() {
      Ok(())
    } else {
      Err(anyhow::anyhow!("Failed to send signal to process {}", pid))
    }
  } else {
    Err(anyhow::anyhow!("Process {} not found", pid))
  }
}

/// Kill multiple processes, returns count of successfully killed
pub fn kill_processes(pids: Vec<u32>) -> Result<u32> {
  let mut killed = 0u32;
  let pids_set: HashSet<u32> = pids.iter().copied().collect();

  for &pid in &pids_set {
    if kill_process(pid).is_ok() {
      killed += 1;
    }
  }
  Ok(killed)
}

// ═══════════════════════════════════════════════════════════════════════
// KAS Handlers — Process Operations
// ═══════════════════════════════════════════════════════════════════════

/// Get all running processes via sysinfo
pub async fn system_get_processes() -> Result<Response<Vec<ProcessEntry>>> {
  let processes = get_processes()?;
  Ok(Response::success(processes, Some("Processes retrieved")))
}

/// Kill a process by PID
pub async fn system_kill_process(pid: u32) -> Result<Response<bool>> {
  kill_process(pid)?;
  Ok(Response::success(
    true,
    Some(&format!("Process {} killed", pid)),
  ))
}

/// Kill multiple selected processes
pub async fn system_kill_selected_processes(pids: Vec<u32>) -> Result<Response<u32>> {
  let killed = kill_processes(pids)?;
  Ok(Response::success(
    killed,
    Some(&format!("Killed {} processes", killed)),
  ))
}
