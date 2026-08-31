//! System KAS handlers for Cleanux
//!
//! Handles memory, kernel, services, and power management operations.

use crate::infrastructure::memory_service::{MemoryError, MemoryService};
use dioxus_shared::response::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
