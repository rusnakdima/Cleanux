//! Automation page — mirrors the template `data-page="automation"` section.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AutomationRecipeEntry {
  pub id: String,
  pub name: String,
  pub description: String,
  pub enabled: bool,
  pub last_run: Option<String>,
  pub schedule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
  pub status: String,
  pub message: String,
  pub freed_bytes: Option<u64>,
}

#[derive(Props, Clone, PartialEq)]
struct RecipeCardProps {
  recipe: AutomationRecipeEntry,
  is_running: bool,
  on_run: Callback<()>,
  on_toggle: Callback<()>,
  on_edit: Callback<()>,
  on_delete: Callback<()>,
}

#[component]
fn RecipeCard(props: RecipeCardProps) -> Element {
  let toggle_class = if props.recipe.enabled {
    "text-emerald-500"
  } else {
    "text-zinc-400"
  };

  rsx! {
      div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
          div { class: "flex items-start gap-4",
              div { class: "flex-1 min-w-0",
                  div { class: "flex items-center gap-2 mb-1",
                      h3 { class: "font-semibold", "{props.recipe.name}" }
                      if props.recipe.enabled {
                          span { class: "px-2 py-0.5 rounded-full text-xs bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-400", "Active" }
                      }
                  }
                  p { class: "text-xs text-zinc-500 dark:text-zinc-400 mb-2", "{props.recipe.description}" }
                  div { class: "flex items-center gap-3 text-xs text-zinc-500 dark:text-zinc-400",
                      span { class: "material-symbols-rounded text-sm", "schedule" }
                      span { "{props.recipe.schedule}" }
                      if let Some(lr) = &props.recipe.last_run {
                          span { class: "ml-2", "Last run: {lr}" }
                      }
                  }
              }
              div { class: "flex items-center gap-2 flex-shrink-0",
                  button {
                      class: "p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors",
                      onclick: move |_| props.on_toggle.call(()),
                      span { class: "material-symbols-rounded {toggle_class}", "toggle_on" }
                  }
                  button {
                      class: "p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors",
                      disabled: props.is_running,
                      onclick: move |_| props.on_run.call(()),
                      span { class: "material-symbols-rounded text-sky-500", "play_arrow" }
                  }
                  button {
                      class: "p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors",
                      onclick: move |_| props.on_edit.call(()),
                      span { class: "material-symbols-rounded text-zinc-400", "edit" }
                  }
                  button {
                      class: "p-2 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors",
                      onclick: move |_| props.on_delete.call(()),
                      span { class: "material-symbols-rounded text-red-400", "delete" }
                  }
              }
          }
      }
  }
}

#[component]
pub fn AutomationPage(state: AppState) -> Element {
  let mut recipes = use_signal(|| Vec::<AutomationRecipeEntry>::new());
  let mut is_loading = use_signal(|| false);
  let mut running_id = use_signal(|| Option::<String>::None);
  let mut status_msg = use_signal(|| String::new());
  let mut show_editor = use_signal(|| false);
  let mut editing_recipe = use_signal(|| Option::<AutomationRecipeEntry>::None);

  // Load recipes on mount
  use_effect(move || {
    is_loading.set(true);
    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      match bridge::invoke_app_command("automation_get_recipes", &serde_json::json!({})) {
        Ok(val) => {
          if let Ok(list) = serde_json::from_value::<Vec<AutomationRecipeEntry>>(val) {
            recipes.set(list);
          }
        }
        Err(_) => {}
      }
      is_loading.set(false);
    });
  });

  let handle_create = move |_| {
    editing_recipe.set(None);
    show_editor.set(true);
  };

  let list = recipes.read();
  let recipe_count = list.len();

  rsx! {
      section { "data-page": "automation",
          class: "space-y-6",

          // Header
          div { class: "flex items-center justify-between",
              div {
                  h2 { class: "font-semibold", "Automation Recipes" }
                  p { class: "text-xs text-zinc-500 dark:text-zinc-400", "{recipe_count} recipes" }
              }
              button {
                  class: "px-4 py-2 rounded-xl bg-gradient-to-r from-cyan-500 to-violet-500 text-white font-medium text-sm hover:opacity-90",
                  onclick: handle_create,
                  "New Recipe"
              }
          }

          // Recipe list
          if list.is_empty() {
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-8 text-center",
                  div { class: "material-symbols-rounded text-4xl text-zinc-300 dark:text-zinc-600 mx-auto mb-3", "auto_awesome" }
                  p { class: "text-sm text-zinc-500 dark:text-zinc-400 mb-4", "No automation recipes yet" }
                  button {
                      class: "px-4 py-2 rounded-xl bg-zinc-900 dark:bg-white text-white dark:text-zinc-900 font-medium text-sm hover:opacity-90",
                      onclick: handle_create,
                      "Create your first recipe"
                  }
              }
          } else {
              div { class: "space-y-3",
                  for recipe in list.iter() {
                      RecipeCard {
                          recipe: recipe.clone(),
                          is_running: running_id.read().as_ref() == Some(&recipe.id),
                          on_run: {
                              let id = recipe.id.clone();
                              move |_| {
                                  running_id.set(Some(id.clone()));
                                  status_msg.set("Running recipe...".to_string());
                                  let rid = id.clone();
                                  let rt = tokio::runtime::Handle::current();
                                  rt.block_on(async {
                                      match bridge::invoke_app_command("automation_execute_recipe", &serde_json::json!({ "id": &rid })) {
                                          Ok(val) => {
                                              if let Ok(result) = serde_json::from_value::<ExecutionResult>(val) {
                                                  status_msg.set(result.message);
                                              } else {
                                                  status_msg.set("Recipe completed".to_string());
                                              }
                                          }
                                          Err(e) => status_msg.set(format!("Error: {}", e)),
                                      }
                                      running_id.set(None);
                                  });
                              }
                          },
                          on_toggle: {
                              let id = recipe.id.clone();
                              move |_| {
                                  let mut current = recipes.read().clone();
                                  if let Some(r) = current.iter_mut().find(|r| r.id == id) {
                                      r.enabled = !r.enabled;
                                      let enabled = r.enabled;
                                      let rid = id.clone();
                                      let rt = tokio::runtime::Handle::current();
                                      rt.block_on(async {
                                          let _ = bridge::invoke_app_command(
                                              "automation_toggle_recipe",
                                              &serde_json::json!({ "id": &rid, "enabled": enabled }),
                                          );
                                      });
                                  }
                                  recipes.set(current);
                              }
                          },
                          on_edit: {
                              let r = recipe.clone();
                              move |_| {
                                  editing_recipe.set(Some(r.clone()));
                                  show_editor.set(true);
                              }
                          },
                          on_delete: {
                              let id = recipe.id.clone();
                              move |_| {
                                  let rid = id.clone();
                                  let rt = tokio::runtime::Handle::current();
                                  rt.block_on(async {
                                      let _ = bridge::invoke_app_command("automation_delete_recipe", &serde_json::json!({ "id": &rid }));
                                      let mut current = recipes.read().clone();
                                      current.retain(|r| r.id != rid);
                                      recipes.set(current);
                                      status_msg.set("Recipe deleted".to_string());
                                  });
                              }
                          },
                      }
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
