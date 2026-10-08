//! Trash Cleaner page — mirrors the template `data-page="trash-cleaner"` section.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrashItem {
  pub name: String,
  pub path: String,
  pub size: u64,
  pub deleted_at: String,
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
pub fn TrashCleanerPage(state: AppState) -> Element {
  let mut trash_items = use_signal(|| Vec::<TrashItem>::new());
  let mut total_size = use_signal(|| 0u64);
  let mut is_scanning = use_signal(|| false);
  let mut status_msg = use_signal(|| String::new());

  let handle_scan = move |_| {
    is_scanning.set(true);
    status_msg.set("Scanning trash...".to_string());

    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      match bridge::invoke_app_command("clean_scan_trash", &serde_json::json!({})) {
        Ok(val) => {
          if let Ok(items) = serde_json::from_value::<Vec<TrashItem>>(val) {
            let total: u64 = items.iter().map(|i| i.size).sum();
            trash_items.set(items);
            total_size.set(total);
            status_msg.set(format!(
              "Found {} items ({})",
              trash_items.read().len(),
              format_bytes(total)
            ));
          }
        }
        Err(e) => status_msg.set(format!("Error: {}", e)),
      }
      is_scanning.set(false);
    });
  };

  let handle_empty = move |_| {
    let items = trash_items.read().clone();
    if items.is_empty() {
      status_msg.set("Trash is already empty".to_string());
      return;
    }

    status_msg.set("Emptying trash...".to_string());
    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      let mut deleted = 0u64;
      for item in &items {
        match bridge::invoke_app_command(
          "storage_delete_file",
          &serde_json::json!({ "path": &item.path }),
        ) {
          Ok(_) => deleted += 1,
          Err(_) => {}
        }
      }
      trash_items.set(vec![]);
      total_size.set(0);
      status_msg.set(format!("Emptied {} items from trash", deleted));
    });
  };

  let items = trash_items.read();
  let items_count = items.len();
  let size_display = format_bytes(*total_size.read());

  rsx! {
      section { "data-page": "trash-cleaner",
          class: "space-y-6",

          // Header with stats
          div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4",
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "material-symbols-rounded text-3xl text-amber-500 mb-2", "delete" }
                  div { class: "text-2xl font-bold", "{size_display}" }
                  div { class: "text-xs text-zinc-500 dark:text-zinc-400", "Trash Size" }
              }
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "material-symbols-rounded text-3xl text-cyan-500 mb-2", "inventory_2" }
                  div { class: "text-2xl font-bold", "{items_count}" }
                  div { class: "text-xs text-zinc-500 dark:text-zinc-400", "Items in Trash" }
              }
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5 flex flex-col items-center justify-center",
                  button {
                      class: "px-6 py-3 rounded-xl bg-gradient-to-r from-amber-500 to-orange-500 text-white font-semibold text-sm hover:opacity-90 disabled:opacity-50",
                      disabled: *is_scanning.read() || items.is_empty(),
                      onclick: handle_empty,
                      "Empty Trash"
                  }
              }
          }

          // Scan button
          div { class: "flex items-center justify-between",
              div {
                  h2 { class: "font-semibold", "Trash Contents" }
                  p { class: "text-xs text-zinc-500 dark:text-zinc-400", "~/.local/share/Trash" }
              }
              button {
                  class: "px-4 py-2 rounded-xl bg-zinc-100 dark:bg-zinc-800 text-sm font-medium hover:bg-zinc-200 dark:hover:bg-zinc-700 flex items-center gap-2",
                  disabled: *is_scanning.read(),
                  onclick: handle_scan,
                  if *is_scanning.read() {
                      span { class: "material-symbols-rounded text-sm animate-spin", "progress_activity" }
                      "Scanning..."
                  } else {
                      span { class: "material-symbols-rounded text-sm", "search" }
                      "Scan Trash"
                  }
              }
          }

          // Trash items list
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 divide-y divide-zinc-200 dark:divide-zinc-800",
              if items.is_empty() {
                  div { class: "p-8 text-center",
                      div { class: "material-symbols-rounded text-4xl text-zinc-300 dark:text-zinc-600 mx-auto mb-3", "delete_empty" }
                      p { class: "text-sm text-zinc-500 dark:text-zinc-400", "Trash is empty" }
                  }
              } else {
                  div { class: "divide-y divide-zinc-100 dark:divide-zinc-800",
                      for item in items.iter() {
                          div { class: "flex items-center gap-4 px-4 py-3 hover:bg-zinc-50 dark:hover:bg-zinc-800/50",
                              span { class: "material-symbols-rounded text-zinc-400", "text_snippet" }
                              div { class: "flex-1 min-w-0",
                                  div { class: "text-sm font-medium truncate", "{item.name}" }
                                  div { class: "text-xs text-zinc-500 dark:text-zinc-400 truncate", "{item.path}" }
                              }
                              div { class: "text-sm text-zinc-600 dark:text-zinc-400 flex-shrink-0",
                                  "{format_bytes(item.size)}"
                              }
                          }
                      }
                  }
              }
          }

          if !status_msg.read().is_empty() {
              div { class: "p-3 rounded-xl bg-cyan-50 dark:bg-cyan-900/20 text-cyan-700 dark:text-cyan-400 text-sm",
                  "{status_msg.read()}"
              }
          }
      }
  }
}
