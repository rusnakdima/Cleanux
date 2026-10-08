//! Log Manager page — mirrors the template `data-page="log-manager"` section.
//!
//! Manages systemd journal, rotated logs, logrotate configs, and /var/log usage.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

fn format_bytes(bytes: u64) -> String {
  if bytes >= 1_073_741_824 {
    format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
  } else if bytes >= 1_048_576 {
    format!("{:.1} MB", bytes as f64 / 1_048_576.0)
  } else if bytes >= 1024 {
    format!("{:.1} KB", bytes as f64 / 1024.0)
  } else {
    format!("{} B", bytes)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
  Journal,
  Rotated,
  Logrotate,
  Largest,
}

impl Tab {
  fn label(&self) -> &'static str {
    match self {
      Tab::Journal => "System Journal",
      Tab::Rotated => "Rotated Logs",
      Tab::Logrotate => "Logrotate",
      Tab::Largest => "Largest Files",
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalSize {
  pub bytes: u64,
  pub human: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalUsage {
  pub total_entries: String,
  pub oldest_timestamp: Option<String>,
  pub newest_timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotatedLogUi {
  pub path: String,
  pub size: u64,
  pub size_human: String,
  pub modified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogrotateConfig {
  pub path: String,
  pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargestLogFile {
  pub path: String,
  pub size: u64,
  pub size_human: String,
  pub modified: String,
}

#[component]
pub fn LogManagerPage(state: AppState) -> Element {
  let mut active_tab = use_signal(|| Tab::Journal);
  let mut journal_size = use_signal(|| Option::<JournalSize>::None);
  let mut journal_usage = use_signal(|| Option::<JournalUsage>::None);
  let mut rotated_logs = use_signal(|| Vec::<RotatedLogUi>::new());
  let rotated_size = use_signal(|| 0u64);
  let mut largest_logs = use_signal(|| Vec::<LargestLogFile>::new());
  let mut var_log_usage = use_signal(|| 0u64);
  let mut logrotate_configs = use_signal(|| Vec::<LogrotateConfig>::new());
  let vacuum_size = use_signal(|| "500M".to_string());
  let vacuum_days = use_signal(|| 30u32);
  let mut is_loading = use_signal(|| false);
  let status_msg = use_signal(|| String::new());
  let selected_logs = use_signal(|| std::collections::HashSet::<String>::new());

  // Load initial data
  use_effect(move || {
    is_loading.set(true);
    // Load journal size
    match bridge::invoke_app_command("journal_get_size", &serde_json::json!({})) {
      Ok(resp) => {
        if let Ok(size) = serde_json::from_value::<JournalSize>(resp) {
          journal_size.set(Some(size));
        }
      }
      Err(_) => {}
    }
    // Load journal usage
    match bridge::invoke_app_command("journal_get_usage", &serde_json::json!({})) {
      Ok(resp) => {
        if let Ok(usage) = serde_json::from_value::<JournalUsage>(resp) {
          journal_usage.set(Some(usage));
        }
      }
      Err(_) => {}
    }
    // Load rotated logs
    match bridge::invoke_app_command("journal_get_rotated_logs", &serde_json::json!({})) {
      Ok(resp) => {
        if let Ok(logs) = serde_json::from_value::<Vec<RotatedLogUi>>(resp) {
          rotated_logs.set(logs);
        }
      }
      Err(_) => {}
    }
    // Load largest logs
    match bridge::invoke_app_command("journal_get_largest_files", &serde_json::json!({})) {
      Ok(resp) => {
        if let Ok(logs) = serde_json::from_value::<Vec<LargestLogFile>>(resp) {
          largest_logs.set(logs);
        }
      }
      Err(_) => {}
    }
    // Load var/log usage
    match bridge::invoke_app_command("journal_get_var_log_usage", &serde_json::json!({})) {
      Ok(resp) => {
        if let Ok(bytes) = serde_json::from_value::<u64>(resp) {
          var_log_usage.set(bytes);
        }
      }
      Err(_) => {}
    }
    // Load logrotate configs
    match bridge::invoke_app_command("journal_get_logrotate_configs", &serde_json::json!({})) {
      Ok(resp) => {
        if let Ok(configs) = serde_json::from_value::<Vec<LogrotateConfig>>(resp) {
          logrotate_configs.set(configs);
        }
      }
      Err(_) => {}
    }
    is_loading.set(false);
  });

  let journal_size_val = format_bytes(journal_size.read().as_ref().map(|s| s.bytes).unwrap_or(0));

  rsx! {
      section { "data-page": "log-manager",
          div { class: "max-w-5xl mx-auto space-y-6",
              div { class: "flex items-center justify-between",
                  div {
                      h2 { class: "text-2xl font-bold", "Log Manager" }
                      p { class: "text-sm text-zinc-500 dark:text-zinc-400", "Manage system logs and free up disk space" }
                  }
                  div { class: "flex gap-3",
                      SummaryStat { label: "Journal", value: journal_size_val }
                      SummaryStat { label: "/var/log", value: format_bytes(*var_log_usage.read()) }
                  }
              }

              // Tab Navigation
              div { class: "flex gap-1 bg-zinc-100 dark:bg-zinc-800 rounded-xl p-1",
                  for tab in [Tab::Journal, Tab::Rotated, Tab::Logrotate, Tab::Largest] {
                      button {
                          class: format!(
                              "flex-1 px-4 py-2 rounded-lg text-sm font-medium transition-colors {}",
                              if *active_tab.read() == tab {
                                  "bg-white dark:bg-zinc-700 shadow-sm text-cyan-600 dark:text-cyan-400"
                              } else {
                                  "text-zinc-500 dark:text-zinc-400 hover:text-zinc-700 dark:hover:text-zinc-200"
                              }
                          ),
                          onclick: move |_| active_tab.set(tab),
                          "{tab.label()}"
                      }
                  }
              }

              // Tab Content
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  match *active_tab.read() {
                      Tab::Journal => rsx! {
                          JournalTab {
                              journal_size: journal_size.clone(),
                              journal_usage: journal_usage.clone(),
                              vacuum_size: vacuum_size.clone(),
                              vacuum_days: vacuum_days.clone(),
                              is_loading: is_loading.clone(),
                              status_msg: status_msg.clone(),
                          }
                      },
                      Tab::Rotated => rsx! {
                          RotatedTab {
                              rotated_logs: rotated_logs.clone(),
                              rotated_size: rotated_size.clone(),
                              selected_logs: selected_logs.clone(),
                              is_loading: is_loading.clone(),
                              status_msg: status_msg.clone(),
                          }
                      },
                      Tab::Logrotate => rsx! {
                          LogrotateTab {
                              configs: logrotate_configs.clone(),
                          }
                      },
                      Tab::Largest => rsx! {
                          LargestTab {
                              logs: largest_logs.clone(),
                              is_loading: is_loading.clone(),
                              status_msg: status_msg.clone(),
                          }
                      },
                  }
              }

              if !status_msg.read().is_empty() {
                  div { class: "text-sm text-zinc-500 dark:text-zinc-400 bg-zinc-100 dark:bg-zinc-800 rounded-lg px-4 py-2",
                      "{status_msg.read()}"
                  }
              }
          }
      }
  }
}

#[derive(Debug, Clone, Props, PartialEq)]
struct SummaryStatProps {
  label: &'static str,
  value: String,
}

#[component]
fn SummaryStat(props: SummaryStatProps) -> Element {
  rsx! {
      div { class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 px-4 py-2",
          p { class: "text-xs text-zinc-500 uppercase tracking-wide", "{props.label}" }
          p { class: "text-sm font-bold", "{props.value}" }
      }
  }
}

#[derive(Debug, Clone, Props, PartialEq)]
struct JournalTabProps {
  journal_size: Signal<Option<JournalSize>>,
  journal_usage: Signal<Option<JournalUsage>>,
  vacuum_size: Signal<String>,
  vacuum_days: Signal<u32>,
  is_loading: Signal<bool>,
  status_msg: Signal<String>,
}

#[component]
fn JournalTab(props: JournalTabProps) -> Element {
  let mut vacuum_size = props.vacuum_size.clone();
  let mut vacuum_days = props.vacuum_days.clone();
  let mut is_loading = props.is_loading.clone();
  let mut status_msg = props.status_msg.clone();

  rsx! {
      div { class: "space-y-5",
          div {
              h3 { class: "text-base font-semibold mb-3", "Systemd Journal" }
              if let Some(ref usage) = *props.journal_usage.read() {
                  div { class: "grid grid-cols-2 md:grid-cols-4 gap-3 mb-4",
                      StatItem { label: "Total Entries", value: "{usage.total_entries}" }
                      StatItem { label: "Total Size", value: props.journal_size.read().as_ref().map(|s| s.human.clone()).unwrap_or_default() }
                      StatItem { label: "Oldest Entry", value: usage.oldest_timestamp.as_deref().unwrap_or("—") }
                      StatItem { label: "Newest Entry", value: usage.newest_timestamp.as_deref().unwrap_or("—") }
                  }
              }

              div { class: "space-y-3",
                  div { class: "flex items-end gap-3",
                      div { class: "flex-1",
                          label { class: "block text-sm font-medium mb-1", "Vacuum by Size" }
                          input {
                              class: "w-full px-3 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm",
                              r#type: "text",
                              placeholder: "500M",
                              value: "{vacuum_size.read()}",
                              oninput: move |e| vacuum_size.set(e.value()),
                          }
                      }
                      button {
                          class: "px-4 py-2 bg-cyan-500 hover:bg-cyan-600 text-white rounded-lg text-sm font-medium transition-colors",
                          onclick: move |_| {
                              let size = vacuum_size.read().clone();
                              is_loading.set(true);
                              let payload = serde_json::json!({ "size": size });
                              match bridge::invoke_app_command("journal_vacuum_by_size", &payload) {
                                  Ok(_) => status_msg.set("Journal vacuumed successfully".to_string()),
                                  Err(e) => status_msg.set(format!("Error: {}", e)),
                              }
                              is_loading.set(false);
                          },
                          "Vacuum by Size"
                      }
                  }
                  div { class: "flex items-end gap-3",
                      div { class: "w-32",
                          label { class: "block text-sm font-medium mb-1", "Days" }
                          input {
                              class: "w-full px-3 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm",
                              r#type: "number",
                              value: "{vacuum_days.read()}",
                              oninput: move |e| { if let Ok(v) = e.value().parse() { vacuum_days.set(v); } },
                          }
                      }
                      button {
                          class: "px-4 py-2 bg-cyan-500 hover:bg-cyan-600 text-white rounded-lg text-sm font-medium transition-colors",
                          onclick: move |_| {
                              let days = *vacuum_days.read();
                              is_loading.set(true);
                              let payload = serde_json::json!({ "days": days });
                              match bridge::invoke_app_command("journal_vacuum_by_days", &payload) {
                                  Ok(_) => status_msg.set(format!("Kept last {} days of journal", days)),
                                  Err(e) => status_msg.set(format!("Error: {}", e)),
                              }
                              is_loading.set(false);
                          },
                          "Vacuum by Days"
                      }
                  }
              }
          }
      }
  }
}

#[derive(Debug, Clone, Props, PartialEq)]
struct RotatedTabProps {
  rotated_logs: Signal<Vec<RotatedLogUi>>,
  rotated_size: Signal<u64>,
  selected_logs: Signal<std::collections::HashSet<String>>,
  is_loading: Signal<bool>,
  status_msg: Signal<String>,
}

#[component]
fn RotatedTab(props: RotatedTabProps) -> Element {
  let rotated_logs = props.rotated_logs.clone();
  let mut selected_logs = props.selected_logs.clone();
  let mut is_loading = props.is_loading.clone();
  let mut status_msg = props.status_msg.clone();

  let logs_list = rotated_logs.read().clone();
  let selected_list = selected_logs.read();
  let all_selected =
    !logs_list.is_empty() && logs_list.iter().all(|l| selected_list.contains(&l.path));

  rsx! {
      div { class: "space-y-4",
          div { class: "flex items-center justify-between",
              div {
                  h3 { class: "text-base font-semibold", "Rotated Logs" }
                  p { class: "text-sm text-zinc-500", "Old compressed logs that can be safely removed" }
              }
              button {
                  class: "px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg text-sm font-medium transition-colors",
                  onclick: move |_| {
                      is_loading.set(true);
                      match bridge::invoke_app_command("journal_clean_rotated_logs", &serde_json::json!({})) {
                          Ok(_) => status_msg.set("Rotated logs cleaned".to_string()),
                          Err(e) => status_msg.set(format!("Error: {}", e)),
                      }
                      is_loading.set(false);
                  },
                  "Clean All"
              }
          }
          if logs_list.is_empty() {
              div { class: "text-center py-8 text-zinc-500",
                  span { class: "material-symbols-rounded text-4xl mb-2 block", "check_circle" }
                  p { "No rotated logs found" }
              }
          } else {
              div { class: "flex items-center gap-2 mb-2",
                  input {
                      class: "w-4 h-4 rounded border-zinc-300",
                      r#type: "checkbox",
                      checked: all_selected,
                      onchange: move |_| {
                          if all_selected {
                              selected_logs.set(std::collections::HashSet::new());
                          } else {
                              let mut selected = std::collections::HashSet::new();
                              for log in logs_list.iter() { selected.insert(log.path.clone()); }
                              selected_logs.set(selected);
                          }
                      },
                  }
                  span { class: "text-sm text-zinc-500", "Select All" }
              }
              div { class: "space-y-1 max-h-80 overflow-y-auto",
                  for log in logs_list.iter() {
                      LogRow {
                          path: log.path.clone(),
                          size: log.size_human.clone(),
                          selected: selected_logs.read().contains(&log.path),
                          on_toggle: {
                              let path = log.path.clone();
                              let mut selected = selected_logs.clone();
                              move || {
                                  let mut s = selected.read().clone();
                                  if s.contains(&path) { s.remove(&path); } else { s.insert(path.clone()); }
                                  selected.set(s);
                              }
                          }
                      }
                  }
              }
          }
      }
  }
}

#[derive(Debug, Clone, Props, PartialEq)]
struct LogrotateTabProps {
  configs: Signal<Vec<LogrotateConfig>>,
}

#[component]
fn LogrotateTab(props: LogrotateTabProps) -> Element {
  let configs_list = props.configs.read().clone();
  rsx! {
      div { class: "space-y-4",
          div {
              h3 { class: "text-base font-semibold mb-3", "Logrotate Configurations" }
              p { class: "text-sm text-zinc-500 mb-4", "System logrotate configs found on this machine" }
          }
          if configs_list.is_empty() {
              div { class: "text-center py-8 text-zinc-500",
                  p { "No logrotate configurations found" }
              }
          } else {
              div { class: "space-y-2",
                  for config in configs_list.iter() {
                      div { class: "flex items-center justify-between px-4 py-3 bg-zinc-50 dark:bg-zinc-800 rounded-lg",
                          span { class: "font-mono text-sm truncate flex-1 mr-4", "{config.path}" }
                          span { class: "text-xs text-zinc-500", "{format_bytes(config.size)}" }
                      }
                  }
              }
          }
      }
  }
}

#[derive(Debug, Clone, Props, PartialEq)]
struct LargestTabProps {
  logs: Signal<Vec<LargestLogFile>>,
  is_loading: Signal<bool>,
  status_msg: Signal<String>,
}

#[component]
fn LargestTab(props: LargestTabProps) -> Element {
  let logs_list = props.logs.read().clone();
  rsx! {
      div { class: "space-y-4",
          div {
              h3 { class: "text-base font-semibold mb-3", "Largest Log Files" }
              p { class: "text-sm text-zinc-500", "Top 10 largest log files in /var/log" }
          }
          if logs_list.is_empty() {
              div { class: "text-center py-8 text-zinc-500",
                  p { "No log files found" }
              }
          } else {
              div { class: "space-y-2",
                  for log in logs_list.iter() {
                      div { class: "flex items-center justify-between px-4 py-3 bg-zinc-50 dark:bg-zinc-800 rounded-lg",
                          div { class: "flex-1 min-w-0 mr-4",
                              p { class: "font-mono text-sm truncate", "{log.path}" }
                              p { class: "text-xs text-zinc-500", "Modified: {log.modified}" }
                          }
                          span { class: "text-sm font-medium text-red-500 whitespace-nowrap", "{log.size_human}" }
                      }
                  }
              }
          }
      }
  }
}

#[derive(Debug, Clone, Props, PartialEq)]
struct StatItemProps {
  label: &'static str,
  value: String,
}

#[component]
fn StatItem(props: StatItemProps) -> Element {
  rsx! {
      div { class: "bg-zinc-50 dark:bg-zinc-800 rounded-lg px-3 py-2",
          p { class: "text-xs text-zinc-500 mb-0.5", "{props.label}" }
          p { class: "text-sm font-semibold", "{props.value}" }
      }
  }
}

#[derive(Debug, Clone, Props, PartialEq)]
struct LogRowProps {
  path: String,
  size: String,
  selected: bool,
  on_toggle: Callback<()>,
}

#[component]
fn LogRow(props: LogRowProps) -> Element {
  rsx! {
      div { class: "flex items-center gap-3 px-3 py-2 hover:bg-zinc-50 dark:hover:bg-zinc-800 rounded-lg",
          input {
              class: "w-4 h-4 rounded border-zinc-300",
              r#type: "checkbox",
              checked: props.selected,
              onchange: move |_| props.on_toggle.call(()),
          }
          div { class: "flex-1 min-w-0",
              p { class: "text-sm font-mono truncate", "{props.path}" }
          }
          span { class: "text-xs text-zinc-500 whitespace-nowrap", "{props.size}" }
      }
  }
}
