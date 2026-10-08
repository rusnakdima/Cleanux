//! Profile page — mirrors the template `data-page="profile"` section.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CleaningProfileEntry {
  pub id: String,
  pub name: String,
  pub description: String,
  pub categories: Vec<String>,
  pub is_active: bool,
  pub created_at: String,
}

#[derive(Props, Clone, PartialEq)]
struct ProfileCardProps {
  profile: CleaningProfileEntry,
  on_switch: Callback<()>,
  on_delete: Callback<()>,
}

#[component]
fn ProfileCard(props: ProfileCardProps) -> Element {
  rsx! {
      div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
          div { class: "flex items-center gap-4",
              div { class: "flex-1 min-w-0",
                  div { class: "flex items-center gap-2 mb-1",
                      h3 { class: "font-semibold", "{props.profile.name}" }
                      if props.profile.is_active {
                          span { class: "px-2 py-0.5 rounded-full text-xs bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-400", "Active" }
                      }
                  }
                  p { class: "text-xs text-zinc-500 dark:text-zinc-400 mb-2", "{props.profile.description}" }
                  div { class: "flex flex-wrap gap-1",
                      for cat in props.profile.categories.iter().take(4) {
                          span { class: "px-2 py-0.5 rounded text-xs bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-400", "{cat}" }
                      }
                  }
              }
              div { class: "flex items-center gap-2 flex-shrink-0",
                  button {
                      class: "p-2 rounded-lg hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors",
                      disabled: props.profile.is_active,
                      onclick: move |_| props.on_switch.call(()),
                      span { class: "material-symbols-rounded text-cyan-500", "check_circle" }
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
pub fn ProfilePage(state: AppState) -> Element {
  let mut profiles = use_signal(|| Vec::<CleaningProfileEntry>::new());
  let mut is_loading = use_signal(|| false);
  let mut show_editor = use_signal(|| false);
  let mut new_profile_name = use_signal(|| String::new());
  let mut new_profile_desc = use_signal(|| String::new());
  let mut status_msg = use_signal(|| String::new());

  // Load profiles on mount
  use_effect(move || {
    is_loading.set(true);
    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      match bridge::invoke_app_command("crud_find_all_cleaning_profiles", &serde_json::json!({})) {
        Ok(val) => {
          if let Ok(list) = serde_json::from_value::<Vec<CleaningProfileEntry>>(val) {
            profiles.set(list);
          }
        }
        Err(_) => {}
      }
      is_loading.set(false);
    });
  });

  let handle_create = move |_| {
    new_profile_name.set(String::new());
    new_profile_desc.set(String::new());
    show_editor.set(true);
  };

  let handle_save = move |_| {
    let name = new_profile_name.read().clone();
    let desc = new_profile_desc.read().clone();
    if name.is_empty() {
      return;
    }

    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      match bridge::invoke_app_command(
        "crud_insert_cleaning_profile",
        &serde_json::json!({
            "name": name,
            "description": desc,
            "categories": vec!["cache", "logs", "trash"]
        }),
      ) {
        Ok(_) => {
          // Reload profiles
          match bridge::invoke_app_command(
            "crud_find_all_cleaning_profiles",
            &serde_json::json!({}),
          ) {
            Ok(val) => {
              if let Ok(list) = serde_json::from_value::<Vec<CleaningProfileEntry>>(val) {
                profiles.set(list);
              }
            }
            Err(_) => {}
          }
          status_msg.set("Profile created".to_string());
          show_editor.set(false);
        }
        Err(e) => status_msg.set(format!("Error: {}", e)),
      }
    });
  };

  let list = profiles.read();
  let profile_count = list.len();

  rsx! {
      section { "data-page": "profile",
          class: "space-y-6",

          // Header
          div { class: "flex items-center justify-between",
              div {
                  h2 { class: "font-semibold", "Cleaning Profiles" }
                  p { class: "text-xs text-zinc-500 dark:text-zinc-400", "{profile_count} profiles" }
              }
              button {
                  class: "px-4 py-2 rounded-xl bg-gradient-to-r from-cyan-500 to-violet-500 text-white font-medium text-sm hover:opacity-90",
                  onclick: handle_create,
                  "New Profile"
              }
          }

          // Profile list
          if list.is_empty() {
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-8 text-center",
                  div { class: "material-symbols-rounded text-4xl text-zinc-300 dark:text-zinc-600 mx-auto mb-3", "person" }
                  p { class: "text-sm text-zinc-500 dark:text-zinc-400 mb-4", "No cleaning profiles yet" }
                  button {
                      class: "px-4 py-2 rounded-xl bg-zinc-900 dark:bg-white text-white dark:text-zinc-900 font-medium text-sm hover:opacity-90",
                      onclick: handle_create,
                      "Create your first profile"
                  }
              }
          } else {
              div { class: "space-y-3",
                  for profile in list.iter() {
                      ProfileCard {
                          profile: profile.clone(),
                          on_switch: {
                              let pid = profile.id.clone();
                              move |_| {
                                  let rt = tokio::runtime::Handle::current();
                                  rt.block_on(async {
                                      match bridge::invoke_app_command("crud_apply_cleaning_profile", &serde_json::json!({ "id": &pid })) {
                                          Ok(_) => {
                                              let mut current = profiles.read().clone();
                                              for p in current.iter_mut() {
                                                  p.is_active = p.id == pid;
                                              }
                                              profiles.set(current);
                                              status_msg.set("Profile activated".to_string());
                                          }
                                          Err(e) => status_msg.set(format!("Error: {}", e)),
                                      }
                                  });
                              }
                          },
                          on_delete: {
                              let pid = profile.id.clone();
                              move |_| {
                                  let rt = tokio::runtime::Handle::current();
                                  rt.block_on(async {
                                      match bridge::invoke_app_command("crud_delete_cleaning_profile", &serde_json::json!({ "id": &pid })) {
                                          Ok(_) => {
                                              let mut current = profiles.read().clone();
                                              current.retain(|p| p.id != pid);
                                              profiles.set(current);
                                              status_msg.set("Profile deleted".to_string());
                                          }
                                          Err(e) => status_msg.set(format!("Error: {}", e)),
                                      }
                                  });
                              }
                          },
                      }
                  }
              }
          }

          // Editor panel
          if *show_editor.read() {
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  h3 { class: "font-semibold mb-4", "New Profile" }
                  div { class: "space-y-3",
                      div {
                          label { class: "block text-xs font-medium text-zinc-500 dark:text-zinc-400 mb-1", "Name" }
                          input {
                              r#type: "text",
                              value: "{new_profile_name.read()}",
                              oninput: move |e| new_profile_name.set(e.value().to_string()),
                              placeholder: "My Profile",
                              class: "w-full px-3 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500"
                          }
                      }
                      div {
                          label { class: "block text-xs font-medium text-zinc-500 dark:text-zinc-400 mb-1", "Description" }
                          input {
                              r#type: "text",
                              value: "{new_profile_desc.read()}",
                              oninput: move |e| new_profile_desc.set(e.value().to_string()),
                              placeholder: "Weekly cleanup preset",
                              class: "w-full px-3 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500"
                          }
                      }
                      div { class: "flex items-center gap-3 pt-2",
                          button {
                              class: "px-4 py-2 rounded-xl bg-zinc-100 dark:bg-zinc-800 text-sm hover:bg-zinc-200 dark:hover:bg-zinc-700",
                              onclick: move |_| show_editor.set(false),
                              "Cancel"
                          }
                          button {
                              class: "px-4 py-2 rounded-xl bg-gradient-to-r from-cyan-500 to-violet-500 text-white font-medium text-sm hover:opacity-90",
                              onclick: handle_save,
                              "Create Profile"
                          }
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
