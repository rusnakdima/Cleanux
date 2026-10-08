//! Processes page — mirrors the template `data-page="processes"` section.

use crate::app::AppState;
use crate::bridge;
use crate::infrastructure::process_service::ProcessEntry;
use dioxus::prelude::*;

const PAGE_SIZE: usize = 50;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct SystemProcessEntry {
  pub pid: u32,
  pub name: String,
  pub cpu_percent: f64,
  pub memory_bytes: u64,
  pub user: String,
  pub state: String,
}

impl From<ProcessEntry> for SystemProcessEntry {
  fn from(p: ProcessEntry) -> Self {
    Self {
      pid: p.pid,
      name: p.name,
      cpu_percent: p.cpu_percent,
      memory_bytes: p.memory_bytes,
      user: p.user,
      state: p.state,
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortColumn {
  Pid,
  Name,
  Cpu,
  Memory,
  User,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortDir {
  Asc,
  Desc,
}

#[component]
pub fn ProcessesPage(state: AppState) -> Element {
  let processes = use_signal(|| Vec::<SystemProcessEntry>::new());
  let is_loading = use_signal(|| false);
  let mut search_query = use_signal(|| String::new());
  let mut sort_col = use_signal(|| SortColumn::Cpu);
  let mut sort_dir = use_signal(|| SortDir::Desc);
  let mut current_page = use_signal(|| 0);
  let mut selected_pids = use_signal(|| std::collections::HashSet::<u32>::new());
  let mut kill_target = use_signal(|| Option::<u32>::None);
  let mut bulk_kill_confirm = use_signal(|| false);

  // Load processes on mount
  use_effect(move || {
    let mut processes_clone = processes.clone();
    let mut is_loading_clone = is_loading.clone();
    is_loading_clone.set(true);
    match bridge::invoke_app_command("system_get_processes", &serde_json::json!({})) {
      Ok(val) => {
        if let Ok(entries) = serde_json::from_value::<Vec<ProcessEntry>>(val) {
          processes_clone.set(entries.into_iter().map(SystemProcessEntry::from).collect());
        }
      }
      Err(_) => {}
    }
    is_loading_clone.set(false);
  });

  let query = search_query.read().to_lowercase();
  let col = *sort_col.read();
  let dir = *sort_dir.read();
  let page = *current_page.read();

  let filtered: Vec<SystemProcessEntry> = processes
    .read()
    .iter()
    .filter(|p| {
      if query.is_empty() {
        return true;
      }
      p.name.to_lowercase().contains(&query)
        || p.pid.to_string().contains(&query)
        || p.user.to_lowercase().contains(&query)
    })
    .cloned()
    .collect();

  let sorted: Vec<SystemProcessEntry> = {
    let mut v = filtered;
    match col {
      SortColumn::Pid => v.sort_by(|a, b| {
        let ord = a.pid.cmp(&b.pid);
        if dir == SortDir::Asc {
          ord
        } else {
          ord.reverse()
        }
      }),
      SortColumn::Name => v.sort_by(|a, b| {
        let ord = a.name.to_lowercase().cmp(&b.name.to_lowercase());
        if dir == SortDir::Asc {
          ord
        } else {
          ord.reverse()
        }
      }),
      SortColumn::Cpu => v.sort_by(|a, b| {
        let ord = a
          .cpu_percent
          .partial_cmp(&b.cpu_percent)
          .unwrap_or(std::cmp::Ordering::Equal);
        if dir == SortDir::Asc {
          ord
        } else {
          ord.reverse()
        }
      }),
      SortColumn::Memory => v.sort_by(|a, b| {
        let ord = a.memory_bytes.cmp(&b.memory_bytes);
        if dir == SortDir::Asc {
          ord
        } else {
          ord.reverse()
        }
      }),
      SortColumn::User => v.sort_by(|a, b| {
        let ord = a.user.to_lowercase().cmp(&b.user.to_lowercase());
        if dir == SortDir::Asc {
          ord
        } else {
          ord.reverse()
        }
      }),
    }
    v
  };

  let total_pages = (sorted.len() + PAGE_SIZE - 1) / PAGE_SIZE;
  let paginated: Vec<SystemProcessEntry> = sorted
    .iter()
    .skip(page * PAGE_SIZE)
    .take(PAGE_SIZE)
    .cloned()
    .collect();

  let selected = selected_pids.read().clone();
  let all_selected = !paginated.is_empty() && paginated.iter().all(|p| selected.contains(&p.pid));
  let _some_selected = paginated.iter().any(|p| selected.contains(&p.pid)) && !all_selected;

  let sort_arrow = |c: SortColumn| -> &'static str {
    if col == c {
      if dir == SortDir::Asc {
        "↑"
      } else {
        "↓"
      }
    } else {
      "↕"
    }
  };

  let _format_bytes = |bytes: u64| -> String {
    if bytes >= 1_073_741_824 {
      format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
      format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
      format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
      format!("{bytes} B")
    }
  };

  rsx! {
      section { "data-page": "processes",
          class: "space-y-6",

          // Header with search and refresh
          div { class: "flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4",
              div {
                  h1 { class: "text-xl font-bold", "Processes" }
                  p { class: "text-sm text-zinc-500 dark:text-zinc-400", "Manage running system processes" }
              }
              div { class: "flex items-center gap-3 w-full sm:w-auto",
                  div { class: "relative flex-1 sm:flex-none sm:w-64",
                      span { class: "material-symbols-rounded absolute left-3 top-1/2 -translate-y-1/2 text-zinc-400 text-lg", "search" }
                      input {
                          r#type: "text",
                          placeholder: "Search processes...",
                          value: "{search_query.read()}",
                          class: "w-full pl-10 pr-4 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-900 text-sm focus:outline-none focus:border-cyan-500",
                          oninput: move |e| {
                              search_query.set(e.value().to_string());
                              current_page.set(0);
                          }
                      }
                  }
                  button {
                      class: "flex items-center gap-2 px-4 py-2 rounded-xl bg-cyan-600 hover:bg-cyan-700 text-white text-sm font-medium transition-colors",
                      disabled: *is_loading.read(),
                      onclick: move |_| {
                          let mut processes_clone = processes.clone();
                          let mut is_loading_clone = is_loading.clone();
                          is_loading_clone.set(true);
                          async move {
                              match bridge::invoke_app_command("system_get_processes", &serde_json::json!({})) {
                                  Ok(val) => {
                                      if let Ok(entries) = serde_json::from_value::<Vec<ProcessEntry>>(val) {
                                          processes_clone.set(entries.into_iter().map(SystemProcessEntry::from).collect());
                                      }
                                  }
                                  Err(_) => {}
                              }
                              is_loading_clone.set(false);
                          }
                      },
                      if *is_loading.read() {
                          span { class: "material-symbols-rounded text-base animate-spin", "progress_activity" }
                      } else {
                          span { class: "material-symbols-rounded text-base", "refresh" }
                      }
                      "Refresh"
                  }
              }
          }

          // Bulk action bar
          if !selected.is_empty() {
              div { class: "flex items-center justify-between p-4 rounded-xl bg-rose-50 dark:bg-rose-900/20 border border-rose-200 dark:border-rose-800",
                  span { class: "text-sm font-medium text-rose-700 dark:text-rose-400", "{selected.len()} process(es) selected" }
                  button {
                      class: "flex items-center gap-2 px-4 py-2 rounded-lg bg-rose-600 hover:bg-rose-700 text-white text-sm font-medium transition-colors",
                      onclick: move |_| bulk_kill_confirm.set(true),
                      span { class: "material-symbols-rounded text-base", "cancel" }
                      "Kill Selected"
                  }
              }
          }

          // Process table
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 overflow-hidden",
              div { class: "overflow-x-auto",
                  table { class: "w-full text-sm",
                      thead { class: "bg-zinc-50 dark:bg-zinc-800/50",
                          tr {
                              th { class: "px-4 py-3 text-left",
                                  label { class: "flex items-center gap-2 cursor-pointer",
                                      input {
                                          r#type: "checkbox",
                                          checked: all_selected,
                                          class: "w-4 h-4 rounded border-zinc-300 dark:border-zinc-600 text-cyan-500 focus:ring-cyan-500",
                                          onchange: move |_| {
                                              let mut selected_clone = selected_pids.clone();
                                              if all_selected {
                                                  // Deselect all on this page
                                                  for p in paginated.iter() {
                                                      selected_clone.write().remove(&p.pid);
                                                  }
                                              } else {
                                                  // Select all on this page
                                                  for p in paginated.iter() {
                                                      selected_clone.write().insert(p.pid);
                                                  }
                                              }
                                          }
                                      }
                                  }
                              }
                              th { class: "px-4 py-3 text-left font-medium text-zinc-600 dark:text-zinc-400 cursor-pointer hover:text-cyan-500",
                                  onclick: move |_| {
                                      if col == SortColumn::Pid {
                                          sort_dir.set(if dir == SortDir::Asc { SortDir::Desc } else { SortDir::Asc });
                                      } else {
                                          sort_col.set(SortColumn::Pid);
                                          sort_dir.set(SortDir::Asc);
                                      }
                                  },
                                  "PID ", span { class: "text-xs", "{sort_arrow(SortColumn::Pid)}" }
                              }
                              th { class: "px-4 py-3 text-left font-medium text-zinc-600 dark:text-zinc-400 cursor-pointer hover:text-cyan-500",
                                  onclick: move |_| {
                                      if col == SortColumn::Name {
                                          sort_dir.set(if dir == SortDir::Asc { SortDir::Desc } else { SortDir::Asc });
                                      } else {
                                          sort_col.set(SortColumn::Name);
                                          sort_dir.set(SortDir::Asc);
                                      }
                                  },
                                  "Name ", span { class: "text-xs", "{sort_arrow(SortColumn::Name)}" }
                              }
                              th { class: "px-4 py-3 text-left font-medium text-zinc-600 dark:text-zinc-400 cursor-pointer hover:text-cyan-500",
                                  onclick: move |_| {
                                      if col == SortColumn::Cpu {
                                          sort_dir.set(if dir == SortDir::Asc { SortDir::Desc } else { SortDir::Asc });
                                      } else {
                                          sort_col.set(SortColumn::Cpu);
                                          sort_dir.set(SortDir::Desc);
                                      }
                                  },
                                  "CPU% ", span { class: "text-xs", "{sort_arrow(SortColumn::Cpu)}" }
                              }
                              th { class: "px-4 py-3 text-left font-medium text-zinc-600 dark:text-zinc-400 cursor-pointer hover:text-cyan-500",
                                  onclick: move |_| {
                                      if col == SortColumn::Memory {
                                          sort_dir.set(if dir == SortDir::Asc { SortDir::Desc } else { SortDir::Asc });
                                      } else {
                                          sort_col.set(SortColumn::Memory);
                                          sort_dir.set(SortDir::Desc);
                                      }
                                  },
                                  "Memory ", span { class: "text-xs", "{sort_arrow(SortColumn::Memory)}" }
                              }
                              th { class: "px-4 py-3 text-left font-medium text-zinc-600 dark:text-zinc-400 cursor-pointer hover:text-cyan-500",
                                  onclick: move |_| {
                                      if col == SortColumn::User {
                                          sort_dir.set(if dir == SortDir::Asc { SortDir::Desc } else { SortDir::Asc });
                                      } else {
                                          sort_col.set(SortColumn::User);
                                          sort_dir.set(SortDir::Asc);
                                      }
                                  },
                                  "User ", span { class: "text-xs", "{sort_arrow(SortColumn::User)}" }
                              }
                              th { class: "px-4 py-3 text-right font-medium text-zinc-600 dark:text-zinc-400", "Actions" }
                          }
                      }
                      tbody { class: "divide-y divide-zinc-100 dark:divide-zinc-800",
                          if paginated.is_empty() {
                              tr {
                                  td { class: "px-4 py-8 text-center text-zinc-400 col-span-7",
                                      if *is_loading.read() {
                                          "Loading processes..."
                                      } else {
                                          "No processes found"
                                      }
                                  }
                              }
                          } else {
                              for process in paginated.iter() {
                                  ProcessRow {
                                      process: process.clone(),
                                      is_selected: selected.contains(&process.pid),
                                      on_toggle_select: move |pid: u32| {
                                          let mut selected_clone = selected_pids.clone();
                                          if selected_clone.read().contains(&pid) {
                                              selected_clone.write().remove(&pid);
                                          } else {
                                              selected_clone.write().insert(pid);
                                          }
                                      },
                                      on_kill: move |pid: u32| {
                                          kill_target.set(Some(pid));
                                      }
                                  }
                              }
                          }
                      }
                  }
              }

              // Pagination
              if total_pages > 1 {
                  div { class: "flex items-center justify-between px-4 py-3 border-t border-zinc-100 dark:border-zinc-800",
                      span { class: "text-sm text-zinc-500 dark:text-zinc-400",
                          "Showing {page * PAGE_SIZE + 1}-{((page + 1) * PAGE_SIZE).min(sorted.len())} of {sorted.len()}"
                      }
                      div { class: "flex items-center gap-2",
                          button {
                              class: "p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 disabled:opacity-50",
                              disabled: page == 0,
                              onclick: move |_| current_page.set(page.saturating_sub(1)),
                              span { class: "material-symbols-rounded", "chevron_left" }
                          }
                          for p in 0..total_pages.min(10) {
                              button {
                                  class: format!("w-8 h-8 rounded-lg text-sm font-medium transition-colors {}",
                                      if p == page {
                                          "bg-cyan-600 text-white"
                                      } else {
                                          "hover:bg-zinc-100 dark:hover:bg-zinc-800"
                                      }
                                  ),
                                  onclick: move |_| current_page.set(p),
                                  "{p + 1}"
                              }
                          }
                          if total_pages > 10 {
                              span { class: "text-zinc-400", "..." }
                              button {
                                  class: "w-8 h-8 rounded-lg text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800",
                                  onclick: move |_| current_page.set(total_pages - 1),
                                  "{total_pages}"
                              }
                          }
                          button {
                              class: "p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 disabled:opacity-50",
                              disabled: page >= total_pages - 1,
                              onclick: move |_| current_page.set((page + 1).min(total_pages - 1)),
                              span { class: "material-symbols-rounded", "chevron_right" }
                          }
                      }
                  }
              }
          }
      }

      // Single kill confirmation dialog
      if let Some(target_pid) = *kill_target.read() {
          div { class: "fixed inset-0 z-[70] bg-black/60 backdrop-blur-sm flex items-center justify-center",
              div { class: "max-w-sm mx-auto bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 shadow-2xl p-5 mx-4",
                  div { class: "w-12 h-12 rounded-full bg-rose-100 dark:bg-rose-900/30 flex items-center justify-center mb-3",
                      span { class: "material-symbols-rounded text-rose-600 dark:text-rose-400", "warning" }
                  }
                  h3 { class: "font-semibold text-lg", "Kill Process" }
                  p { class: "text-sm text-zinc-600 dark:text-zinc-400 mt-2",
                      "Are you sure you want to kill process with PID ", strong { "{target_pid}" }, "?"
                  }
                  p { class: "text-xs text-rose-500 mt-1", "This action cannot be undone." }
                  div { class: "flex justify-end gap-2 mt-4",
                      button {
                          class: "px-4 py-2 rounded-lg text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800",
                          onclick: move |_| kill_target.set(None),
                          "Cancel"
                      }
                      button {
                          class: "px-4 py-2 rounded-lg text-white text-sm font-medium bg-rose-600 hover:bg-rose-700",
                          onclick: move |_| {
                              let pid = target_pid;
                              kill_target.set(None);
                              async move {
                                  let _ = bridge::invoke_app_command("system_kill_process", &serde_json::json!({ "pid": pid }));
                                  // Refresh process list
                                  let mut processes_clone = processes.clone();
                                  match bridge::invoke_app_command("system_get_processes", &serde_json::json!({})) {
                                      Ok(val) => {
                                          if let Ok(entries) = serde_json::from_value::<Vec<ProcessEntry>>(val) {
                                              processes_clone.set(entries.into_iter().map(SystemProcessEntry::from).collect());
                                          }
                                      }
                                      Err(_) => {}
                                  }
                              }
                          },
                          "Kill Process"
                      }
                  }
              }
          }
      }

      // Bulk kill confirmation dialog
      if *bulk_kill_confirm.read() {
          div { class: "fixed inset-0 z-[70] bg-black/60 backdrop-blur-sm flex items-center justify-center",
              div { class: "max-w-sm mx-auto bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 shadow-2xl p-5 mx-4",
                  div { class: "w-12 h-12 rounded-full bg-rose-100 dark:bg-rose-900/30 flex items-center justify-center mb-3",
                      span { class: "material-symbols-rounded text-rose-600 dark:text-rose-400", "warning" }
                  }
                  h3 { class: "font-semibold text-lg", "Kill Selected Processes" }
                  p { class: "text-sm text-zinc-600 dark:text-zinc-400 mt-2",
                      "Are you sure you want to kill ", strong { "{selected.len()}" }, " selected process(es)?"
                  }
                  p { class: "text-xs text-rose-500 mt-1", "This action cannot be undone." }
                  div { class: "flex justify-end gap-2 mt-4",
                      button {
                          class: "px-4 py-2 rounded-lg text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800",
                          onclick: move |_| bulk_kill_confirm.set(false),
                          "Cancel"
                      }
                      button {
                          class: "px-4 py-2 rounded-lg text-white text-sm font-medium bg-rose-600 hover:bg-rose-700",
                          onclick: move |_| {
                              let pids: Vec<u32> = selected.iter().cloned().collect();
                              bulk_kill_confirm.set(false);
                              selected_pids.write().clear();
                              async move {
                                  let _ = bridge::invoke_app_command("system_kill_selected_processes", &serde_json::json!({ "pids": pids }));
                                  // Refresh process list
                                  let mut processes_clone = processes.clone();
                                  match bridge::invoke_app_command("system_get_processes", &serde_json::json!({})) {
                                      Ok(val) => {
                                          if let Ok(entries) = serde_json::from_value::<Vec<ProcessEntry>>(val) {
                                              processes_clone.set(entries.into_iter().map(SystemProcessEntry::from).collect());
                                          }
                                      }
                                      Err(_) => {}
                                  }
                              }
                          },
                          "Kill All Selected"
                      }
                  }
              }
          }
      }
  }
}

#[derive(Clone, Props, PartialEq)]
struct ProcessRowProps {
  process: SystemProcessEntry,
  is_selected: bool,
  on_toggle_select: EventHandler<u32>,
  on_kill: EventHandler<u32>,
}

#[component]
fn ProcessRow(props: ProcessRowProps) -> Element {
  let p = props.process;
  let cpu_class = if p.cpu_percent >= 80.0 {
    "text-rose-500"
  } else if p.cpu_percent >= 50.0 {
    "text-amber-500"
  } else {
    "text-zinc-600 dark:text-zinc-400"
  };

  let format_bytes = |bytes: u64| -> String {
    if bytes >= 1_073_741_824 {
      format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
      format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
      format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
      format!("{bytes} B")
    }
  };

  rsx! {
      tr { class: "hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors",
          td { class: "px-4 py-3",
              input {
                  r#type: "checkbox",
                  checked: props.is_selected,
                  class: "w-4 h-4 rounded border-zinc-300 dark:border-zinc-600 text-cyan-500 focus:ring-cyan-500",
                  onchange: move |_| props.on_toggle_select.call(p.pid)
              }
          }
          td { class: "px-4 py-3 font-mono text-zinc-600 dark:text-zinc-400", "{p.pid}" }
          td { class: "px-4 py-3 font-medium truncate max-w-[200px]", "{p.name}" }
          td { class: "px-4 py-3 {cpu_class} font-medium", "{p.cpu_percent:.1}%" }
          td { class: "px-4 py-3 text-zinc-600 dark:text-zinc-400", "{format_bytes(p.memory_bytes)}" }
          td { class: "px-4 py-3 text-zinc-600 dark:text-zinc-400 truncate max-w-[120px]", "{p.user}" }
          td { class: "px-4 py-3 text-right",
              button {
                  class: "p-2 rounded-lg hover:bg-rose-100 dark:hover:bg-rose-900/30 text-zinc-400 hover:text-rose-500 transition-colors",
                  title: "Kill process",
                  onclick: move |_| props.on_kill.call(p.pid),
                  span { class: "material-symbols-rounded text-base", "cancel" }
              }
          }
      }
  }
}
