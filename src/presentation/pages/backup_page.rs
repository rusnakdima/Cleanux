//! Backup page — mirrors the template `data-page="backup"` section.
//!
//! Manages backup creation, restoration, and deletion.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackupEntry {
  pub id: String,
  pub name: String,
  pub date: String,
  pub size: u64,
  pub path: String,
  #[serde(rename = "backupType", default)]
  pub backup_type: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FilterTab {
  All,
  System,
  User,
  Scheduled,
}

impl FilterTab {
  fn label(&self) -> &'static str {
    match self {
      FilterTab::All => "All",
      FilterTab::System => "System",
      FilterTab::User => "User",
      FilterTab::Scheduled => "Scheduled",
    }
  }
}

fn format_bytes(bytes: u64) -> String {
  const KB: u64 = 1024;
  const MB: u64 = KB * 1024;
  const GB: u64 = MB * 1024;
  if bytes >= GB {
    format!("{:.1} GB", bytes as f64 / GB as f64)
  } else if bytes >= MB {
    format!("{:.1} MB", bytes as f64 / MB as f64)
  } else if bytes >= KB {
    format!("{:.1} KB", bytes as f64 / KB as f64)
  } else {
    format!("{} B", bytes)
  }
}

fn format_date(date: &str) -> String {
  // Try parsing ISO date and display in readable format
  if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date) {
    dt.format("%b %d, %Y · %H:%M").to_string()
  } else {
    date.to_string()
  }
}

#[derive(Clone, Props, PartialEq)]
struct BackupRowProps {
  backup: BackupEntry,
  on_restore: Callback<String>,
  on_delete: Callback<String>,
}

#[component]
fn BackupRow(props: BackupRowProps) -> Element {
  let backup_id = props.backup.id.clone();
  let backup_id_for_delete = props.backup.id.clone();
  let backup_name = props.backup.name.clone();
  let backup_date = props.backup.date.clone();
  let backup_type = props.backup.backup_type.clone();
  let backup_size = props.backup.size;
  rsx! {
      tr { class: "border-b border-zinc-100 dark:border-zinc-800 hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors",
          td { class: "px-4 py-3",
              div { class: "flex items-center gap-3",
                  div { class: "w-8 h-8 rounded-lg bg-blue-100 dark:bg-blue-900/30 flex items-center justify-center text-sm",
                      "💾"
                  }
                  div {
                      p { class: "font-medium text-sm", "{backup_name}" }
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400", "{format_date(&backup_date)}" }
                  }
              }
          }
          td { class: "px-4 py-3 text-sm text-zinc-600 dark:text-zinc-400",
              "{backup_type}"
          }
          td { class: "px-4 py-3 text-right",
              p { class: "font-medium text-sm", "{format_bytes(backup_size)}" }
          }
          td { class: "px-4 py-3",
              div { class: "flex justify-end gap-2",
                  button {
                      class: "px-3 py-1 rounded-lg border border-zinc-200 dark:border-zinc-700 hover:bg-blue-50 dark:hover:bg-blue-900/20 text-xs font-medium transition-colors text-blue-600 dark:text-blue-400",
                      onclick: move |_| props.on_restore.call(backup_id.clone()),
                      "Restore"
                  }
                  button {
                      class: "px-3 py-1 rounded-lg border border-zinc-200 dark:border-zinc-700 hover:bg-red-50 dark:hover:bg-red-900/20 text-xs font-medium transition-colors text-red-500",
                      onclick: move |_| props.on_delete.call(backup_id_for_delete.clone()),
                      "Delete"
                  }
              }
          }
      }
  }
}

#[component]
pub fn BackupPage(state: AppState) -> Element {
  let mut backups = use_signal(|| Vec::<BackupEntry>::new());
  let mut backup_dir = use_signal(|| String::new());
  let mut filter = use_signal(|| FilterTab::All);
  let mut is_creating = use_signal(|| false);
  let mut is_restoring = use_signal(|| String::new());
  let mut new_backup_name = use_signal(|| String::new());
  let mut status_msg = use_signal(|| String::new());
  let mut status_type = use_signal(|| "info".to_string());

  // Load backups and backup dir on mount
  use_effect(move || {
    // Load backup dir
    let result = bridge::invoke_app_command("storage_get_backup_dir", &serde_json::json!({}));
    if let Ok(val) = result {
      if let Some(data) = val.get("data") {
        if let Some(dir) = data.as_str() {
          backup_dir.set(dir.to_string());
        }
      }
    }
    // Load backups
    let result = bridge::invoke_app_command("backup_list", &serde_json::json!({}));
    if let Ok(val) = result {
      if let Some(data) = val.get("data") {
        if let Ok(list) = serde_json::from_value::<Vec<BackupEntry>>(data.clone()) {
          backups.set(list);
        }
      }
    }
  });

  let filtered_backups = {
    let all = backups.read();
    let f = *filter.read();
    if f == FilterTab::All {
      all.clone()
    } else {
      all
        .clone()
        .into_iter()
        .filter(|b| {
          let t = b.backup_type.to_lowercase();
          match f {
            FilterTab::System => t == "system",
            FilterTab::User => t == "manual" || t == "user",
            FilterTab::Scheduled => t == "scheduled",
            FilterTab::All => true,
          }
        })
        .collect()
    }
  };

  let total_size: u64 = filtered_backups.iter().map(|b| b.size).sum();

  rsx! {
      section { "data-page": "backup",
          class: "space-y-6",

          // Header
          div { class: "flex items-center justify-between",
              div {
                  h1 { class: "text-2xl font-bold", "Backups" }
                  p { class: "text-sm text-zinc-500 dark:text-zinc-400 mt-1",
                      "Manage system backups and restore points"
                  }
              }
              div { class: "flex items-center gap-3",
                  div { class: "text-right",
                      p { class: "text-sm font-medium", "{filtered_backups.len()} backups" }
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400",
                          "{format_bytes(total_size)} total"
                      }
                  }
                  button {
                      class: "px-4 py-2 rounded-xl bg-blue-500 hover:bg-blue-600 text-white font-semibold text-sm transition-colors shadow-sm",
                      onclick: move |_| {
                          if new_backup_name.read().trim().is_empty() {
                              status_msg.set("Please enter a backup name".to_string());
                              status_type.set("error".to_string());
                              return;
                          }
                          is_creating.set(true);
                          let name = new_backup_name.read().clone();
                          let result = bridge::invoke_app_command("backup_create", &serde_json::json!({ "name": &name }));
                          is_creating.set(false);
                          if result.is_ok() {
                              status_msg.set("Backup created successfully".to_string());
                              status_type.set("success".to_string());
                              new_backup_name.set(String::new());
                              // Refresh list
                              let result = bridge::invoke_app_command("backup_list", &serde_json::json!({}));
                              if let Ok(val) = result {
                                  if let Some(data) = val.get("data") {
                                      if let Ok(list) = serde_json::from_value::<Vec<BackupEntry>>(data.clone()) {
                                          backups.set(list);
                                      }
                                  }
                              }
                          } else {
                              status_msg.set("Failed to create backup".to_string());
                              status_type.set("error".to_string());
                          }
                      },
                      "Create Backup"
                  }
              }
          }

          // Backup name input
          div { class: "flex gap-3",
              input {
                  r#type: "text",
                  class: "flex-1 px-4 py-2.5 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-900 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500",
                  placeholder: "Backup name…",
                  value: "{new_backup_name}",
                  oninput: move |evt| new_backup_name.set(evt.value().clone()),
                  onkeydown: move |evt| {
                      if evt.key() == Key::Enter {
                          if new_backup_name.read().trim().is_empty() {
                              status_msg.set("Please enter a backup name".to_string());
                              status_type.set("error".to_string());
                              return;
                          }
                          is_creating.set(true);
                          let name = new_backup_name.read().clone();
                          let result = bridge::invoke_app_command("backup_create", &serde_json::json!({ "name": &name }));
                          is_creating.set(false);
                          if result.is_ok() {
                              status_msg.set("Backup created successfully".to_string());
                              status_type.set("success".to_string());
                              new_backup_name.set(String::new());
                              let result = bridge::invoke_app_command("backup_list", &serde_json::json!({}));
                              if let Ok(val) = result {
                                  if let Some(data) = val.get("data") {
                                      if let Ok(list) = serde_json::from_value::<Vec<BackupEntry>>(data.clone()) {
                                          backups.set(list);
                                      }
                                  }
                              }
                          } else {
                              status_msg.set("Failed to create backup".to_string());
                              status_type.set("error".to_string());
                          }
                      }
                  },
              }
          }

          // Status message
          if !status_msg.read().is_empty() {
              div {
                  class: format!(
                      "rounded-xl p-4 text-sm {}",
                      if *status_type.read() == "success" {
                          "bg-green-50 dark:bg-green-950/30 border border-green-200 dark:border-green-800 text-green-700 dark:text-green-300"
                      } else {
                          "bg-red-50 dark:bg-red-950/30 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-300"
                      }
                  ),
                  "{status_msg.read()}"
              }
          }

          // Backup location
          div { class: "bg-zinc-50 dark:bg-zinc-800/50 rounded-xl px-4 py-3 flex items-center gap-2",
              span { class: "text-xs text-zinc-500 dark:text-zinc-400", "📁" }
              span { class: "text-xs text-zinc-600 dark:text-zinc-300 font-mono", "{backup_dir.read()}" }
          }

          // Filter tabs
          div { class: "flex gap-1 bg-zinc-100 dark:bg-zinc-800/50 rounded-xl p-1",
              for f in [FilterTab::All, FilterTab::System, FilterTab::User, FilterTab::Scheduled] {
                  button {
                      class: format!(
                          "flex-1 px-4 py-2 rounded-lg text-xs font-medium transition-all {}",
                          if *filter.read() == f {
                              "bg-white dark:bg-zinc-700 shadow-sm text-zinc-900 dark:text-white"
                          } else {
                              "text-zinc-500 dark:text-zinc-400 hover:text-zinc-700 dark:hover:text-zinc-200"
                          }
                      ),
                      onclick: move |_| filter.set(f),
                      "{f.label()}"
                  }
              }
          }

          // Backup list
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 overflow-hidden",
              if filtered_backups.is_empty() {
                  div { class: "flex flex-col items-center justify-center py-16 text-center",
                      div { class: "text-4xl mb-3", "💾" }
                      p { class: "font-medium", "No backups yet" }
                      p { class: "text-sm text-zinc-500 dark:text-zinc-400 mt-1",
                          "Create your first backup to protect your settings"
                      }
                  }
              } else {
                  table { class: "w-full",
                      thead {
                          tr { class: "border-b border-zinc-100 dark:border-zinc-800",
                              th { class: "px-4 py-3 text-left text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Name / Date" }
                              th { class: "px-4 py-3 text-left text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Type" }
                              th { class: "px-4 py-3 text-right text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Size" }
                              th { class: "px-4 py-3 text-right text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Actions" }
                          }
                      }
                      tbody {
                          for backup in filtered_backups.iter() {
                              BackupRow {
                                  key: "{backup.id}",
                                  backup: backup.clone(),
                                  on_restore: move |id: String| {
                                      is_restoring.set(id.clone());
                                      let result = bridge::invoke_app_command("backup_restore", &serde_json::json!({ "id": &id }));
                                      is_restoring.set(String::new());
                                      if result.is_ok() {
                                          status_msg.set("Backup restored successfully".to_string());
                                          status_type.set("success".to_string());
                                      } else {
                                          status_msg.set("Failed to restore backup".to_string());
                                          status_type.set("error".to_string());
                                      }
                                  },
                                  on_delete: move |id: String| {
                                      let result = bridge::invoke_app_command("backup_delete", &serde_json::json!({ "id": &id }));
                                      if result.is_ok() {
                                          status_msg.set("Backup deleted".to_string());
                                          status_type.set("success".to_string());
                                          let current = (*backups.read()).clone();
                                          let filtered: Vec<_> = current.into_iter().filter(|b| b.id != id).collect();
                                          backups.set(filtered);
                                      } else {
                                          status_msg.set("Failed to delete backup".to_string());
                                          status_type.set("error".to_string());
                                      }
                                  },
                              }
                          }
                      }
                  }
              }
          }
      }
  }
}
