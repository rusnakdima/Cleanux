//! System Repair page — mirrors the template `data-page="system-repair"` section.
//!
//! Provides repair functionality for:
//! - Broken symlinks detection and removal
//! - Orphaned package detection and removal
//! - Icon cache repair (fc-cache)
//! - Permission fixes for home directory

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Broken symlink entry returned by `clean_find_broken_symlinks`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokenSymlink {
  /// Path to the broken symlink.
  pub path: String,
  /// Original target (if known).
  pub target: Option<String>,
}

/// Orphaned package entry returned by `clean_find_orphaned`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrphanedPackage {
  pub name: String,
  pub version: String,
  pub size: Option<u64>,
}

/// Result of a repair operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairResult {
  pub success: bool,
  pub message: String,
  pub items_processed: usize,
}

/// Icon cache repair result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconCacheResult {
  pub success: bool,
  pub message: String,
}

/// Permission repair result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionResult {
  pub success: bool,
  pub message: String,
  pub dirs_fixed: u32,
  pub files_fixed: u32,
}

fn format_size(bytes: u64) -> String {
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

#[component]
pub fn SystemRepairPage(state: AppState) -> Element {
  // Broken symlinks state
  let mut broken_symlinks = use_signal(|| Vec::<BrokenSymlink>::new());
  let mut selected_symlinks = use_signal(|| std::collections::HashSet::<String>::new());
  let mut is_scanning_symlinks = use_signal(|| false);

  // Orphaned packages state
  let mut orphaned_packages = use_signal(|| Vec::<OrphanedPackage>::new());
  let mut selected_packages = use_signal(|| std::collections::HashSet::<String>::new());
  let mut is_scanning_packages = use_signal(|| false);

  // Icon cache state
  let mut is_repairing_icon_cache = use_signal(|| false);
  let mut icon_cache_result = use_signal(|| Option::<IconCacheResult>::None);

  // Permissions state
  let mut is_repairing_permissions = use_signal(|| false);
  let mut permission_result = use_signal(|| Option::<PermissionResult>::None);

  // UI state
  let mut is_repairing_symlinks = use_signal(|| false);
  let mut is_removing_packages = use_signal(|| false);
  let mut show_confirm = use_signal(|| false);
  let mut confirm_action = use_signal(|| String::new());
  let mut pending_count = use_signal(|| 0usize);
  let mut status_message = use_signal(|| Option::<String>::None);
  let mut repair_result = use_signal(|| Option::<RepairResult>::None);

  // Pre-compute values
  let symlinks_count = broken_symlinks.read().len();
  let packages_count = orphaned_packages.read().len();
  let selected_symlinks_count = selected_symlinks.read().len();
  let selected_packages_count = selected_packages.read().len();
  let show_confirm_val = *show_confirm.read();
  let confirm_act = confirm_action.read().clone();
  let pending_cnt = *pending_count.read();
  let status_msg = status_message.read().clone();
  let repair_res = repair_result.read().clone();
  let icon_res = icon_cache_result.read().clone();
  let perm_res = permission_result.read().clone();

  rsx! {
      section { "data-page": "system-repair",
          class: "space-y-6",

          // Header
          div { class: "flex items-center justify-between",
              div {
                  h1 { class: "text-xl font-bold", "System Repair" }
                  p { class: "text-sm text-zinc-500 dark:text-zinc-400", "Fix broken symlinks, orphaned packages, and system issues" }
              }
          }

          // Top row: Quick repair cards
          div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",

              // Icon Cache Repair Card
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "flex items-center gap-4 mb-4",
                      div { class: "w-12 h-12 rounded-xl bg-gradient-to-br from-violet-500 to-purple-600 flex items-center justify-center",
                          span { class: "material-symbols-rounded text-white text-2xl", "image" }
                      }
                      div { class: "flex-1",
                          h2 { class: "font-semibold", "Icon Cache" }
                          p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Rebuild font and icon cache (fc-cache)" }
                      }
                  }
                  button {
                      class: format!(
                          "w-full px-4 py-2.5 rounded-xl font-medium text-sm transition-colors flex items-center justify-center gap-2 {}",
                          if *is_repairing_icon_cache.read() {
                              "bg-zinc-100 dark:bg-zinc-800 text-zinc-400 cursor-not-allowed".to_string()
                          } else {
                              "bg-gradient-to-r from-violet-500 to-purple-600 text-white hover:opacity-90".to_string()
                          }
                      ),
                      disabled: *is_repairing_icon_cache.read(),
                      onclick: move |_| {
                          is_repairing_icon_cache.set(true);
                          icon_cache_result.set(None);

                          let result = if let Ok(res) = bridge::invoke_app_command("repair_icon_cache", &serde_json::json!({})) {
                              IconCacheResult {
                                  success: res.get("success").and_then(|v| v.as_bool()).unwrap_or(false),
                                  message: res.get("message").and_then(|v| v.as_str()).unwrap_or("Icon cache repair complete").to_string(),
                              }
                          } else {
                              IconCacheResult {
                                  success: false,
                                  message: "Icon cache repair failed".to_string(),
                              }
                          };

                          is_repairing_icon_cache.set(false);
                          icon_cache_result.set(Some(result));
                      },
                      span { class: "material-symbols-rounded text-lg", "build" }
                      if *is_repairing_icon_cache.read() { "Repairing..." } else { "Repair Icon Cache" }
                  }
                  if let Some(res) = icon_res {
                      div { class: if res.success {
                              "mt-3 p-3 rounded-lg border bg-emerald-50 dark:bg-emerald-950/20 border-emerald-200 dark:border-emerald-800"
                          } else {
                              "mt-3 p-3 rounded-lg border bg-rose-50 dark:bg-rose-950/20 border-rose-200 dark:border-rose-800"
                          },
                          div { class: "flex items-center gap-2",
                              span { class: if res.success { "material-symbols-rounded text-emerald-500" } else { "material-symbols-rounded text-rose-500" },
                                  if res.success { "check_circle" } else { "error" }
                              }
                              span { class: "text-sm", "{res.message}" }
                          }
                      }
                  }
              }

              // Permissions Repair Card
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "flex items-center gap-4 mb-4",
                      div { class: "w-12 h-12 rounded-xl bg-gradient-to-br from-cyan-500 to-blue-600 flex items-center justify-center",
                          span { class: "material-symbols-rounded text-white text-2xl", "security" }
                      }
                      div { class: "flex-1",
                          h2 { class: "font-semibold", "Permissions" }
                          p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Fix file and directory permissions in home" }
                      }
                  }
                  button {
                      class: format!(
                          "w-full px-4 py-2.5 rounded-xl font-medium text-sm transition-colors flex items-center justify-center gap-2 {}",
                          if *is_repairing_permissions.read() {
                              "bg-zinc-100 dark:bg-zinc-800 text-zinc-400 cursor-not-allowed".to_string()
                          } else {
                              "bg-gradient-to-r from-cyan-500 to-blue-600 text-white hover:opacity-90".to_string()
                          }
                      ),
                      disabled: *is_repairing_permissions.read(),
                      onclick: move |_| {
                          is_repairing_permissions.set(true);
                          permission_result.set(None);

                          let result = if let Ok(res) = bridge::invoke_app_command("repair_permissions", &serde_json::json!({})) {
                              PermissionResult {
                                  success: res.get("success").and_then(|v| v.as_bool()).unwrap_or(false),
                                  message: res.get("message").and_then(|v| v.as_str()).unwrap_or("Permission repair complete").to_string(),
                                  dirs_fixed: res.get("dirs_fixed").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                                  files_fixed: res.get("files_fixed").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                              }
                          } else {
                              PermissionResult {
                                  success: false,
                                  message: "Permission repair failed".to_string(),
                                  dirs_fixed: 0,
                                  files_fixed: 0,
                              }
                          };

                          is_repairing_permissions.set(false);
                          permission_result.set(Some(result));
                      },
                      span { class: "material-symbols-rounded text-lg", "tune" }
                      if *is_repairing_permissions.read() { "Repairing..." } else { "Repair Permissions" }
                  }
                  if let Some(res) = perm_res {
                      div { class: if res.success {
                              "mt-3 p-3 rounded-lg border bg-emerald-50 dark:bg-emerald-950/20 border-emerald-200 dark:border-emerald-800"
                          } else {
                              "mt-3 p-3 rounded-lg border bg-rose-50 dark:bg-rose-950/20 border-rose-200 dark:border-rose-800"
                          },
                          div { class: "flex items-center gap-2",
                              span { class: if res.success { "material-symbols-rounded text-emerald-500" } else { "material-symbols-rounded text-rose-500" },
                                  if res.success { "check_circle" } else { "error" }
                              }
                              span { class: "text-sm", "{res.message}" }
                          }
                      }
                  }
              }
          }

          // Middle row: Scan cards for broken symlinks and orphaned packages
          div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",

              // Broken Symlinks Card
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "flex items-center gap-4 mb-4",
                      div { class: "w-12 h-12 rounded-xl bg-gradient-to-br from-amber-500 to-orange-600 flex items-center justify-center",
                          span { class: "material-symbols-rounded text-white text-2xl", "link_off" }
                      }
                      div { class: "flex-1",
                          h2 { class: "font-semibold", "Broken Symlinks" }
                          p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Find and remove invalid symbolic links" }
                      }
                      if *is_scanning_symlinks.read() {
                          div { class: "flex items-center gap-2 text-sm text-amber-500",
                              span { class: "material-symbols-rounded animate-spin text-lg", "progress_activity" }
                              "Scanning..."
                          }
                      } else if symlinks_count > 0 {
                          span { class: "text-xs px-2.5 py-1 rounded-full bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-400",
                              "{symlinks_count} found"
                          }
                      } else {
                          span { class: "text-xs px-2.5 py-1 rounded-full bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-400",
                              "Clean"
                          }
                      }
                  }
                  button {
                      class: format!(
                          "w-full px-4 py-2.5 rounded-xl font-medium text-sm transition-colors flex items-center justify-center gap-2 {}",
                          if *is_scanning_symlinks.read() {
                              "bg-zinc-100 dark:bg-zinc-800 text-zinc-400 cursor-not-allowed".to_string()
                          } else {
                              "bg-gradient-to-r from-amber-500 to-orange-600 text-white hover:opacity-90".to_string()
                          }
                      ),
                      disabled: *is_scanning_symlinks.read(),
                      onclick: move |_| {
                          is_scanning_symlinks.set(true);
                          broken_symlinks.set(Vec::new());
                          selected_symlinks.set(std::collections::HashSet::new());

                          if let Ok(result) = bridge::invoke_app_command("clean_find_broken_symlinks", &serde_json::json!({})) {
                              if let Some(data) = result.get("data") {
                                  if let Ok(items) = serde_json::from_value::<Vec<BrokenSymlink>>(data.clone()) {
                                      broken_symlinks.set(items);
                                  } else if let Ok(items) = serde_json::from_value::<Vec<Value>>(data.clone()) {
                                      let parsed: Vec<BrokenSymlink> = items
                                          .iter()
                                          .filter_map(|v| {
                                              Some(BrokenSymlink {
                                                  path: v.get("path")?.as_str()?.to_string(),
                                                  target: v.get("target").and_then(|t| t.as_str()).map(String::from),
                                              })
                                          })
                                          .collect();
                                      broken_symlinks.set(parsed);
                                  }
                              }
                          }
                          is_scanning_symlinks.set(false);
                      },
                      span { class: "material-symbols-rounded text-lg", "search" }
                      if *is_scanning_symlinks.read() { "Scanning..." } else { "Scan for Broken Symlinks" }
                  }

                  // Symlinks list
                  if !broken_symlinks.read().is_empty() {
                      div { class: "mt-4 space-y-2 max-h-64 overflow-y-auto",
                          for symlink in broken_symlinks.read().to_vec() {
                              div { class: "flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors",
                                  input {
                                      r#type: "checkbox",
                                      class: "w-4 h-4 rounded border-zinc-300 dark:border-zinc-600 text-amber-500 focus:ring-amber-500",
                                      checked: selected_symlinks.read().contains(&symlink.path),
                                      onchange: {
                                          let symlink_path = symlink.path.clone();
                                          move |_| {
                                              let mut sel = selected_symlinks.read().clone();
                                              if sel.contains(&symlink_path) {
                                                  sel.remove(&symlink_path);
                                              } else {
                                                  sel.insert(symlink_path.clone());
                                              }
                                              selected_symlinks.set(sel);
                                          }
                                      }
                                  }
                                  div { class: "flex-1 min-w-0",
                                      span { class: "block font-mono text-sm truncate text-rose-600 dark:text-rose-400", "{symlink.path}" }
                                      if let Some(target) = &symlink.target {
                                          span { class: "block text-xs text-zinc-400 truncate", "→ {target}" }
                                      }
                                  }
                              }
                          }
                      }
                      if selected_symlinks_count > 0 {
                          div { class: "mt-3 flex justify-end",
                              button {
                                  class: "px-4 py-2 rounded-xl bg-rose-500 hover:bg-rose-600 text-white font-medium text-sm transition-colors flex items-center gap-2",
                                  onclick: move |_| {
                                      confirm_action.set("remove_symlinks".to_string());
                                      pending_count.set(selected_symlinks.read().len());
                                      show_confirm.set(true);
                                  },
                                  span { class: "material-symbols-rounded text-lg", "delete" }
                                  "Remove Selected ({selected_symlinks_count})"
                              }
                          }
                      }
                  }
              }

              // Orphaned Packages Card
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "flex items-center gap-4 mb-4",
                      div { class: "w-12 h-12 rounded-xl bg-gradient-to-br from-rose-500 to-pink-600 flex items-center justify-center",
                          span { class: "material-symbols-rounded text-white text-2xl", "package_2" }
                      }
                      div { class: "flex-1",
                          h2 { class: "font-semibold", "Orphaned Packages" }
                          p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Find and remove unused dependencies" }
                      }
                      if *is_scanning_packages.read() {
                          div { class: "flex items-center gap-2 text-sm text-rose-500",
                              span { class: "material-symbols-rounded animate-spin text-lg", "progress_activity" }
                              "Scanning..."
                          }
                      } else if packages_count > 0 {
                          span { class: "text-xs px-2.5 py-1 rounded-full bg-rose-100 dark:bg-rose-900/30 text-rose-700 dark:text-rose-400",
                              "{packages_count} found"
                          }
                      } else {
                          span { class: "text-xs px-2.5 py-1 rounded-full bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-400",
                              "Clean"
                          }
                      }
                  }
                  button {
                      class: format!(
                          "w-full px-4 py-2.5 rounded-xl font-medium text-sm transition-colors flex items-center justify-center gap-2 {}",
                          if *is_scanning_packages.read() {
                              "bg-zinc-100 dark:bg-zinc-800 text-zinc-400 cursor-not-allowed".to_string()
                          } else {
                              "bg-gradient-to-r from-rose-500 to-pink-600 text-white hover:opacity-90".to_string()
                          }
                      ),
                      disabled: *is_scanning_packages.read(),
                      onclick: move |_| {
                          is_scanning_packages.set(true);
                          orphaned_packages.set(Vec::new());
                          selected_packages.set(std::collections::HashSet::new());

                          if let Ok(result) = bridge::invoke_app_command("clean_find_orphaned", &serde_json::json!({})) {
                              if let Some(data) = result.get("data") {
                                  if let Ok(items) = serde_json::from_value::<Vec<OrphanedPackage>>(data.clone()) {
                                      orphaned_packages.set(items);
                                  }
                              }
                          }
                          is_scanning_packages.set(false);
                      },
                      span { class: "material-symbols-rounded text-lg", "search" }
                      if *is_scanning_packages.read() { "Scanning..." } else { "Scan for Orphaned Packages" }
                  }

                  // Packages list
                  if !orphaned_packages.read().is_empty() {
                      div { class: "mt-4 space-y-2 max-h-64 overflow-y-auto",
                          for pkg in orphaned_packages.read().to_vec() {
                              div { class: "flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors",
                                  input {
                                      r#type: "checkbox",
                                      class: "w-4 h-4 rounded border-zinc-300 dark:border-zinc-600 text-rose-500 focus:ring-rose-500",
                                      checked: selected_packages.read().contains(&pkg.name),
                                      onchange: {
                                          let pkg_name = pkg.name.clone();
                                          move |_| {
                                              let mut sel = selected_packages.read().clone();
                                              if sel.contains(&pkg_name) {
                                                  sel.remove(&pkg_name);
                                              } else {
                                                  sel.insert(pkg_name.clone());
                                              }
                                              selected_packages.set(sel);
                                          }
                                      }
                                  }
                                  div { class: "flex-1 min-w-0",
                                      span { class: "block font-mono text-sm truncate", "{pkg.name}" }
                                      span { class: "block text-xs text-zinc-400", "v{pkg.version}" }
                                  }
                                  if let Some(size) = pkg.size {
                                      span { class: "text-xs text-zinc-400", "{format_size(size)}" }
                                  }
                              }
                          }
                      }
                      if selected_packages_count > 0 {
                          div { class: "mt-3 flex justify-end",
                              button {
                                  class: "px-4 py-2 rounded-xl bg-rose-500 hover:bg-rose-600 text-white font-medium text-sm transition-colors flex items-center gap-2",
                                  onclick: move |_| {
                                      confirm_action.set("remove_packages".to_string());
                                      pending_count.set(selected_packages.read().len());
                                      show_confirm.set(true);
                                  },
                                  span { class: "material-symbols-rounded text-lg", "delete" }
                                  "Remove Selected ({selected_packages_count})"
                              }
                          }
                      }
                  }
              }
          }

          // Status message
          if let Some(msg) = status_msg {
              div { class: "p-4 rounded-xl bg-emerald-50 dark:bg-emerald-950/20 border border-emerald-200 dark:border-emerald-800",
                  div { class: "flex items-center gap-3",
                      span { class: "material-symbols-rounded text-emerald-500 text-xl", "check_circle" }
                      span { class: "text-sm text-emerald-700 dark:text-emerald-300", "{msg}" }
                  }
              }
          }

          if let Some(res) = repair_res {
              div { class: if res.success {
                      "p-4 rounded-xl bg-emerald-50 dark:bg-emerald-950/20 border border-emerald-200 dark:border-emerald-800"
                  } else {
                      "p-4 rounded-xl bg-rose-50 dark:bg-rose-950/20 border border-rose-200 dark:border-rose-800"
                  },
                  div { class: "flex items-center gap-3",
                      span { class: if res.success { "material-symbols-rounded text-emerald-500 text-xl" } else { "material-symbols-rounded text-rose-500 text-xl" },
                          if res.success { "check_circle" } else { "error" }
                      }
                      span { class: "text-sm", "{res.message}" }
                  }
              }
          }

          // Confirmation Dialog
          if show_confirm_val {
              div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
                  div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-6 max-w-sm w-full mx-4 shadow-2xl",
                      div { class: "flex items-center gap-3 mb-4",
                          div { class: "w-10 h-10 rounded-full bg-amber-100 dark:bg-amber-900/30 flex items-center justify-center",
                              span { class: "material-symbols-rounded text-amber-600 dark:text-amber-400", "warning" }
                          }
                          div {
                              h3 { class: "font-semibold", "Confirm Action" }
                              p { class: "text-sm text-zinc-500 dark:text-zinc-400", "This action cannot be undone" }
                          }
                      }
                      div { class: "flex items-center gap-3 p-3 rounded-lg bg-rose-50 dark:bg-rose-950/20 border border-rose-200 dark:border-rose-800 mb-4",
                          span { class: "material-symbols-rounded text-rose-500", "info" }
                          if confirm_act == "remove_symlinks" {
                              p { class: "text-sm text-rose-700 dark:text-rose-300",
                                  "Remove {pending_cnt} broken symlink(s)?"
                              }
                          } else if confirm_act == "remove_packages" {
                              p { class: "text-sm text-rose-700 dark:text-rose-300",
                                  "Remove {pending_cnt} orphaned package(s)?"
                              }
                          } else if confirm_act == "repair_all" {
                              p { class: "text-sm text-rose-700 dark:text-rose-300",
                                  "Run all available repairs?"
                              }
                          }
                      }
                      div { class: "flex gap-3",
                          button {
                              class: "flex-1 px-4 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 font-medium text-sm hover:bg-zinc-50 dark:hover:bg-zinc-800 transition-colors",
                              onclick: move |_| {
                                  show_confirm.set(false);
                                  confirm_action.set(String::new());
                              },
                              "Cancel"
                          }
                          button {
                              class: "flex-1 px-4 py-2 rounded-xl bg-rose-500 hover:bg-rose-600 text-white font-medium text-sm transition-colors",
                              onclick: move |_| {
                                  show_confirm.set(false);
                                  let action = confirm_action.read().clone();

                                  if action == "remove_symlinks" {
                                      let selected: Vec<String> = selected_symlinks.read().iter().cloned().collect();
                                      is_repairing_symlinks.set(true);

                                      if let Ok(res) = bridge::invoke_app_command("repair_remove_broken_symlinks", &serde_json::json!({ "paths": &selected })) {
                                          let removed = res.get("removed").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                                          repair_result.set(Some(RepairResult {
                                              success: true,
                                              message: format!("Removed {} broken symlinks", removed),
                                              items_processed: removed,
                                          }));
                                      }

                                      is_repairing_symlinks.set(false);
                                      selected_symlinks.set(std::collections::HashSet::new());
                                      // Rescan
                                      broken_symlinks.set(Vec::new());
                                  } else if action == "remove_packages" {
                                      let selected: Vec<String> = selected_packages.read().iter().cloned().collect();
                                      is_removing_packages.set(true);

                                      if let Ok(res) = bridge::invoke_app_command("repair_remove_orphaned", &serde_json::json!({ "packages": &selected })) {
                                          let removed = res.get("removed").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                                          repair_result.set(Some(RepairResult {
                                              success: true,
                                              message: format!("Removed {} orphaned packages", removed),
                                              items_processed: removed,
                                          }));
                                      }

                                      is_removing_packages.set(false);
                                      selected_packages.set(std::collections::HashSet::new());
                                      // Rescan
                                      orphaned_packages.set(Vec::new());
                                  } else if action == "repair_all" {
                                      // Run icon cache repair inline
                                      is_repairing_icon_cache.set(true);
                                      if let Ok(res) = bridge::invoke_app_command("repair_icon_cache", &serde_json::json!({})) {
                                          icon_cache_result.set(Some(IconCacheResult {
                                              success: res.get("success").and_then(|v| v.as_bool()).unwrap_or(false),
                                              message: res.get("message").and_then(|v| v.as_str()).unwrap_or("Done").to_string(),
                                          }));
                                      }
                                      is_repairing_icon_cache.set(false);

                                      // Run permissions repair inline
                                      is_repairing_permissions.set(true);
                                      if let Ok(res) = bridge::invoke_app_command("repair_permissions", &serde_json::json!({})) {
                                          permission_result.set(Some(PermissionResult {
                                              success: res.get("success").and_then(|v| v.as_bool()).unwrap_or(false),
                                              message: res.get("message").and_then(|v| v.as_str()).unwrap_or("Done").to_string(),
                                              dirs_fixed: 0,
                                              files_fixed: 0,
                                          }));
                                      }
                                      is_repairing_permissions.set(false);

                                      status_message.set(Some("All repairs completed".to_string()));
                                  }

                                  confirm_action.set(String::new());
                              },
                              "Confirm"
                          }
                      }
                  }
              }
          }

          // Repair All button at bottom
          div { class: "flex justify-center pt-4",
              button {
                  class: "px-6 py-3 rounded-xl bg-gradient-to-r from-violet-600 via-purple-600 to-fuchsia-600 text-white font-medium text-sm transition-colors hover:opacity-90 flex items-center gap-2",
                  onclick: move |_| {
                      confirm_action.set("repair_all".to_string());
                      pending_count.set(0);
                      show_confirm.set(true);
                  },
                  span { class: "material-symbols-rounded text-xl", "build" }
                  "Repair All System Issues"
              }
          }
      }
  }
}
