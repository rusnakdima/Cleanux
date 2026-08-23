//! HealthSnapshot domain entity

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSnapshot {
    pub id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub overall_score: f64,
    pub disk: DiskHealth,
    pub memory: MemoryHealth,
    pub temperature: TemperatureHealth,
    pub system_packages: PackageHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskHealth {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub health_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryHealth {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub health_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureHealth {
    pub cpu_temp: Option<f64>,
    pub gpu_temp: Option<f64>,
    pub max_temp: f64,
    pub health_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageHealth {
    pub total_packages: i64,
    pub out_of_date: i64,
    pub orphan_count: i64,
}
