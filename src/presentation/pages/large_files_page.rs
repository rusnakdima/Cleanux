//! Large Files page — mirrors the template `data-page="large-files"` section.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LargeFileEntry {
  pub name: String,
  pub path: String,
  pub size: u64,
  pub modified: String,
}

fn format_bytes(bytes: u64) -> String {
  const GB: u64 = 1024 * 1024 * 1024;
  const MB: u64 = 1024 * 1024;
  const KB: u64 = 1024;
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

#[component]
pub fn LargeFilesPage(state: AppState) -> Element {
  let mut scan_path = use_signal(|| String::from("/home"));
  let mut min_size = use_signal(|| String::from("100MB"));
  let mut files = use_signal(|| Vec::<LargeFileEntry>::new());
  let mut is_scanning = use_signal(|| false);
  let mut is_deleting = use_signal(|| false);
  let mut scan_status = use_signal(|| "Ready".to_string());
  let mut delete_result = use_signal(|| String::new());
  let mut selected_paths = use_signal(|| Vec::<String>::new());

  let handle_scan = move |_| {
    is_scanning.set(true);
    scan_status.set("Scanning for large files...".to_string());
    files.set(vec![]);
    selected_paths.set(vec![]);

    let _path = scan_path.read().clone();
    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      // Use dashboard's large files summary for count, then get actual list via get_large_files
      match bridge::invoke_app_command("dashboard_get_large_files_summary", &serde_json::json!({}))
      {
        Ok(_val) => {
          // Get files from global state
          let large_files = crate::global_state::get_large_files();
          let entries: Vec<LargeFileEntry> = large_files
            .into_iter()
            .map(|lf| {
              // Parse size string back to u64 (rough approximation)
              let size: u64 = lf
                .size
                .replace(|c: char| !c.is_ascii_digit(), "")
                .parse()
                .unwrap_or(0)
                * 1024
                * 1024;
              LargeFileEntry {
                name: lf.name,
                path: lf.path,
                size,
                modified: lf.age,
              }
            })
            .collect();
          let count = entries.len();
          files.set(entries);
          scan_status.set(format!("Found {} large files", count));
        }
        Err(e) => scan_status.set(format!("Error: {}", e)),
      }
      is_scanning.set(false);
    });
  };

  let handle_delete = move |_| {
    is_deleting.set(true);
    delete_result.set("Deleting files...".to_string());

    let to_delete = selected_paths.read().clone();
    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
      let mut deleted = 0;
      for path in to_delete.iter() {
        match bridge::invoke_app_command(
          "storage_delete_file",
          &serde_json::json!({ "path": path }),
        ) {
          Ok(_) => deleted += 1,
          Err(_) => {}
        }
      }
      delete_result.set(format!("Deleted {} files", deleted));

      // Refresh list
      let remaining: Vec<_> = files
        .read()
        .iter()
        .filter(|f| !selected_paths.read().contains(&f.path))
        .cloned()
        .collect();
      files.set(remaining);
      selected_paths.set(vec![]);
      is_deleting.set(false);
    });
  };

  let file_list = files.read();

  rsx! {
      section { "data-page": "large-files",
          class: "space-y-6",

          // Search controls
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              div { class: "flex items-end gap-3",
                  div { class: "flex-1",
                      label { class: "block text-xs font-medium text-zinc-500 dark:text-zinc-400 mb-1.5", "Scan Path" }
                      input {
                          r#type: "text",
                          value: "{scan_path.read()}",
                          oninput: move |e| scan_path.set(e.value().to_string()),
                          class: "w-full px-3 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500"
                      }
                  }
                  div { class: "w-32",
                      label { class: "block text-xs font-medium text-zinc-500 dark:text-zinc-400 mb-1.5", "Min Size" }
                      input {
                          r#type: "text",
                          value: "{min_size.read()}",
                          oninput: move |e| min_size.set(e.value().to_string()),
                          class: "w-full px-3 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500"
                      }
                  }
                  button {
                      class: "px-5 py-2 rounded-xl bg-zinc-900 dark:bg-white text-white dark:text-zinc-900 font-medium text-sm hover:opacity-90 disabled:opacity-50",
                      disabled: *is_scanning.read(),
                      onclick: handle_scan,
                      if *is_scanning.read() { "Scanning..." } else { "Scan" }
                  }
              }
              p { class: "text-xs text-zinc-500 dark:text-zinc-400 mt-2", "Status: {scan_status}" }
          }

          // Results
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              div { class: "flex items-center justify-between mb-4",
                  h2 { class: "font-semibold", "Large Files" }
                  if !file_list.is_empty() {
                      button {
                          class: "px-4 py-2 rounded-xl bg-gradient-to-r from-red-500 to-rose-500 text-white font-medium text-sm hover:opacity-90 disabled:opacity-50",
                          disabled: selected_paths.read().is_empty() || *is_deleting.read(),
                          onclick: handle_delete,
                          if *is_deleting.read() { "Deleting..." } else { "Delete Selected" }
                      }
                  }
              }

              if file_list.is_empty() {
                  div { class: "py-8 text-center text-sm text-zinc-500 dark:text-zinc-400",
                      "Set a path and click Scan to find large files"
                  }
              } else {
                  div { class: "space-y-2",
                      div { class: "flex items-center gap-3 px-4 py-2 text-xs font-medium text-zinc-500 dark:text-zinc-400 uppercase tracking-wide",
                          div { class: "w-6" }
                          div { class: "flex-1", "Name" }
                          div { class: "w-24 text-right", "Size" }
                          div { class: "w-20 text-right", "Age" }
                      }
                      for file in file_list.iter() {
                          LargeFileRow {
                              file: file.clone(),
                              file_path: file.path.clone(),
                              is_selected: selected_paths.read().contains(&file.path),
                              on_toggle: {
                                  let fp = file.path.clone();
                                  move |_| {
                                      let mut sel = selected_paths.read().clone();
                                      if sel.contains(&fp) {
                                          sel.retain(|p| p != &fp);
                                      } else {
                                          sel.push(fp.clone());
                                      }
                                      selected_paths.set(sel);
                                  }
                              }
                          }
                      }
                  }
              }

              if !delete_result.read().is_empty() {
                  div { class: "mt-4 p-3 rounded-xl bg-emerald-50 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400 text-sm",
                      "{delete_result.read()}"
                  }
              }
          }
      }
  }
}

#[derive(Props, Clone, PartialEq)]
struct LargeFileRowProps {
  file: LargeFileEntry,
  file_path: String,
  is_selected: bool,
  on_toggle: Callback<String>,
}

#[component]
fn LargeFileRow(props: LargeFileRowProps) -> Element {
  let icon = if props.file.name.ends_with(".mp4") || props.file.name.ends_with(".mkv") {
    "movie"
  } else if props.file.name.ends_with(".jpg") || props.file.name.ends_with(".png") {
    "image"
  } else if props.file.name.ends_with(".tar")
    || props.file.name.ends_with(".gz")
    || props.file.name.ends_with(".zip")
  {
    "folder_zip"
  } else {
    "insert_drive_file"
  };

  let fp = props.file_path.clone();
  let fp2 = fp.clone();
  rsx! {
      div {
          class: "flex items-center gap-3 px-4 py-3 rounded-xl hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors cursor-pointer",
          onclick: move |_| props.on_toggle.call(fp.clone()),
          input {
              r#type: "checkbox",
              checked: props.is_selected,
              class: "w-4 h-4 rounded border-zinc-300 dark:border-zinc-600 text-red-500 focus:ring-red-500",
              onchange: move |_| props.on_toggle.call(fp2.clone()),
          }
          span { class: "material-symbols-rounded text-zinc-400 text-xl flex-shrink-0", "{icon}" }
          div { class: "flex-1 min-w-0",
              div { class: "text-sm font-medium truncate", "{props.file.name}" }
              div { class: "text-xs text-zinc-500 dark:text-zinc-400 truncate", "{props.file.path}" }
          }
          div { class: "w-24 text-right text-sm text-zinc-600 dark:text-zinc-400 flex-shrink-0",
              "{format_bytes(props.file.size)}"
          }
          div { class: "w-20 text-right text-xs text-zinc-500 dark:text-zinc-400 flex-shrink-0",
              "{props.file.modified}"
          }
      }
  }
}
