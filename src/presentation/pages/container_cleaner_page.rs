//! Container Cleaner page — mirrors the template `data-page="container-cleaner"` section.
//!
//! Manages Docker and Podman containers, images, and volumes.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

fn format_bytes(bytes: u64) -> String {
  if bytes >= 1_073_741_824 {
    format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
  } else if bytes >= 1_048_576 {
    format!("{:.1} MB", bytes as f64 / 1_048_576.0)
  } else if bytes >= 1024 {
    format!("{:.1} KB", bytes as f64 / 1024.0)
  } else {
    format!("{} B", bytes)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContainerSummary {
  docker: DockerInfo,
  podman: PodmanInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DockerInfo {
  available: bool,
  images: u64,
  containers: u64,
  volumes: u64,
  size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PodmanInfo {
  available: bool,
  images: u64,
  containers: u64,
  size: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContainerType {
  Docker,
  Podman,
}

impl ContainerType {
  fn label(&self) -> &'static str {
    match self {
      ContainerType::Docker => "Docker",
      ContainerType::Podman => "Podman",
    }
  }
  fn icon(&self) -> &'static str {
    match self {
      ContainerType::Docker => "sailing",
      ContainerType::Podman => "inventory_2",
    }
  }
}

#[component]
pub fn ContainerCleanerPage(state: AppState) -> Element {
  let mut active_type = use_signal(|| ContainerType::Docker);
  let mut docker_info = use_signal(|| Option::<DockerInfo>::None);
  let mut podman_info = use_signal(|| Option::<PodmanInfo>::None);
  let mut is_loading = use_signal(|| false);
  let mut status_msg = use_signal(|| String::new());
  let mut last_cleaned = use_signal(|| Option::<String>::None);

  // Load initial summary
  use_effect(move || {
    is_loading.set(true);
    if let Ok(resp) =
      bridge::invoke_app_command("clean_get_container_summary", &serde_json::json!({}))
    {
      if let Ok(summary) = serde_json::from_value::<ContainerSummary>(resp) {
        docker_info.set(Some(summary.docker));
        podman_info.set(Some(summary.podman));
      }
    }
    is_loading.set(false);
  });

  let docker_available = docker_info
    .read()
    .as_ref()
    .map(|i| i.available)
    .unwrap_or(false);
  let podman_available = podman_info
    .read()
    .as_ref()
    .map(|i| i.available)
    .unwrap_or(false);

  rsx! {
      section { "data-page": "container-cleaner",
          div { class: "max-w-5xl mx-auto space-y-6",
              div { class: "flex items-center justify-between",
                  div {
                      h2 { class: "text-2xl font-bold", "Container Cleaner" }
                      p { class: "text-sm text-zinc-500 dark:text-zinc-400", "Manage Docker and Podman containers, images, and volumes" }
                  }
                  if let Some(ref t) = *last_cleaned.read() {
                      div { class: "text-xs text-zinc-500", "Last cleaned: {t}" }
                  }
              }

              // Type Selector
              div { class: "flex gap-1 bg-zinc-100 dark:bg-zinc-800 rounded-xl p-1 max-w-md",
                  for ct in [ContainerType::Docker, ContainerType::Podman] {
                      button {
                          class: format!(
                              "flex-1 flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg text-sm font-medium transition-colors {}",
                              if *active_type.read() == ct {
                                  "bg-white dark:bg-zinc-700 text-zinc-900 dark:text-white shadow-sm"
                              } else {
                                  "text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-white"
                              }
                          ),
                          onclick: move |_| active_type.set(ct),
                          span { class: "material-symbols-rounded text-xl", "{ct.icon()}" }
                          "{ct.label()}"
                      }
                  }
              }

              // Info Cards
              if *active_type.read() == ContainerType::Docker {
                  if let Some(ref info) = *docker_info.read() {
                      div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                          SummaryCard { label: "Images", value: "{info.images}", icon: "photo_library" }
                          SummaryCard { label: "Containers", value: "{info.containers}", icon: "view_in_ar" }
                          SummaryCard { label: "Volumes", value: "{info.volumes}", icon: "hard_disk" }
                          SummaryCard { label: "Size", value: info.size.clone(), icon: "storage" }
                      }
                      if !info.available {
                          div { class: "bg-amber-50 dark:bg-amber-950 border border-amber-200 dark:border-amber-800 rounded-xl p-4",
                              p { class: "text-sm text-amber-700 dark:text-amber-300", "Docker is not available on this system" }
                          }
                      }
                  }
              } else {
                  if let Some(ref info) = *podman_info.read() {
                      div { class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                          SummaryCard { label: "Images", value: "{info.images}", icon: "photo_library" }
                          SummaryCard { label: "Containers", value: "{info.containers}", icon: "view_in_ar" }
                          SummaryCard { label: "Size", value: info.size.clone(), icon: "storage" }
                      }
                      if !info.available {
                          div { class: "bg-amber-50 dark:bg-amber-950 border border-amber-200 dark:border-amber-800 rounded-xl p-4",
                              p { class: "text-sm text-amber-700 dark:text-amber-300", "Podman is not available on this system" }
                          }
                      }
                  }
              }

              // Prune Action
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-6",
                  div { class: "flex items-start gap-4",
                      span { class: "material-symbols-rounded text-4xl text-cyan-500", "cleanup" }
                      div { class: "flex-1",
                          h3 { class: "text-lg font-semibold mb-1", "System Prune" }
                          p { class: "text-sm text-zinc-500 mb-4",
                              "Remove all unused containers, images, and build cache. Running containers are preserved."
                          }
                          button {
                              class: "px-5 py-2.5 bg-red-500 hover:bg-red-600 text-white rounded-xl text-sm font-medium transition-colors",
                              disabled: *is_loading.read()
                                  || (*active_type.read() == ContainerType::Docker && !docker_available)
                                  || (*active_type.read() == ContainerType::Podman && !podman_available),
                              onclick: move |_| {
                                  if *active_type.read() == ContainerType::Docker {
                                      is_loading.set(true);
                                      status_msg.set("Pruning Docker...".to_string());
                                      match bridge::invoke_app_command("clean_docker_prune", &serde_json::json!({})) {
                                          Ok(resp) => {
                                              if let Ok(freed) = serde_json::from_value::<u64>(resp) {
                                                  status_msg.set(format!("Docker pruned, freed {}", format_bytes(freed)));
                                              } else {
                                                  status_msg.set("Docker pruned".to_string());
                                              }
                                              last_cleaned.set(Some(chrono::Local::now().format("%Y-%m-%d %H:%M").to_string()));
                                          }
                                          Err(e) => { status_msg.set(format!("Error: {}", e)); }
                                      }
                                      is_loading.set(false);
                                  }
                              },
                              if *is_loading.read() { "Pruning..." } else { "Run System Prune" }
                          }
                      }
                  }
              }

              // Info Banner
              div { class: "bg-cyan-50 dark:bg-cyan-950 border border-cyan-200 dark:border-cyan-800 rounded-xl p-4",
                  div { class: "flex items-start gap-3",
                      span { class: "material-symbols-rounded text-cyan-600 dark:text-cyan-400 flex-shrink-0", "info" }
                      div {
                          p { class: "text-sm font-medium text-cyan-900 dark:text-cyan-200", "About Container Cleanup" }
                          p { class: "text-xs text-cyan-700 dark:text-cyan-300 mt-1",
                              "Docker/Podman cleanup commands only remove unused resources. "
                              "Running containers and their data volumes are preserved."
                          }
                      }
                  }
              }

              if !status_msg.read().is_empty() {
                  div { class: "text-sm text-zinc-500 dark:text-zinc-400 bg-zinc-100 dark:bg-zinc-800 rounded-lg px-4 py-2",
                      "{status_msg.read()}"
                  }
              }
          }
      }
  }
}

#[derive(Debug, Clone, Props, PartialEq)]
struct SummaryCardProps {
  label: String,
  value: String,
  icon: &'static str,
}

#[component]
fn SummaryCard(props: SummaryCardProps) -> Element {
  rsx! {
      div { class: "bg-white dark:bg-zinc-900 rounded-xl border border-zinc-200 dark:border-zinc-800 px-4 py-3",
          div { class: "flex items-center gap-2 mb-1",
              span { class: "material-symbols-rounded text-lg text-cyan-500", "{props.icon}" }
              p { class: "text-xs text-zinc-500 uppercase tracking-wide", "{props.label}" }
          }
          p { class: "text-xl font-bold", "{props.value}" }
      }
  }
}
