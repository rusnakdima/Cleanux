//! App Residue Cleaner page — mirrors the template `data-page="app-residue-cleaner"` section.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResidueItem {
  pub app_name: String,
  pub path: String,
  pub residue_type: String,
  pub size: u64,
  pub modified: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ApiResponse<T> {
  status: String,
  message: String,
  data: Option<T>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResidueSummary {
  pub configs_count: u32,
  pub data_count: u32,
  pub caches_count: u32,
  pub orphaned_count: u32,
  pub total_size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
  Configs,
  Data,
  Caches,
  Orphaned,
}

impl Tab {
  fn label(&self) -> &'static str {
    match self {
      Tab::Configs => "Configs",
      Tab::Data => "Data",
      Tab::Caches => "Caches",
      Tab::Orphaned => "Orphaned",
    }
  }
  fn command(&self) -> &'static str {
    match self {
      Tab::Configs => "clean_scan_user_configs",
      Tab::Data => "clean_scan_user_data",
      Tab::Caches => "clean_scan_user_caches",
      Tab::Orphaned => "clean_scan_home_residues",
    }
  }
}

#[derive(Clone, Props, PartialEq)]
struct ResidueRowProps {
  item: ResidueItem,
  selected: Signal<std::collections::HashSet<String>>,
  on_preview: Signal<Option<ResidueItem>>,
}

impl ResidueRowProps {
  fn new(
    item: ResidueItem,
    selected: Signal<std::collections::HashSet<String>>,
    on_preview: Signal<Option<ResidueItem>>,
  ) -> Self {
    Self {
      item,
      selected,
      on_preview,
    }
  }
}

#[component]
fn ResidueRow(mut props: ResidueRowProps) -> Element {
  let item_path = props.item.path.clone();
  let is_selected = props.selected.read().contains(&item_path);

  rsx! {
      tr { class: "hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors",
          td { class: "px-3 py-3",
              input {
                  r#type: "checkbox",
                  class: "rounded border-zinc-300 dark:border-zinc-600 text-cyan-500 focus:ring-cyan-500",
                  checked: is_selected,
                  onchange: move |_| {
                      let mut sel = props.selected.write();
                      if sel.contains(&item_path) {
                          sel.remove(&item_path);
                      } else {
                          sel.insert(item_path.clone());
                      }
                  }
              }
          }
          td { class: "px-3 py-3 font-medium", "{props.item.app_name}" }
          td { class: "px-3 py-3 text-zinc-500 dark:text-zinc-400 truncate max-w-xs", title: "{props.item.path}",
              "{props.item.path}"
          }
          td { class: "px-3 py-3 text-zinc-500 dark:text-zinc-400",
              "{format_size(props.item.size)}"
          }
          td { class: "px-3 py-3 text-zinc-500 dark:text-zinc-400 text-xs",
              "{props.item.modified}"
          }
          td { class: "px-3 py-3",
              button {
                  class: "p-1.5 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-700 transition-colors",
                  title: "Preview",
                  onclick: move |_| props.on_preview.set(Some(props.item.clone())),
                  span { class: "material-symbols-rounded text-lg text-zinc-400", "visibility" }
              }
          }
      }
  }
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
pub fn AppResidueCleanerPage(state: AppState) -> Element {
  let mut active_tab = use_signal(|| Tab::Configs);
  let mut items = use_signal(|| Vec::<ResidueItem>::new());
  let mut selected = use_signal(|| std::collections::HashSet::<String>::new());
  let loading = use_signal(|| false);
  let mut search_query = use_signal(|| String::new());
  let mut current_page = use_signal(|| 0);
  let mut show_preview = use_signal(|| Option::<ResidueItem>::None);
  let mut show_confirm = use_signal(|| false);
  let items_per_page: usize = 20;

  // Load data when tab changes
  let active_tab_clone = *active_tab.read();
  use_effect(move || {
    let tab = active_tab_clone;
    let mut items_clone = items.clone();
    let mut loading_clone = loading.clone();
    loading_clone.set(true);
    if let Ok(val) = bridge::invoke_app_command(tab.command(), &serde_json::json!({})) {
      if let Ok(resp) = serde_json::from_value::<ApiResponse<Vec<ResidueItem>>>(val) {
        if let Some(data) = resp.data {
          items_clone.set(data);
        }
      }
    }
    loading_clone.set(false);
  });

  // Filter items by search - derive from signals
  let query = search_query.read().to_lowercase();
  let filtered: Vec<ResidueItem> = items
    .read()
    .iter()
    .filter(|item| {
      if query.is_empty() {
        true
      } else {
        item.app_name.to_lowercase().contains(&query) || item.path.to_lowercase().contains(&query)
      }
    })
    .cloned()
    .collect();

  // Pagination
  let total_pages = (filtered.len() + items_per_page - 1) / items_per_page;
  let page = *current_page.read();
  let paginated: Vec<ResidueItem> = filtered
    .iter()
    .skip(page * items_per_page)
    .take(items_per_page)
    .cloned()
    .collect();

  // Select all state
  let all_selected = !filtered.is_empty()
    && filtered
      .iter()
      .all(|item| selected.read().contains(&item.path));

  // Clone filtered for use in closures
  let filtered_clone = filtered.clone();

  let tab_items_count = |tab: Tab| -> usize {
    match tab {
      Tab::Configs => items
        .read()
        .iter()
        .filter(|i| i.residue_type == "config")
        .count(),
      Tab::Data => items
        .read()
        .iter()
        .filter(|i| i.residue_type == "data")
        .count(),
      Tab::Caches => items
        .read()
        .iter()
        .filter(|i| i.residue_type == "cache")
        .count(),
      Tab::Orphaned => items.read().len(),
    }
  };

  rsx! {
      section { "data-page": "app-residue-cleaner",
          class: "space-y-6",

          // Backup warning banner
          div { class: "bg-amber-50 dark:bg-amber-950/30 border border-amber-200 dark:border-amber-800 rounded-xl p-4 flex items-start gap-3",
              span { class: "material-symbols-rounded text-amber-500 text-xl flex-shrink-0 mt-0.5", "info" }
              div { class: "flex-1",
                  p { class: "text-sm font-medium text-amber-800 dark:text-amber-300", "Backup Recommended" }
                  p { class: "text-xs text-amber-700 dark:text-amber-400 mt-0.5", "Removing app residues may prevent apps from preserving their settings. Consider backing up important data before cleaning." }
              }
          }

          // Main card
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              // Header
              div { class: "flex items-center justify-between mb-4",
                  div { class: "flex items-center gap-2",
                      h2 { class: "font-semibold", "App Residue Cleaner" }
                      span { class: "text-xs font-medium px-2 py-0.5 rounded-full bg-cyan-100 dark:bg-cyan-900/30 text-cyan-700 dark:text-cyan-400",
                          "{filtered.len()} items"
                      }
                  }
                  button {
                      class: "text-xs text-zinc-400 hover:text-cyan-500 transition-colors flex items-center gap-1",
                      onclick: move |_| {
                          let tab = *active_tab.read();
                          let mut items_clone = items.clone();
                          let mut loading_clone = loading.clone();
                          loading_clone.set(true);
                          if let Ok(val) = bridge::invoke_app_command(tab.command(), &serde_json::json!({})) {
                              if let Ok(resp) = serde_json::from_value::<ApiResponse<Vec<ResidueItem>>>(val) {
                                  if let Some(data) = resp.data {
                                      items_clone.set(data);
                                  }
                              }
                          }
                          loading_clone.set(false);
                      },
                      span { class: "material-symbols-rounded text-sm", "refresh" }
                      "Refresh"
                  }
              }

              // Search input
              div { class: "relative mb-4",
                  span { class: "material-symbols-rounded absolute left-3 top-1/2 -translate-y-1/2 text-zinc-400 text-lg", "search" }
                  input {
                      r#type: "text",
                      class: "w-full pl-9 pr-4 py-2.5 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-zinc-50 dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500 focus:border-transparent",
                      placeholder: "Search by app name or path...",
                      value: "{search_query.read()}",
                      oninput: move |e| {
                          search_query.set(e.value().to_string());
                          current_page.set(0);
                      }
                  }
              }

              // Tab bar
              div { class: "flex border-b border-zinc-200 dark:border-zinc-700 mb-4",
                  for tab in [Tab::Configs, Tab::Data, Tab::Caches, Tab::Orphaned] {
                      button {
                          class: format!(
                              "flex-1 py-2.5 px-3 text-sm font-medium border-b-2 transition-colors {}",
                              if *active_tab.read() == tab {
                                  "border-cyan-500 text-cyan-600 dark:text-cyan-400"
                              } else {
                                  "border-transparent text-zinc-500 hover:text-zinc-700 dark:text-zinc-400"
                              }
                          ),
                          onclick: move |_| {
                              active_tab.set(tab);
                              selected.set(std::collections::HashSet::new());
                              current_page.set(0);
                          },
                          "{tab.label()}"
                          span { class: "ml-1.5 text-xs px-1.5 py-0.5 rounded bg-zinc-100 dark:bg-zinc-800",
                              "{tab_items_count(tab)}"
                          }
                      }
                  }
              }

              // Loading state
              if loading() {
                  div { class: "flex items-center justify-center py-12",
                      span { class: "material-symbols-rounded text-3xl text-cyan-500 animate-spin", "progress_activity" }
                  }
              } else if filtered.is_empty() {
                  // Empty state
                  div { class: "text-center py-12",
                      span { class: "material-symbols-rounded text-4xl text-zinc-300 dark:text-zinc-600", "check_circle" }
                      p { class: "text-sm text-zinc-500 dark:text-zinc-400 mt-2", "No residues found in this category" }
                  }
              } else {
                  // Table
                  div { class: "overflow-x-auto",
                      table { class: "w-full text-sm",
                          thead { class: "bg-zinc-50 dark:bg-zinc-800/50",
                              tr { class: "text-left text-xs text-zinc-500 dark:text-zinc-400 uppercase tracking-wider",
                                  th { class: "px-3 py-2.5 w-10",
                                      input {
                                          r#type: "checkbox",
                                          class: "rounded border-zinc-300 dark:border-zinc-600 text-cyan-500 focus:ring-cyan-500",
                                          checked: all_selected,
                                          onchange: move |_| {
                                              if all_selected {
                                                  // Deselect all
                                                  let mut sel = selected.write();
                                                  for item in filtered_clone.iter() {
                                                      sel.remove(&item.path);
                                                  }
                                              } else {
                                                  // Select all
                                                  let mut sel = selected.write();
                                                  for item in filtered_clone.iter() {
                                                      sel.insert(item.path.clone());
                                                  }
                                              }
                                          }
                                      }
                                  }
                                  th { class: "px-3 py-2.5", "App Name" }
                                  th { class: "px-3 py-2.5", "Path" }
                                  th { class: "px-3 py-2.5", "Size" }
                                  th { class: "px-3 py-2.5", "Modified" }
                                  th { class: "px-3 py-2.5 w-20", "" }
                              }
                          }
                          tbody { class: "divide-y divide-zinc-100 dark:divide-zinc-800",
                              for item in paginated {
                                  ResidueRow {
                                      key: "{item.path}",
                                      item: item.clone(),
                                      selected: selected,
                                      on_preview: show_preview
                                  }
                              }
                          }
                      }
                  }

                  // Pagination
                  if total_pages > 1 {
                      div { class: "flex items-center justify-between mt-4 pt-3 border-t border-zinc-100 dark:border-zinc-800",
                          button {
                              class: "px-3 py-1.5 rounded-lg text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800 disabled:opacity-50 disabled:cursor-not-allowed transition-colors",
                              disabled: *current_page.read() == 0,
                              onclick: move |_| {
                                  let val: usize = *current_page.read();
                                  current_page.set(val.saturating_sub(1));
                              },
                              "Previous"
                          }
                          span { class: "text-xs text-zinc-500 dark:text-zinc-400",
                              "Page {*current_page.read() + 1} of {total_pages}"
                          }
                          button {
                              class: "px-3 py-1.5 rounded-lg text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800 disabled:opacity-50 disabled:cursor-not-allowed transition-colors",
                              disabled: *current_page.read() >= total_pages - 1,
                              onclick: move |_| {
                                  let val: usize = *current_page.read();
                                  current_page.set(val + 1);
                              },
                              "Next"
                          }
                      }
                  }
              }
          }

          // Clean selected button
          if !selected.read().is_empty() {
              div { class: "flex items-center justify-between bg-cyan-50 dark:bg-cyan-900/20 rounded-xl p-4",
                  span { class: "text-sm text-cyan-700 dark:text-cyan-300",
                      "{selected.read().len()} item(s) selected ({format_size(filtered.iter().filter(|i| selected.read().contains(&i.path)).map(|i| i.size).sum())})"
                  }
                  button {
                      class: "px-4 py-2 rounded-lg bg-cyan-600 hover:bg-cyan-700 text-white text-sm font-medium transition-colors",
                      onclick: move |_| show_confirm.set(true),
                      "Clean Selected"
                  }
              }
          }
      }

      // Preview modal
      if let Some(item) = show_preview() {
          div { class: "fixed inset-0 z-[70] bg-black/60 backdrop-blur-sm flex items-center justify-center",
              div { class: "max-w-lg mx-auto bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 shadow-2xl p-5 mx-4",
                  div { class: "flex items-center justify-between mb-4",
                      h3 { class: "font-semibold", "Residue Details" }
                      button {
                          class: "p-1.5 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800",
                          onclick: move |_| show_preview.set(None),
                          span { class: "material-symbols-rounded text-xl", "close" }
                      }
                  }
                  div { class: "space-y-3",
                      div { class: "flex items-center gap-3 p-3 rounded-xl bg-zinc-50 dark:bg-zinc-800",
                          div { class: "w-10 h-10 rounded-lg bg-violet-100 dark:bg-violet-900/30 flex items-center justify-center flex-shrink-0",
                              span { class: "material-symbols-rounded text-violet-600 dark:text-violet-400", "apps" }
                          }
                          div { class: "flex-1 min-w-0",
                              div { class: "text-sm font-medium", "{item.app_name}" }
                              div { class: "text-xs text-zinc-500 dark:text-zinc-400 capitalize", "{item.residue_type}" }
                          }
                          div { class: "text-right",
                              div { class: "text-sm font-semibold", "{format_size(item.size)}" }
                              div { class: "text-xs text-zinc-500 dark:text-zinc-400", "{item.modified}" }
                          }
                      }
                      div { class: "p-3 rounded-xl bg-zinc-50 dark:bg-zinc-800",
                          p { class: "text-xs text-zinc-500 dark:text-zinc-400 mb-1", "Path" }
                          p { class: "text-sm font-mono break-all", "{item.path}" }
                      }
                  }
                  div { class: "flex justify-end gap-2 mt-4",
                      button {
                          class: "px-4 py-2 rounded-lg text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800",
                          onclick: move |_| show_preview.set(None),
                          "Close"
                      }
                  }
              }
          }
      }

      // Confirmation dialog
      if show_confirm() {
          div { class: "fixed inset-0 z-[70] bg-black/60 backdrop-blur-sm flex items-center justify-center",
              div { class: "max-w-sm mx-auto bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 shadow-2xl p-5 mx-4",
                  div { class: "w-12 h-12 rounded-full bg-amber-100 dark:bg-amber-900/30 flex items-center justify-center mb-3",
                      span { class: "material-symbols-rounded text-amber-600 dark:text-amber-400", "warning" }
                  }
                  h3 { class: "font-semibold text-lg", "Clean Selected Residues?" }
                  p { class: "text-sm text-zinc-600 dark:text-zinc-400 mt-2",
                      "This will permanently delete {selected.read().len()} item(s). This action cannot be undone."
                  }
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
                              let sel_items: Vec<ResidueItem> = items.read()
                                  .iter()
                                  .filter(|i| selected.read().contains(&i.path))
                                  .cloned()
                                  .collect();
                              let _ = bridge::invoke_app_command(
                                  "clean_multiple_app_residues",
                                  &serde_json::json!({ "items": sel_items })
                              );
                              selected.set(std::collections::HashSet::new());
                              // Refresh current tab
                              let tab = *active_tab.read();
                              let mut items_clone = items.clone();
                              if let Ok(val) = bridge::invoke_app_command(tab.command(), &serde_json::json!({})) {
                                  if let Ok(resp) = serde_json::from_value::<ApiResponse<Vec<ResidueItem>>>(val) {
                                      if let Some(data) = resp.data {
                                          items_clone.set(data);
                                      }
                                  }
                              }
                          },
                          "Clean"
                      }
                  }
              }
          }
      }
  }
}
