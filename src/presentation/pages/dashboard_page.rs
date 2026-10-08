//! Dashboard page — mirrors the template `data-page="dashboard"` section.

use crate::app::AppState;
use crate::global_state;
use dioxus::prelude::*;

#[component]
pub fn DashboardPage(state: AppState) -> Element {
  let _categories = global_state::get_categories();
  let scans = global_state::get_scan_history();
  let schedules: Vec<_> = global_state::get_schedules()
    .into_iter()
    .filter(|s| s.enabled)
    .collect();

  // Live monitoring snapshot — refreshed every 5s by the background loop
  // started in `main()`. Falls back to placeholders until the first tick lands.
  let monitor = global_state::get_monitoring_snapshot();
  let (ram_used, ram_total, mem_pct, cpu_pct) = match &monitor {
    Some(s) => (
      s.memory_used_mb as f64 / 1024.0,
      s.memory_total_mb as f64 / 1024.0,
      s.memory_percent,
      s.cpu_percent,
    ),
    None => (0.0, 0.0, 0.0, 0.0),
  };
  let cpu_temp = monitor.as_ref().and_then(|s| s.cpu_temp_c);
  let health_score: u32 = if monitor.is_some() {
    // Simple health heuristic: 100 - mem% (clamped), bonus for low CPU temp.
    let mem_part = (100.0 - mem_pct).clamp(0.0, 100.0) as u32;
    let temp_penalty = match cpu_temp {
      Some(t) if t > 80.0 => 10,
      Some(t) if t > 70.0 => 5,
      _ => 0,
    };
    mem_part.saturating_sub(temp_penalty)
  } else {
    92
  };
  let ram_label = if monitor.is_some() {
    format!(
      "RAM {:.1}/{:.1} GB · CPU {:.0}%",
      ram_used, ram_total, cpu_pct
    )
  } else {
    "Awaiting first sample…".to_string()
  };

  rsx! {
      section { "data-page": "dashboard",
          class: "space-y-6",

          // Stat grid
          div { class: "grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-3",
              StatCard {
                  label: "Disk Usage",
                  icon: "storage",
                  icon_color: "text-emerald-500",
                  value: "412".to_string(),
                  unit: "/ 1000 GB",
                  detail: "41% used".to_string(),
                  bar_pct: 41,
                  bar_gradient: "from-emerald-500 to-cyan-500",
              }
              StatCard {
                  label: "Junk Found",
                  icon: "cleaning_services",
                  icon_color: "text-amber-500",
                  value: "8.42".to_string(),
                  unit: "GB reclaimable",
                  detail: "↓ 1.2 GB since last scan".to_string(),
                  bar_pct: 0,
                  bar_gradient: "",
              }
              StatCard {
                  label: "Freed Total",
                  icon: "history",
                  icon_color: "text-violet-500",
                  value: "146.7".to_string(),
                  unit: "GB lifetime",
                  detail: "Last cleanup 2 days ago".to_string(),
                  bar_pct: 0,
                  bar_gradient: "",
              }
              StatCard {
                  label: "System Health",
                  icon: "monitor_heart",
                  icon_color: "text-sky-500",
                  value: health_score.to_string(),
                  unit: "/ 100 score",
                  detail: ram_label,
                  bar_pct: 0,
                  bar_gradient: "",
              }
          }

          // Quick actions row
          div { class: "grid grid-cols-1 lg:grid-cols-3 gap-4",
              // Quick clean card
              div { class: "lg:col-span-2 bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "flex items-center justify-between mb-4",
                      div {
                          h2 { class: "font-semibold", "Quick Clean" }
                          p { class: "text-xs text-zinc-500 dark:text-zinc-400", "One-tap cleanup of selected categories" }
                      }
                      button {
                          class: "text-xs font-medium px-3 py-1.5 rounded-full bg-zinc-100 dark:bg-zinc-800 hover:bg-zinc-200 dark:hover:bg-zinc-700 flex items-center gap-1",
                          onclick: move |_| { state.navigate(crate::app::Page::Clean); },
                          span { class: "material-symbols-rounded text-sm", "schedule" }
                          "Schedule"
                      }
                  }
                  div { class: "flex items-center gap-3",
                      button {
                          class: "flex-1 bg-gradient-to-r from-cyan-500 to-violet-500 text-white font-medium py-3 rounded-xl flex items-center justify-center gap-2 hover:opacity-90 transition-opacity",
                          onclick: move |_| {
                              // Trigger scan and navigate to Clean page
                              let selected: Vec<String> = global_state::get_categories()
                                  .iter().filter(|c| c.selected).map(|c| c.id.clone()).collect();
                              let _ = crate::bridge::invoke_app_command("scan_cache_categories", &serde_json::json!({ "categories": &selected }));
                              let _ = crate::bridge::invoke_app_command("start_quick_clean", &serde_json::json!({ "categories": &selected }));
                              state.navigate(crate::app::Page::Clean);
                          },
                          span { class: "material-symbols-rounded", "play_arrow" }
                          "Start Cleaning"
                      }
                      button {
                          class: "px-4 py-3 rounded-xl border border-zinc-200 dark:border-zinc-800 hover:bg-zinc-50 dark:hover:bg-zinc-800 transition-colors",
                          onclick: move |_| { state.navigate(crate::app::Page::Clean); },
                          span { class: "material-symbols-rounded", "event" }
                      }
                  }
              }

              // Scheduled jobs
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  h2 { class: "font-semibold mb-3", "Scheduled Jobs" }
                  div { class: "space-y-2 text-sm",
                      if schedules.is_empty() {
                          div { class: "text-xs text-zinc-500", "No active schedules" }
                      } else {
                          for s in schedules.iter().take(3) {
                              div { class: "flex items-center gap-2 text-zinc-600 dark:text-zinc-400",
                                  span { class: "material-symbols-rounded text-cyan-500 text-base", "schedule" }
                                  span { class: "truncate", "{s.name}" }
                              }
                          }
                      }
                  }
              }
          }

          // Recent scans
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800",
              div { class: "flex items-center justify-between p-5 pb-3",
                  h2 { class: "font-semibold", "Recent Scans" }
                  button {
                      class: "text-xs text-cyan-600 dark:text-cyan-400 font-medium",
                      onclick: move |_| { state.navigate(crate::app::Page::Clean); },
                      "View all"
                  }
              }
              div { class: "divide-y divide-zinc-200 dark:divide-zinc-800",
                  for scan in scans.iter() {
                      ScanRow { scan: scan.clone() }
                  }
              }
          }
      }
  }
}

#[derive(Props, Clone, PartialEq)]
struct StatCardProps {
  label: &'static str,
  icon: &'static str,
  icon_color: &'static str,
  value: String,
  unit: &'static str,
  detail: String,
  bar_pct: u8,
  bar_gradient: &'static str,
}

#[component]
fn StatCard(props: StatCardProps) -> Element {
  rsx! {
      div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
          div { class: "flex items-center justify-between mb-3",
              span { class: "text-xs font-medium text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "{props.label}" }
              span { class: "material-symbols-rounded {props.icon_color}", "{props.icon}" }
          }
          div { class: "flex items-baseline gap-1.5 mb-2",
              span { class: "text-3xl font-bold", "{props.value}" }
              span { class: "text-sm text-zinc-500 dark:text-zinc-400", "{props.unit}" }
          }
          if props.bar_pct > 0 {
              div { class: "w-full h-2 rounded-full bg-zinc-100 dark:bg-zinc-800 overflow-hidden",
                  div { class: "h-full bg-gradient-to-r {props.bar_gradient} rounded-full", style: "width: {props.bar_pct}%", "" }
              }
          }
          p { class: "text-xs text-zinc-500 dark:text-zinc-400 mt-2", "{props.detail}" }
      }
  }
}

#[component]
fn ScanRow(scan: crate::global_state::ScanEntry) -> Element {
  let status_color = if scan.status == "complete" {
    "bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-400"
  } else {
    "bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-400"
  };
  let status_icon = if scan.status == "complete" {
    "check_circle"
  } else {
    "cancel"
  };

  rsx! {
      div { class: "flex items-center gap-3 px-5 py-3 hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors",
          div { class: "w-9 h-9 rounded-lg {status_color} flex items-center justify-center flex-shrink-0",
              span { class: "material-symbols-rounded text-lg", "{status_icon}" }
          }
          div { class: "flex-1 min-w-0",
              div { class: "text-sm font-medium truncate", "{scan.date}" }
              div { class: "text-xs text-zinc-500 dark:text-zinc-400", "{scan.files} files · {scan.duration}" }
          }
          div { class: "text-right flex-shrink-0",
              div { class: "text-sm font-semibold text-emerald-600 dark:text-emerald-400", "{scan.freed}" }
              div { class: "text-xs text-zinc-500 dark:text-zinc-400", "freed" }
          }
      }
  }
}
