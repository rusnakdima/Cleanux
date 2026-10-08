//! Clipboard page — clipboard history management.

use crate::app::AppState;
use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ClipboardItem {
  pub id: u64,
  pub content: String,
  pub content_type: String,
  pub preview: String,
  pub timestamp: i64,
  pub pinned: bool,
}

fn generate_id() -> u64 {
  NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

fn format_timestamp(ts: i64) -> String {
  let now = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .map(|d| d.as_secs() as i64)
    .unwrap_or(0);
  let diff = now - ts;
  if diff < 60 {
    "Just now".to_string()
  } else if diff < 3600 {
    format!("{}m ago", diff / 60)
  } else if diff < 86400 {
    format!("{}h ago", diff / 3600)
  } else {
    format!("{}d ago", diff / 86400)
  }
}

fn _truncate_preview(content: &str, max_len: usize) -> String {
  if content.len() <= max_len {
    content.to_string()
  } else {
    format!("{}...", &content[..max_len])
  }
}

fn _detect_content_type(content: &str) -> String {
  if content.starts_with("file://")
    || content.contains("\n")
      && content
        .lines()
        .all(|l| l.starts_with("/") || l.starts_with("~"))
  {
    "file".to_string()
  } else if content.starts_with("data:image/") {
    "image".to_string()
  } else {
    "text".to_string()
  }
}

#[derive(Clone, Props, PartialEq)]
struct ClipboardItemRowProps {
  item: ClipboardItem,
  on_copy: EventHandler<u64>,
  on_pin: EventHandler<u64>,
  on_delete: EventHandler<u64>,
}

#[component]
fn ClipboardItemRow(props: ClipboardItemRowProps) -> Element {
  let icon = match props.item.content_type.as_str() {
    "text" => "text_fields",
    "image" => "image",
    "file" => "folder",
    _ => "content_paste",
  };

  rsx! {
      div {
          class: "flex items-start gap-4 p-4 bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 hover:border-zinc-300 dark:hover:border-zinc-700 transition-colors group",
          div {
              class: "flex-shrink-0 w-10 h-10 rounded-lg bg-cyan-50 dark:bg-cyan-900/30 flex items-center justify-center",
              span { class: "material-symbols-rounded text-cyan-600 dark:text-cyan-400 text-lg", "{icon}" }
          }
          div { class: "flex-1 min-w-0",
              div { class: "flex items-center gap-2 mb-1",
                  span { class: "text-sm font-medium text-zinc-900 dark:text-zinc-100 truncate", "{props.item.preview}" }
                  if props.item.pinned {
                      span { class: "flex-shrink-0 material-symbols-rounded text-amber-500 text-sm", "push_pin" }
                  }
              }
              div { class: "flex items-center gap-3 text-xs text-zinc-500 dark:text-zinc-400",
                  span { class: "uppercase font-medium", "{props.item.content_type}" }
                  span { "•" }
                  span { "{format_timestamp(props.item.timestamp)}" }
              }
          }
          div { class: "flex-shrink-0 flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity",
              button {
                  class: "p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-500 hover:text-cyan-600 dark:hover:text-cyan-400 transition-colors",
                  title: "Copy again",
                  onclick: move |_| props.on_copy.call(props.item.id),
                  span { class: "material-symbols-rounded text-base", "content_copy" }
              }
              button {
                  class: "p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-500 hover:text-amber-600 dark:hover:text-amber-400 transition-colors",
                  title: if props.item.pinned { "Unpin" } else { "Pin" },
                  onclick: move |_| props.on_pin.call(props.item.id),
                  span { class: "material-symbols-rounded text-base", if props.item.pinned { "push_pin" } else { "push_pin" } }
              }
              button {
                  class: "p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-500 hover:text-red-600 dark:hover:text-red-400 transition-colors",
                  title: "Delete",
                  onclick: move |_| props.on_delete.call(props.item.id),
                  span { class: "material-symbols-rounded text-base", "delete" }
              }
          }
      }
  }
}

#[component]
pub fn ClipboardPage(state: AppState) -> Element {
  let mut clipboard_items = use_signal(|| Vec::<ClipboardItem>::new());
  let mut search_query = use_signal(|| String::new());
  let mut max_history = use_signal(|| 100usize);
  let mut clear_on_reboot = use_signal(|| false);
  let is_monitoring = use_signal(|| false);

  // Load initial demo data
  use_effect(move || {
    let demo_items = vec![
            ClipboardItem {
                id: generate_id(),
                content: "Hello, this is a text clipboard item!".to_string(),
                content_type: "text".to_string(),
                preview: "Hello, this is a text clipbo...".to_string(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs() as i64)
                    .unwrap_or(0)
                    - 300,
                pinned: false,
            },
            ClipboardItem {
                id: generate_id(),
                content: "file:///home/user/Documents/report.pdf".to_string(),
                content_type: "file".to_string(),
                preview: "/home/user/Documents/report.pdf".to_string(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs() as i64)
                    .unwrap_or(0)
                    - 3600,
                pinned: true,
            },
            ClipboardItem {
                id: generate_id(),
                content: "Important meeting notes from yesterday's standup with the team discussing the new feature roadmap.".to_string(),
                content_type: "text".to_string(),
                preview: "Important meeting notes from ye...".to_string(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs() as i64)
                    .unwrap_or(0)
                    - 86400,
                pinned: false,
            },
        ];
    clipboard_items.set(demo_items);
  });

  let handle_clear_all = {
    let mut clipboard_items_clone = clipboard_items.clone();
    move |_| {
      let mut items = clipboard_items_clone.write();
      items.retain(|i| i.pinned);
    }
  };

  let handle_toggle_monitoring = {
    let mut is_monitoring_clone = is_monitoring.clone();
    move |_| {
      let current = *is_monitoring_clone.read();
      is_monitoring_clone.set(!current);
    }
  };

  let query = search_query.read().to_lowercase();
  let filtered_items: Vec<ClipboardItem> = clipboard_items
    .read()
    .iter()
    .filter(|item| {
      if query.is_empty() {
        return true;
      }
      item.content.to_lowercase().contains(&query)
        || item.content_type.to_lowercase().contains(&query)
    })
    .cloned()
    .collect();

  let pinned_count = clipboard_items.read().iter().filter(|i| i.pinned).count();
  let unpinned_count = clipboard_items.read().iter().filter(|i| !i.pinned).count();

  rsx! {
      section { "data-page": "clipboard",
          class: "space-y-6",

          // Header
          div { class: "flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4",
              div {
                  h1 { class: "text-xl font-bold", "Clipboard" }
                  p { class: "text-sm text-zinc-500 dark:text-zinc-400", "Track and manage your clipboard history" }
              }
              div { class: "flex items-center gap-3",
                  button {
                      class: if *is_monitoring.read() {
                          "flex items-center gap-2 px-4 py-2 rounded-xl bg-red-500 hover:bg-red-600 text-white text-sm font-medium transition-colors"
                      } else {
                          "flex items-center gap-2 px-4 py-2 rounded-xl bg-cyan-600 hover:bg-cyan-700 text-white text-sm font-medium transition-colors"
                      },
                      onclick: handle_toggle_monitoring,
                      span { class: "material-symbols-rounded text-base", if *is_monitoring.read() { "stop" } else { "play_arrow" } }
                      if *is_monitoring.read() { "Stop Monitoring" } else { "Start Monitoring" }
                  }
                  button {
                      class: "flex items-center gap-2 px-4 py-2 rounded-xl bg-red-500 hover:bg-red-600 text-white text-sm font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                      disabled: unpinned_count == 0,
                      onclick: handle_clear_all,
                      span { class: "material-symbols-rounded text-base", "delete_sweep" }
                      "Clear All"
                  }
              }
          }

          // Stats and Settings Row
          div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4",
              div { class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 p-4",
                  div { class: "flex items-center gap-3",
                      div { class: "w-10 h-10 rounded-lg bg-cyan-50 dark:bg-cyan-900/30 flex items-center justify-center",
                          span { class: "material-symbols-rounded text-cyan-600 dark:text-cyan-400", "content_paste" }
                      }
                      div {
                          div { class: "text-2xl font-bold", "{clipboard_items.read().len()}" }
                          div { class: "text-xs text-zinc-500 dark:text-zinc-400", "Total items" }
                      }
                  }
              }
              div { class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 p-4",
                  div { class: "flex items-center gap-3",
                      div { class: "w-10 h-10 rounded-lg bg-amber-50 dark:bg-amber-900/30 flex items-center justify-center",
                          span { class: "material-symbols-rounded text-amber-600 dark:text-amber-400", "push_pin" }
                      }
                      div {
                          div { class: "text-2xl font-bold", "{pinned_count}" }
                          div { class: "text-xs text-zinc-500 dark:text-zinc-400", "Pinned" }
                      }
                  }
              }
              div { class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 p-4",
                  div { class: "flex items-center justify-between",
                      div { class: "flex items-center gap-3",
                          div { class: "w-10 h-10 rounded-lg bg-zinc-100 dark:bg-zinc-800 flex items-center justify-center",
                              span { class: "material-symbols-rounded text-zinc-600 dark:text-zinc-400", "settings" }
                          }
                          div {
                              div { class: "text-sm font-medium", "Max History" }
                              div { class: "text-xs text-zinc-500 dark:text-zinc-400", "{*max_history.read()} items" }
                          }
                      }
                      div { class: "flex items-center gap-2",
                          button {
                              class: "p-1.5 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300",
                              onclick: move |_| {
                                  let mut val = max_history.write();
                                  if *val > 10 { *val -= 10; }
                              },
                              span { class: "material-symbols-rounded text-base", "remove" }
                          }
                          button {
                              class: "p-1.5 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300",
                              onclick: move |_| {
                                  let mut val = max_history.write();
                                  if *val < 1000 { *val += 10; }
                              },
                              span { class: "material-symbols-rounded text-base", "add" }
                          }
                      }
                  }
              }
          }

          // Clear on Reboot Toggle
          div { class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 p-4",
              label { class: "flex items-center justify-between cursor-pointer",
                  div { class: "flex items-center gap-3",
                      div { class: "w-10 h-10 rounded-lg bg-purple-50 dark:bg-purple-900/30 flex items-center justify-center",
                          span { class: "material-symbols-rounded text-purple-600 dark:text-purple-400", "restart_alt" }
                      }
                      div {
                          div { class: "font-medium", "Clear on Reboot" }
                          div { class: "text-xs text-zinc-500 dark:text-zinc-400", "Automatically clear clipboard history when the app restarts" }
                      }
                  }
                  input {
                      r#type: "checkbox",
                      checked: *clear_on_reboot.read(),
                      class: "sr-only peer",
                      onchange: move |e| clear_on_reboot.set(e.checked()),
                  }
                  div { class: "w-11 h-6 bg-zinc-200 dark:bg-zinc-700 peer-checked:bg-cyan-500 rounded-full relative transition-colors after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:w-5 after:h-5 after:bg-white after:rounded-full after:transition-transform peer-checked:after:translate-x-5", "" }
              }
          }

          // Search
          div { class: "relative",
              span { class: "material-symbols-rounded absolute left-3 top-1/2 -translate-y-1/2 text-zinc-400 text-lg", "search" }
              input {
                  r#type: "text",
                  placeholder: "Search clipboard history...",
                  value: "{search_query.read()}",
                  class: "w-full pl-10 pr-4 py-3 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-900 text-sm focus:outline-none focus:border-cyan-500",
                  oninput: move |e| search_query.set(e.value().to_string()),
              }
          }

          // Loading / Empty state
          if filtered_items.is_empty() {
              div { class: "flex flex-col items-center justify-center py-16 text-center",
                  div { class: "w-16 h-16 rounded-full bg-zinc-100 dark:bg-zinc-800 flex items-center justify-center mb-4",
                      span { class: "material-symbols-rounded text-zinc-400 text-3xl", "content_paste_off" }
                  }
                  div { class: "text-lg font-medium text-zinc-900 dark:text-zinc-100", "No clipboard items" }
                  div { class: "text-sm text-zinc-500 dark:text-zinc-400 mt-1",
                      if query.is_empty() {
                          "Start copying text to build your history"
                      } else {
                          "No items match your search"
                      }
                  }
              }
          } else {
              div { class: "space-y-2",
                  for item in filtered_items {
                      ClipboardItemRow {
                          key: "{item.id}",
                          item: item.clone(),
                          on_copy: move |id| {
                              let clipboard_clone = clipboard_items.clone();
                              if clipboard_clone.read().iter().any(|i| i.id == id) {
                                  tracing::info!("Copied item {} to clipboard", id);
                              }
                          },
                          on_pin: move |id| {
                              let mut clipboard_clone = clipboard_items.clone();
                              let mut items = clipboard_clone.write();
                              if let Some(i) = items.iter_mut().find(|i| i.id == id) {
                                  i.pinned = !i.pinned;
                              }
                          },
                          on_delete: move |id| {
                              let mut clipboard_clone = clipboard_items.clone();
                              let mut items = clipboard_clone.write();
                              items.retain(|i| i.id != id);
                          },
                      }
                  }
              }
          }
      }
  }
}
