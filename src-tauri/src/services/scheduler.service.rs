/* helpers */
use crate::helpers::{data_string, success_response};
use crate::models::ResponseModel;
/* errors */
use crate::models::AppError;
/* sys lib */
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CleaningType {
  Cache,
  Trash,
  Logs,
  LargeFiles,
  All,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduleConfig {
  pub enabled: bool,
  pub interval_hours: u32,
  pub cleaning_type: CleaningType,
  pub paths: Vec<String>,
  pub last_run: Option<String>,
  pub next_run: Option<String>,
}

impl Default for ScheduleConfig {
  fn default() -> Self {
    ScheduleConfig {
      enabled: false,
      interval_hours: 24,
      cleaning_type: CleaningType::All,
      paths: Vec::new(),
      last_run: None,
      next_run: None,
    }
  }
}

pub struct SchedulerService;

fn get_config_path() -> PathBuf {
  let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
  let cleanux_dir = config_dir.join("cleanux");
  if !cleanux_dir.exists() {
    let _ = fs::create_dir_all(&cleanux_dir);
  }
  cleanux_dir.join("schedule.json")
}

fn calculate_next_run(interval_hours: u32) -> Option<String> {
  use std::time::{SystemTime, UNIX_EPOCH};
  let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
  let interval_secs = interval_hours as u64 * 3600;
  let next_secs = now + interval_secs;
  let next_time = UNIX_EPOCH + std::time::Duration::from_secs(next_secs);
  let datetime = chrono::DateTime::<chrono::Utc>::from(next_time);
  Some(datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string())
}

#[allow(non_snake_case)]
impl SchedulerService {
  pub fn get_schedule() -> Result<ResponseModel, ResponseModel> {
    Self::get_schedule_inner().map_err(|e| e.into_response())
  }

  fn get_schedule_inner() -> Result<ResponseModel, AppError> {
    let config_path = get_config_path();
    if !config_path.exists() {
      return Ok(success_response(
        "No schedule configured",
        data_string("null"),
      ));
    }
    let content = fs::read_to_string(&config_path)
      .map_err(|e| AppError::message(format!("Failed to read schedule: {}", e)))?;
    let config: ScheduleConfig = serde_json::from_str(&content)
      .map_err(|e| AppError::message(format!("Failed to parse schedule: {}", e)))?;
    let json = serde_json::to_value(&config)
      .map_err(|e| AppError::message(format!("Failed to serialize schedule: {}", e)))?;
    Ok(success_response(
      "Schedule retrieved successfully",
      serde_json::from_value(json).unwrap(),
    ))
  }

  pub fn save_schedule(config: ScheduleConfig) -> Result<ResponseModel, ResponseModel> {
    Self::save_schedule_inner(config).map_err(|e| e.into_response())
  }

  fn save_schedule_inner(mut config: ScheduleConfig) -> Result<ResponseModel, AppError> {
    let config_path = get_config_path();
    if config.enabled {
      config.next_run = calculate_next_run(config.interval_hours);
    } else {
      config.next_run = None;
    }
    let json = serde_json::to_string_pretty(&config)
      .map_err(|e| AppError::message(format!("Failed to serialize schedule: {}", e)))?;
    fs::write(&config_path, json)
      .map_err(|e| AppError::message(format!("Failed to write schedule: {}", e)))?;
    Self::setup_systemd_timer(&config)?;
    Ok(success_response(
      "Schedule saved successfully",
      data_string("saved"),
    ))
  }

  pub fn delete_schedule() -> Result<ResponseModel, ResponseModel> {
    Self::delete_schedule_inner().map_err(|e| e.into_response())
  }

  fn delete_schedule_inner() -> Result<ResponseModel, AppError> {
    let config_path = get_config_path();
    if config_path.exists() {
      fs::remove_file(&config_path)
        .map_err(|e| AppError::message(format!("Failed to delete schedule: {}", e)))?;
    }
    Self::remove_systemd_timer()?;
    Ok(success_response(
      "Schedule deleted successfully",
      data_string("deleted"),
    ))
  }

  fn setup_systemd_timer(config: &ScheduleConfig) -> Result<(), AppError> {
    use std::process::Command;
    let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
    let service_content = format!(
            "[Unit]\nDescription=Cleanux Scheduled Cleaning\n\n[Service]\nType=oneshot\nExecStart=/usr/bin/cleanux-cli run --type={}\n",
            match config.cleaning_type {
                CleaningType::Cache => "cache",
                CleaningType::Trash => "trash",
                CleaningType::Logs => "logs",
                CleaningType::LargeFiles => "largefiles",
                CleaningType::All => "all",
            }
        );
    let timer_content = format!(
            "[Unit]\nDescription=Cleanux Scheduled Cleaning Timer\n\n[Timer]\nOnBootSec={}h\nOnUnitActiveSec={}h\nPersistent=true\n\n[Install]\nWantedBy=timers.target\n",
            config.interval_hours, config.interval_hours
        );
    let systemd_user_dir = format!("/home/{}/.config/systemd/user", user);
    let _ = fs::create_dir_all(&systemd_user_dir);
    let service_path = format!("{}/cleanux-cleaning.service", systemd_user_dir);
    let timer_path = format!("{}/cleanux-cleaning.timer", systemd_user_dir);
    fs::write(&service_path, service_content)
      .map_err(|e| AppError::message(format!("Failed to write service: {}", e)))?;
    fs::write(&timer_path, timer_content)
      .map_err(|e| AppError::message(format!("Failed to write timer: {}", e)))?;
    let _ = Command::new("systemctl")
      .args(&["--user", "daemon-reload"])
      .output();
    if config.enabled {
      let _ = Command::new("systemctl")
        .args(&["--user", "enable", "cleanux-cleaning.timer"])
        .output();
      let _ = Command::new("systemctl")
        .args(&["--user", "start", "cleanux-cleaning.timer"])
        .output();
    }
    Ok(())
  }

  fn remove_systemd_timer() -> Result<(), AppError> {
    use std::process::Command;
    let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
    let systemd_user_dir = format!("/home/{}/.config/systemd/user", user);
    let _ = Command::new("systemctl")
      .args(&["--user", "stop", "cleanux-cleaning.timer"])
      .output();
    let _ = Command::new("systemctl")
      .args(&["--user", "disable", "cleanux-cleaning.timer"])
      .output();
    let service_path = format!("{}/cleanux-cleaning.service", systemd_user_dir);
    let timer_path = format!("{}/cleanux-cleaning.timer", systemd_user_dir);
    let _ = fs::remove_file(service_path);
    let _ = fs::remove_file(timer_path);
    let _ = Command::new("systemctl")
      .args(&["--user", "daemon-reload"])
      .output();
    Ok(())
  }

  pub fn run_cleaning(cleaning_type: CleaningType) -> Result<ResponseModel, ResponseModel> {
    use crate::services::cleaner_service::CleanerService;
    let cleaner = CleanerService;
    match cleaning_type {
      CleaningType::Cache => cleaner.clearCache(),
      CleaningType::Trash => cleaner.clearTrash(),
      CleaningType::Logs => cleaner.clearAllLogs(),
      CleaningType::LargeFiles => cleaner.clearAllLargeFiles(),
      CleaningType::All => {
        let _ = cleaner.clearCache();
        let _ = cleaner.clearTrash();
        let _ = cleaner.clearAllLogs();
        cleaner.clearAllLargeFiles()
      }
    }
  }
}
