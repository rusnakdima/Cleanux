//! Junk Cleaner page — mirrors the template `data-page="junk-cleaner"` section.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JunkItem {
  pub path: String,
  pub size: u64,
}

fn format_bytes(bytes: u64) -> String {
  const GB: u64 = 1024 * 1024 * 1024;
  const MB: u64 = 1024 * 1024;
  const KB: u64 = 1024;
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
pub fn JunkCleanerPage(state: AppState) -> Element {
  let mut junk_summary =
    use_signal(|| Option::<crate::application::handlers::cleaner_handlers::JunkSummary>::None);
  let mut is_scanning = use_signal(|| false);
  let mut is_cleaning = use_signal(|| false);
  let mut scan_status = use_signal(|| "Ready to scan".to_string());
  let mut selected_paths = use_signal(|| Vec::<String>::new());
  let mut scan_results = use_signal(|| Vec::<JunkItem>::new());
  let mut clean_result = use_signal(|| String::new());

  // Load summary on mount
  use_effect(move || {
    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      match bridge::invoke_app_command("clean_get_junk_summary", &serde_json::json!({})) {
        Ok(val) => {
          if let Ok(summary) = serde_json::from_value::<
            crate::application::handlers::cleaner_handlers::JunkSummary,
          >(val)
          {
            junk_summary.set(Some(summary));
          }
        }
        Err(_) => {}
      }
    });
  });

  let handle_scan = move |_| {
    is_scanning.set(true);
    scan_status.set("Scanning for junk...".to_string());
    scan_results.set(vec![]);
    selected_paths.set(vec![]);

    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      match bridge::invoke_app_command("clean_scan_cache", &serde_json::json!({})) {
        Ok(val) => {
          if let Ok(items) = serde_json::from_value::<Vec<JunkItem>>(val) {
            scan_results.set(items);
            scan_status.set(format!("Found {} items", scan_results.read().len()));
          } else {
            scan_status.set("Scan complete - no junk found".to_string());
          }
        }
        Err(e) => scan_status.set(format!("Scan error: {}", e)),
      }
      is_scanning.set(false);
    });
  };

  let handle_clean = move |_| {
    is_cleaning.set(true);
    clean_result.set("Cleaning...".to_string());

    let to_clean = selected_paths.read().clone();

    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      let mut cleaned = 0u64;
      for path in to_clean.iter() {
        match bridge::invoke_app_command(
          "storage_delete_file",
          &serde_json::json!({ "path": path }),
        ) {
          Ok(_) => cleaned += 1,
          Err(_) => {}
        }
      }

      // Refresh summary
      match bridge::invoke_app_command("clean_get_junk_summary", &serde_json::json!({})) {
        Ok(val) => {
          if let Ok(summary) = serde_json::from_value::<
            crate::application::handlers::cleaner_handlers::JunkSummary,
          >(val)
          {
            junk_summary.set(Some(summary));
          }
        }
        Err(_) => {}
      }

      scan_results.set(vec![]);
      selected_paths.set(vec![]);
      clean_result.set(format!("Cleaned {} items", cleaned));
      is_cleaning.set(false);
    });
  };

  let summary = junk_summary.read();

  rsx! {
      section { "data-page": "junk-cleaner",
          class: "space-y-6",

          // Summary cards
          div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4",
              if let Some(s) = summary.as_ref() {
                  div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                      div { class: "flex items-center justify-between mb-2",
                          span { class: "text-xs font-medium text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Cache" }
                          span { class: "material-symbols-rounded text-amber-500", "language" }
                      }
                      div { class: "flex items-baseline gap-1.5 mb-1",
                          span { class: "text-3xl font-bold", "{format_bytes(s.cache)}" }
                      }
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Browser & app caches" }
                  }
                  div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                      div { class: "flex items-center justify-between mb-2",
                          span { class: "text-xs font-medium text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Logs" }
                          span { class: "material-symbols-rounded text-sky-500", "article" }
                      }
                      div { class: "flex items-baseline gap-1.5 mb-1",
                          span { class: "text-3xl font-bold", "{format_bytes(s.logs)}" }
                      }
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400", "System logs" }
                  }
                  div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                      div { class: "flex items-center justify-between mb-2",
                          span { class: "text-xs font-medium text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Large Files" }
                          span { class: "material-symbols-rounded text-violet-500", "folder" }
                      }
                      div { class: "flex items-baseline gap-1.5 mb-1",
                          span { class: "text-3xl font-bold", "{format_bytes(s.large_files)}" }
                      }
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Duplicate & large files" }
                  }
              } else {
                  div { class: "col-span-3 bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                      div { class: "flex items-center justify-between",
                          div {
                              h2 { class: "font-semibold", "Junk Summary" }
                              p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Run a scan to detect junk files" }
                          }
                          button {
                              class: "px-4 py-2 rounded-xl bg-zinc-900 dark:bg-white text-white dark:text-zinc-900 font-medium text-sm hover:opacity-90",
                              onclick: handle_scan,
                              "Scan"
                          }
                      }
                  }
              }
          }

          // Actions row
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              div { class: "flex items-center justify-between mb-4",
                  div {
                      h2 { class: "font-semibold", "Scan & Clean" }
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400", "{scan_status}" }
                  }
                  div { class: "flex items-center gap-3",
                      button {
                          class: "px-4 py-2 rounded-xl bg-zinc-100 dark:bg-zinc-800 hover:bg-zinc-200 dark:hover:bg-zinc-700 font-medium text-sm disabled:opacity-50",
                          disabled: *is_scanning.read(),
                          onclick: handle_scan,
                          if *is_scanning.read() { "Scanning..." } else { "Scan" }
                      }
                      button {
                          class: "px-4 py-2 rounded-xl bg-gradient-to-r from-amber-500 to-orange-500 text-white font-medium text-sm hover:opacity-90 disabled:opacity-50",
                          disabled: scan_results.read().is_empty() || *is_cleaning.read(),
                          onclick: handle_clean,
                          if *is_cleaning.read() { "Cleaning..." } else { "Clean Selected" }
                      }
                  }
              }

              // Scan results list
              if !scan_results.read().is_empty() {
                  div { class: "space-y-2",
                      for item in scan_results.read().iter() {
                          JunkItemRow {
                              item: item.clone(),
                              selected: selected_paths.read().contains(&item.path),
                              on_toggle: move |path: String| {
                                  let mut current = selected_paths.read().clone();
                                  if current.contains(&path) {
                                      current.retain(|p| p != &path);
                                  } else {
                                      current.push(path);
                                  }
                                  selected_paths.set(current);
                              }
                          }
                      }
                  }
              } else {
                  div { class: "py-8 text-center text-sm text-zinc-500 dark:text-zinc-400",
                      "Run a scan to find junk files"
                  }
              }

              if !clean_result.read().is_empty() {
                  div { class: "mt-4 p-3 rounded-xl bg-emerald-50 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400 text-sm",
                      "{clean_result.read()}"
                  }
              }
          }
      }
  }
}

#[derive(Props, Clone, PartialEq)]
struct JunkItemRowProps {
  item: JunkItem,
  selected: bool,
  on_toggle: Callback<String>,
}

#[component]
fn JunkItemRow(props: JunkItemRowProps) -> Element {
  let item_path = props.item.path.clone();
  let item_path2 = item_path.clone();
  rsx! {
      div {
          class: "flex items-center gap-3 px-4 py-3 rounded-xl hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors cursor-pointer",
          onclick: move |_| props.on_toggle.call(item_path.clone()),
          input {
              r#type: "checkbox",
              checked: props.selected,
              class: "w-4 h-4 rounded border-zinc-300 dark:border-zinc-600 text-amber-500 focus:ring-amber-500",
              onchange: move |_| props.on_toggle.call(item_path2.clone()),
          }
          div { class: "flex-1 min-w-0",
              div { class: "text-sm font-medium truncate", "{props.item.path}" }
              div { class: "text-xs text-zinc-500 dark:text-zinc-400", "cache" }
          }
          div { class: "text-sm text-zinc-600 dark:text-zinc-400 flex-shrink-0",
              "{format_bytes(props.item.size)}"
          }
      }
  }
}
