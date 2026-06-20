/* sys lib */
use crate::models::Response;
use crate::utils::stdout_string;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use std::time::{Duration, Instant};
const CACHE_TTL_SECS: u64 = 5;
struct CachedTemperature {
  readings: Vec<TemperatureInfo>,
  timestamp: Instant,
}
struct TempCache {
  data: Mutex<Option<CachedTemperature>>,
}
impl TempCache {
  fn new() -> Self {
    Self {
      data: Mutex::new(None),
    }
  }
  fn get(&self) -> Option<Vec<TemperatureInfo>> {
    let guard = self.data.lock().ok()?;
    let cache = guard.as_ref()?;
    if cache.timestamp.elapsed() < Duration::from_secs(CACHE_TTL_SECS) {
      Some(cache.readings.clone())
    } else {
      None
    }
  }
  fn set(&self, readings: Vec<TemperatureInfo>) {
    if let Ok(mut guard) = self.data.lock() {
      *guard = Some(CachedTemperature {
        readings,
        timestamp: Instant::now(),
      });
    }
  }
}
static TEMP_CACHE: std::sync::OnceLock<TempCache> = std::sync::OnceLock::new();
fn get_temp_cache() -> &'static TempCache {
  TEMP_CACHE.get_or_init(TempCache::new)
}
pub struct TemperatureService;
#[derive(Debug, Clone, serde::Serialize)]
pub struct TemperatureInfo {
  pub name: String,
  pub sensor_type: String,
  pub temperature_celsius: f32,
  pub max_temp: f32,
}
impl TemperatureService {
  fn collect_temperatures() -> Vec<TemperatureInfo> {
    let mut temperatures: Vec<TemperatureInfo> = Vec::new();
    temperatures.extend(Self::read_thermal_zones());
    temperatures.extend(Self::read_hwmon_temperatures());
    temperatures.extend(Self::parse_sensors_output());
    temperatures
  }
  pub fn get_temperatures() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    let temperatures = if let Some(cached) = get_temp_cache().get() {
      cached
    } else {
      let temps = Self::collect_temperatures();
      get_temp_cache().set(temps.clone());
      temps
    };
    let mut cpu_temps: Vec<TemperatureInfo> = temperatures
      .iter()
      .filter(|t| t.sensor_type == "cpu")
      .cloned()
      .collect();
    cpu_temps.sort_by(|a, b| {
      a.temperature_celsius
        .partial_cmp(&b.temperature_celsius)
        .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut gpu_temps: Vec<TemperatureInfo> = temperatures
      .iter()
      .filter(|t| t.sensor_type == "gpu")
      .cloned()
      .collect();
    gpu_temps.sort_by(|a, b| {
      b.temperature_celsius
        .partial_cmp(&a.temperature_celsius)
        .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut result: Vec<TemperatureInfo> = Vec::new();
    if let Some(cpu) = cpu_temps.first() {
      result.push(cpu.clone());
    }
    result.extend(gpu_temps);
    if result.is_empty() {
      result.push(TemperatureInfo {
        name: "CPU".to_string(),
        sensor_type: "cpu".to_string(),
        temperature_celsius: 0.0,
        max_temp: 100.0,
      });
    }
    let json_values: Vec<serde_json::Value> = result
      .iter()
      .map(|t| serde_json::to_value(t).unwrap_or(serde_json::Value::Null))
      .collect();
    Ok(Response::success(
      "Temperatures retrieved successfully".to_string(),
      serde_json::Value::Array(json_values),
    ))
  }
  fn read_thermal_zones() -> Vec<TemperatureInfo> {
    let mut temps = Vec::new();
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
                .unwrap_or("unknown")
                .to_string();
              let sensor_type = Self::categorize_sensor(&name);
              temps.push(TemperatureInfo {
                name,
                sensor_type,
                temperature_celsius: temp_celsius,
                max_temp: 100.0,
              });
            }
          }
        }
      }
    }
    temps
  }
  fn read_hwmon_temperatures() -> Vec<TemperatureInfo> {
    let mut temps = Vec::new();
    let hwmon_base = Path::new("/sys/class/hwmon");
    if let Ok(entries) = fs::read_dir(hwmon_base) {
      for entry in entries.flatten() {
        let path = entry.path();
        let name_file = path.join("name");
        let temp_file = path.join("temp1_input");
        let sensor_name = if name_file.exists() {
          fs::read_to_string(&name_file)
            .map(|s| s.trim().to_string())
            .unwrap_or_default()
        } else {
          path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("hwmon")
            .to_string()
        };
        if temp_file.exists() {
          if let Ok(content) = fs::read_to_string(&temp_file) {
            if let Ok(temp_millidegrees) = content.trim().parse::<i32>() {
              let temp_celsius = temp_millidegrees as f32 / 1000.0;
              let full_name = format!("hwmon-{}", sensor_name);
              let sensor_type = Self::categorize_sensor(&sensor_name);
              temps.push(TemperatureInfo {
                name: full_name,
                sensor_type,
                temperature_celsius: temp_celsius,
                max_temp: 100.0,
              });
            }
          }
        }
      }
    }
    temps
  }
  fn parse_sensors_output() -> Vec<TemperatureInfo> {
    let output = match Command::new("sensors").output() {
      Ok(o) => o,
      Err(_) => return Vec::new(),
    };
    if !output.status.success() {
      return Vec::new();
    }
    let stdout = stdout_string(&output);
    let mut temps = Vec::new();
    let mut current_section = String::new();
    for line in stdout.lines() {
      let line = line.trim();
      if line.ends_with(':') && !line.contains("Adapter") {
        current_section = line.trim_end_matches(':').to_string();
        continue;
      }
      if line.starts_with("temp1:") || line.starts_with("CPU Temperature:") {
        if let Some(temp) = Self::extract_temp(line) {
          temps.push(TemperatureInfo {
            name: if current_section.is_empty() {
              "CPU".to_string()
            } else {
              current_section.clone()
            },
            sensor_type: "cpu".to_string(),
            temperature_celsius: temp,
            max_temp: 100.0,
          });
        }
      } else if line.starts_with("GPU")
        || line.contains("GPU")
        || current_section.to_lowercase().contains("gpu")
      {
        if let Some(temp) = Self::extract_temp(line) {
          temps.push(TemperatureInfo {
            name: if current_section.is_empty() {
              "GPU".to_string()
            } else {
              current_section.clone()
            },
            sensor_type: "gpu".to_string(),
            temperature_celsius: temp,
            max_temp: 100.0,
          });
        }
      }
    }
    temps
  }
  fn extract_temp(line: &str) -> Option<f32> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    for part in parts {
      if part.ends_with("°C") || part.ends_with("°C") {
        let temp_str = part.trim_end_matches("°C").trim_end_matches("+");
        if let Ok(temp) = temp_str.parse::<f32>() {
          return Some(temp);
        }
      }
      if let Ok(temp) = part.parse::<f32>() {
        if temp < 150.0 && temp > 0.0 {
          return Some(temp);
        }
      }
    }
    None
  }
  fn categorize_sensor(name: &str) -> String {
    let name_lower = name.to_lowercase();
    if name_lower.contains("cpu") || name_lower.contains("core") || name_lower.contains("pkg") {
      return "cpu".to_string();
    }
    if name_lower.contains("gpu")
      || name_lower.contains("graphics")
      || name_lower.contains("nvidia")
      || name_lower.contains("amd")
    {
      return "gpu".to_string();
    }
    "cpu".to_string()
  }
  pub fn get_cpu_temperature() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    let temperatures = if let Some(cached) = get_temp_cache().get() {
      cached
    } else {
      let temps = Self::collect_temperatures();
      get_temp_cache().set(temps.clone());
      temps
    };
    let mut cpu_temps: Vec<TemperatureInfo> = temperatures
      .iter()
      .filter(|t| t.sensor_type == "cpu")
      .cloned()
      .collect();
    cpu_temps.sort_by(|a, b| {
      a.temperature_celsius
        .partial_cmp(&b.temperature_celsius)
        .unwrap_or(std::cmp::Ordering::Equal)
    });
    if let Some(cpu) = cpu_temps.first() {
      Ok(Response::success(
        "CPU temperature retrieved successfully".to_string(),
        serde_json::to_value(cpu).unwrap_or(serde_json::Value::Null),
      ))
    } else {
      Ok(Response::success(
        "No CPU temperature found".to_string(),
        serde_json::to_value(TemperatureInfo {
          name: "CPU".to_string(),
          sensor_type: "cpu".to_string(),
          temperature_celsius: 0.0,
          max_temp: 100.0,
        })
        .unwrap_or(serde_json::Value::Null),
      ))
    }
  }
  pub fn get_gpu_temperature() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
    let temperatures = if let Some(cached) = get_temp_cache().get() {
      cached
    } else {
      let temps = Self::collect_temperatures();
      get_temp_cache().set(temps.clone());
      temps
    };
    let mut gpu_temps: Vec<TemperatureInfo> = temperatures
      .iter()
      .filter(|t| t.sensor_type == "gpu")
      .cloned()
      .collect();
    gpu_temps.sort_by(|a, b| {
      b.temperature_celsius
        .partial_cmp(&a.temperature_celsius)
        .unwrap_or(std::cmp::Ordering::Equal)
    });
    if let Some(gpu) = gpu_temps.first() {
      Ok(Response::success(
        "GPU temperature retrieved successfully".to_string(),
        serde_json::to_value(gpu).unwrap_or(serde_json::Value::Null),
      ))
    } else {
      Ok(Response::success(
        "No GPU temperature found".to_string(),
        serde_json::to_value(TemperatureInfo {
          name: "GPU".to_string(),
          sensor_type: "gpu".to_string(),
          temperature_celsius: 0.0,
          max_temp: 100.0,
        })
        .unwrap_or(serde_json::Value::Null),
      ))
    }
  }
}
