//! Automation Recipe Editor page — mirrors the template `data-page="automation-recipe-editor"` section.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecipeAction {
  pub id: String,
  pub action_type: String,
  pub target: String,
  pub params: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecipeTrigger {
  pub trigger_type: String,
  pub schedule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AutomationRecipeForm {
  pub id: Option<String>,
  pub name: String,
  pub description: String,
  pub enabled: bool,
  pub trigger: RecipeTrigger,
  pub actions: Vec<RecipeAction>,
}

impl Default for AutomationRecipeForm {
  fn default() -> Self {
    Self {
      id: None,
      name: String::new(),
      description: String::new(),
      enabled: true,
      trigger: RecipeTrigger {
        trigger_type: "manual".to_string(),
        schedule: "".to_string(),
      },
      actions: vec![RecipeAction {
        id: "1".to_string(),
        action_type: "scan".to_string(),
        target: "junk".to_string(),
        params: std::collections::HashMap::new(),
      }],
    }
  }
}

#[derive(Props, Clone, PartialEq)]
struct ActionBuilderRowProps {
  action: RecipeAction,
  on_update: Callback<RecipeAction>,
  on_remove: Callback<String>,
}

#[component]
fn ActionBuilderRow(props: ActionBuilderRowProps) -> Element {
  let icons: std::collections::HashMap<&str, &str> = [
    ("scan", "search"),
    ("clean", "cleaning_services"),
    ("optimize", "speed"),
    ("custom", "tune"),
  ]
  .into_iter()
  .collect();

  let icon = icons
    .get(props.action.action_type.as_str())
    .copied()
    .unwrap_or("tune");
  let action_id = props.action.id.clone();

  let action_clone = props.action.clone();
  let action_for_select = action_clone.clone();
  rsx! {
      div { class: "flex items-center gap-3 px-4 py-3 rounded-xl border border-zinc-200 dark:border-zinc-700",
          span { class: "material-symbols-rounded text-zinc-400 flex-shrink-0", "{icon}" }

          select {
              class: "px-3 py-1.5 rounded-lg border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500",
              onchange: move |e| {
                  let mut a = action_for_select.clone();
                  a.action_type = e.value().to_string();
                  props.on_update.call(a);
              },
              value: "{props.action.action_type}",
              option { value: "scan", "Scan" }
              option { value: "clean", "Clean" }
              option { value: "optimize", "Optimize" }
              option { value: "custom", "Custom" }
          }

          input {
              r#type: "text",
              value: "{props.action.target}",
              oninput: move |e| {
                  let mut a = action_clone.clone();
                  a.target = e.value().to_string();
                  props.on_update.call(a);
              },
              placeholder: "Target",
              class: "flex-1 px-3 py-1.5 rounded-lg border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500"
          }

          button {
              class: "p-1.5 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors flex-shrink-0",
              onclick: move |_| props.on_remove.call(action_id.clone()),
              span { class: "material-symbols-rounded text-red-400 text-lg", "close" }
          }
      }
  }
}

#[component]
pub fn AutomationRecipeEditorPage(state: AppState) -> Element {
  let mut form = use_signal(|| AutomationRecipeForm::default());
  let mut status_msg = use_signal(|| String::new());
  let mut is_saving = use_signal(|| false);

  let handle_save = move |_| {
    is_saving.set(true);
    let data = form.read().clone();

    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      let cmd = if data.id.is_some() {
        "automation_update_recipe"
      } else {
        "automation_insert_recipe"
      };

      match bridge::invoke_app_command(cmd, &serde_json::json!({ "recipe": &data })) {
        Ok(_) => {
          status_msg.set("Recipe saved successfully".to_string());
          is_saving.set(false);
        }
        Err(e) => {
          status_msg.set(format!("Error saving: {}", e));
          is_saving.set(false);
        }
      }
    });
  };

  let handle_add_action = move |_| {
    let mut current = form.read().clone();
    let new_id = (current.actions.len() + 1).to_string();
    current.actions.push(RecipeAction {
      id: new_id,
      action_type: "clean".to_string(),
      target: "junk".to_string(),
      params: std::collections::HashMap::new(),
    });
    form.set(current);
  };

  let handle_remove_action = move |id: String| {
    let mut current = form.read().clone();
    current.actions.retain(|a| a.id != id);
    form.set(current);
  };

  let handle_update_action = move |a: RecipeAction| {
    let mut current = form.read().clone();
    if let Some(existing) = current.actions.iter_mut().find(|x| x.id == a.id) {
      *existing = a;
    }
    form.set(current);
  };

  let current = form.read();
  let is_empty = current.actions.is_empty();
  let name_empty = current.name.is_empty();

  rsx! {
      section { "data-page": "automation-recipe-editor",
          class: "space-y-6",

          // Header
          div { class: "flex items-center justify-between",
              div {
                  h2 { class: "font-semibold", "Recipe Editor" }
                  p { class: "text-xs text-zinc-500 dark:text-zinc-400",
                      if current.id.is_some() { "Editing existing recipe" } else { "Create a new automation recipe" }
                  }
              }
              div { class: "flex items-center gap-3",
                  button {
                      class: "px-4 py-2 rounded-xl bg-zinc-100 dark:bg-zinc-800 text-sm hover:bg-zinc-200 dark:hover:bg-zinc-700",
                      onclick: move |_| {
                          state.navigate(crate::app::Page::Settings);
                      },
                      "Cancel"
                  }
                  button {
                      class: "px-4 py-2 rounded-xl bg-gradient-to-r from-cyan-500 to-violet-500 text-white font-medium text-sm hover:opacity-90 disabled:opacity-50",
                      disabled: *is_saving.read() || name_empty,
                      onclick: handle_save,
                      if *is_saving.read() { "Saving..." } else { "Save Recipe" }
                  }
              }
          }

          // Basic info
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              h3 { class: "font-semibold mb-4", "Basic Information" }
              div { class: "space-y-3",
                  div {
                      label { class: "block text-xs font-medium text-zinc-500 dark:text-zinc-400 mb-1", "Recipe Name" }
                      input {
                          r#type: "text",
                          value: "{current.name}",
                          oninput: move |e| {
                              let mut f = form.read().clone();
                              f.name = e.value().to_string();
                              form.set(f);
                          },
                          placeholder: "Weekly Cleanup",
                          class: "w-full px-3 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500"
                      }
                  }
                  div {
                      label { class: "block text-xs font-medium text-zinc-500 dark:text-zinc-400 mb-1", "Description" }
                      input {
                          r#type: "text",
                          value: "{current.description}",
                          oninput: move |e| {
                              let mut f = form.read().clone();
                              f.description = e.value().to_string();
                              form.set(f);
                          },
                          placeholder: "Run a weekly junk scan and clean",
                          class: "w-full px-3 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500"
                      }
                  }
                  div { class: "flex items-center gap-2",
                      input {
                          r#type: "checkbox",
                          checked: current.enabled,
                          onchange: move |_| {
                              let mut f = form.read().clone();
                              f.enabled = !f.enabled;
                              form.set(f);
                          },
                          class: "w-4 h-4 rounded"
                      }
                      span { class: "text-sm", "Enabled" }
                  }
              }
          }

          // Trigger configuration
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              h3 { class: "font-semibold mb-4", "Trigger" }
              div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                  div {
                      label { class: "block text-xs font-medium text-zinc-500 dark:text-zinc-400 mb-1", "Trigger Type" }
                      select {
                          class: "w-full px-3 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500",
                          onchange: move |e| {
                              let mut f = form.read().clone();
                              f.trigger.trigger_type = e.value().to_string();
                              form.set(f);
                          },
                          value: "{current.trigger.trigger_type}",
                          option { value: "manual", "Manual" }
                          option { value: "schedule", "Scheduled" }
                          option { value: "startup", "On Startup" }
                      }
                  }
                  div {
                      label { class: "block text-xs font-medium text-zinc-500 dark:text-zinc-400 mb-1", "Schedule" }
                      input {
                          r#type: "text",
                          value: "{current.trigger.schedule}",
                          oninput: move |e| {
                              let mut f = form.read().clone();
                              f.trigger.schedule = e.value().to_string();
                              form.set(f);
                          },
                          placeholder: "0 2 * * 0 (cron)",
                          class: "w-full px-3 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500"
                      }
                  }
              }
          }

          // Actions builder
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              div { class: "flex items-center justify-between mb-4",
                  h3 { class: "font-semibold", "Actions" }
                  button {
                      class: "px-3 py-1.5 rounded-xl bg-zinc-100 dark:bg-zinc-800 text-xs font-medium hover:bg-zinc-200 dark:hover:bg-zinc-700 flex items-center gap-1",
                      onclick: handle_add_action,
                      span { class: "material-symbols-rounded text-sm", "add" }
                      "Add Action"
                  }
              }

              div { class: "space-y-3",
                  for action in current.actions.iter() {
                      ActionBuilderRow {
                          action: action.clone(),
                          on_update: handle_update_action,
                          on_remove: handle_remove_action,
                      }
                  }
              }

              if is_empty {
                  div { class: "py-6 text-center text-sm text-zinc-500 dark:text-zinc-400",
                      "Add actions to define what this recipe does"
                  }
              }
          }

          if !status_msg.read().is_empty() {
              div { class: "p-3 rounded-xl bg-emerald-50 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400 text-sm",
                  "{status_msg.read()}"
              }
          }
      }
  }
}
