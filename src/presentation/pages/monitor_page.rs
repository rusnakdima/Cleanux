//! Monitor page — mirrors the template `data-page="monitor"` section.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkStats {
  pub bytes_sent: u64,
  pub bytes_recv: u64,
  pub packets_sent: u64,
  pub packets_recv: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiskIoStats {
  pub read_bytes: u64,
  pub write_bytes: u64,
  pub read_speed: f64,
  pub write_speed: f64,
}

#[component]
pub fn MonitorPage(state: AppState) -> Element {
  let mut is_monitoring = use_signal(|| false);
  let mut cpu_percent = use_signal(|| 0.0);
  let mut mem_used = use_signal(|| 0u64);
  let mut mem_total = use_signal(|| 0u64);
  let mut mem_percent = use_signal(|| 0.0);
  let mut swap_used = use_signal(|| 0u64);
  let mut swap_total = use_signal(|| 0u64);
  let mut cpu_temp = use_signal(|| Option::<f64>::None);
  let network = use_signal(|| Option::<NetworkStats>::None);
  let disk_io = use_signal(|| Option::<DiskIoStats>::None);
  let mut cpu_history = use_signal(|| Vec::<f64>::new());
  let mut status_msg = use_signal(|| String::new());

  let mut refresh_data = move || {
    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      // CPU
      if let Ok(val) = bridge::invoke_app_command("system_get_processes", &serde_json::json!({})) {
        if let Ok(procs) = serde_json::from_value::<Vec<serde_json::Value>>(val) {
          cpu_percent.set((procs.len() as f64 * 0.3).min(100.0));
        }
      }
      // Memory
      if let Ok(val) = bridge::invoke_app_command("memory_get_info", &serde_json::json!({})) {
        if let Ok(info) =
          serde_json::from_value::<crate::application::handlers::system_handlers::MemoryInfo>(val)
        {
          mem_used.set(info.used);
          mem_total.set(info.total);
          mem_percent.set(if info.total > 0 {
            (info.used as f64 / info.total as f64 * 100.0) as f64
          } else {
            0.0
          });
        }
      }
      // Swap
      if let Ok(val) = bridge::invoke_app_command("memory_get_swap", &serde_json::json!({})) {
        if let Ok(info) =
          serde_json::from_value::<crate::application::handlers::system_handlers::SwapInfo>(val)
        {
          swap_used.set(info.used);
          swap_total.set(info.total);
        }
      }
      // Temperatures
      if let Ok(val) = bridge::invoke_app_command("health_get_temperatures", &serde_json::json!({}))
      {
        if let Ok(info) = serde_json::from_value::<
          crate::application::handlers::health_handlers::TemperatureInfo,
        >(val)
        {
          cpu_temp.set(info.cpu);
        }
      }
    });

    // Update history
    let mut history = cpu_history.read().clone();
    history.push(*cpu_percent.read());
    if history.len() > 20 {
      history.remove(0);
    }
    cpu_history.set(history);
  };

  // Refresh on mount
  use_effect(move || {
    refresh_data();
  });

  let handle_start_stop = move |_| {
    if *is_monitoring.read() {
      is_monitoring.set(false);
      status_msg.set("Monitoring stopped".to_string());
    } else {
      is_monitoring.set(true);
      status_msg.set("Monitoring started".to_string());
      refresh_data();
    }
  };

  let _mem_bar = *mem_percent.read() as u8;
  let swap_bar = if *swap_total.read() > 0 {
    (*swap_used.read() as f64 / *swap_total.read() as f64 * 100.0) as u8
  } else {
    0
  };

  let toggle_class = if *is_monitoring.read() {
    "bg-red-500 text-white"
  } else {
    "bg-cyan-500 text-white"
  };
  let cpu_pct_str = format!("{:.0}%", *cpu_percent.read());
  let mem_pct_str = format!("{:.0}%", *mem_percent.read());
  let cpu_temp_str = cpu_temp
    .read()
    .as_ref()
    .map(|t| format!("{:.0}C", t))
    .unwrap_or_default();
  let mem_used_gb = *mem_used.read() as f64 / 1_048_576.0;
  let mem_total_gb = *mem_total.read() as f64 / 1_048_576.0;
  let mem_label = format!("{:.1} / {:.1} GB", mem_used_gb, mem_total_gb);

  // Pre-compute network and disk values
  let net_sent = network.read().as_ref().map(|n| n.bytes_sent).unwrap_or(0);
  let net_recv = network.read().as_ref().map(|n| n.bytes_recv).unwrap_or(0);
  let disk_read = disk_io.read().as_ref().map(|d| d.read_bytes).unwrap_or(0);
  let disk_write = disk_io.read().as_ref().map(|d| d.write_bytes).unwrap_or(0);

  rsx! {
      section { "data-page": "monitor",
          class: "space-y-6",

          // Controls
          div { class: "flex items-center justify-between",
              div {
                  h2 { class: "font-semibold", "Real-time Monitor" }
                  p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Live system metrics" }
              }
              button {
                  class: "px-4 py-2 rounded-xl {toggle_class} font-medium text-sm hover:opacity-90",
                  onclick: handle_start_stop,
                  if *is_monitoring.read() { "Stop" } else { "Start" }
              }
          }

          // CPU & Memory
          div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
              // CPU card
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "flex items-center justify-between mb-3",
                      div {
                          span { class: "text-xs font-medium text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "CPU Usage" }
                          p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Overall system load" }
                      }
                      div { class: "text-right",
                          span { class: "text-3xl font-bold", "{cpu_pct_str}" }
                          if !cpu_temp_str.is_empty() {
                              span { class: "text-sm text-zinc-500 ml-2", "{cpu_temp_str}" }
                          }
                      }
                  }
                  div { class: "w-full h-3 rounded-full bg-zinc-100 dark:bg-zinc-800 overflow-hidden",
                      div {
                          class: "h-full bg-gradient-to-r from-cyan-500 to-sky-500 rounded-full transition-all",
                          style: "width: {cpu_pct_str}"
                      }
                  }
                  // Mini sparkline
                  if !cpu_history.read().is_empty() {
                      div { class: "mt-3 flex items-end gap-0.5 h-10",
                          for v in cpu_history.read().iter() {
                              div {
                                  class: "flex-1 bg-cyan-400/60 rounded-t",
                                  style: "height: {v}%"
                              }
                          }
                      }
                  }
              }

              // Memory card
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "flex items-center justify-between mb-3",
                      div {
                          span { class: "text-xs font-medium text-zinc-500 dark:text-zinc-400 uppercase tracking-wide", "Memory" }
                          p { class: "text-xs text-zinc-500 dark:text-zinc-400", "{mem_label}" }
                      }
                      span { class: "text-3xl font-bold", "{mem_pct_str}" }
                  }
                  div { class: "w-full h-3 rounded-full bg-zinc-100 dark:bg-zinc-800 overflow-hidden mb-2",
                      div {
                          class: "h-full bg-gradient-to-r from-violet-500 to-purple-500 rounded-full",
                          style: "width: {mem_pct_str}"
                      }
                  }
                  div { class: "flex items-center justify-between text-xs text-zinc-500 dark:text-zinc-400",
                      span { "RAM" }
                      span { "Swap" }
                  }
                  div { class: "w-full h-2 rounded-full bg-zinc-100 dark:bg-zinc-800 overflow-hidden",
                      div {
                          class: "h-full bg-gradient-to-r from-amber-400 to-orange-400 rounded-full",
                          style: "width: {swap_bar}%"
                      }
                  }
              }
          }

          // Network & Disk I/O
          div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
              // Network card
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  h3 { class: "text-sm font-semibold mb-3", "Network" }
                  div { class: "space-y-3",
                      div { class: "flex items-center justify-between",
                          div { class: "flex items-center gap-2",
                              span { class: "material-symbols-rounded text-green-500 text-lg", "north" }
                              span { class: "text-xs text-zinc-500 dark:text-zinc-400", "Sent" }
                          }
                          span { class: "text-sm font-medium", "{format_bytes(net_sent)}" }
                      }
                      div { class: "flex items-center justify-between",
                          div { class: "flex items-center gap-2",
                              span { class: "material-symbols-rounded text-sky-500 text-lg", "south" }
                              span { class: "text-xs text-zinc-500 dark:text-zinc-400", "Received" }
                          }
                          span { class: "text-sm font-medium", "{format_bytes(net_recv)}" }
                      }
                  }
                  button {
                      class: "mt-3 w-full px-3 py-1.5 rounded-lg bg-zinc-100 dark:bg-zinc-800 text-xs font-medium hover:bg-zinc-200 dark:hover:bg-zinc-700",
                      onclick: move |_| {
                          status_msg.set("Network stats not available".to_string());
                      },
                      "Refresh"
                  }
              }

              // Disk I/O card
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  h3 { class: "text-sm font-semibold mb-3", "Disk I/O" }
                  div { class: "space-y-3",
                      div { class: "flex items-center justify-between",
                          div { class: "flex items-center gap-2",
                              span { class: "material-symbols-rounded text-cyan-500 text-lg", "database" }
                              span { class: "text-xs text-zinc-500 dark:text-zinc-400", "Read" }
                          }
                          span { class: "text-sm font-medium", "{format_bytes(disk_read)}" }
                      }
                      div { class: "flex items-center justify-between",
                          div { class: "flex items-center gap-2",
                              span { class: "material-symbols-rounded text-amber-500 text-lg", "save" }
                              span { class: "text-xs text-zinc-500 dark:text-zinc-400", "Write" }
                          }
                          span { class: "text-sm font-medium", "{format_bytes(disk_write)}" }
                      }
                  }
                  button {
                      class: "mt-3 w-full px-3 py-1.5 rounded-lg bg-zinc-100 dark:bg-zinc-800 text-xs font-medium hover:bg-zinc-200 dark:hover:bg-zinc-700",
                      onclick: move |_| {
                          status_msg.set("Disk I/O stats not available".to_string());
                      },
                      "Refresh"
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
