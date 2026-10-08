//! Memory optimizer page — mirrors the template `data-page="memory-optimizer"` section.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MemoryInfo {
  #[allow(dead_code)]
  pub total: u64,
  pub used: u64,
  pub available: u64,
  pub cached: u64,
  pub memory_percent: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SwapInfo {
  #[allow(dead_code)]
  pub total: u64,
  pub used: u64,
  pub free: u64,
  pub swap_percent: f32,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct MemProcess {
  pub pid: u32,
  pub name: String,
  pub memory_mb: f64,
  pub cpu_percent: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MemProcessInfo {
  pub processes: Vec<MemProcess>,
  #[allow(dead_code)]
  pub total_memory_mb: u64,
  #[allow(dead_code)]
  pub used_memory_mb: u64,
}

fn format_bytes(bytes: u64) -> String {
  const GIB: u64 = 1024 * 1024 * 1024;
  const MIB: u64 = 1024 * 1024;
  if bytes >= GIB {
    format!("{:.1} GB", bytes as f64 / GIB as f64)
  } else {
    format!("{:.0} MB", bytes as f64 / MIB as f64)
  }
}

fn format_mb(mb: f64) -> String {
  if mb >= 1024.0 {
    format!("{:.1} GB", mb / 1024.0)
  } else {
    format!("{:.0} MB", mb)
  }
}

#[derive(Clone, Props, PartialEq)]
struct MemProcessRowProps {
  proc: MemProcess,
  kill_confirm: Signal<Option<u32>>,
}

#[component]
fn MemProcessRow(props: MemProcessRowProps) -> Element {
  let mut kill_confirm = props.kill_confirm.clone();
  let pid = props.proc.pid;
  rsx! {
      tr { class: "hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors",
          td { class: "px-4 py-3 font-mono text-xs text-zinc-500 dark:text-zinc-400", "{props.proc.pid}" }
          td { class: "px-4 py-3 font-medium truncate max-w-[200px]", "{props.proc.name}" }
          td { class: "px-4 py-3 text-right", "{format_mb(props.proc.memory_mb)}" }
          td { class: "px-4 py-3 text-right", "{props.proc.cpu_percent:.1}%" }
          td { class: "px-4 py-3 text-center",
              button {
                  onclick: move |_| kill_confirm.set(Some(pid)),
                  class: "p-1.5 rounded hover:bg-red-100 dark:hover:bg-red-900/30 text-red-600 dark:text-red-400 transition-colors",
                  title: "Kill process",
                  span { class: "material-symbols-rounded text-sm", "close" }
              }
          }
      }
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SortBy {
  Memory,
  Cpu,
  Pid,
}

#[component]
pub fn MemoryOptimizerPage(state: AppState) -> Element {
  let memory_info = use_signal(|| Option::<MemoryInfo>::None);
  let swap_info = use_signal(|| Option::<SwapInfo>::None);
  let processes = use_signal(|| Vec::<MemProcess>::new());
  let _auto_refresh = use_signal(|| false);
  let mut sort_by = use_signal(|| SortBy::Memory);
  let mut kill_confirm = use_signal(|| Option::<u32>::None);
  let mut optimizing = use_signal(|| false);
  let mut error_msg = use_signal(|| Option::<String>::None);
  let refreshing = use_signal(|| false);

  // Load memory info on mount
  {
    let mut mem_info = memory_info.clone();
    let mut swp_info = swap_info.clone();
    let mut procs = processes.clone();
    use_effect(move || {
      if let Ok(val) = bridge::invoke_app_command("memory_get_info", &serde_json::json!({})) {
        if let Ok(info) =
          serde_json::from_value::<crate::application::handlers::system_handlers::MemoryInfo>(val)
        {
          mem_info.set(Some(MemoryInfo {
            total: info.total,
            used: info.used,
            available: info.available,
            cached: info.cached,
            memory_percent: if info.total > 0 {
              (info.used as f32 / info.total as f32) * 100.0
            } else {
              0.0
            },
          }));
        }
      }
      if let Ok(val) = bridge::invoke_app_command("memory_get_swap", &serde_json::json!({})) {
        if let Ok(info) =
          serde_json::from_value::<crate::application::handlers::system_handlers::SwapInfo>(val)
        {
          swp_info.set(Some(SwapInfo {
            total: info.total,
            used: info.used,
            free: info.total.saturating_sub(info.used),
            swap_percent: if info.total > 0 {
              (info.used as f32 / info.total as f32) * 100.0
            } else {
              0.0
            },
          }));
        }
      }
      if let Ok(val) =
        bridge::invoke_app_command("memory_get_process_memory", &serde_json::json!({}))
      {
        if let Ok(info) = serde_json::from_value::<MemProcessInfo>(val) {
          procs.set(info.processes);
        }
      }
    });
  }

  let mem = memory_info.read();
  let swp = swap_info.read();
  let proc_list = processes.read();
  let sort = *sort_by.read();

  let sorted_procs = {
    let mut p = proc_list.clone();
    match sort {
      SortBy::Memory => p.sort_by(|a, b| {
        b.memory_mb
          .partial_cmp(&a.memory_mb)
          .unwrap_or(std::cmp::Ordering::Equal)
      }),
      SortBy::Cpu => p.sort_by(|a, b| {
        b.cpu_percent
          .partial_cmp(&a.cpu_percent)
          .unwrap_or(std::cmp::Ordering::Equal)
      }),
      SortBy::Pid => p.sort_by(|a, b| a.pid.cmp(&b.pid)),
    }
    p
  };

  let mem_pct = mem.as_ref().map(|m| m.memory_percent).unwrap_or(0.0);
  let swap_pct = swp.as_ref().map(|s| s.swap_percent).unwrap_or(0.0);

  rsx! {
      section {
          "data-page": "memory-optimizer",
          class: "space-y-6",

          // Page header
          div {
              class: "flex items-center justify-between",
              div {
                  h1 { class: "text-2xl font-semibold", "Memory Optimizer" }
                  p { class: "text-sm text-zinc-500 dark:text-zinc-400", "Monitor and optimize system memory usage" }
              }
              div {
                  class: "flex items-center gap-3",
                  button {
                      onclick: move |_| {
                          let mut mem_info = memory_info.clone();
                          let mut swp_info = swap_info.clone();
                          let mut procs = processes.clone();
                          let mut ref_now = refreshing;
                          ref_now.set(true);
                          use_effect(move || {
                              if let Ok(val) = bridge::invoke_app_command("memory_get_info", &serde_json::json!({})) {
                                  if let Ok(info) = serde_json::from_value::<crate::application::handlers::system_handlers::MemoryInfo>(val) {
                                      mem_info.set(Some(MemoryInfo {
                                          total: info.total,
                                          used: info.used,
                                          available: info.available,
                                          cached: info.cached,
                                          memory_percent: if info.total > 0 { (info.used as f32 / info.total as f32) * 100.0 } else { 0.0 },
                                      }));
                                  }
                              }
                              if let Ok(val) = bridge::invoke_app_command("memory_get_swap", &serde_json::json!({})) {
                                  if let Ok(info) = serde_json::from_value::<crate::application::handlers::system_handlers::SwapInfo>(val) {
                                      swp_info.set(Some(SwapInfo {
                                          total: info.total,
                                          used: info.used,
                                          free: info.total.saturating_sub(info.used),
                                          swap_percent: if info.total > 0 { (info.used as f32 / info.total as f32) * 100.0 } else { 0.0 },
                                      }));
                                  }
                              }
                              if let Ok(val) = bridge::invoke_app_command("memory_get_process_memory", &serde_json::json!({})) {
                                  if let Ok(info) = serde_json::from_value::<MemProcessInfo>(val) {
                                      procs.set(info.processes);
                                  }
                              }
                              ref_now.set(false);
                          });
                      },
                      class: "flex items-center gap-2 px-3 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 hover:bg-zinc-100 dark:hover:bg-zinc-800 text-sm transition-colors",
                      span { class: "material-symbols-rounded text-sm", "refresh" }
                      span { "Refresh" }
                  }
                  button {
                      onclick: move |_| {
                          optimizing.set(true);
                          error_msg.set(None);
                          match bridge::invoke_app_command("memory_optimize", &serde_json::json!({})) {
                              Ok(_) => {
                                  let mut mem_info = memory_info.clone();
                                  if let Ok(val) = bridge::invoke_app_command("memory_get_info", &serde_json::json!({})) {
                                      if let Ok(info) = serde_json::from_value::<crate::application::handlers::system_handlers::MemoryInfo>(val) {
                                          mem_info.set(Some(MemoryInfo {
                                              total: info.total,
                                              used: info.used,
                                              available: info.available,
                                              cached: info.cached,
                                              memory_percent: if info.total > 0 { (info.used as f32 / info.total as f32) * 100.0 } else { 0.0 },
                                          }));
                                      }
                                  }
                              }
                              Err(e) => error_msg.set(Some(e.to_string())),
                          }
                          optimizing.set(false);
                      },
                      disabled: *optimizing.read(),
                      class: "flex items-center gap-2 px-4 py-2 bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 text-white rounded-lg text-sm font-medium transition-colors",
                      if *optimizing.read() {
                          span { class: "material-symbols-rounded text-sm animate-spin", "progress_activity" }
                          span { "Optimizing..." }
                      } else {
                          span { class: "material-symbols-rounded text-sm", "memory" }
                          span { "Optimize Memory" }
                      }
                  }
              }
          }

          // Error display
          if let Some(err) = error_msg.read().as_ref() {
              div {
                  class: "p-3 rounded-lg bg-red-500/10 border border-red-500/30 text-red-600 dark:text-red-400 text-sm",
                  "{err}"
              }
          }

          // Gauge cards row
          div {
              class: "grid grid-cols-1 md:grid-cols-2 gap-4",

              // Memory gauge card
              div {
                  class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 p-5",
                  h2 { class: "text-sm font-medium text-zinc-500 dark:text-zinc-400 mb-4", "Memory Usage" }
                  div {
                      class: "flex items-center justify-center",
                      MemoryGauge { percent: mem_pct }
                  }
                  div {
                      class: "mt-4 grid grid-cols-3 gap-2 text-center text-xs text-zinc-500 dark:text-zinc-400",
                      div {
                          span { "Used" }
                          br {}
                          span { class: "font-semibold text-zinc-900 dark:text-zinc-100", "{format_bytes(mem.as_ref().map(|m| m.used).unwrap_or(0))}" }
                      }
                      div {
                          span { "Cached" }
                          br {}
                          span { class: "font-semibold text-zinc-900 dark:text-zinc-100", "{format_bytes(mem.as_ref().map(|m| m.cached).unwrap_or(0))}" }
                      }
                      div {
                          span { "Available" }
                          br {}
                          span { class: "font-semibold text-zinc-900 dark:text-zinc-100", "{format_bytes(mem.as_ref().map(|m| m.available).unwrap_or(0))}" }
                      }
                  }
              }

              // Swap gauge card
              div {
                  class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 p-5",
                  h2 { class: "text-sm font-medium text-zinc-500 dark:text-zinc-400 mb-4", "Swap Usage" }
                  div {
                      class: "flex items-center justify-center",
                      SwapGauge { percent: swap_pct }
                  }
                  div {
                      class: "mt-4 grid grid-cols-2 gap-2 text-center text-xs text-zinc-500 dark:text-zinc-400",
                      div {
                          span { "Used" }
                          br {}
                          span { class: "font-semibold text-zinc-900 dark:text-zinc-100", "{format_bytes(swp.as_ref().map(|s| s.used).unwrap_or(0))}" }
                      }
                      div {
                          span { "Free" }
                          br {}
                          span { class: "font-semibold text-zinc-900 dark:text-zinc-100", "{format_bytes(swp.as_ref().map(|s| s.free).unwrap_or(0))}" }
                      }
                  }
              }
          }

          // Process list
          div {
              class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 overflow-hidden",
              div {
                  class: "px-5 py-4 border-b border-zinc-200 dark:border-zinc-800 flex items-center justify-between",
                  h2 { class: "font-semibold", "Process Memory" }
                  div {
                      class: "flex items-center gap-1 text-xs text-zinc-500 dark:text-zinc-400",
                      "Sort by:",
                      button {
                          onclick: move |_| sort_by.set(SortBy::Memory),
                          class: if sort == SortBy::Memory { "px-2 py-1 rounded text-indigo-600 dark:text-indigo-400 font-medium" } else { "px-2 py-1 rounded hover:bg-zinc-100 dark:hover:bg-zinc-800" },
                          "Memory"
                      }
                      button {
                          onclick: move |_| sort_by.set(SortBy::Cpu),
                          class: if sort == SortBy::Cpu { "px-2 py-1 rounded text-indigo-600 dark:text-indigo-400 font-medium" } else { "px-2 py-1 rounded hover:bg-zinc-100 dark:hover:bg-zinc-800" },
                          "CPU"
                      }
                      button {
                          onclick: move |_| sort_by.set(SortBy::Pid),
                          class: if sort == SortBy::Pid { "px-2 py-1 rounded text-indigo-600 dark:text-indigo-400 font-medium" } else { "px-2 py-1 rounded hover:bg-zinc-100 dark:hover:bg-zinc-800" },
                          "PID"
                      }
                  }
              }
              div {
                  class: "overflow-x-auto",
                  table {
                      class: "w-full text-sm",
                      thead {
                          class: "bg-zinc-50 dark:bg-zinc-800/50 text-xs text-zinc-500 dark:text-zinc-400 uppercase",
                          tr {
                              th { class: "px-4 py-3 text-left font-medium", "PID" }
                              th { class: "px-4 py-3 text-left font-medium", "Name" }
                              th { class: "px-4 py-3 text-right font-medium", "Memory" }
                              th { class: "px-4 py-3 text-right font-medium", "CPU %" }
                              th { class: "px-4 py-3 text-center font-medium", "Action" }
                          }
                      }
                      tbody {
                          class: "divide-y divide-zinc-100 dark:divide-zinc-800",
                          for proc in sorted_procs.iter().take(50) {
                              MemProcessRow {
                                  key: "{proc.pid}",
                                  proc: proc.clone(),
                                  kill_confirm: kill_confirm
                              }
                          }
                      }
                  }
              }
              if sorted_procs.len() > 50 {
                  div {
                      class: "px-5 py-3 text-xs text-zinc-500 dark:text-zinc-400 border-t border-zinc-100 dark:border-zinc-800",
                      "Showing 50 of {sorted_procs.len()} processes. Kill high-memory processes to free RAM."
                  }
              }
          }
      }

      // Kill confirmation dialog
      if let Some(pid) = *kill_confirm.read() {
          div {
              class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm",
              div {
                  class: "bg-white dark:bg-zinc-900 rounded-xl shadow-2xl p-6 max-w-sm w-full mx-4",
                  h3 { class: "text-lg font-semibold mb-2", "Kill Process?" }
                  p { class: "text-sm text-zinc-500 dark:text-zinc-400 mb-4", "Are you sure you want to terminate process {pid}? Unsaved work may be lost." }
                  div {
                      class: "flex gap-3 justify-end",
                      button {
                          onclick: move |_| kill_confirm.set(None),
                          class: "px-4 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 hover:bg-zinc-100 dark:hover:bg-zinc-800 text-sm transition-colors",
                          "Cancel"
                      }
                      button {
                          onclick: move |_| {
                              let target = pid;
                              kill_confirm.set(None);
                              let _ = bridge::invoke_app_command("system_kill_process", &serde_json::json!({"pid": target}));
                              let mut procs = processes.clone();
                              if let Ok(val) = bridge::invoke_app_command("memory_get_process_memory", &serde_json::json!({})) {
                                  if let Ok(info) = serde_json::from_value::<MemProcessInfo>(val) {
                                      procs.set(info.processes);
                                  }
                              }
                          },
                          class: "px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg text-sm font-medium transition-colors",
                          "Kill Process"
                      }
                  }
              }
          }
      }
  }
}

#[component]
fn MemoryGauge(percent: f32) -> Element {
  let clamped = percent.clamp(0.0, 100.0);
  let radius = 70.0;
  let circumference = 2.0 * std::f64::consts::PI * radius;
  let offset = circumference * (1.0 - clamped as f64 / 100.0);
  let stroke_color = if clamped > 90.0 {
    "#ef4444"
  } else if clamped > 70.0 {
    "#f59e0b"
  } else {
    "#6366f1"
  };

  rsx! {
      div {
          class: "relative w-44 h-44",
          svg {
              width: "176",
              height: "176",
              view_box: "0 0 176 176",
              circle {
                  cx: "88",
                  cy: "88",
                  r: "{radius}",
                  fill: "none",
                  stroke: "currentColor",
                  "stroke-width": "12",
                  class: "text-zinc-200 dark:text-zinc-800",
              }
              circle {
                  cx: "88",
                  cy: "88",
                  r: "{radius}",
                  fill: "none",
                  stroke: "{stroke_color}",
                  "stroke-width": "12",
                  "stroke-linecap": "round",
                  "stroke-dasharray": "{circumference}",
                  "stroke-dashoffset": "{offset}",
                  transform: "rotate(-90 88 88)",
                  class: "transition-all duration-700 ease-out",
              }
          }
          div {
              class: "absolute inset-0 flex flex-col items-center justify-center",
              span { class: "text-3xl font-bold", "{clamped:.0}%" }
              span { class: "text-xs text-zinc-500 dark:text-zinc-400", "used" }
          }
      }
  }
}

#[component]
fn SwapGauge(percent: f32) -> Element {
  let clamped = percent.clamp(0.0, 100.0);
  let radius = 70.0;
  let circumference = 2.0 * std::f64::consts::PI * radius;
  let offset = circumference * (1.0 - clamped as f64 / 100.0);
  let stroke_color = if clamped > 90.0 {
    "#ef4444"
  } else if clamped > 50.0 {
    "#f59e0b"
  } else {
    "#22c55e"
  };

  rsx! {
      div {
          class: "relative w-44 h-44",
          svg {
              width: "176",
              height: "176",
              view_box: "0 0 176 176",
              circle {
                  cx: "88",
                  cy: "88",
                  r: "{radius}",
                  fill: "none",
                  stroke: "currentColor",
                  "stroke-width": "12",
                  class: "text-zinc-200 dark:text-zinc-800",
              }
              circle {
                  cx: "88",
                  cy: "88",
                  r: "{radius}",
                  fill: "none",
                  stroke: "{stroke_color}",
                  "stroke-width": "12",
                  "stroke-linecap": "round",
                  "stroke-dasharray": "{circumference}",
                  "stroke-dashoffset": "{offset}",
                  transform: "rotate(-90 88 88)",
                  class: "transition-all duration-700 ease-out",
              }
          }
          div {
              class: "absolute inset-0 flex flex-col items-center justify-center",
              span { class: "text-3xl font-bold", "{clamped:.0}%" }
              span { class: "text-xs text-zinc-500 dark:text-zinc-400", "used" }
          }
      }
  }
}
