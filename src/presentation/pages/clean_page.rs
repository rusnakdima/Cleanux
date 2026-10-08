//! Clean page — mirrors the template `data-page="clean"` section.

use crate::app::AppState;
use crate::bridge;
use crate::global_state::{self, CategoryEntry, CleaningStage};
use dioxus::prelude::*;

#[component]
pub fn CleanPage(state: AppState) -> Element {
  // Use local signals for this page's interactive state
  let mut categories = use_signal(|| global_state::get_categories());
  let stages = use_signal(|| global_state::get_cleaning_stages());
  let mut is_running = use_signal(|| false);
  let mut clean_status = use_signal(|| "Idle".to_string());
  let btn_label = if *is_running.read() { "Pause" } else { "Start" };

  // Load fresh category sizes from handler on mount
  use_effect(move || {
    // Bridge call is sync (creates its own runtime)
    let result = bridge::invoke_app_command("get_category_sizes", &serde_json::json!({}));
    if let Ok(val) = result {
      if let Some(items) = val.get("data").and_then(|d| d.get("items")) {
        if let Ok(sizes) = serde_json::from_value::<
          Vec<crate::application::handlers::cleaner_handlers::CategorySize>,
        >(items.clone())
        {
          let cats: Vec<CategoryEntry> = sizes
            .into_iter()
            .map(|cs| CategoryEntry {
              id: cs.id,
              label: cs.name,
              size: crate::application::handlers::format_size(cs.size_bytes),
              count: cs.item_count as u32,
              color: "blue".to_string(),
              selected: true,
            })
            .collect();
          if !cats.is_empty() {
            categories.set(cats);
          }
        }
      }
    }
  });

  rsx! {
      section { "data-page": "clean",
          class: "space-y-6",

          // Cleaning progress
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              div { class: "flex items-center justify-between mb-4",
                  div {
                      h2 { class: "font-semibold", "Cleaning Progress" }
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400", "{clean_status}" }
                  }
                  button {
                      class: "px-4 py-2 rounded-xl bg-zinc-900 dark:bg-white text-white dark:text-zinc-900 font-medium text-sm hover:opacity-90",
                      onclick: move |_| {
                          if *is_running.read() {
                              // Pause
                              is_running.set(false);
                              clean_status.set("Paused".to_string());
                          } else {
                              // Start scan + clean
                              is_running.set(true);
                              clean_status.set("Scanning…".to_string());
                              let selected: Vec<String> = global_state::get_categories()
                                  .iter().filter(|c| c.selected).map(|c| c.id.clone()).collect();
                              let _ = bridge::invoke_app_command("scan_cache_categories", &serde_json::json!({ "categories": &selected }));
                              let _ = bridge::invoke_app_command("start_quick_clean", &serde_json::json!({ "categories": &selected }));
                              clean_status.set("Cleaning complete".to_string());
                              is_running.set(false);
                          }
                      },
                      "{btn_label}"
                  }
              }
              div { class: "space-y-2.5",
                  for stage in stages.read().iter() {
                      CleaningStageRow { stage: stage.clone() }
                  }
              }
          }

          // Categories
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              div { class: "flex items-center justify-between mb-4",
                  h2 { class: "font-semibold", "Categories" }
                  button {
                      class: "text-xs font-medium px-3 py-1.5 rounded-full bg-zinc-100 dark:bg-zinc-800 hover:bg-zinc-200 dark:hover:bg-zinc-700",
                      onclick: move |_| {
                          let cats = global_state::get_categories();
                          let any = cats.iter().any(|c| !c.selected);
                          for c in cats.iter() {
                              global_state::set_category_selected(&c.id, any);
                          }
                          categories.set(global_state::get_categories());
                      },
                      "Toggle all"
                  }
              }
              div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3",
                  for cat in categories.read().iter() {
                      CategoryCard { cat: cat.clone() }
                  }
              }
          }
      }
  }
}

#[component]
fn CleaningStageRow(stage: CleaningStage) -> Element {
  let color_map: std::collections::HashMap<&str, &str> = [
    ("done", "bg-emerald-500"),
    ("running", "bg-cyan-500"),
    ("pending", "bg-zinc-300 dark:bg-zinc-700"),
  ]
  .into_iter()
  .collect();
  let status_text: std::collections::HashMap<&str, &str> = [
    ("done", "Done"),
    ("running", "Running"),
    ("pending", "Pending"),
  ]
  .into_iter()
  .collect();

  let bar_class = color_map
    .get(stage.status.as_str())
    .copied()
    .unwrap_or("bg-zinc-300");
  let status_str = status_text
    .get(stage.status.as_str())
    .copied()
    .unwrap_or("Pending");

  rsx! {
      div { class: "space-y-1.5",
          div { class: "flex items-center justify-between text-sm",
              div { class: "flex items-center gap-2",
                  span { class: "w-1.5 h-1.5 rounded-full {bar_class}", "" }
                  span { class: "font-medium", "{stage.label}" }
              }
              span { class: "text-xs text-zinc-500 dark:text-zinc-400 font-mono", "{stage.percent}% · {status_str}" }
          }
          div { class: "w-full h-1.5 rounded-full bg-zinc-100 dark:bg-zinc-800 overflow-hidden",
              div { class: "h-full {bar_class} rounded-full transition-all", style: "width: {stage.percent}%", "" }
          }
      }
  }
}

#[component]
fn CategoryCard(cat: CategoryEntry) -> Element {
  let color_map: std::collections::HashMap<&str, &str> = [
    ("emerald", "bg-emerald-500"),
    ("sky", "bg-sky-500"),
    ("amber", "bg-amber-500"),
    ("violet", "bg-violet-500"),
    ("rose", "bg-rose-500"),
    ("indigo", "bg-indigo-500"),
    ("pink", "bg-pink-500"),
    ("teal", "bg-teal-500"),
  ]
  .into_iter()
  .collect();
  let dot_class = color_map
    .get(cat.color.as_str())
    .copied()
    .unwrap_or("bg-zinc-400");
  let selected = cat.selected;

  rsx! {
      div {
          class: "border border-zinc-200 dark:border-zinc-800 rounded-xl p-4 cursor-pointer hover:border-cyan-500 transition-colors",
          onclick: move |_| {
              global_state::set_category_selected(&cat.id, !cat.selected);
          },
          div { class: "flex items-center justify-between mb-2",
              span { class: "w-2 h-2 rounded-full {dot_class}", "" }
              span {
                  class: if selected {
                      "text-xs px-2 py-0.5 rounded-full bg-cyan-100 dark:bg-cyan-900/30 text-cyan-700 dark:text-cyan-400"
                  } else {
                      "text-xs px-2 py-0.5 rounded-full bg-zinc-100 dark:bg-zinc-800 text-zinc-500"
                  },
                  if selected { "Selected" } else { "Skipped" }
              }
          }
          div { class: "text-sm font-semibold", "{cat.label}" }
          div { class: "text-xs text-zinc-500 dark:text-zinc-400 mt-1", "{cat.size} · {cat.count} items" }
      }
  }
}
