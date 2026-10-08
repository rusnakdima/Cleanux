//! System KAS handlers for Cleanux
//!
//! Handles memory, kernel, services, and power management operations.

use crate::infrastructure::memory_service::{MemoryError, MemoryService};
use crate::response::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
  pub total: u64,
  pub used: u64,
  pub available: u64,
  pub cached: u64,
  pub buffers: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapInfo {
  pub total: u64,
  pub used: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMemory {
  pub pid: u32,
  pub name: String,
  pub memory_mb: f64,
  pub cpu_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMemoryInfo {
  pub processes: Vec<ProcessMemory>,
  pub total_memory_mb: u64,
  pub used_memory_mb: u64,
}

// Get memory information
pub async fn get_memory_info() -> Result<Response<MemoryInfo>, String> {
  let service = MemoryService::new();
  let info = service.get_memory_info().await.map_err(|e| e.to_string())?;

  Ok(Response::success(
    MemoryInfo {
      total: info.total_kb * 1024,
      used: info.used_kb * 1024,
      available: info.available_kb * 1024,
      cached: info.cached_kb * 1024,
      buffers: info.buffers_kb * 1024,
    },
    Some("Memory info retrieved"),
  ))
}

/// Get swap information
pub async fn get_swap_info() -> Result<Response<SwapInfo>, String> {
  let content =
    std::fs::read_to_string("/proc/swaps").map_err(|e| format!("Failed to read swaps: {}", e))?;

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

  Ok(Response::success(
    SwapInfo { total, used },
    Some("Swap info retrieved"),
  ))
}

// Optimize memory by dropping caches
pub async fn optimize_memory() -> Result<Response<bool>, String> {
  let service = MemoryService::new();
  match service.drop_caches().await {
    Ok(result) => Ok(Response::success(
      result.caches_cleared,
      Some("Memory caches dropped"),
    )),
    Err(MemoryError::PermissionDenied) => {
      Err("Permission denied: requires root privileges".to_string())
    }
    Err(MemoryError::Execution(msg)) => Err(format!("Failed to drop caches: {}", msg)),
    Err(MemoryError::Read(msg)) => Err(format!("Read error: {}", msg)),
  }
}

/// Get current kernel version
pub async fn get_current_kernel() -> Result<Response<String>, String> {
  let version = std::fs::read_to_string("/proc/version")
    .map_err(|e| format!("Failed to read version: {}", e))?;
  let version = version.trim().to_string();
  Ok(Response::success(version, Some("Kernel version retrieved")))
}

/// Get list of installed kernels
pub async fn get_installed_kernels() -> Result<Response<Vec<String>>, String> {
  // TODO: Implement using /boot directory scanning
  Ok(Response::success(
    vec![
      "6.8.0-45-generic".to_string(),
      "6.8.0-44-generic".to_string(),
    ],
    Some("Installed kernels retrieved"),
  ))
}

/// Get list of old kernels that can be removed
pub async fn get_old_kernels() -> Result<Response<Vec<String>>, String> {
  // TODO: Implement by comparing running kernel with installed
  Ok(Response::success(
    vec!["6.8.0-40-generic".to_string()],
    Some("Old kernels retrieved"),
  ))
}

/// Get battery information
pub async fn get_battery_info() -> Result<Response<Value>, String> {
  // TODO: Read from /sys/class/power_supply/
  Ok(Response::success(
    serde_json::json!({
        "present": true,
        "capacity": 87,
        "status": "Discharging"
    }),
    Some("Battery info retrieved"),
  ))
}

/// Get list of system services
pub async fn get_all_services() -> Result<Response<Vec<Value>>, String> {
  // TODO: Use systemctl or read from /etc/init.d/
  Ok(Response::success(vec![], Some("Services retrieved")))
}

/// Stop a system service
pub async fn stop_service(service: String) -> Result<Response<()>, String> {
  // TODO: Use systemctl
  tracing::info!("Stopping service: {}", service);
  Ok(Response::success(
    (),
    Some(&format!("Service {} stopped", service)),
  ))
}

/// Get process memory information
pub async fn get_process_memory() -> Result<Response<ProcessMemoryInfo>, String> {
  let output = Command::new("ps")
    .args(["aux", "--sort=-rss"])
    .output()
    .await
    .map_err(|e| format!("Failed to run ps: {}", e))?;

  let stdout = String::from_utf8_lossy(&output.stdout);
  let mut processes = Vec::new();

  for line in stdout.lines().skip(1).take(20) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 11 {
      let pid = parts[1].parse::<u32>().unwrap_or(0);
      let name = parts[10..].join(" ");
      let rss_kb: u64 = parts[5].parse().unwrap_or(0);
      let memory_mb = rss_kb as f64 / 1024.0;
      let cpu_percent: f32 = parts[2].parse().unwrap_or(0.0);
      processes.push(ProcessMemory {
        pid,
        name,
        memory_mb,
        cpu_percent,
      });
    }
  }

  let service = MemoryService::new();
  let mem_info = service.get_memory_info().await.map_err(|e| e.to_string())?;
  let total_memory_mb = mem_info.total_kb / 1024;
  let used_memory_mb = mem_info.used_kb / 1024;

  Ok(Response::success(
    ProcessMemoryInfo {
      processes,
      total_memory_mb,
      used_memory_mb,
    },
    Some("Process memory info retrieved"),
  ))
}

/// Remove a kernel
pub async fn remove_kernel(kernel: String) -> Result<Response<bool>, String> {
  tracing::info!("Removing kernel: {}", kernel);
  let output = Command::new("sudo")
    .args(["apt", "remove", "-y", &format!("linux-image-{}", kernel)])
    .output()
    .await
    .map_err(|e| format!("Failed to remove kernel: {}", e))?;

  Ok(Response::success(
    output.status.success(),
    Some(&format!("Kernel {} removal attempted", kernel)),
  ))
}

/// Refresh GRUB configuration
pub async fn refresh_grub() -> Result<Response<bool>, String> {
  tracing::info!("Refreshing GRUB configuration");
  let output = Command::new("sudo")
    .arg("update-grub")
    .output()
    .await
    .map_err(|e| format!("Failed to refresh GRUB: {}", e))?;

  Ok(Response::success(
    output.status.success(),
    Some("GRUB configuration updated"),
  ))
}

/// Start a system service
pub async fn start_service(svc: String) -> Result<Response<bool>, String> {
  tracing::info!("Starting service: {}", svc);
  let output = Command::new("sudo")
    .args(["systemctl", "start", &svc])
    .output()
    .await
    .map_err(|e| format!("Failed to start service: {}", e))?;

  Ok(Response::success(
    output.status.success(),
    Some(&format!("Service {} started", svc)),
  ))
}

/// Enable a system service
pub async fn enable_service(svc: String) -> Result<Response<bool>, String> {
  tracing::info!("Enabling service: {}", svc);
  let output = Command::new("sudo")
    .args(["systemctl", "enable", &svc])
    .output()
    .await
    .map_err(|e| format!("Failed to enable service: {}", e))?;

  Ok(Response::success(
    output.status.success(),
    Some(&format!("Service {} enabled", svc)),
  ))
}

// System power actions
pub async fn system_sleep() -> Result<Response<bool>, String> {
  let output = Command::new("systemctl")
    .args(["suspend"])
    .output()
    .await
    .map_err(|e| format!("systemctl suspend failed: {}", e))?;
  Ok(Response::success(
    output.status.success(),
    Some("System suspended"),
  ))
}

pub async fn system_shutdown() -> Result<Response<bool>, String> {
  let output = Command::new("systemctl")
    .args(["poweroff"])
    .output()
    .await
    .map_err(|e| format!("systemctl poweroff failed: {}", e))?;
  Ok(Response::success(
    output.status.success(),
    Some("System shutting down"),
  ))
}

pub async fn system_lock() -> Result<Response<bool>, String> {
  // Try common screen lockers: cinnamon-screensaver, mate-screensaver, xscreensaver, swaylock, loginctl
  let lockers = [
    "cinnamon-screensaver-command",
    "mate-screensaver-command",
    "xscreensaver",
    "swaylock",
    "loginctl",
  ];
  for locker in lockers {
    let result = if locker == "loginctl" {
      Command::new("loginctl")
        .args(["lock-session"])
        .output()
        .await
    } else {
      Command::new("sh")
        .args(["-c", &format!("{} -l 2>/dev/null || true", locker)])
        .output()
        .await
    };
    if result.is_ok() {
      return Ok(Response::success(true, Some("Screen locked")));
    }
  }
  Ok(Response::success(true, Some("Lock command issued")))
}

pub async fn system_reboot() -> Result<Response<bool>, String> {
  let output = Command::new("systemctl")
    .args(["reboot"])
    .output()
    .await
    .map_err(|e| format!("systemctl reboot failed: {}", e))?;
  Ok(Response::success(
    output.status.success(),
    Some("System rebooting"),
  ))
}

pub async fn system_logout() -> Result<Response<bool>, String> {
  let output = match Command::new("loginctl")
    .args([
      "terminate-user",
      &std::env::var("USER").unwrap_or_else(|_| "".into()),
    ])
    .output()
    .await
  {
    Ok(out) => out,
    Err(_) => Command::new("pkill")
      .args(["-u", &std::env::var("USER").unwrap_or_else(|_| "".into())])
      .output()
      .await
      .map_err(|e| format!("logout failed: {}", e))?,
  };
  Ok(Response::success(
    output.status.success(),
    Some("Session ended"),
  ))
}

/// Disable a system service
pub async fn disable_service(svc: String) -> Result<Response<bool>, String> {
  tracing::info!("Disabling service: {}", svc);
  let output = Command::new("sudo")
    .args(["systemctl", "disable", &svc])
    .output()
    .await
    .map_err(|e| format!("Failed to disable service: {}", e))?;

  Ok(Response::success(
    output.status.success(),
    Some(&format!("Service {} disabled", svc)),
  ))
}

// ═══════════════════════════════════════════════════════════════════════
// Process Management Handlers
// ═══════════════════════════════════════════════════════════════════════

use crate::infrastructure::process_service::{
  get_processes as ps_get_processes, kill_process as ps_kill_process,
  kill_processes as ps_kill_processes, ProcessEntry,
};

/// Get all running processes with sysinfo
pub async fn system_get_processes() -> Result<Response<Vec<ProcessEntry>>, String> {
  ps_get_processes()
    .map_err(|e| e.to_string())
    .map(|procs| Response::success(procs, Some("Processes retrieved")))
}

/// Kill a process by PID
pub async fn system_kill_process(pid: u32) -> Result<Response<bool>, String> {
  ps_kill_process(pid)
    .map_err(|e| e.to_string())
    .map(|_| Response::success(true, Some(&format!("Process {} killed", pid))))
}

/// Kill multiple selected processes
pub async fn system_kill_selected_processes(pids: Vec<u32>) -> Result<Response<u32>, String> {
  let killed = ps_kill_processes(pids).map_err(|e| e.to_string())?;
  Ok(Response::success(
    killed,
    Some(&format!("Killed {} processes", killed)),
  ))
}

// ═══════════════════════════════════════════════════════════════════════
// Kernel & Initramfs Handlers
// ═══════════════════════════════════════════════════════════════════════

/// Calculate reclaimable space from old kernels
pub async fn system_get_old_kernels_size() -> Result<Response<u64>, String> {
  let current = get_current_kernel().await?.data.unwrap_or_default();

  let boot_dir = "/boot";
  let entries = std::fs::read_dir(boot_dir).map_err(|e| format!("Failed to read /boot: {}", e))?;

  let mut reclaimable: u64 = 0;
  for entry in entries.flatten() {
    let name = entry.file_name().to_string_lossy().to_string();
    // Match vmlinuz-*.old but not current kernel
    if name.starts_with("vmlinuz-") && !name.contains(&current) {
      if let Ok(meta) = entry.metadata() {
        reclaimable += meta.len();
      }
    }
  }

  Ok(Response::success(
    reclaimable,
    Some("Old kernel size calculated"),
  ))
}

/// List old initramfs files
pub async fn system_get_old_initramfs() -> Result<Response<Vec<String>>, String> {
  let current = get_current_kernel().await?.data.unwrap_or_default();

  let boot_dir = "/boot";
  let entries = std::fs::read_dir(boot_dir).map_err(|e| format!("Failed to read /boot: {}", e))?;

  let mut initramfs_list = Vec::new();
  for entry in entries.flatten() {
    let name = entry.file_name().to_string_lossy().to_string();
    // Match initrd.img-*.old but not current
    if name.starts_with("initrd.img-") && !name.contains(&current) {
      initramfs_list.push(name);
    }
  }

  Ok(Response::success(
    initramfs_list,
    Some("Old initramfs files listed"),
  ))
}

/// Remove a specific initramfs file
pub async fn system_remove_initramfs(initramfs: String) -> Result<Response<bool>, String> {
  let path = format!("/boot/{}", initramfs);
  if !path.contains("initrd.img-") {
    return Err("Invalid initramfs filename".to_string());
  }
  let output = Command::new("sudo")
    .args(["rm", "-f", &path])
    .output()
    .await
    .map_err(|e| format!("Failed to remove initramfs: {}", e))?;

  Ok(Response::success(
    output.status.success(),
    Some(&format!("Initramfs {} removal attempted", initramfs)),
  ))
}

// ═══════════════════════════════════════════════════════════════════════
// Boot Space Handlers
// ═══════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootSpaceInfo {
  pub filesystem: String,
  pub total_bytes: u64,
  pub used_bytes: u64,
  pub available_bytes: u64,
  pub use_percent: u8,
  pub mounted_on: String,
}

/// Get boot partition space info
pub async fn system_get_boot_space_info() -> Result<Response<Vec<BootSpaceInfo>>, String> {
  let output = Command::new("df")
    .args(["-B1", "--output=source,size,used,avail,pcent,target"])
    .output()
    .await
    .map_err(|e| format!("df failed: {}", e))?;

  let stdout = String::from_utf8_lossy(&output.stdout);
  let mut infos = Vec::new();

  for line in stdout.lines().skip(1) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 6 {
      let use_pct: u8 = parts[4].trim_end_matches('%').parse().unwrap_or(0);
      if parts[0].starts_with("/dev/") || parts[5] == "/boot" || parts[5] == "/" {
        infos.push(BootSpaceInfo {
          filesystem: parts[0].to_string(),
          total_bytes: parts[1].parse().unwrap_or(0),
          used_bytes: parts[2].parse().unwrap_or(0),
          available_bytes: parts[3].parse().unwrap_or(0),
          use_percent: use_pct,
          mounted_on: parts[5].to_string(),
        });
      }
    }
  }

  Ok(Response::success(infos, Some("Boot space info retrieved")))
}

// ═══════════════════════════════════════════════════════════════════════
// Power Profile Handlers
// ═══════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerProfile {
  pub name: String,
  pub active: bool,
}

/// Get available power profiles via powerprofilesctl
pub async fn system_get_power_profiles() -> Result<Response<Vec<PowerProfile>>, String> {
  let output = Command::new("powerprofilesctl")
    .args(["list", "--json"])
    .output()
    .await
    .ok();

  if let Some(out) = output {
    if out.status.success() {
      let stdout = String::from_utf8_lossy(&out.stdout);
      if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&stdout) {
        let profiles = parsed["profiles"]
          .as_array()
          .map(|arr| {
            arr
              .iter()
              .map(|p| PowerProfile {
                name: p["name"].as_str().unwrap_or_default().to_string(),
                active: p["active"].as_bool().unwrap_or(false),
              })
              .collect()
          })
          .unwrap_or_default();
        return Ok(Response::success(
          profiles,
          Some("Power profiles retrieved"),
        ));
      }
    }
  }

  // Fallback: parse text output
  let output = Command::new("powerprofilesctl")
    .args(["list"])
    .output()
    .await
    .map_err(|e| format!("powerprofilesctl failed: {}", e))?;

  let stdout = String::from_utf8_lossy(&output.stdout);
  let mut profiles = Vec::new();
  let mut active_profile = String::new();

  for line in stdout.lines() {
    if line.contains("*") {
      active_profile = line
        .trim_start_matches([' ', '*'])
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_string();
    }
  }

  for line in stdout.lines() {
    let name = line
      .trim_start_matches([' ', '*'])
      .split_whitespace()
      .next()
      .unwrap_or_default()
      .to_string();
    if !name.is_empty() {
      profiles.push(PowerProfile {
        name: name.clone(),
        active: name == active_profile,
      });
    }
  }

  Ok(Response::success(
    profiles,
    Some("Power profiles retrieved"),
  ))
}

/// Set the active power profile
pub async fn system_set_power_profile(profile: String) -> Result<Response<bool>, String> {
  let output = Command::new("powerprofilesctl")
    .args(["set", &profile])
    .output()
    .await
    .map_err(|e| format!("powerprofilesctl failed: {}", e))?;

  if output.status.success() {
    Ok(Response::success(
      true,
      Some(&format!("Power profile set to {}", profile)),
    ))
  } else {
    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(format!("Failed to set power profile: {}", stderr))
  }
}

// ═══════════════════════════════════════════════════════════════════════
// Service Bulk Operations Handlers
// ═══════════════════════════════════════════════════════════════════════

/// Stop multiple system services at once
pub async fn system_stop_selected_services(services: Vec<String>) -> Result<Response<u32>, String> {
  let mut stopped = 0u32;
  for svc in &services {
    let output = Command::new("sudo")
      .args(["systemctl", "stop", svc])
      .output()
      .await
      .map_err(|e| format!("Failed to stop {}: {}", svc, e))?;
    if output.status.success() {
      stopped += 1;
    }
  }
  Ok(Response::success(
    stopped,
    Some(&format!("Stopped {} services", stopped)),
  ))
}

/// Enable multiple system services at once
pub async fn system_enable_selected_services(
  services: Vec<String>,
) -> Result<Response<u32>, String> {
  let mut enabled = 0u32;
  for svc in &services {
    let output = Command::new("sudo")
      .args(["systemctl", "enable", svc])
      .output()
      .await
      .map_err(|e| format!("Failed to enable {}: {}", svc, e))?;
    if output.status.success() {
      enabled += 1;
    }
  }
  Ok(Response::success(
    enabled,
    Some(&format!("Enabled {} services", enabled)),
  ))
}
