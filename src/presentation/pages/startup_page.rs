//! Startup page — mirrors the template `data-page="startup"` section.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct StartupItem {
  pub id: String,
  pub name: String,
  pub command: String,
  pub enabled: bool,
  pub source: String,
  pub path: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum StartupCategory {
  All,
  Systemd,
  Xdg,
  Cron,
}

impl StartupCategory {
  fn label(&self) -> &'static str {
    match self {
      StartupCategory::All => "All",
      StartupCategory::Systemd => "Systemd Services",
      StartupCategory::Xdg => "XDG Autostart",
      StartupCategory::Cron => "Cron @reboot",
    }
  }

  fn matches(&self, source: &str) -> bool {
    match self {
      StartupCategory::All => true,
      StartupCategory::Systemd => source == "systemd",
      StartupCategory::Xdg => source == "xdg",
      StartupCategory::Cron => source == "cron",
    }
  }
}

#[component]
pub fn StartupPage(state: AppState) -> Element {
  let startup_items = use_signal(|| Vec::<StartupItem>::new());
  let mut search_query = use_signal(|| String::new());
  let mut selected_category = use_signal(|| StartupCategory::All);
  let is_loading = use_signal(|| false);
  let error_message = use_signal(|| Option::<String>::None);

  // Load processes on mount
  use_effect(move || {
    let mut items_clone = startup_items.clone();
    let mut is_loading_clone = is_loading.clone();
    let mut error_clone = error_message.clone();

    is_loading_clone.set(true);
    error_clone.set(None);

    match bridge::invoke_app_command("clean_get_startup_items", &serde_json::json!({})) {
      Ok(val) => {
        // Try to parse as Response<Vec<StartupItem>>
        if let Ok(items) = serde_json::from_value::<Vec<StartupItem>>(val.clone()) {
          items_clone.set(items);
        } else if let Ok(resp) = serde_json::from_value::<serde_json::Value>(val) {
          if let Some(data) = resp.get("data") {
            if let Ok(items) = serde_json::from_value::<Vec<StartupItem>>(data.clone()) {
              items_clone.set(items);
            }
          }
        }
        error_clone.set(None);
      }
      Err(e) => {
        error_clone.set(Some(e.to_string()));
      }
    }
    is_loading_clone.set(false);
  });

  // Compute filtered items
  let query = search_query.read().to_lowercase();
  let category = selected_category.read().clone();
  let items = startup_items.read();

  let filtered: Vec<StartupItem> = items
    .iter()
    .filter(|item| {
      let matches_category = category.matches(&item.source);
      let matches_search = query.is_empty()
        || item.name.to_lowercase().contains(&query)
        || item.command.to_lowercase().contains(&query);
      matches_category && matches_search
    })
    .cloned()
    .collect();

  let tab_all_active = if *selected_category.read() == StartupCategory::All {
    "active"
  } else {
    ""
  };
  let tab_systemd_active = if *selected_category.read() == StartupCategory::Systemd {
    "active"
  } else {
    ""
  };
  let tab_xdg_active = if *selected_category.read() == StartupCategory::Xdg {
    "active"
  } else {
    ""
  };
  let tab_cron_active = if *selected_category.read() == StartupCategory::Cron {
    "active"
  } else {
    ""
  };

  let handle_refresh = move |_| {
    let mut items_clone = startup_items.clone();
    let mut is_loading_clone = is_loading.clone();
    let mut error_clone = error_message.clone();

    is_loading_clone.set(true);
    error_clone.set(None);

    match bridge::invoke_app_command("clean_get_startup_items", &serde_json::json!({})) {
      Ok(val) => {
        if let Ok(items) = serde_json::from_value::<Vec<StartupItem>>(val.clone()) {
          items_clone.set(items);
        } else if let Ok(resp) = serde_json::from_value::<serde_json::Value>(val) {
          if let Some(data) = resp.get("data") {
            if let Ok(items) = serde_json::from_value::<Vec<StartupItem>>(data.clone()) {
              items_clone.set(items);
            }
          }
        }
        error_clone.set(None);
      }
      Err(e) => {
        error_clone.set(Some(e.to_string()));
      }
    }
    is_loading_clone.set(false);
  };

  rsx! {
      div {
          class: "page",
          div {
              class: "page-header",
              div {
                  class: "page-title",
                  h1 { "Startup Programs" }
                  p { "Manage applications that run at system boot" }
              }
              button {
                  class: "btn btn-secondary",
                  onclick: handle_refresh,
                  disabled: *is_loading.read(),
                  if *is_loading.read() {
                      span { "Refreshing..." }
                  } else {
                      span { "Refresh" }
                  }
              }
          }

          if let Some(err) = error_message.read().as_ref() {
              div {
                  class: "alert alert-error",
                  "{err}"
              }
          }

          div {
              class: "filters",
              input {
                  r#type: "text",
                  class: "search-input",
                  placeholder: "Search startup items...",
                  value: "{search_query.read()}",
                  oninput: move |e| {
                      search_query.set(e.value().to_string());
                  }
              }

              div {
                  class: "category-tabs",
                  button {
                      class: "tab-btn {tab_all_active}",
                      onclick: move |_| selected_category.set(StartupCategory::All),
                      "All"
                  }
                  button {
                      class: "tab-btn {tab_systemd_active}",
                      onclick: move |_| selected_category.set(StartupCategory::Systemd),
                      "Systemd"
                  }
                  button {
                      class: "tab-btn {tab_xdg_active}",
                      onclick: move |_| selected_category.set(StartupCategory::Xdg),
                      "XDG"
                  }
                  button {
                      class: "tab-btn {tab_cron_active}",
                      onclick: move |_| selected_category.set(StartupCategory::Cron),
                      "Cron"
                  }
              }
          }

          div {
              class: "startup-list",
              if filtered.is_empty() {
                  div {
                      class: "empty-state",
                      p { "No startup items found" }
                  }
              } else {
                  for item in filtered.iter() {
                      StartupItemRow {
                          key: "{item.id}",
                          item: item.clone(),
                      }
                  }
              }
          }
      }
  }
}

#[derive(Clone, Props, PartialEq)]
struct StartupItemRowProps {
  item: StartupItem,
}

#[component]
fn StartupItemRow(props: StartupItemRowProps) -> Element {
  let mut show_menu = use_signal(|| false);

  // Clone values needed by closures before creating them
  let item_id = props.item.id.clone();
  let item_enabled = props.item.enabled;
  let item_path = props.item.path.clone();
  let item_source = props.item.source.clone();

  let item_disabled_class = if !item_enabled { "disabled" } else { "" };

  let source_label = match item_source.as_str() {
    "systemd" => "Systemd",
    "xdg" => "XDG",
    "cron" => "Cron",
    other => other,
  };

  let toggle_active_class = if item_enabled { "active" } else { "" };

  // Clone item_id separately for each closure since closures are move
  let item_id_toggle = item_id.clone();
  let item_id_disable = item_id.clone();
  let item_path_clone = item_path.clone();

  let handle_toggle = move |_| {
    let enabled = !item_enabled;
    let payload = serde_json::json!({ "id": item_id_toggle, "enabled": enabled });
    let _ = bridge::invoke_app_command("clean_set_startup_enabled", &payload);
  };

  let handle_disable = move |_| {
    let payload = serde_json::json!({ "id": item_id_disable });
    let _ = bridge::invoke_app_command("clean_disable_startup_item", &payload);
  };

  let handle_open_location = move |_| {
    if let Some(ref path) = item_path_clone {
      if !path.is_empty() {
        let payload = serde_json::json!({ "path": path });
        let _ = bridge::invoke_app_command("opener_open_folder", &payload);
      }
    }
  };

  rsx! {
      div {
          class: "startup-item {item_disabled_class}",
          div {
              class: "startup-item-main",
              div {
                  class: "startup-item-toggle",
                  button {
                      class: "toggle-btn {toggle_active_class}",
                      onclick: handle_toggle,
                      span {
                          class: "toggle-track {toggle_active_class}",
                          span { class: "toggle-thumb" }
                      }
                  }
              }

              div {
                  class: "startup-item-info",
                  div {
                      class: "startup-item-header",
                      span { class: "startup-item-name", "{props.item.name}" }
                      span {
                          class: "startup-item-badge badge-{props.item.source}",
                          "{source_label}"
                      }
                  }
                  div { class: "startup-item-command", "{props.item.command}" }
              }

              div {
                  class: "startup-item-actions",
                  if item_path.is_some() {
                      button {
                          class: "btn btn-ghost btn-sm",
                          onclick: handle_open_location,
                          title: "Open file location",
                          "📁"
                      }
                  }
                  div {
                      class: "dropdown",
                      button {
                          class: "btn btn-ghost btn-sm",
                          onclick: move |_| show_menu.toggle(),
                          "⋯"
                      }
                      if show_menu() {
                          div {
                              class: "dropdown-menu",
                              button {
                                  class: "dropdown-item",
                                  onclick: move |_| {
                                      handle_disable(());
                                      show_menu.set(false);
                                  },
                                  "Disable Permanently"
                              }
                          }
                      }
                  }
              }
          }
      }
  }
}
