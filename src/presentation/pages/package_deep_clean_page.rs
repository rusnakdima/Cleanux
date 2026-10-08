//! Package Deep Clean page — mirrors the template `data-page="package-deep-clean"` section.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PackageCacheInfo {
  pub manager: String,
  pub cache_size: u64,
  pub package_count: usize,
  pub partial_count: usize,
  pub orphaned_count: usize,
  pub available: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PackageCacheSummary {
  pub managers: Vec<PackageCacheInfo>,
  pub total_reclaimable: u64,
}

#[derive(Clone, Props, PartialEq)]
struct ManagerTabProps {
  manager: String,
  active_tab: Signal<String>,
}

#[component]
fn ManagerTab(props: ManagerTabProps) -> Element {
  let mut active_tab = props.active_tab.clone();
  let manager = props.manager.clone();
  let is_active = *props.active_tab.read() == manager;
  rsx! {
      button {
          class: format!(
              "px-4 py-2.5 text-sm font-medium rounded-xl border transition-all {}",
              if is_active {
                  "bg-cyan-600 text-white border-cyan-600"
              } else {
                  "border-zinc-200 dark:border-zinc-700 text-zinc-600 dark:text-zinc-400 hover:border-cyan-400 hover:text-cyan-600 dark:hover:text-cyan-400"
              }
          ),
          onclick: move |_| active_tab.set(manager.clone()),
          "{manager}"
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

#[component]
pub fn PackageDeepCleanPage(state: AppState) -> Element {
  let cache_summary = use_signal(|| Option::<PackageCacheSummary>::None);
  let active_tab = use_signal(|| String::from("apt"));
  let is_cleaning = use_signal(|| false);
  let mut show_confirm = use_signal(|| false);
  let mut confirm_manager = use_signal(|| String::new());
  let last_clean_result = use_signal(|| Option::<String>::None);

  // Load package cache info on mount
  use_effect(move || {
    let mut summary_clone = cache_summary.clone();
    if let Ok(val) = bridge::invoke_app_command("package_get_cache_summary", &serde_json::json!({}))
    {
      if let Ok(summary) = serde_json::from_value::<PackageCacheSummary>(val) {
        summary_clone.set(Some(summary));
      }
    }
  });

  // Extract owned values
  let managers: Vec<String> = cache_summary
    .read()
    .as_ref()
    .map(|s| {
      s.managers
        .iter()
        .filter(|m| m.available)
        .map(|m| m.manager.clone())
        .collect()
    })
    .unwrap_or_default();

  let current_info: Option<PackageCacheInfo> = cache_summary.read().as_ref().and_then(|s| {
    s.managers
      .iter()
      .find(|m| m.manager == *active_tab.read())
      .cloned()
  });

  let total_reclaimable = cache_summary
    .read()
    .as_ref()
    .map(|s| s.total_reclaimable)
    .unwrap_or(0);

  rsx! {
      section { "data-page": "package-deep-clean",
          class: "space-y-6",

          // Header
          div { class: "flex items-center justify-between",
              div {
                  h1 { class: "text-2xl font-bold", "Package Deep Clean" }
                  p { class: "text-sm text-zinc-500 dark:text-zinc-400 mt-1", "Clean package manager caches and reclaim disk space" }
              }
              div { class: "flex items-center gap-3",
                  button {
                      class: "flex items-center gap-2 px-4 py-2 rounded-xl bg-cyan-600 hover:bg-cyan-700 text-white text-sm font-medium transition-colors",
                      onclick: move |_| {
                          let mut summary_clone = cache_summary.clone();
                          async move {
                              if let Ok(val) = bridge::invoke_app_command("package_get_cache_summary", &serde_json::json!({})) {
                                  if let Ok(summary) = serde_json::from_value::<PackageCacheSummary>(val) {
                                      summary_clone.set(Some(summary));
                                  }
                              }
                          }
                      },
                      span { class: "material-symbols-rounded text-base", "refresh" }
                      "Refresh"
                  }
              }
          }

          // Reclaimable space card
          div { class: "bg-gradient-to-br from-cyan-500/10 to-violet-500/10 dark:from-cyan-500/5 dark:to-violet-500/5 rounded-2xl border border-cyan-200 dark:border-cyan-800 p-6",
              div { class: "flex items-center gap-4",
                  div { class: "w-14 h-14 rounded-2xl bg-cyan-100 dark:bg-cyan-900/30 flex items-center justify-center",
                      span { class: "material-symbols-rounded text-cyan-600 dark:text-cyan-400 text-3xl", "storage" }
                  }
                  div { class: "flex-1",
                      div { class: "text-3xl font-bold", "{format_bytes(total_reclaimable)}" }
                      div { class: "text-sm text-zinc-500 dark:text-zinc-400", "Total reclaimable space across all package managers" }
                  }
              }
          }

          // Package manager tabs
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 overflow-hidden",
              div { class: "flex border-b border-zinc-200 dark:border-zinc-800 overflow-x-auto",
                  for manager in managers.iter().cloned() {
                      ManagerTab {
                          key: "{manager}",
                          manager: manager,
                          active_tab: active_tab
                      }
                  }
              }

              div { class: "p-5",
                  if let Some(info) = current_info {
                      div { class: "grid grid-cols-2 md:grid-cols-4 gap-4 mb-6",
                          // Cache size
                          div { class: "bg-zinc-50 dark:bg-zinc-800/50 rounded-xl p-4",
                              div { class: "flex items-center gap-2 mb-2",
                                  span { class: "material-symbols-rounded text-zinc-400 text-base", "folder" }
                                  span { class: "text-xs text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Cache Size" }
                              }
                              div { class: "text-xl font-semibold", "{format_bytes(info.cache_size)}" }
                          }

                          // Package count
                          div { class: "bg-zinc-50 dark:bg-zinc-800/50 rounded-xl p-4",
                              div { class: "flex items-center gap-2 mb-2",
                                  span { class: "material-symbols-rounded text-zinc-400 text-base", "inbox" }
                                  span { class: "text-xs text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Packages" }
                              }
                              div { class: "text-xl font-semibold", "{info.package_count}" }
                          }

                          // Partial packages
                          div { class: "bg-zinc-50 dark:bg-zinc-800/50 rounded-xl p-4",
                              div { class: "flex items-center gap-2 mb-2",
                                  span { class: "material-symbols-rounded text-zinc-400 text-base", "warning" }
                                  span { class: "text-xs text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Partial" }
                              }
                              div { class: "text-xl font-semibold", "{info.partial_count}" }
                          }

                          // Orphaned packages
                          div { class: "bg-zinc-50 dark:bg-zinc-800/50 rounded-xl p-4",
                              div { class: "flex items-center gap-2 mb-2",
                                  span { class: "material-symbols-rounded text-zinc-400 text-base", "package_2" }
                                  span { class: "text-xs text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Orphaned" }
                              }
                              div { class: "text-xl font-semibold", "{info.orphaned_count}" }
                          }
                      }

                      // Clean actions
                      div { class: "flex items-center gap-3",
                          button {
                              class: "flex-1 flex items-center justify-center gap-2 px-4 py-3 rounded-xl bg-rose-600 hover:bg-rose-700 text-white font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                              disabled: is_cleaning() || info.cache_size == 0,
                              onclick: move |_| {
                                  confirm_manager.set(info.manager.clone());
                                  show_confirm.set(true);
                              },
                              span { class: "material-symbols-rounded text-base", "delete" }
                              "Clean All Caches"
                          }
                      }

                      if let Some(result) = last_clean_result.read().as_ref() {
                          div { class: "mt-4 p-3 rounded-xl bg-emerald-50 dark:bg-emerald-900/20 border border-emerald-200 dark:border-emerald-800",
                              div { class: "flex items-center gap-2 text-emerald-700 dark:text-emerald-400",
                                  span { class: "material-symbols-rounded text-base", "check_circle" }
                                  span { class: "text-sm font-medium", "{result}" }
                              }
                          }
                      }
                  } else {
                      div { class: "text-center py-8 text-zinc-400",
                          span { class: "material-symbols-rounded text-4xl mb-2 block", "hourglass_empty" }
                          "Loading package manager info..."
                      }
                  }
              }
          }

          // Info card
          div { class: "bg-amber-50 dark:bg-amber-900/10 rounded-xl border border-amber-200 dark:border-amber-800 p-4",
              div { class: "flex items-start gap-3",
                  span { class: "material-symbols-rounded text-amber-600 dark:text-amber-400 text-xl flex-shrink-0 mt-0.5", "info" }
                  div {
                      div { class: "text-sm font-medium text-amber-800 dark:text-amber-200", "About Package Cache Cleaning" }
                      div { class: "text-xs text-amber-700 dark:text-amber-300 mt-1 leading-relaxed",
                          "Package managers store downloaded packages in cache for reinstallation. "
                          "Cleaning these caches frees disk space but may increase download time if you need to reinstall packages."
                      }
                  }
              }
          }
      }

      // Confirmation dialog
      if show_confirm() {
          div { class: "fixed inset-0 z-[70] bg-black/60 backdrop-blur-sm flex items-center justify-center",
              div { class: "max-w-sm mx-auto bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 shadow-2xl p-5 mx-4",
                  div { class: "w-12 h-12 rounded-full bg-rose-100 dark:bg-rose-900/30 flex items-center justify-center mb-3",
                      span { class: "material-symbols-rounded text-rose-600 dark:text-rose-400", "warning" }
                  }
                  h3 { class: "font-semibold text-lg", "Clean Package Cache" }
                  p { class: "text-sm text-zinc-600 dark:text-zinc-400 mt-2",
                      "Are you sure you want to clean the "
                      span { class: "font-medium", "{confirm_manager.read()}" }
                      " package cache? This will delete all cached packages."
                  }
                  div { class: "text-xs text-zinc-500 dark:text-zinc-400 mt-1", "This action cannot be undone." }
                  div { class: "flex justify-end gap-2 mt-4",
                      button {
                          class: "px-4 py-2 rounded-lg text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800",
                          onclick: move |_| show_confirm.set(false),
                          "Cancel"
                      }
                      button {
                          class: "px-4 py-2 rounded-lg bg-rose-600 hover:bg-rose-700 text-white text-sm font-medium",
                          onclick: move |_| {
                              show_confirm.set(false);
                              let manager = confirm_manager.read().clone();
                              let mut is_cleaning_clone = is_cleaning.clone();
                              let mut result_clone = last_clean_result.clone();
                              let mut summary_clone = cache_summary.clone();
                              is_cleaning_clone.set(true);
                              async move {
                                  if let Ok(val) = bridge::invoke_app_command("package_clean_package_cache", &serde_json::json!({ "manager": &manager })) {
                                      let freed = val.get("freed").and_then(|v| v.as_u64()).unwrap_or(0);
                                      result_clone.set(Some(format!("Cleaned {} of cache", format_bytes(freed))));
                                  }
                                  // Refresh summary
                                  if let Ok(val) = bridge::invoke_app_command("package_get_cache_summary", &serde_json::json!({})) {
                                      if let Ok(summary) = serde_json::from_value::<PackageCacheSummary>(val) {
                                          summary_clone.set(Some(summary));
                                      }
                                  }
                                  is_cleaning_clone.set(false);
                              }
                          },
                          "Clean Cache"
                      }
                  }
              }
          }
      }
  }
}
