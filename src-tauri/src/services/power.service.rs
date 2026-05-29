/* sys lib */
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::models::{DataValue, ResponseModel, ResponseStatus};

const CACHE_TTL_SECS: u64 = 10;

struct CachedBatteryInfo {
  info: Option<BatteryInfo>,
  timestamp: Instant,
}

struct BatteryCache {
  data: Mutex<Option<CachedBatteryInfo>>,
}

impl BatteryCache {
  fn new() -> Self {
    Self {
      data: Mutex::new(None),
    }
  }

  fn get(&self) -> Option<Option<BatteryInfo>> {
    let guard = self.data.lock().ok()?;
    let cache = guard.as_ref()?;
    if cache.timestamp.elapsed() < Duration::from_secs(CACHE_TTL_SECS) {
      Some(cache.info.clone())
    } else {
      None
    }
  }

  fn set(&self, info: Option<BatteryInfo>) {
    if let Ok(mut guard) = self.data.lock() {
      *guard = Some(CachedBatteryInfo {
        info,
        timestamp: Instant::now(),
      });
    }
  }

  fn clear(&self) {
    if let Ok(mut guard) = self.data.lock() {
      *guard = None;
    }
  }
}

static BATTERY_CACHE: std::sync::OnceLock<BatteryCache> = std::sync::OnceLock::new();

fn get_battery_cache() -> &'static BatteryCache {
  BATTERY_CACHE.get_or_init(BatteryCache::new)
}

pub struct PowerService;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BatteryInfo {
  pub present: bool,
  pub charge_percent: u8,
  pub health_percent: u8,
  pub cycles: u32,
  pub temperature: f32,
  pub status: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PowerProfile {
  pub name: String,
  pub profile_type: String,
  pub active: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ThermalInfo {
  pub name: String,
  pub temperature_celsius: f32,
  pub max_temp: f32,
}

impl PowerService {
  pub fn get_battery_info() -> Result<ResponseModel, ResponseModel> {
    let battery_info = match get_battery_cache().get() {
      Some(info) => info,
      None => {
        let info = Self::collect_battery_info();
        get_battery_cache().set(info.clone());
        info
      }
    };

    let json_value = serde_json::to_value(&battery_info).map_err(|e| e.to_string())?;

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "Battery info retrieved successfully".to_string(),
      data: DataValue::Object(json_value),
    })
  }

  fn collect_battery_info() -> Option<BatteryInfo> {
    let power_supply_base = Path::new("/sys/class/power_supply");

    let entries = match fs::read_dir(power_supply_base) {
      Ok(e) => e,
      Err(_) => return None,
    };

    for entry in entries.flatten() {
      let path = entry.path();
      let uevent_file = path.join("uevent");

      if !uevent_file.exists() {
        continue;
      }

      let content = match fs::read_to_string(&uevent_file) {
        Ok(c) => c,
        Err(_) => continue,
      };

      let mut present = false;
      let mut power_supply_type = String::new();
      let mut charge_full: u64 = 0;
      let mut charge_full_design: u64 = 0;
      let mut charge_now: u64 = 0;
      let mut status = String::new();
      let mut cycle_count: u32 = 0;

      for line in content.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
          continue;
        }
        let key = parts[0];
        let value = parts[1];

        match key {
          "POWER_SUPPLY_PRESENT" => {
            present = value == "1";
          }
          "POWER_SUPPLY_TYPE" => {
            power_supply_type = value.to_string();
          }
          "POWER_SUPPLY_CHARGE_FULL" => {
            charge_full = value.parse().unwrap_or(0);
          }
          "POWER_SUPPLY_ENERGY_FULL" => {
            if charge_full == 0 {
              charge_full = value.parse().unwrap_or(0);
            }
          }
          "POWER_SUPPLY_CHARGE_FULL_DESIGN" => {
            charge_full_design = value.parse().unwrap_or(0);
          }
          "POWER_SUPPLY_ENERGY_FULL_DESIGN" => {
            if charge_full_design == 0 {
              charge_full_design = value.parse().unwrap_or(0);
            }
          }
          "POWER_SUPPLY_CHARGE_NOW" => {
            charge_now = value.parse().unwrap_or(0);
          }
          "POWER_SUPPLY_ENERGY_NOW" => {
            if charge_now == 0 {
              charge_now = value.parse().unwrap_or(0);
            }
          }
          "POWER_SUPPLY_STATUS" => {
            status = value.to_string();
          }
          "POWER_SUPPLY_CYCLE_COUNT" => {
            cycle_count = value.parse().unwrap_or(0);
          }
          _ => {}
        }
      }

      if !present || power_supply_type != "Battery" {
        continue;
      }

      let charge_percent = if charge_full > 0 {
        ((charge_now as f64 / charge_full as f64) * 100.0) as u8
      } else if charge_full_design > 0 {
        ((charge_now as f64 / charge_full_design as f64) * 100.0) as u8
      } else {
        50
      };

      let health_percent = if charge_full_design > 0 && charge_full > 0 {
        ((charge_full as f64 / charge_full_design as f64) * 100.0) as u8
      } else {
        100
      };

      let temperature = Self::read_battery_temperature(&path);

      return Some(BatteryInfo {
        present,
        charge_percent: charge_percent.min(100),
        health_percent: health_percent.min(100),
        cycles: cycle_count,
        temperature,
        status,
      });
    }

    None
  }

  fn read_battery_temperature(battery_path: &Path) -> f32 {
    let temp_file = battery_path.join("temp");
    if temp_file.exists() {
      if let Ok(content) = fs::read_to_string(&temp_file) {
        if let Ok(temp_millidegrees) = content.trim().parse::<i32>() {
          return temp_millidegrees as f32 / 1000.0;
        }
      }
    }

    let uevent_file = battery_path.join("uevent");
    if let Ok(content) = fs::read_to_string(&uevent_file) {
      for line in content.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 && parts[0] == "POWER_SUPPLY_TEMP" {
          if let Ok(temp) = parts[1].parse::<i32>() {
            return temp as f32 / 10.0;
          }
        }
      }
    }

    0.0
  }

  pub fn get_power_profiles() -> Result<ResponseModel, ResponseModel> {
    let profiles = Self::collect_power_profiles();

    let json_values: Vec<serde_json::Value> = profiles
      .iter()
      .map(|p| serde_json::to_value(p).unwrap_or(serde_json::Value::Null))
      .collect();

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "Power profiles retrieved successfully".to_string(),
      data: DataValue::Array(json_values),
    })
  }

  fn collect_power_profiles() -> Vec<PowerProfile> {
    let mut profiles = Vec::new();

    if let Ok(output) = Command::new("powerprofilesctl").arg("list").output() {
      if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        profiles = Self::parse_powerprofilesctl_output(&stdout);
      }
    }

    if profiles.is_empty() {
      if let Ok(output) = Command::new("systemctl")
        .arg("--user")
        .arg("status")
        .arg("power-profiles-daemon")
        .output()
      {
        if output.status.success() {
          profiles.push(PowerProfile {
            name: "balanced".to_string(),
            profile_type: "balanced".to_string(),
            active: true,
          });
        }
      }
    }

    if profiles.is_empty() {
      profiles.push(PowerProfile {
        name: "power-saver".to_string(),
        profile_type: "power-saver".to_string(),
        active: false,
      });
      profiles.push(PowerProfile {
        name: "balanced".to_string(),
        profile_type: "balanced".to_string(),
        active: true,
      });
      profiles.push(PowerProfile {
        name: "performance".to_string(),
        profile_type: "performance".to_string(),
        active: false,
      });
    }

    profiles
  }

  fn parse_powerprofilesctl_output(output: &str) -> Vec<PowerProfile> {
    let mut profiles = Vec::new();
    let mut active_profile = String::new();

    for line in output.lines() {
      if line.contains('*') {
        if let Some(name) = line.split_whitespace().nth(1) {
          active_profile = name.to_string();
        }
      }
    }

    let profile_names = ["power-saver", "balanced", "performance"];
    for name in profile_names {
      profiles.push(PowerProfile {
        name: name.to_string(),
        profile_type: name.to_string(),
        active: name == active_profile,
      });
    }

    profiles
  }

  pub fn set_power_profile(profile: String) -> Result<ResponseModel, ResponseModel> {
    let profile_lower = profile.to_lowercase();

    let valid_profiles = ["power-saver", "balanced", "performance"];
    if !valid_profiles.contains(&profile_lower.as_str()) {
      return Ok(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Invalid power profile: {}", profile),
        data: DataValue::Bool(false),
      });
    }

    let result = Command::new("powerprofilesctl")
      .arg("set")
      .arg(&profile_lower)
      .output();

    match result {
      Ok(output) => {
        if output.status.success() {
          Ok(ResponseModel {
            status: ResponseStatus::Success,
            message: format!("Power profile set to {}", profile),
            data: DataValue::Bool(true),
          })
        } else {
          let stderr = String::from_utf8_lossy(&output.stderr);
          if stderr.contains("Permission denied") || stderr.contains("not authorized") {
            Self::set_power_profile_systemd(&profile_lower)
          } else {
            Ok(ResponseModel {
              status: ResponseStatus::Error,
              message: format!("Failed to set power profile: {}", stderr),
              data: DataValue::Bool(false),
            })
          }
        }
      }
      Err(_) => Self::set_power_profile_systemd(&profile_lower),
    }
  }

  fn set_power_profile_systemd(profile: &str) -> Result<ResponseModel, ResponseModel> {
    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: format!("Power profile set to {} (systemd)", profile),
      data: DataValue::Bool(true),
    })
  }

  pub fn get_thermal_info() -> Result<ResponseModel, ResponseModel> {
    let thermals = Self::collect_thermal_info();

    let json_values: Vec<serde_json::Value> = thermals
      .iter()
      .map(|t| serde_json::to_value(t).unwrap_or(serde_json::Value::Null))
      .collect();

    Ok(ResponseModel {
      status: ResponseStatus::Success,
      message: "Thermal info retrieved successfully".to_string(),
      data: DataValue::Array(json_values),
    })
  }

  fn collect_thermal_info() -> Vec<ThermalInfo> {
    let mut thermals = Vec::new();
    let thermal_base = Path::new("/sys/class/thermal");

    if let Ok(entries) = fs::read_dir(thermal_base) {
      for entry in entries.flatten() {
        let path = entry.path();
        let temp_file = path.join("temp");

        if temp_file.exists() {
          if let Ok(content) = fs::read_to_string(&temp_file) {
            if let Ok(temp_millidegrees) = content.trim().parse::<i32>() {
              let temp_celsius = temp_millidegrees as f32 / 1000.0;
              let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("thermal")
                .to_string();

              thermals.push(ThermalInfo {
                name,
                temperature_celsius: temp_celsius,
                max_temp: 100.0,
              });
            }
          }
        }
      }
    }

    if thermals.is_empty() {
      if let Ok(entries) = fs::read_dir(Path::new("/sys/class/hwmon")) {
        for entry in entries.flatten() {
          let path = entry.path();
          let temp_file = path.join("temp1_input");

          if temp_file.exists() {
            if let Ok(content) = fs::read_to_string(&temp_file) {
              if let Ok(temp_millidegrees) = content.trim().parse::<i32>() {
                let temp_celsius = temp_millidegrees as f32 / 1000.0;
                let name = path
                  .file_name()
                  .and_then(|n| n.to_str())
                  .unwrap_or("hwmon")
                  .to_string();

                thermals.push(ThermalInfo {
                  name,
                  temperature_celsius: temp_celsius,
                  max_temp: 100.0,
                });
              }
            }
          }
        }
      }
    }

    thermals
  }

  pub fn clear_cache() {
    get_battery_cache().clear();
  }
}
