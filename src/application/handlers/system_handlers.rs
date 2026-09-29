//! System KAS handlers for Cleanux
//!
//! Handles memory, kernel, services, and power management operations.

use crate::infrastructure::memory_service::{MemoryError, MemoryService};
use dioxus_shared::response::Response;
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
    let content = std::fs::read_to_string("/proc/swaps")
        .map_err(|e| format!("Failed to read swaps: {}", e))?;

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
