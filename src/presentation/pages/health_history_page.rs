//! Health History page — mirrors the template `data-page="health-history"` section.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HealthSnapshotEntry {
  pub id: String,
  pub date: String,
  pub health_score: f64,
  pub cpu_temp: Option<f64>,
  pub memory_percent: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthTrend {
  pub period: String,
  pub avg_score: f64,
  pub trend: String,
}

#[component]
pub fn HealthHistoryPage(state: AppState) -> Element {
  let mut snapshots = use_signal(|| Vec::<HealthSnapshotEntry>::new());
  let mut trends = use_signal(|| Option::<HealthTrend>::None);
  let mut is_loading = use_signal(|| false);
  let mut compare_id1 = use_signal(|| Option::<String>::None);
  let mut compare_id2 = use_signal(|| Option::<String>::None);
  let mut compare_result = use_signal(|| String::new());

  // Load history on mount
  use_effect(move || {
    is_loading.set(true);
    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      match bridge::invoke_app_command("health_get_history", &serde_json::json!({})) {
        Ok(val) => {
          if let Ok(list) = serde_json::from_value::<Vec<HealthSnapshotEntry>>(val) {
            snapshots.set(list);
          }
        }
        Err(_) => {}
      }
      match bridge::invoke_app_command("health_get_trends", &serde_json::json!({})) {
        Ok(val) => {
          if let Ok(t) = serde_json::from_value::<HealthTrend>(val) {
            trends.set(Some(t));
          }
        }
        Err(_) => {}
      }
      is_loading.set(false);
    });
  });

  let handle_save_snapshot = move |_| {
    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      let _ = bridge::invoke_app_command("health_save_snapshot", &serde_json::json!({}));
      // Refresh list
      match bridge::invoke_app_command("health_get_history", &serde_json::json!({})) {
        Ok(val) => {
          if let Ok(list) = serde_json::from_value::<Vec<HealthSnapshotEntry>>(val) {
            snapshots.set(list);
          }
        }
        Err(_) => {}
      }
    });
  };

  let handle_compare = move |_| {
    let id1 = compare_id1.read().clone();
    let id2 = compare_id2.read().clone();
    if id1.is_none() || id2.is_none() {
      compare_result.set("Select two snapshots to compare".to_string());
      return;
    }

    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      match bridge::invoke_app_command(
        "health_compare_snapshots",
        &serde_json::json!({ "id1": id1.as_ref().unwrap(), "id2": id2.as_ref().unwrap() }),
      ) {
        Ok(val) => {
          if let Ok(msg) = serde_json::from_value::<String>(val) {
            compare_result.set(msg);
          }
        }
        Err(e) => compare_result.set(format!("Compare error: {}", e)),
      }
    });
  };

  let list = snapshots.read();
  let trend = trends.read();
  let avg_score_str = trend
    .as_ref()
    .map(|t| format!("{:.0}", t.avg_score))
    .unwrap_or_default();
  let trend_class = if trend
    .as_ref()
    .map(|t| t.trend == "improving")
    .unwrap_or(false)
  {
    "bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-400"
  } else {
    "bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400"
  };
  let compare1_display = compare_id1
    .read()
    .as_ref()
    .map(|s| s.as_str())
    .unwrap_or("Select first...")
    .to_string();
  let compare2_display = compare_id2
    .read()
    .as_ref()
    .map(|s| s.as_str())
    .unwrap_or("Select second...")
    .to_string();
  let snapshots_count = list.len();

  rsx! {
      section { "data-page": "health-history",
          class: "space-y-6",

          // Trends overview
          if let Some(t) = trend.as_ref() {
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "flex items-center justify-between",
                      div {
                          h2 { class: "font-semibold", "Health Trends" }
                          p { class: "text-xs text-zinc-500 dark:text-zinc-400", "{t.period}" }
                      }
                      div { class: "flex items-center gap-4",
                          div { class: "text-right",
                              div { class: "text-2xl font-bold", "{avg_score_str}" }
                              div { class: "text-xs text-zinc-500 dark:text-zinc-400", "avg score" }
                          }
                          div { class: "px-3 py-1 rounded-full text-sm font-medium {trend_class}",
                              "{t.trend}"
                          }
                      }
                  }
              }
          }

          // Snapshots list
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              div { class: "flex items-center justify-between mb-4",
                  div {
                      h2 { class: "font-semibold", "Saved Snapshots" }
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400", "{snapshots_count} snapshots" }
                  }
                  button {
                      class: "px-4 py-2 rounded-xl bg-zinc-900 dark:bg-white text-white dark:text-zinc-900 font-medium text-sm hover:opacity-90",
                      onclick: handle_save_snapshot,
                      "Save Snapshot"
                  }
              }

              if list.is_empty() {
                  div { class: "py-8 text-center text-sm text-zinc-500 dark:text-zinc-400",
                      "No health snapshots saved yet. Click Save Snapshot to create one."
                  }
              } else {
                  div { class: "space-y-2",
                      for snap in list.iter() {
                          SnapshotRow {
                              snap: snap.clone(),
                              on_select: move |id: String| {
                                  if compare_id1.read().is_none() {
                                      compare_id1.set(Some(id));
                                  } else if compare_id2.read().is_none() {
                                      compare_id2.set(Some(id));
                                  }
                              }
                          }
                      }
                  }
              }
          }

          // Compare section
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              h2 { class: "font-semibold mb-4", "Compare Snapshots" }
              div { class: "flex items-center gap-3 mb-4",
                  div { class: "flex-1 px-3 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 text-sm",
                      "{compare1_display}"
                  }
                  span { class: "material-symbols-rounded text-zinc-400", "compare_arrows" }
                  div { class: "flex-1 px-3 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 text-sm",
                      "{compare2_display}"
                  }
                  button {
                      class: "px-4 py-2 rounded-xl bg-sky-500 text-white font-medium text-sm hover:opacity-90",
                      onclick: handle_compare,
                      "Compare"
                  }
              }
              if !compare_result.read().is_empty() {
                  div { class: "p-3 rounded-xl bg-sky-50 dark:bg-sky-900/20 text-sky-700 dark:text-sky-400 text-sm",
                      "{compare_result.read()}"
                  }
              }
          }
      }
  }
}

#[derive(Props, Clone, PartialEq)]
struct SnapshotRowProps {
  snap: HealthSnapshotEntry,
  on_select: Callback<String>,
}

#[component]
fn SnapshotRow(props: SnapshotRowProps) -> Element {
  let score_color = if props.snap.health_score >= 80.0 {
    "text-emerald-500"
  } else if props.snap.health_score >= 60.0 {
    "text-amber-500"
  } else {
    "text-red-500"
  };
  let score_str = format!("{:.0}", props.snap.health_score);
  let temp_str = props
    .snap
    .cpu_temp
    .map(|t| format!("{:.0}C", t))
    .unwrap_or_default();
  let mem_str = props
    .snap
    .memory_percent
    .map(|m| format!("{:.0}%", m))
    .unwrap_or_default();

  rsx! {
      div {
          class: "flex items-center gap-3 px-4 py-3 rounded-xl hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors cursor-pointer",
          onclick: move |_| props.on_select.call(props.snap.id.clone()),
          div { class: "flex-1 min-w-0",
              div { class: "text-sm font-medium", "{props.snap.date}" }
              if !temp_str.is_empty() {
                  div { class: "text-xs text-zinc-500 dark:text-zinc-400", "CPU: {temp_str}" }
              }
          }
          if !mem_str.is_empty() {
              div { class: "text-xs text-zinc-500 dark:text-zinc-400 flex-shrink-0",
                  "RAM: {mem_str}"
              }
          }
          div { class: "text-xl font-bold {score_color} flex-shrink-0",
              "{score_str}"
          }
      }
  }
}
