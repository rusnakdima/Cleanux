//! Duplicate Finder page — mirrors the template `data-page="duplicate-finder"` section.
//!
//! Finds duplicate files by SHA-256 hash, groups them, and allows bulk deletion.

use crate::app::AppState;
use crate::application::handlers::storage_handlers::DuplicateGroup;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Local duplicate group UI model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DuplicateGroupUi {
  pub hash: String,
  pub paths: Vec<String>,
  pub size: u64,
}

impl From<DuplicateGroup> for DuplicateGroupUi {
  fn from(g: DuplicateGroup) -> Self {
    DuplicateGroupUi {
      hash: g.hash,
      paths: g.paths,
      size: g.size,
    }
  }
}

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

#[component]
pub fn DuplicateFinderPage(state: AppState) -> Element {
  let mut scan_path = use_signal(|| {
    dirs::home_dir()
      .unwrap_or_default()
      .to_string_lossy()
      .to_string()
  });
  let mut is_scanning = use_signal(|| false);
  let mut scan_result = use_signal(|| Option::<Vec<DuplicateGroupUi>>::None);
  let mut selected_groups =
    use_signal(|| std::collections::HashMap::<String, std::collections::HashSet<usize>>::new());
  let mut show_confirm = use_signal(|| false);
  let mut status_msg = use_signal(|| String::new());

  let total_wasted: u64 = scan_result
    .read()
    .as_ref()
    .map(|groups| {
      groups
        .iter()
        .map(|g| g.size * (g.paths.len().saturating_sub(1)) as u64)
        .sum::<u64>()
    })
    .unwrap_or(0);
  let total_groups = scan_result.read().as_ref().map(|g| g.len()).unwrap_or(0);
  let total_files: usize = scan_result
    .read()
    .as_ref()
    .map(|g| g.iter().map(|gr| gr.paths.len()).sum::<usize>())
    .unwrap_or(0);
  let selected_count: usize = selected_groups.read().values().map(|s| s.len()).sum();

  rsx! {
      section { "data-page": "duplicate-finder",
          div { class: "max-w-5xl mx-auto space-y-6",
              div { class: "flex items-center justify-between",
                  div {
                      h2 { class: "text-2xl font-bold", "Duplicate Finder" }
                      p { class: "text-sm text-zinc-500 dark:text-zinc-400", "Find and remove duplicate files to free up space" }
                  }
              }

              // Scan Controls Card
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "flex flex-col md:flex-row gap-4",
                      div { class: "flex-1",
                          label { class: "block text-sm font-medium mb-1.5", "Directory to Scan" }
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
                              scan_result.set(None);
                              status_msg.set("Scanning...".to_string());
                              let path = scan_path.read().clone();
                              let payload = serde_json::json!({ "path": path });
                              match bridge::invoke_app_command("storage_find_duplicates", &payload) {
                                  Ok(resp) => {
                                      if let Some(data) = resp.get("data").and_then(|v| v.as_array().cloned()) {
                                          if let Ok(groups) = serde_json::from_value::<Vec<DuplicateGroup>>(serde_json::Value::Array(data)) {
                                              let ui_groups: Vec<DuplicateGroupUi> = groups.into_iter().map(Into::into).collect();
                                              let count = ui_groups.len();
                                              scan_result.set(Some(ui_groups));
                                              status_msg.set(format!("Found {} duplicate groups", count));
                                          }
                                      }
                                  }
                                  Err(e) => { status_msg.set(format!("Error: {}", e)); }
                              }
                              is_scanning.set(false);
                          },
                          if *is_scanning.read() {
                              span { class: "flex items-center gap-2", "Scanning..." }
                          } else {
                              span { class: "flex items-center gap-2", "Scan" }
                          }
                      }
                  }
              }

              // Status
              if !status_msg.read().is_empty() {
                  div { class: "text-sm text-zinc-500 dark:text-zinc-400", "{status_msg.read()}" }
              }

              // Summary
              if let Some(ref groups) = *scan_result.read() {
                  div { class: "flex gap-4 flex-wrap",
                      div { class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 px-4 py-3",
                          p { class: "text-xs text-zinc-500 uppercase tracking-wide", "Total Wasted" }
                          p { class: "text-xl font-bold text-red-500", "{format_bytes(total_wasted)}" }
                      }
                      div { class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 px-4 py-3",
                          p { class: "text-xs text-zinc-500 uppercase tracking-wide", "Duplicate Groups" }
                          p { class: "text-xl font-bold", "{total_groups}" }
                      }
                      div { class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 px-4 py-3",
                          p { class: "text-xs text-zinc-500 uppercase tracking-wide", "Total Files" }
                          p { class: "text-xl font-bold", "{total_files}" }
                      }
                  }

                  // Group List
                  div { class: "space-y-3",
                      for group in groups {
                          DuplicateGroupCard {
                              group: group.clone(),
                              selected: selected_groups.read().get(&group.hash).cloned().unwrap_or_default(),
                              on_toggle: move |(hash, idx): (String, usize)| {
                                  let mut map = selected_groups.read().clone();
                                  let entry = map.entry(hash).or_insert_with(std::collections::HashSet::new);
                                  if entry.contains(&idx) { entry.remove(&idx); } else { entry.insert(idx); }
                                  selected_groups.set(map);
                              },
                          }
                      }
                  }

                  // Delete Action Bar
                  div { class: "sticky bottom-20 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-xl p-4 flex items-center justify-between",
                      div { class: "text-sm text-zinc-500",
                          "{selected_count} files selected"
                      }
                      button {
                          class: "px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg text-sm font-medium transition-colors",
                          onclick: move |_| show_confirm.set(true),
                          "Delete Selected"
                      }
                  }

                  // Confirmation Dialog
                  if *show_confirm.read() {
                      ConfirmDialog {
                          selected_count,
                          on_confirm: move |_| {
                              show_confirm.set(false);
                              status_msg.set("Delete not yet wired — implement storage_delete_files handler".to_string());
                          },
                          on_cancel: move |_| show_confirm.set(false),
                      }
                  }
              }
          }
      }
  }
}

#[derive(Debug, Clone, Props, PartialEq)]
struct DuplicateGroupCardProps {
  group: DuplicateGroupUi,
  selected: std::collections::HashSet<usize>,
  on_toggle: Callback<(String, usize)>,
}

#[component]
fn DuplicateGroupCard(props: DuplicateGroupCardProps) -> Element {
  let mut expanded = use_signal(|| false);
  let wasted = props.group.size * (props.group.paths.len().saturating_sub(1)) as u64;
  let hash = props.group.hash.clone();

  rsx! {
      div { class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 overflow-hidden",
          div { class: "px-4 py-3 flex items-center justify-between cursor-pointer",
              onclick: move |_| expanded.set(!expanded()),
              div { class: "flex items-center gap-3",
                  span { class: "material-symbols-rounded text-zinc-400", "file_copy" }
                  div {
                      div { class: "text-sm font-medium",
                          "{props.group.paths.len()} files · {format_bytes(props.group.size)} each"
                      }
                      div { class: "text-xs text-zinc-500",
                          "Hash: {hash} · Wasted: {format_bytes(wasted)}"
                      }
                  }
              }
              div { class: "flex items-center gap-2",
                  button {
                      class: "text-xs text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300",
                      onclick: move |e| { e.stop_propagation(); expanded.set(!expanded()); },
                      if expanded() { "Collapse" } else { "Expand" }
                  }
                  span { class: "material-symbols-rounded text-zinc-400",
                      if expanded() { "expand_less" } else { "expand_more" }
                  }
              }
          }
          if *expanded.read() {
              div { class: "border-t border-zinc-200 dark:border-zinc-800 divide-y divide-zinc-100 dark:divide-zinc-800",
                  for (idx, file_path) in props.group.paths.iter().enumerate() {
                      DuplicateFileRow {
                          idx,
                          file_path: file_path.clone(),
                          hash: hash.clone(),
                          selected: props.selected.contains(&idx),
                          on_toggle: props.on_toggle.clone(),
                      }
                  }
              }
          }
      }
  }
}

#[derive(Debug, Clone, Props, PartialEq)]
struct DuplicateFileRowProps {
  idx: usize,
  file_path: String,
  hash: String,
  selected: bool,
  on_toggle: Callback<(String, usize)>,
}

#[component]
fn DuplicateFileRow(props: DuplicateFileRowProps) -> Element {
  let hash = props.hash.clone();
  let idx = props.idx;
  rsx! {
      div { class: "px-4 py-2.5 flex items-center gap-3",
          input {
              class: "w-4 h-4 rounded border-zinc-300",
              r#type: "checkbox",
              checked: props.selected,
              onchange: move |_| props.on_toggle.call((hash.clone(), idx)),
          }
          div { class: "flex-1 min-w-0",
              p { class: "text-sm font-mono truncate", "{props.file_path}" }
          }
      }
  }
}

#[derive(Debug, Clone, Props, PartialEq)]
struct ConfirmDialogProps {
  selected_count: usize,
  on_confirm: Callback<()>,
  on_cancel: Callback<()>,
}

#[component]
fn ConfirmDialog(props: ConfirmDialogProps) -> Element {
  rsx! {
      div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-6 w-full max-w-sm mx-4",
              div { class: "flex items-center gap-3 mb-4",
                  span { class: "material-symbols-rounded text-3xl text-red-500", "warning" }
                  div {
                      h3 { class: "text-lg font-bold", "Confirm Deletion" }
                      p { class: "text-sm text-zinc-500", "Delete {props.selected_count} duplicate files?" }
                  }
              }
              p { class: "text-sm text-zinc-600 dark:text-zinc-400 mb-5",
                  "This action cannot be undone. Deleted files will be moved to trash."
              }
              div { class: "flex gap-3 justify-end",
                  button {
                      class: "px-4 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800",
                      onclick: move |_| props.on_cancel.call(()),
                      "Cancel"
                  }
                  button {
                      class: "px-4 py-2 rounded-lg bg-red-500 hover:bg-red-600 text-white text-sm font-medium",
                      onclick: move |_| props.on_confirm.call(()),
                      "Delete"
                  }
              }
          }
      }
  }
}
