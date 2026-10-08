//! Disk Usage page — mirrors the template `data-page="disk-usage"` section.
//!
//! Visualizes disk space usage as a treemap and allows drill-down into directories.

use crate::app::AppState;
use crate::application::handlers::storage_handlers::DirectoryInfo;
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

fn basename(path: &str) -> String {
  std::path::Path::new(path)
    .file_name()
    .map(|s| s.to_string_lossy().to_string())
    .unwrap_or_else(|| path.to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirEntry {
  pub name: String,
  pub path: String,
  pub size: u64,
  pub size_human: String,
  pub is_dir: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DirectoryScanData {
  pub path: String,
  pub size: u64,
  pub files: u64,
  pub subdirs: u64,
}

/// Handler response wrapper for storage_scan_directory
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScanDirectoryResponse {
  pub status: String,
  pub message: String,
  pub data: Option<DirectoryInfo>,
}

#[component]
pub fn DiskUsagePage(state: AppState) -> Element {
  let home = dirs::home_dir()
    .unwrap_or_default()
    .to_string_lossy()
    .to_string();
  let mut scan_path = use_signal(|| home.clone());
  let mut is_scanning = use_signal(|| false);
  let mut current_dir = use_signal(|| Option::<DirectoryInfo>::None);
  let mut status_msg = use_signal(|| String::new());

  rsx! {
      section { "data-page": "disk-usage",
          div { class: "max-w-5xl mx-auto space-y-6",
              div { class: "flex items-center justify-between",
                  div {
                      h2 { class: "text-2xl font-bold", "Disk Usage" }
                      p { class: "text-sm text-zinc-500 dark:text-zinc-400", "Analyze disk space usage and find large directories" }
                  }
                  div { class: "flex gap-2",
                      button {
                          class: "px-4 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800",
                          onclick: move |_| {
                              scan_path.set(home.clone());
                          },
                          "Home"
                      }
                      button {
                          class: "px-4 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800",
                          onclick: move |_| {
                              scan_path.set("/".to_string());
                          },
                          "Root"
                      }
                  }
              }

              // Scan Controls
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "flex flex-col md:flex-row gap-4",
                      div { class: "flex-1",
                          label { class: "block text-sm font-medium mb-1.5", "Directory to Analyze" }
                          input {
                              class: "w-full px-3 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500",
                              r#type: "text",
                              value: "{scan_path.read()}",
                              oninput: move |e| scan_path.set(e.value().clone()),
                          }
                      }
                      button {
                          class: format!(
                              "mt-6 px-5 py-2 rounded-lg font-medium text-sm transition-colors {}",
                              if *is_scanning.read() {
                                  "bg-zinc-300 dark:bg-zinc-700 text-zinc-500 cursor-not-allowed"
                              } else {
                                  "bg-cyan-500 hover:bg-cyan-600 text-white"
                              }
                          ),
                          disabled: *is_scanning.read(),
                          onclick: move |_| {
                              is_scanning.set(true);
                              current_dir.set(None);
                              status_msg.set("Scanning...".to_string());
                              let path = scan_path.read().clone();
                              let payload = serde_json::json!({ "path": path });
                              match bridge::invoke_app_command("storage_scan_directory", &payload) {
                                  Ok(resp) => {
                                      if let Ok(result) = serde_json::from_value::<ScanDirectoryResponse>(resp) {
                                          if let Some(data) = result.data {
                                              current_dir.set(Some(data));
                                              status_msg.set("Scan complete".to_string());
                                          }
                                      }
                                  }
                                  Err(e) => { status_msg.set(format!("Error: {}", e)); }
                              }
                              is_scanning.set(false);
                          },
                          if *is_scanning.read() { "Scanning..." } else { "Analyze" }
                      }
                  }
              }

              // Status
              if !status_msg.read().is_empty() {
                  div { class: "text-sm text-zinc-500 dark:text-zinc-400", "{status_msg.read()}" }
              }

              // Current Path
              if let Some(ref dir) = *current_dir.read() {
                  div { class: "text-sm text-zinc-500 font-mono truncate", "{dir.path}" }
              }

              // Directory Info
              if let Some(ref dir) = *current_dir.read() {
                  div { class: "flex gap-4 flex-wrap mb-4",
                      div { class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 px-4 py-3",
                          p { class: "text-xs text-zinc-500 uppercase tracking-wide", "Total Size" }
                          p { class: "text-xl font-bold", "{format_bytes(dir.size)}" }
                      }
                      div { class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 px-4 py-3",
                          p { class: "text-xs text-zinc-500 uppercase tracking-wide", "Files" }
                          p { class: "text-xl font-bold", "{dir.files}" }
                      }
                      div { class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 px-4 py-3",
                          p { class: "text-xs text-zinc-500 uppercase tracking-wide", "Subdirs" }
                          p { class: "text-xl font-bold", "{dir.subdirs}" }
                      }
                  }
              }

              // Quick Actions
              if current_dir.read().is_some() {
                  div { class: "flex gap-3 flex-wrap",
                      button {
                          class: "px-4 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800 flex items-center gap-2",
                          onclick: move |_| {
                              state.navigate(crate::app::Page::Files);
                          },
                          span { class: "material-symbols-rounded text-lg", "description" }
                          "Large Files"
                      }
                      button {
                          class: "px-4 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800 flex items-center gap-2",
                          onclick: move |_| {
                              state.navigate(crate::app::Page::DuplicateFinder);
                          },
                          span { class: "material-symbols-rounded text-lg", "file_copy" }
                          "Duplicates"
                      }
                  }
              }

              // Empty State
              if current_dir.read().is_none() && !*is_scanning.read() {
                  div { class: "text-center py-16 text-zinc-400",
                      span { class: "material-symbols-rounded text-6xl mb-4 block", "storage" }
                      p { class: "text-lg font-medium", "No data scanned yet" }
                      p { class: "text-sm", "Enter a path and click Analyze to get started" }
                  }
              }
          }
      }
  }
}
