//! Dev Cleaner page — mirrors the template `data-page="dev-cleaner"` section.
//!
//! Cleans developer tool caches: npm, pip, Cargo, Go, Maven, Gradle.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevCacheSummary {
  #[serde(rename = "npmBytes")]
  pub npm_bytes: u64,
  #[serde(rename = "pipBytes")]
  pub pip_bytes: u64,
  #[serde(rename = "cargoBytes")]
  pub cargo_bytes: u64,
  #[serde(rename = "goBytes")]
  pub go_bytes: u64,
  #[serde(rename = "mavenBytes")]
  pub maven_bytes: u64,
  #[serde(rename = "gradleBytes")]
  pub gradle_bytes: u64,
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

#[derive(Clone, Props, PartialEq)]
struct ToolCardProps {
  name: String,
  icon: String,
  path: String,
  size: String,
  on_clean: Callback<()>,
}

#[component]
fn ToolCard(props: ToolCardProps) -> Element {
  rsx! {
      div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-700 p-5 hover:shadow-md transition-all",
          div { class: "flex items-start justify-between mb-3",
              div { class: "flex items-center gap-3",
                  div { class: "w-10 h-10 rounded-xl bg-zinc-100 dark:bg-zinc-800 flex items-center justify-center text-xl",
                      "{props.icon}"
                  }
                  div {
                      p { class: "font-semibold text-sm", "{props.name}" }
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400 mt-0.5 font-mono max-w-48 truncate",
                          title: "{props.path}",
                          "{props.path}"
                      }
                  }
              }
          }
          div { class: "mb-4",
              p { class: "text-2xl font-bold", "{props.size}" }
              p { class: "text-xs text-zinc-500 dark:text-zinc-400", "cached" }
          }
          div { class: "flex gap-2",
              button {
                  class: "flex-1 px-3 py-2 rounded-lg border border-zinc-200 dark:border-zinc-700 hover:bg-zinc-50 dark:hover:bg-zinc-800 text-xs font-medium transition-colors text-zinc-600 dark:text-zinc-400",
                  onclick: move |_| props.on_clean.call(()),
                  "Clean"
              }
          }
      }
  }
}

#[component]
pub fn DevCleanerPage(state: AppState) -> Element {
  let mut summary = use_signal(|| DevCacheSummary {
    npm_bytes: 0,
    pip_bytes: 0,
    cargo_bytes: 0,
    go_bytes: 0,
    maven_bytes: 0,
    gradle_bytes: 0,
  });
  let mut last_freed = use_signal(|| String::new());

  // Load summary on mount
  use_effect(move || {
    let result = bridge::invoke_app_command("dev_cache_get_summary", &serde_json::json!({}));
    if let Ok(val) = result {
      if let Ok(s) = serde_json::from_value::<DevCacheSummary>(val) {
        summary.set(s);
      }
    }
  });

  let total = summary.read().npm_bytes
    + summary.read().pip_bytes
    + summary.read().cargo_bytes
    + summary.read().go_bytes
    + summary.read().maven_bytes
    + summary.read().gradle_bytes;

  let tools: Vec<(&str, &str, &str, u64)> = vec![
    ("npm", "📦", "~/.npm", summary.read().npm_bytes),
    ("pip", "🐍", "~/.cache/pip", summary.read().pip_bytes),
    ("Cargo", "🦀", "~/.cargo", summary.read().cargo_bytes),
    ("Go", "🐹", "~/go/pkg/mod", summary.read().go_bytes),
    (
      "Maven",
      "🧂",
      "~/.m2/repository",
      summary.read().maven_bytes,
    ),
    (
      "Gradle",
      "🔧",
      "~/.gradle/caches",
      summary.read().gradle_bytes,
    ),
  ];

  rsx! {
      section { "data-page": "dev-cleaner",
          class: "space-y-6",

          // Header
          div { class: "flex items-center justify-between",
              div {
                  h1 { class: "text-2xl font-bold", "Developer Caches" }
                  p { class: "text-sm text-zinc-500 dark:text-zinc-400 mt-1",
                      "Clean npm, pip, Cargo, Go, Maven and Gradle cache directories"
                  }
              }
              div { class: "text-right",
                  p { class: "text-2xl font-bold text-red-500", "{format_bytes(total)}" }
                  p { class: "text-xs text-zinc-500 dark:text-zinc-400", "total reclaimable" }
              }
          }

          // Status banner
          if !last_freed.read().is_empty() {
              div { class: "bg-green-50 dark:bg-green-950/30 border border-green-200 dark:border-green-800 rounded-xl p-4",
                  p { class: "text-sm text-green-700 dark:text-green-300",
                      "✅ Cleaned {last_freed.read()} of cache files"
                  }
              }
          }

          // Tool grid
          div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
              for (name, icon, path, size) in tools {
                  ToolCard {
                      name: name.to_string(),
                      icon: icon.to_string(),
                      path: path.to_string(),
                      size: format_bytes(size),
                      on_clean: move |_| {
                          let cmd = match name {
                              "npm" => "dev_cache_clean_npm",
                              "pip" => "dev_cache_clean_pip",
                              "Cargo" => "dev_cache_clean_cargo",
                              "Go" => "dev_cache_clean_go",
                              "Maven" => "dev_cache_clean_maven",
                              "Gradle" => "dev_cache_clean_gradle",
                              _ => return,
                          };
                          let _ = bridge::invoke_app_command(cmd, &serde_json::json!({}));
                          // Refresh summary
                          let result = bridge::invoke_app_command("dev_cache_get_summary", &serde_json::json!({}));
                          if let Ok(val) = result {
                              if let Ok(s) = serde_json::from_value::<DevCacheSummary>(val) {
                                  summary.set(s.clone());
                                  let total = s.npm_bytes + s.pip_bytes + s.cargo_bytes + s.go_bytes + s.maven_bytes + s.gradle_bytes;
                                  last_freed.set(format_bytes(total));
                              }
                          }
                      },
                  }
              }
          }

          // Clean All button
          div { class: "flex justify-end gap-3",
              button {
                  class: "px-6 py-2.5 rounded-xl bg-red-500 hover:bg-red-600 text-white font-semibold text-sm transition-colors shadow-sm",
                  onclick: move |_| {
                      let _ = bridge::invoke_app_command("dev_cache_clean_all", &serde_json::json!({}));
                      let result = bridge::invoke_app_command("dev_cache_get_summary", &serde_json::json!({}));
                      if let Ok(val) = result {
                          if let Ok(s) = serde_json::from_value::<DevCacheSummary>(val) {
                              summary.set(s);
                          }
                      }
                  },
                  "Clean All Developer Caches"
              }
          }
      }
  }
}
