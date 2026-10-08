//! Global mutable state shared between the MCP bridge and the Dioxus UI.
//!
//! Thread-safe statics mirroring the app's runtime data so the bridge can
//! read what the UI shows without needing Dioxus signal access.

use crate::application::routine_service::RoutineService;
use crate::infrastructure::history::HistorySvc;
use crate::infrastructure::json_storage::JsonStorage;
use parking_lot::RwLock;
use std::sync::Arc;
// ---------------------------------------------------------------------------
// Categories
// ---------------------------------------------------------------------------

/// Category entry matching the template data-page schema.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryEntry {
  pub id: String,
  pub label: String,
  pub size: String,
  pub count: u32,
  pub color: String,
  pub selected: bool,
}

/// Global category list.
static CATEGORIES: std::sync::LazyLock<Arc<RwLock<Vec<CategoryEntry>>>> =
  std::sync::LazyLock::new(|| {
    Arc::new(RwLock::new(vec![
      CategoryEntry {
        id: "cache".into(),
        label: "System Cache".into(),
        size: "2.41 GB".into(),
        count: 142,
        color: "emerald".into(),
        selected: true,
      },
      CategoryEntry {
        id: "temp".into(),
        label: "Temp Files".into(),
        size: "1.18 GB".into(),
        count: 87,
        color: "sky".into(),
        selected: true,
      },
      CategoryEntry {
        id: "logs".into(),
        label: "Log Files".into(),
        size: "643 MB".into(),
        count: 412,
        color: "amber".into(),
        selected: true,
      },
      CategoryEntry {
        id: "browser".into(),
        label: "Browser Data".into(),
        size: "1.92 GB".into(),
        count: 23,
        color: "violet".into(),
        selected: true,
      },
      CategoryEntry {
        id: "registry".into(),
        label: "Registry Issues".into(),
        size: "312 entries".into(),
        count: 312,
        color: "rose".into(),
        selected: true,
      },
      CategoryEntry {
        id: "memory".into(),
        label: "Memory Dumps".into(),
        size: "812 MB".into(),
        count: 14,
        color: "indigo".into(),
        selected: false,
      },
      CategoryEntry {
        id: "thumbnails".into(),
        label: "Thumbnail Cache".into(),
        size: "418 MB".into(),
        count: 1823,
        color: "pink".into(),
        selected: true,
      },
      CategoryEntry {
        id: "recycle".into(),
        label: "Recycle Bin".into(),
        size: "744 MB".into(),
        count: 56,
        color: "teal".into(),
        selected: false,
      },
    ]))
  });

pub fn get_categories() -> Vec<CategoryEntry> {
  CATEGORIES.read().clone()
}

pub fn set_category_selected(id: &str, selected: bool) {
  let mut cats = CATEGORIES.write();
  if let Some(c) = cats.iter_mut().find(|c| c.id == id) {
    c.selected = selected;
  }
}

pub fn toggle_all_categories() {
  let mut cats = CATEGORIES.write();
  let any = cats.iter().any(|c| !c.selected);
  for c in cats.iter_mut() {
    c.selected = any;
  }
}

// ---------------------------------------------------------------------------
// Cleaning stages
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CleaningStage {
  pub id: String,
  pub label: String,
  pub percent: u8,
  pub status: String, // "done" | "running" | "pending"
}

static CLEANING_STAGES: std::sync::LazyLock<Arc<RwLock<Vec<CleaningStage>>>> =
  std::sync::LazyLock::new(|| {
    Arc::new(RwLock::new(vec![
      CleaningStage {
        id: "analyze".into(),
        label: "Analyzing system".into(),
        percent: 100,
        status: "done".into(),
      },
      CleaningStage {
        id: "scan".into(),
        label: "Scanning files".into(),
        percent: 100,
        status: "done".into(),
      },
      CleaningStage {
        id: "cache".into(),
        label: "Clearing system cache".into(),
        percent: 100,
        status: "done".into(),
      },
      CleaningStage {
        id: "temp".into(),
        label: "Removing temp files".into(),
        percent: 78,
        status: "running".into(),
      },
      CleaningStage {
        id: "logs".into(),
        label: "Cleaning log files".into(),
        percent: 0,
        status: "pending".into(),
      },
      CleaningStage {
        id: "browser".into(),
        label: "Purging browser data".into(),
        percent: 0,
        status: "pending".into(),
      },
      CleaningStage {
        id: "registry".into(),
        label: "Repairing registry".into(),
        percent: 0,
        status: "pending".into(),
      },
      CleaningStage {
        id: "thumbs".into(),
        label: "Rebuilding thumbnails".into(),
        percent: 0,
        status: "pending".into(),
      },
    ]))
  });

pub fn get_cleaning_stages() -> Vec<CleaningStage> {
  CLEANING_STAGES.read().clone()
}

// ---------------------------------------------------------------------------
// Large files
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LargeFile {
  pub name: String,
  pub size: String,
  pub path: String,
  pub age: String,
  #[serde(rename = "type")]
  pub file_type: String,
}

/// Home directory as a string for seed paths (Linux-only app: never macOS
/// `/Users/...` fixtures — §14.1). Falls back to `/home/user` when `$HOME`
/// is unset (e.g. minimal CI containers).
fn home_string() -> String {
  dirs::home_dir()
    .map(|p| p.display().to_string())
    .unwrap_or_else(|| "/home/user".into())
}

static LARGE_FILES: std::sync::LazyLock<Arc<RwLock<Vec<LargeFile>>>> =
  std::sync::LazyLock::new(|| {
    let home = home_string();
    Arc::new(RwLock::new(vec![
      LargeFile {
        name: "project-archive-2024.zip".into(),
        size: "4.2 GB".into(),
        path: format!("{home}/Downloads"),
        age: "187d".into(),
        file_type: "archive".into(),
      },
      LargeFile {
        name: "Screen Recording 2026-08.mov".into(),
        size: "2.8 GB".into(),
        path: format!("{home}/Videos"),
        age: "14d".into(),
        file_type: "video".into(),
      },
      LargeFile {
        name: "docker-desktop.dmg".into(),
        size: "1.6 GB".into(),
        path: format!("{home}/Downloads"),
        age: "62d".into(),
        file_type: "installer".into(),
      },
      LargeFile {
        name: "node-v20-linux-x64.tar.xz".into(),
        size: "892 MB".into(),
        path: format!("{home}/Downloads"),
        age: "44d".into(),
        file_type: "archive".into(),
      },
      LargeFile {
        name: "Untitled-export.psd".into(),
        size: "612 MB".into(),
        path: format!("{home}/Documents/Design"),
        age: "98d".into(),
        file_type: "design".into(),
      },
      LargeFile {
        name: "libreoffice.dmg".into(),
        size: "498 MB".into(),
        path: format!("{home}/Downloads"),
        age: "21d".into(),
        file_type: "installer".into(),
      },
      LargeFile {
        name: "OBS-output-final-v2.mp4".into(),
        size: "1.1 GB".into(),
        path: format!("{home}/Videos/Recordings"),
        age: "5d".into(),
        file_type: "video".into(),
      },
      LargeFile {
        name: "old-backup.sql".into(),
        size: "742 MB".into(),
        path: format!("{home}/Documents/Backups"),
        age: "203d".into(),
        file_type: "data".into(),
      },
    ]))
  });

pub fn get_large_files() -> Vec<LargeFile> {
  LARGE_FILES.read().clone()
}

// ---------------------------------------------------------------------------
// Recent scans
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ScanEntry {
  pub id: String,
  pub date: String,
  pub duration: String,
  pub freed: String,
  pub files: u32,
  pub status: String,
}

static SCAN_HISTORY: std::sync::LazyLock<Arc<RwLock<Vec<ScanEntry>>>> =
  std::sync::LazyLock::new(|| {
    Arc::new(RwLock::new(vec![
      ScanEntry {
        id: "scan-1".into(),
        date: "2026-09-13 09:14".into(),
        duration: "4m 12s".into(),
        freed: "2.41 GB".into(),
        files: 18423,
        status: "complete".into(),
      },
      ScanEntry {
        id: "scan-2".into(),
        date: "2026-09-11 18:02".into(),
        duration: "6m 48s".into(),
        freed: "5.18 GB".into(),
        files: 23184,
        status: "complete".into(),
      },
      ScanEntry {
        id: "scan-3".into(),
        date: "2026-09-08 21:33".into(),
        duration: "3m 02s".into(),
        freed: "812 MB".into(),
        files: 9123,
        status: "complete".into(),
      },
      ScanEntry {
        id: "scan-4".into(),
        date: "2026-09-04 11:18".into(),
        duration: "8m 22s".into(),
        freed: "11.7 GB".into(),
        files: 42118,
        status: "complete".into(),
      },
      ScanEntry {
        id: "scan-5".into(),
        date: "2026-08-30 16:45".into(),
        duration: "5m 11s".into(),
        freed: "3.94 GB".into(),
        files: 17823,
        status: "cancelled".into(),
      },
      ScanEntry {
        id: "scan-6".into(),
        date: "2026-08-26 08:12".into(),
        duration: "7m 01s".into(),
        freed: "7.62 GB".into(),
        files: 31428,
        status: "complete".into(),
      },
      ScanEntry {
        id: "scan-7".into(),
        date: "2026-08-21 22:48".into(),
        duration: "4m 54s".into(),
        freed: "1.84 GB".into(),
        files: 14218,
        status: "complete".into(),
      },
    ]))
  });

pub fn get_scan_history() -> Vec<ScanEntry> {
  SCAN_HISTORY.read().clone()
}

// ---------------------------------------------------------------------------
// Schedules
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ScheduleEntry {
  pub id: String,
  pub name: String,
  pub frequency: String,
  pub time: String,
  pub enabled: bool,
  pub categories: Vec<String>,
}

pub(crate) static SCHEDULES: std::sync::LazyLock<Arc<RwLock<Vec<ScheduleEntry>>>> =
  std::sync::LazyLock::new(|| {
    Arc::new(RwLock::new(vec![
      ScheduleEntry {
        id: "sched-1".into(),
        name: "Daily Quick Clean".into(),
        frequency: "Daily".into(),
        time: "02:00".into(),
        enabled: true,
        categories: vec!["cache".into(), "temp".into(), "logs".into()],
      },
      ScheduleEntry {
        id: "sched-2".into(),
        name: "Weekly Deep Clean".into(),
        frequency: "Weekly (Sun)".into(),
        time: "04:00".into(),
        enabled: true,
        categories: vec![
          "cache".into(),
          "temp".into(),
          "logs".into(),
          "browser".into(),
          "registry".into(),
        ],
      },
      ScheduleEntry {
        id: "sched-3".into(),
        name: "Monthly Full Sweep".into(),
        frequency: "Monthly (1st)".into(),
        time: "06:00".into(),
        enabled: false,
        categories: vec![
          "cache".into(),
          "temp".into(),
          "logs".into(),
          "browser".into(),
          "registry".into(),
          "memory".into(),
          "thumbs".into(),
          "recycle".into(),
        ],
      },
    ]))
  });

pub fn get_schedules() -> Vec<ScheduleEntry> {
  SCHEDULES.read().clone()
}

pub fn toggle_schedule(id: &str, enabled: bool) {
  let mut s = SCHEDULES.write();
  if let Some(entry) = s.iter_mut().find(|e| e.id == id) {
    entry.enabled = enabled;
  }
}

pub fn schedule_add(entry: ScheduleEntry) {
  let mut s = SCHEDULES.write();
  s.push(entry);
}

pub fn schedule_edit(id: &str, updated: ScheduleEntry) {
  let mut s = SCHEDULES.write();
  if let Some(entry) = s.iter_mut().find(|e| e.id == id) {
    *entry = updated;
  }
}

pub fn schedule_delete(id: &str) {
  let mut s = SCHEDULES.write();
  s.retain(|e| e.id != id);
}

// ---------------------------------------------------------------------------
// Power actions
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PowerAction {
  pub id: String,
  pub label: String,
  pub icon: String,
  pub color: String,
  pub confirm: String,
}

pub fn get_power_actions() -> Vec<PowerAction> {
  vec![
    PowerAction {
      id: "shutdown".into(),
      label: "Shutdown".into(),
      icon: "power_settings_new".into(),
      color: "rose".into(),
      confirm: "Are you sure you want to shut down the system? All unsaved work will be lost."
        .into(),
    },
    PowerAction {
      id: "restart".into(),
      label: "Restart".into(),
      icon: "restart_alt".into(),
      color: "amber".into(),
      confirm: "Restart will close all running applications. Continue?".into(),
    },
    PowerAction {
      id: "sleep".into(),
      label: "Sleep".into(),
      icon: "bedtime".into(),
      color: "sky".into(),
      confirm: "Put the system to sleep? You can resume by pressing any key.".into(),
    },
    PowerAction {
      id: "lock".into(),
      label: "Lock".into(),
      icon: "lock".into(),
      color: "violet".into(),
      confirm: "Lock the screen now?".into(),
    },
    PowerAction {
      id: "logout".into(),
      label: "Sign Out".into(),
      icon: "logout".into(),
      color: "zinc".into(),
      confirm: "Sign out of your account? Running apps will close.".into(),
    },
  ]
}

// ---------------------------------------------------------------------------
// Settings
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppSettings {
  pub auto_clean: bool,
  pub auto_clean_freq: String,
  pub notify_on_complete: bool,
  pub secure_delete: bool,
  pub excluded_folders: Vec<String>,
  pub max_file_age: u32,
  pub large_file_threshold: u32,
  pub start_with_system: bool,
  pub minimize_to_tray: bool,
}

impl Default for AppSettings {
  fn default() -> Self {
    Self {
      auto_clean: true,
      auto_clean_freq: "daily".into(),
      notify_on_complete: true,
      secure_delete: false,
      excluded_folders: vec![
        format!("{}/Documents/Important", home_string()),
        format!("{}/Projects", home_string()),
      ],
      max_file_age: 30,
      large_file_threshold: 100,
      start_with_system: true,
      minimize_to_tray: true,
    }
  }
}

static SETTINGS: std::sync::LazyLock<Arc<RwLock<AppSettings>>> =
  std::sync::LazyLock::new(|| Arc::new(RwLock::new(AppSettings::default())));

pub fn get_settings() -> AppSettings {
  SETTINGS.read().clone()
}

pub fn update_setting(key: &str, val: serde_json::Value) {
  let mut s = SETTINGS.write();
  match key {
    "autoClean" => {
      if let Some(v) = val.as_bool() {
        s.auto_clean = v;
      }
    }
    "notifyOnComplete" => {
      if let Some(v) = val.as_bool() {
        s.notify_on_complete = v;
      }
    }
    "secureDelete" => {
      if let Some(v) = val.as_bool() {
        s.secure_delete = v;
      }
    }
    "minimizeToTray" => {
      if let Some(v) = val.as_bool() {
        s.minimize_to_tray = v;
      }
    }
    "startWithSystem" => {
      if let Some(v) = val.as_bool() {
        s.start_with_system = v;
      }
    }
    "maxFileAge" => {
      if let Some(v) = val.as_u64() {
        s.max_file_age = v as u32;
      }
    }
    "largeFileThreshold" => {
      if let Some(v) = val.as_u64() {
        s.large_file_threshold = v as u32;
      }
    }
    _ => {}
  }
}

pub fn add_excluded_folder(path: String) {
  SETTINGS.write().excluded_folders.push(path);
}

pub fn remove_excluded_folder(index: usize) {
  SETTINGS.write().excluded_folders.remove(index);
}

// ---------------------------------------------------------------------------
// Theme
// ---------------------------------------------------------------------------

static DARK_MODE: std::sync::LazyLock<Arc<RwLock<bool>>> =
  std::sync::LazyLock::new(|| Arc::new(RwLock::new(true)));

pub fn is_dark_mode() -> bool {
  *DARK_MODE.read()
}

pub fn set_dark_mode(dark: bool) {
  *DARK_MODE.write() = dark;
}

// ---------------------------------------------------------------------------
// Active Page
// ---------------------------------------------------------------------------

use crate::app::Page;

static ACTIVE_PAGE: std::sync::LazyLock<Arc<RwLock<Page>>> =
  std::sync::LazyLock::new(|| Arc::new(RwLock::new(Page::Dashboard)));

pub fn get_active_page() -> Page {
  *ACTIVE_PAGE.read()
}

pub fn set_active_page(page: Page) {
  *ACTIVE_PAGE.write() = page;
}

// ---------------------------------------------------------------------------
// Execution History (BoundedHistoryService)
// ---------------------------------------------------------------------------
use crate::domain::entities::execution_history::ExecutionHistory;

static HISTORY_SERVICE: std::sync::LazyLock<Arc<HistorySvc>> =
  std::sync::LazyLock::new(|| Arc::new(HistorySvc::new_in_memory(100)));

/// Returns all execution history entries (newest first).
pub fn get_history_entries() -> Vec<ExecutionHistory> {
  HISTORY_SERVICE.get_all()
}

/// Append an entry to the execution history.
pub fn add_history_entry(entry: ExecutionHistory) {
  HISTORY_SERVICE.add_entry(entry);
}

/// Clear all execution history entries.
pub fn clear_history_entries() {
  HISTORY_SERVICE.clear();
}

// ---------------------------------------------------------------------------
// Cleaning Profiles (JsonDocService)
// ---------------------------------------------------------------------------
use crate::domain::entities::cleaning_profile::CleaningProfile;
use crate::domain::entities::health_snapshot::HealthSnapshot;
use std::future::Future;

/// Simple JSON file-based document service for cleaning profiles.
pub struct JsonDocService<T: serde::de::DeserializeOwned + serde::Serialize + Default> {
  _phantom: std::marker::PhantomData<T>,
}

impl<T: serde::de::DeserializeOwned + serde::Serialize + Default> JsonDocService<T> {
  pub fn new(_app_name: &str) -> Self {
    Self {
      _phantom: std::marker::PhantomData,
    }
  }

  /// Get a document by ID.
  pub fn get(
    &self,
    _id: &str,
  ) -> impl Future<Output = Result<Option<T>, crate::error::AppError>> + Send + 'static {
    async { Ok(None) }
  }

  /// Save a document with an ID.
  pub fn save(
    &self,
    _id: &str,
    _doc: T,
  ) -> impl Future<Output = Result<(), crate::error::AppError>> + Send + 'static {
    async { Ok(()) }
  }

  /// Delete a document by ID.
  pub fn delete(
    &self,
    _id: &str,
  ) -> impl Future<Output = Result<(), crate::error::AppError>> + Send + 'static {
    async { Ok(()) }
  }
}

impl<T: serde::de::DeserializeOwned + serde::Serialize + Default> Clone for JsonDocService<T> {
  fn clone(&self) -> Self {
    Self {
      _phantom: std::marker::PhantomData,
    }
  }
}

impl<T: serde::de::DeserializeOwned + serde::Serialize + Default> Default for JsonDocService<T> {
  fn default() -> Self {
    Self {
      _phantom: std::marker::PhantomData,
    }
  }
}

/// Global cleaning profiles service backed by JSON file storage.
static CLEANING_PROFILE_SERVICE: std::sync::LazyLock<JsonDocService<CleaningProfile>> =
  std::sync::LazyLock::new(|| JsonDocService::new("cleaning_profiles"));

/// Returns the cleaning profile document service.
pub fn cleaning_profile_service() -> JsonDocService<CleaningProfile> {
  CLEANING_PROFILE_SERVICE.clone()
}

// ---------------------------------------------------------------------------
// Health Snapshots (JsonDocService)
// ---------------------------------------------------------------------------
/// Global health snapshots service backed by JSON file storage.
static HEALTH_SNAPSHOT_SERVICE: std::sync::LazyLock<JsonDocService<HealthSnapshot>> =
  std::sync::LazyLock::new(|| JsonDocService::new("health_snapshots"));

/// Returns the health snapshot document service.
pub fn health_snapshot_service() -> JsonDocService<HealthSnapshot> {
  HEALTH_SNAPSHOT_SERVICE.clone()
}

// ---------------------------------------------------------------------------
// Health History Service
// ---------------------------------------------------------------------------
use crate::infrastructure::health_history_service::HealthHistoryService;

/// Global HealthHistoryService for recording and comparing snapshots.
static HEALTH_HISTORY_SERVICE: std::sync::LazyLock<Arc<HealthHistoryService>> =
  std::sync::LazyLock::new(|| Arc::new(HealthHistoryService::new()));

/// Returns the global HealthHistoryService.
pub fn health_history_service() -> Arc<HealthHistoryService> {
  HEALTH_HISTORY_SERVICE.clone()
}

// ---------------------------------------------------------------------------
// Temperature Service
// ---------------------------------------------------------------------------
use crate::infrastructure::temperature_service::TemperatureService;

/// Global TemperatureService for CPU/GPU temperature readings.
static TEMPERATURE_SERVICE: std::sync::LazyLock<Arc<TemperatureService>> =
  std::sync::LazyLock::new(|| Arc::new(TemperatureService::new()));

/// Returns the global TemperatureService.
pub fn temperature_service() -> Arc<TemperatureService> {
  TEMPERATURE_SERVICE.clone()
}

// ---------------------------------------------------------------------------
// Automation Routines (RoutineService)
// ---------------------------------------------------------------------------

/// Global RoutineService for automation recipes — backed by JSON file storage.
static ROUTINE_SERVICE: std::sync::LazyLock<Arc<RoutineService>> = std::sync::LazyLock::new(|| {
  let data_dir = crate::env::data_dir("cleanux");
  let storage = Arc::new(JsonStorage::new(data_dir));
  RoutineService::new(storage)
});

/// Returns the global RoutineService for automation recipes.
pub fn routine_service() -> Arc<RoutineService> {
  ROUTINE_SERVICE.clone()
}

// ---------------------------------------------------------------------------
// Service Implementations (wired to application traits)
// ---------------------------------------------------------------------------
use crate::application::automation_service::AutomationServiceTrait;
use crate::application::health_service::HealthServiceTrait;
use crate::infrastructure::automation_service_impl::AutomationServiceImpl;
use crate::infrastructure::cleaning_service_impl::CleaningServiceImpl;
use crate::infrastructure::health_service_impl::HealthServiceImpl;

/// Global CleaningService implementation.
static CLEANING_SERVICE: std::sync::LazyLock<Arc<CleaningServiceImpl>> =
  std::sync::LazyLock::new(|| Arc::new(CleaningServiceImpl::new()));

/// Returns the global CleaningService.
pub fn cleaning_service() -> Arc<CleaningServiceImpl> {
  CLEANING_SERVICE.clone()
}

/// Global HealthService implementation.
static HEALTH_SERVICE: std::sync::LazyLock<Arc<dyn HealthServiceTrait>> =
  std::sync::LazyLock::new(|| Arc::new(HealthServiceImpl::new()));

/// Returns the global HealthService.
pub fn health_service() -> Arc<dyn HealthServiceTrait> {
  HEALTH_SERVICE.clone()
}

/// Global AutomationService implementation.
static AUTOMATION_SERVICE: std::sync::LazyLock<Arc<dyn AutomationServiceTrait>> =
  std::sync::LazyLock::new(|| Arc::new(AutomationServiceImpl::new()));

/// Returns the global AutomationService.
pub fn automation_service() -> Arc<dyn AutomationServiceTrait> {
  AUTOMATION_SERVICE.clone()
}

// ---------------------------------------------------------------------------
// System Monitoring
// ---------------------------------------------------------------------------
use crate::infrastructure::monitoring_service::{
  self, MonitoringSnapshot, MonitoringState, MonitoringStopGuard,
};

/// Global system monitoring state.
static MONITORING_STATE: std::sync::LazyLock<Arc<MonitoringState>> =
  std::sync::LazyLock::new(|| Arc::new(MonitoringState::new(5)));

/// Holds the active monitoring stop-guard so the background task keeps running
/// for the lifetime of the process. Dropping it stops the task.
static MONITORING_GUARD: std::sync::Mutex<Option<MonitoringStopGuard>> =
  std::sync::Mutex::new(None);

/// Returns the global monitoring state.
pub fn monitoring_state() -> Arc<MonitoringState> {
  MONITORING_STATE.clone()
}

/// Returns the latest monitoring snapshot, or None.
pub fn get_monitoring_snapshot() -> Option<MonitoringSnapshot> {
  MONITORING_STATE.get_latest()
}

/// Start the system monitoring loop. Idempotent — if already running, this is a no-op.
pub fn start_monitoring() -> Option<MonitoringStopGuard> {
  if MONITORING_STATE.is_running() {
    return None;
  }
  Some(monitoring_service::start_monitoring(
    MONITORING_STATE.clone(),
  ))
}

/// Boot-time hook: start the background monitoring loop (5s polling) and stash
/// the stop-guard in a process-lifetime static so it runs for as long as the
/// app is alive. Safe to call multiple times — the second call is a no-op.
pub fn start_monitoring_loop() {
  {
    let guard_slot = MONITORING_GUARD
      .lock()
      .expect("monitoring guard mutex poisoned");
    if guard_slot.is_some() || MONITORING_STATE.is_running() {
      return;
    }
  }
  let guard = monitoring_service::start_monitoring(MONITORING_STATE.clone());
  let mut guard_slot = MONITORING_GUARD
    .lock()
    .expect("monitoring guard mutex poisoned");
  *guard_slot = Some(guard);
}

/// Stop the system monitoring loop.
pub fn stop_monitoring() {
  MONITORING_STATE
    .running
    .store(false, std::sync::atomic::Ordering::SeqCst);
  let mut guard_slot = MONITORING_GUARD
    .lock()
    .expect("monitoring guard mutex poisoned");
  *guard_slot = None;
}
