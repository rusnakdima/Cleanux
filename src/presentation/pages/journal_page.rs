//! Journal page — mirrors the template `data-page="journal"` section.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalSize {
  pub bytes: u64,
  pub human: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargestLogFile {
  pub path: String,
  pub size_human: String,
}

fn fmt_bytes(bytes: u64) -> String {
  const GB: u64 = 1024 * 1024 * 1024;
  const MB: u64 = 1024 * 1024;
  if bytes >= GB {
    format!("{:.1} GB", bytes as f64 / GB as f64)
  } else if bytes >= MB {
    format!("{:.1} MB", bytes as f64 / MB as f64)
  } else {
    format!("{} KB", bytes / 1024)
  }
}

#[component]
pub fn JournalPage(state: AppState) -> Element {
  let mut journal_size = use_signal(|| Option::<String>::None);
  let mut var_log_usage = use_signal(|| Option::<u64>::None);
  let mut largest_logs = use_signal(|| Vec::<LargestLogFile>::new());
  let mut is_vacuuming = use_signal(|| false);
  let mut vacuum_status = use_signal(|| String::new());
  let mut vacuum_size = use_signal(|| String::from("500M"));
  let mut vacuum_days = use_signal(|| 30u32);

  // Load data on mount
  use_effect(move || {
    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      // Journal size
      if let Ok(val) = bridge::invoke_app_command("journal_get_size", &serde_json::json!({})) {
        if let Ok(size) = serde_json::from_value::<JournalSize>(val) {
          journal_size.set(Some(size.human));
        }
      }
      // Var/log usage
      if let Ok(val) =
        bridge::invoke_app_command("journal_get_var_log_usage", &serde_json::json!({}))
      {
        if let Ok(bytes) = serde_json::from_value::<u64>(val) {
          var_log_usage.set(Some(bytes));
        }
      }
      // Largest logs
      if let Ok(val) = bridge::invoke_app_command(
        "journal_get_largest_files",
        &serde_json::json!({ "limit": 10 }),
      ) {
        if let Ok(logs) = serde_json::from_value::<Vec<LargestLogFile>>(val) {
          largest_logs.set(logs);
        }
      }
    });
  });

  let handle_vacuum_size = move |_| {
    is_vacuuming.set(true);
    vacuum_status.set("Vacuuming journal...".to_string());
    let size = vacuum_size.read().clone();

    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      match bridge::invoke_app_command(
        "journal_vacuum_by_size",
        &serde_json::json!({ "size": &size }),
      ) {
        Ok(_) => vacuum_status.set(format!("Journal vacuumed to {}", size)),
        Err(e) => vacuum_status.set(format!("Error: {}", e)),
      }
      is_vacuuming.set(false);

      // Refresh
      if let Ok(val) = bridge::invoke_app_command("journal_get_size", &serde_json::json!({})) {
        if let Ok(size) = serde_json::from_value::<JournalSize>(val) {
          journal_size.set(Some(size.human));
        }
      }
    });
  };

  let handle_vacuum_days = move |_| {
    is_vacuuming.set(true);
    vacuum_status.set("Vacuuming journal...".to_string());
    let days = *vacuum_days.read();

    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      match bridge::invoke_app_command(
        "journal_vacuum_by_days",
        &serde_json::json!({ "days": days }),
      ) {
        Ok(_) => vacuum_status.set(format!("Kept last {} days of journal", days)),
        Err(e) => vacuum_status.set(format!("Error: {}", e)),
      }
      is_vacuuming.set(false);

      // Refresh
      if let Ok(val) = bridge::invoke_app_command("journal_get_size", &serde_json::json!({})) {
        if let Ok(size) = serde_json::from_value::<JournalSize>(val) {
          journal_size.set(Some(size.human));
        }
      }
    });
  };

  let journal_size_display = journal_size
    .read()
    .as_ref()
    .map(|s| s.as_str())
    .unwrap_or("-")
    .to_string();
  let var_log_display = var_log_usage
    .read()
    .as_ref()
    .map(|b| fmt_bytes(*b))
    .unwrap_or_else(|| "-".to_string())
    .to_string();

  rsx! {
      section { "data-page": "journal",
          class: "space-y-6",

          // Overview cards
          div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "flex items-center justify-between mb-2",
                      span { class: "text-xs font-medium text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Journal Size" }
                      span { class: "material-symbols-rounded text-violet-500", "article" }
                  }
                  div { class: "flex items-baseline gap-1.5",
                      span { class: "text-3xl font-bold", "{journal_size_display}" }
                  }
                  div { class: "text-xs text-zinc-500 dark:text-zinc-400 mt-1", "/var/log/journal" }
              }
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "flex items-center justify-between mb-2",
                      span { class: "text-xs font-medium text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "/var/log Usage" }
                      span { class: "material-symbols-rounded text-amber-500", "folder" }
                  }
                  div { class: "flex items-baseline gap-1.5",
                      span { class: "text-3xl font-bold", "{var_log_display}" }
                  }
              }
          }

          // Vacuum controls
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              h2 { class: "font-semibold mb-4", "Vacuum Controls" }
              div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                  // By size
                  div { class: "space-y-3",
                      h3 { class: "text-sm font-medium text-zinc-600 dark:text-zinc-400", "By Size" }
                      div { class: "flex items-center gap-3",
                          input {
                              r#type: "text",
                              value: "{vacuum_size.read()}",
                              oninput: move |e| vacuum_size.set(e.value().to_string()),
                              class: "flex-1 px-3 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-violet-500"
                          }
                          button {
                              class: "px-4 py-2 rounded-xl bg-violet-500 text-white font-medium text-sm hover:opacity-90 disabled:opacity-50",
                              disabled: *is_vacuuming.read(),
                              onclick: handle_vacuum_size,
                              "Apply"
                          }
                      }
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400", "e.g. 500M, 1G, 2G" }
                  }
                  // By days
                  div { class: "space-y-3",
                      h3 { class: "text-sm font-medium text-zinc-600 dark:text-zinc-400", "By Days" }
                      div { class: "flex items-center gap-3",
                          input {
                              r#type: "number",
                              value: "{vacuum_days.read()}",
                              oninput: move |e| vacuum_days.set(e.value().parse().unwrap_or(30)),
                              class: "w-24 px-3 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-violet-500"
                          }
                          span { class: "text-sm text-zinc-500", "days" }
                          button {
                              class: "px-4 py-2 rounded-xl bg-violet-500 text-white font-medium text-sm hover:opacity-90 disabled:opacity-50",
                              disabled: *is_vacuuming.read(),
                              onclick: handle_vacuum_days,
                              "Apply"
                          }
                      }
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Keep only the last N days" }
                  }
              }

              if !vacuum_status.read().is_empty() {
                  div { class: "mt-4 p-3 rounded-xl bg-emerald-50 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400 text-sm",
                      "{vacuum_status.read()}"
                  }
              }
          }

          // Largest log files
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              h2 { class: "font-semibold mb-4", "Largest Log Files" }
              if largest_logs.read().is_empty() {
                  div { class: "py-6 text-center text-sm text-zinc-500 dark:text-zinc-400",
                      "No large log files found"
                  }
              } else {
                  div { class: "space-y-2",
                      for log in largest_logs.read().iter() {
                          div { class: "flex items-center justify-between px-4 py-2 rounded-xl hover:bg-zinc-50 dark:hover:bg-zinc-800/50",
                              div { class: "flex-1 min-w-0",
                                  div { class: "text-sm truncate", "{log.path}" }
                              }
                              div { class: "text-sm text-zinc-600 dark:text-zinc-400 flex-shrink-0 ml-4",
                                  "{log.size_human}"
                              }
                          }
                      }
                  }
              }
          }
      }
  }
}
