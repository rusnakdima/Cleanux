//! Settings page — mirrors the template `data-page="settings"` section.

use crate::app::AppState;
use crate::global_state;
use dioxus::prelude::*;

#[component]
pub fn SettingsPage(state: AppState) -> Element {
  let mut settings = use_signal(|| global_state::get_settings());

  rsx! {
      section { "data-page": "settings",
          class: "space-y-6",

          // General toggles
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5 space-y-4",
              h2 { class: "font-semibold", "General" }
              ToggleRow {
                  label: "Auto-clean",
                  desc: "Run a quick clean on a schedule",
                  checked: settings.read().auto_clean,
                  on_change: move |v| {
                      global_state::update_setting("autoClean", serde_json::json!(v));
                      settings.set(global_state::get_settings());
                  },
              }
              ToggleRow {
                  label: "Notify on complete",
                  desc: "Show system notification when scan finishes",
                  checked: settings.read().notify_on_complete,
                  on_change: move |v| {
                      global_state::update_setting("notifyOnComplete", serde_json::json!(v));
                      settings.set(global_state::get_settings());
                  },
              }
              ToggleRow {
                  label: "Secure delete",
                  desc: "Overwrite files before removal (slower)",
                  checked: settings.read().secure_delete,
                  on_change: move |v| {
                      global_state::update_setting("secureDelete", serde_json::json!(v));
                      settings.set(global_state::get_settings());
                  },
              }
              ToggleRow {
                  label: "Minimize to tray",
                  desc: "Keep running in background when window closes",
                  checked: settings.read().minimize_to_tray,
                  on_change: move |v| {
                      global_state::update_setting("minimizeToTray", serde_json::json!(v));
                      settings.set(global_state::get_settings());
                  },
              }
              ToggleRow {
                  label: "Launch on system start",
                  desc: "Auto-start when the OS boots",
                  checked: settings.read().start_with_system,
                  on_change: move |v| {
                      global_state::update_setting("startWithSystem", serde_json::json!(v));
                      settings.set(global_state::get_settings());
                  },
              }
          }

          // Thresholds
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5 space-y-4",
              h2 { class: "font-semibold", "Thresholds" }
              RangeRow {
                  label: "Min file age (days)",
                  value: settings.read().max_file_age,
                  min: 0,
                  max: 365,
                  on_change: move |v| {
                      global_state::update_setting("maxFileAge", serde_json::json!(v));
                      settings.set(global_state::get_settings());
                  },
              }
              RangeRow {
                  label: "Large file threshold (MB)",
                  value: settings.read().large_file_threshold,
                  min: 50,
                  max: 2000,
                  step: 50,
                  on_change: move |v| {
                      global_state::update_setting("largeFileThreshold", serde_json::json!(v));
                      settings.set(global_state::get_settings());
                  },
              }
          }

          // Excluded folders
          ExcludedFolders {}
      }
  }
}

#[derive(Clone, PartialEq, Props)]
struct ToggleRowProps {
  label: &'static str,
  desc: &'static str,
  checked: bool,
  on_change: EventHandler<bool>,
}

#[component]
fn ToggleRow(props: ToggleRowProps) -> Element {
  rsx! {
      label { class: "flex items-center justify-between cursor-pointer",
          div {
              div { class: "font-medium text-sm", "{props.label}" }
              div { class: "text-xs text-zinc-500 dark:text-zinc-400", "{props.desc}" }
          }
          input {
              r#type: "checkbox",
              checked: props.checked,
              class: "sr-only peer",
              onchange: move |e| props.on_change.call(e.checked()),
          }
          div { class: "w-11 h-6 bg-zinc-200 dark:bg-zinc-700 peer-checked:bg-cyan-500 rounded-full relative transition-colors after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:w-5 after:h-5 after:bg-white after:rounded-full after:transition-transform peer-checked:after:translate-x-5", "" }
      }
  }
}

#[derive(Clone, PartialEq, Props)]
struct RangeRowProps {
  label: &'static str,
  value: u32,
  min: u32,
  max: u32,
  #[props(default = 1)]
  step: u32,
  on_change: EventHandler<u32>,
}

#[component]
fn RangeRow(props: RangeRowProps) -> Element {
  let val_str = props.value.to_string();

  rsx! {
      div {
          div { class: "flex items-center justify-between mb-2",
              label { class: "text-sm font-medium", "{props.label}" }
              span { class: "text-sm font-mono", "{val_str}" }
          }
          input {
              r#type: "range",
              min: "{props.min}",
              max: "{props.max}",
              step: "{props.step}",
              value: "{val_str}",
              class: "w-full accent-cyan-500",
              oninput: move |e| {
                  if let Ok(v) = e.value().parse::<u32>() {
                      props.on_change.call(v);
                  }
              },
          }
      }
  }
}

#[component]
fn ExcludedFolders() -> Element {
  let mut folders = use_signal(|| global_state::get_settings().excluded_folders);

  rsx! {
      div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
          h2 { class: "font-semibold mb-3", "Excluded Folders" }
          div { class: "space-y-2",
              if folders.read().is_empty() {
                  div { class: "text-sm text-zinc-500", "No excluded folders" }
              } else {
                  for (i, folder) in folders.read().iter().enumerate() {
                      div { class: "flex items-center gap-3 p-2.5 rounded-lg bg-zinc-50 dark:bg-zinc-800/50",
                          span { class: "material-symbols-rounded text-zinc-400", "folder_off" }
                          span { class: "flex-1 text-sm font-mono truncate", "{folder}" }
                          button {
                              class: "p-1 rounded hover:bg-zinc-200 dark:hover:bg-zinc-700",
                              onclick: move |_| {
                                  global_state::remove_excluded_folder(i);
                                  folders.set(global_state::get_settings().excluded_folders.clone());
                              },
                              span { class: "material-symbols-rounded text-base", "close" }
                          }
                      }
                  }
              }
          }
      }
  }
}
