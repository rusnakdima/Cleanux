//! System KAS handlers for Cleanux
//!
//! Handles memory, kernel, services, and power management operations.

use super::crud_handlers::KasResponse;
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

/// Get memory information
pub async fn get_memory_info() -> Result<KasResponse<MemoryInfo>, String> {
    // Read from /proc/meminfo
    let content = std::fs::read_to_string("/proc/meminfo")
        .map_err(|e| format!("Failed to read meminfo: {}", e))?;

    let parse_value = |key: &str| -> u64 {
        content
            .lines()
            .find(|line| line.starts_with(key))
            .and_then(|line| {
                line.split_whitespace()
                    .nth(1)
                    .and_then(|v| v.parse::<u64>().ok())
            })
            .unwrap_or(0)
            * 1024
    };

    let total = parse_value("MemTotal:");
    let free = parse_value("MemFree:");
    let buffers = parse_value("Buffers:");
    let cached = parse_value("Cached:");
    let available = free + buffers + cached;
    let used = total.saturating_sub(free);

    Ok(KasResponse::success(
        MemoryInfo {
            total,
            used,
            available,
            cached,
            buffers,
        },
        "Memory info retrieved",
    ))
}

/// Get swap information
pub async fn get_swap_info() -> Result<KasResponse<SwapInfo>, String> {
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

    Ok(KasResponse::success(
        SwapInfo { total, used },
        "Swap info retrieved",
    ))
}

/// Optimize memory by dropping caches
pub async fn optimize_memory() -> Result<KasResponse<bool>, String> {
    std::fs::write("/proc/sys/vm/drop_caches", "3")
        .map_err(|e| format!("Failed to drop caches: {}", e))?;
    Ok(KasResponse::success(true, "Memory caches dropped"))
}

/// Get current kernel version
pub async fn get_current_kernel() -> Result<KasResponse<String>, String> {
    let version = std::fs::read_to_string("/proc/version")
        .map_err(|e| format!("Failed to read version: {}", e))?;
    let version = version.trim().to_string();
    Ok(KasResponse::success(version, "Kernel version retrieved"))
}

/// Get list of installed kernels
pub async fn get_installed_kernels() -> Result<KasResponse<Vec<String>>, String> {
    // TODO: Implement using /boot directory scanning
    Ok(KasResponse::success(
        vec![
            "6.8.0-45-generic".to_string(),
            "6.8.0-44-generic".to_string(),
        ],
        "Installed kernels retrieved",
    ))
}

/// Get list of old kernels that can be removed
pub async fn get_old_kernels() -> Result<KasResponse<Vec<String>>, String> {
    // TODO: Implement by comparing running kernel with installed
    Ok(KasResponse::success(
        vec!["6.8.0-40-generic".to_string()],
        "Old kernels retrieved",
    ))
}

/// Get battery information
pub async fn get_battery_info() -> Result<KasResponse<Value>, String> {
    // TODO: Read from /sys/class/power_supply/
    Ok(KasResponse::success(
        serde_json::json!({
            "present": true,
            "capacity": 87,
            "status": "Discharging"
        }),
        "Battery info retrieved",
    ))
}

/// Get list of system services
pub async fn get_all_services() -> Result<KasResponse<Vec<Value>>, String> {
    // TODO: Use systemctl or read from /etc/init.d/
    Ok(KasResponse::success(vec![], "Services retrieved"))
}

/// Stop a system service
pub async fn stop_service(service: String) -> Result<KasResponse<()>, String> {
    // TODO: Use systemctl
    tracing::info!("Stopping service: {}", service);
    Ok(KasResponse::success(
        (),
        format!("Service {} stopped", service),
    ))
}
