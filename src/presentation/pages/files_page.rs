//! Large Files Finder page — mirrors the template `data-page="large-files"` section.
//!
//! Scans directories to find large files with configurable threshold and filters.

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LargeFile {
  pub name: String,
  pub path: String,
  pub size: u64,
  pub size_human: String,
  pub file_type: String,
  pub extension: String,
  pub modified: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SizeUnit {
  KB,
  MB,
  GB,
}

impl SizeUnit {
  fn to_bytes(&self, value: u64) -> u64 {
    match self {
      SizeUnit::KB => value * 1024,
      SizeUnit::MB => value * 1024 * 1024,
      SizeUnit::GB => value * 1024 * 1024 * 1024,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileTypeFilter {
  All,
  Archives,
  Videos,
  Images,
  Documents,
  Other,
}

impl FileTypeFilter {
  fn label(&self) -> &'static str {
    match self {
      FileTypeFilter::All => "All",
      FileTypeFilter::Archives => "Archives",
      FileTypeFilter::Videos => "Videos",
      FileTypeFilter::Images => "Images",
      FileTypeFilter::Documents => "Documents",
      FileTypeFilter::Other => "Other",
    }
  }

  fn matches(&self, ext: &str) -> bool {
    match self {
      FileTypeFilter::All => true,
      FileTypeFilter::Archives => matches!(
        ext.to_lowercase().as_str(),
        "zip" | "tar" | "gz" | "rar" | "7z" | "xz" | "bz2"
      ),
      FileTypeFilter::Videos => matches!(
        ext.to_lowercase().as_str(),
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm"
      ),
      FileTypeFilter::Images => matches!(
        ext.to_lowercase().as_str(),
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg"
      ),
      FileTypeFilter::Documents => matches!(
        ext.to_lowercase().as_str(),
        "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt"
      ),
      FileTypeFilter::Other => !matches!(
        ext.to_lowercase().as_str(),
        "zip"
          | "tar"
          | "gz"
          | "rar"
          | "7z"
          | "xz"
          | "bz2"
          | "mp4"
          | "mkv"
          | "avi"
          | "mov"
          | "wmv"
          | "flv"
          | "webm"
          | "jpg"
          | "jpeg"
          | "png"
          | "gif"
          | "bmp"
          | "webp"
          | "svg"
          | "pdf"
          | "doc"
          | "docx"
          | "txt"
          | "rtf"
          | "odt"
      ),
    }
  }
}

fn file_icon(ext: &str) -> &'static str {
  match ext.to_lowercase().as_str() {
    "zip" | "tar" | "gz" | "rar" | "7z" | "xz" | "bz2" => "folder_zip",
    "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" => "movie",
    "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" => "image",
    "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt" => "description",
    "iso" | "img" => "disc_full",
    _ => "insert_drive_file",
  }
}

#[component]
pub fn FilesPage(state: AppState) -> Element {
  let mut scan_path = use_signal(|| {
    dirs::home_dir()
      .unwrap_or_default()
      .to_string_lossy()
      .to_string()
  });
  let mut threshold_value = use_signal(|| 100u64);
  let mut threshold_unit = use_signal(|| SizeUnit::MB);
  let mut type_filter = use_signal(|| FileTypeFilter::All);
  let mut exclude_dirs = use_signal(|| String::new());
  let mut is_scanning = use_signal(|| false);
  let mut files = use_signal(|| Vec::<LargeFile>::new());
  let mut search_query = use_signal(|| String::new());
  let mut status_msg = use_signal(|| String::new());
  let mut selected = use_signal(|| std::collections::HashSet::<String>::new());
  let mut show_confirm = use_signal(|| false);

  // Filter files based on threshold and type
  let threshold_bytes = threshold_unit.read().to_bytes(*threshold_value.read());
  let exclude_list: Vec<String> = exclude_dirs
    .read()
    .split(',')
    .map(|s| s.trim().to_string())
    .filter(|s| !s.is_empty())
    .collect();

  let all_files = files.read();
  let q = search_query.read().to_lowercase();

  let filtered: Vec<LargeFile> = all_files
    .iter()
    .filter(|f| {
      if f.size < threshold_bytes {
        return false;
      }
      if !type_filter.read().matches(&f.extension) {
        return false;
      }
      if !q.is_empty() && !f.name.to_lowercase().contains(&q) && !f.path.to_lowercase().contains(&q)
      {
        return false;
      }
      for exclude in &exclude_list {
        if f.path.contains(exclude) {
          return false;
        }
      }
      true
    })
    .cloned()
    .collect();

  let total_size: u64 = filtered.iter().map(|f| f.size).sum();
  let selected_count = selected.read().len();
  let selected_size: u64 = filtered
    .iter()
    .filter(|f| selected.read().contains(&f.path))
    .map(|f| f.size)
    .sum();
  let all_selected =
    !filtered.is_empty() && filtered.iter().all(|f| selected.read().contains(&f.path));
  let filtered_count = filtered.len();
  let filtered_list = filtered;
  let all_filtered_paths: std::collections::HashSet<String> =
    filtered_list.iter().map(|f| f.path.clone()).collect();

  rsx! {
      section { "data-page": "large-files",
          div { class: "max-w-5xl mx-auto space-y-6",
              div { class: "flex items-center justify-between",
                  div {
                      h2 { class: "text-2xl font-bold", "Large Files" }
                      p { class: "text-sm text-zinc-500 dark:text-zinc-400", "Find large files consuming disk space" }
                  }
                  if filtered_count > 0 {
                      div { class: "text-sm text-zinc-500",
                          "{filtered_count} files · {format_bytes(total_size)}"
                      }
                  }
              }

              // Scan Controls
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5 space-y-4",
                  div { class: "flex flex-col md:flex-row gap-4",
                      div { class: "flex-1",
                          label { class: "block text-sm font-medium mb-1.5", "Directory to Scan" }
                          input {
                              class: "w-full px-3 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500",
                              r#type: "text",
                              value: "{scan_path.read()}",
                              oninput: move |e| scan_path.set(e.value().clone()),
                          }
                      }
                      button {
                          class: format!(
                              "mt-6 px-5 py-2 rounded-lg font-medium text-sm transition-colors {}",
                              if *is_scanning.read() {
                                  "bg-zinc-300 dark:bg-zinc-700 text-zinc-500 cursor-not-allowed"
                              } else {
                                  "bg-cyan-500 hover:bg-cyan-600 text-white"
                              }
                          ),
                          disabled: *is_scanning.read(),
                          onclick: move |_| {
                              is_scanning.set(true);
                              status_msg.set("Scanning...".to_string());
                              let path = scan_path.read().clone();
                              let payload = serde_json::json!({ "path": path });
                              match bridge::invoke_app_command("storage_scan_directory", &payload) {
                                  Ok(resp) => {
                                      if let Ok(result) = serde_json::from_value::<crate::application::handlers::storage_handlers::DirectoryInfo>(resp) {
                                          files.set(vec![]);
                                          status_msg.set(format!("Scanned: {} files, {} subdirs in {}",
                                              result.files, result.subdirs, format_bytes(result.size)));
                                      }
                                  }
                                  Err(e) => { status_msg.set(format!("Error: {}", e)); }
                              }
                              is_scanning.set(false);
                          },
                          if *is_scanning.read() {
                              span { class: "flex items-center gap-2", "Scanning..." }
                          } else {
                              span { class: "flex items-center gap-2", "Scan" }
                          }
                      }
                  }

                  // Size Threshold
                  div { class: "flex flex-col sm:flex-row gap-4 items-end",
                      div { class: "flex-1",
                          label { class: "block text-sm font-medium mb-1.5", "Minimum File Size" }
                          div { class: "flex gap-2",
                              input {
                                  class: "w-24 px-3 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500",
                                  r#type: "number",
                                  min: "1",
                                  value: "{threshold_value.read()}",
                                  oninput: move |e| { if let Ok(v) = e.value().parse() { threshold_value.set(v); } },
                              }
                              select {
                                  class: "px-3 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm",
                                  onchange: move |e| {
                                      threshold_unit.set(match e.value().as_str() {
                                          "kb" => SizeUnit::KB,
                                          "gb" => SizeUnit::GB,
                                          _ => SizeUnit::MB,
                                      });
                                  },
                                  option { value: "kb", "KB" }
                                  option { value: "mb", selected: *threshold_unit.read() == SizeUnit::MB, "MB" }
                                  option { value: "gb", "GB" }
                              }
                          }
                      }
                      div { class: "flex-1",
                          label { class: "block text-sm font-medium mb-1.5", "Exclude Directories" }
                          input {
                              class: "w-full px-3 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500",
                              r#type: "text",
                              placeholder: "node_modules, .git, cache",
                              value: "{exclude_dirs.read()}",
                              oninput: move |e| exclude_dirs.set(e.value().clone()),
                          }
                      }
                  }

                  // Type Filter
                  div { class: "flex flex-wrap gap-2",
                      span { class: "text-sm text-zinc-500 self-center mr-2", "Type:" }
                      for tf in [FileTypeFilter::All, FileTypeFilter::Archives, FileTypeFilter::Videos, FileTypeFilter::Images, FileTypeFilter::Documents, FileTypeFilter::Other] {
                          button {
                              class: format!(
                                  "px-3 py-1.5 rounded-lg text-xs font-medium transition-colors {}",
                                  if *type_filter.read() == tf {
                                      "bg-cyan-100 dark:bg-cyan-900 text-cyan-700 dark:text-cyan-300"
                                  } else {
                                      "bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-400 hover:bg-zinc-200 dark:hover:bg-zinc-700"
                                  }
                              ),
                              onclick: move |_| type_filter.set(tf),
                              "{tf.label()}"
                          }
                      }
                  }
              }

              // Status
              if !status_msg.read().is_empty() {
                  div { class: "text-sm text-zinc-500 dark:text-zinc-400", "{status_msg.read()}" }
              }

              // File List
              if *is_scanning.read() {
                  div { class: "text-center py-16 text-zinc-400",
                      div { class: "animate-pulse mb-4", "Scanning for large files..." }
                      div { class: "w-full max-w-md mx-auto h-2 bg-zinc-200 dark:bg-zinc-700 rounded-full overflow-hidden",
                          div { class: "h-full bg-cyan-500 animate-pulse", style: "width: 60%" }
                      }
                  }
              } else if filtered_count == 0 {
                  div { class: "text-center py-16 text-zinc-400",
                      span { class: "material-symbols-rounded text-6xl mb-4 block", "folder_open" }
                      p { class: "text-lg font-medium", "No large files found" }
                      p { class: "text-sm", "Try lowering the size threshold or changing filters" }
                  }
              } else {
                  div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 overflow-hidden",
                      div { class: "flex items-center gap-3 px-4 py-3 border-b border-zinc-200 dark:border-zinc-800 bg-zinc-50 dark:bg-zinc-800/50",
                          input {
                              class: "w-4 h-4 rounded border-zinc-300",
                              r#type: "checkbox",
                              checked: all_selected,
                              onchange: move |_| {
                                  if all_selected {
                                      selected.set(std::collections::HashSet::new());
                                  } else {
                                      selected.set(all_filtered_paths.clone());
                                  }
                              },
                          }
                          span { class: "flex-1 text-xs font-medium text-zinc-500 uppercase", "File" }
                          span { class: "w-24 text-xs font-medium text-zinc-500 uppercase text-right", "Size" }
                          span { class: "w-16 text-xs font-medium text-zinc-500 uppercase text-center", "Actions" }
                      }
                      div { class: "divide-y divide-zinc-100 dark:divide-zinc-800 max-h-96 overflow-y-auto",
                          for file in filtered_list {
                              LargeFileRow {
                                  file: file.clone(),
                                  selected: selected.read().contains(&file.path),
                                  on_toggle: move |path: String| {
                                      let mut s = selected.read().clone();
                                      if s.contains(&path) { s.remove(&path); } else { s.insert(path); }
                                      selected.set(s);
                                  },
                              }
                          }
                      }
                  }

                  // Action Bar
                  div { class: "sticky bottom-20 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-xl p-4 flex items-center justify-between",
                      div { class: "text-sm text-zinc-500",
                          "{selected_count} selected ({format_bytes(selected_size)})"
                      }
                      div { class: "flex gap-2",
                          button {
                              class: "px-4 py-2 border border-zinc-300 dark:border-zinc-700 rounded-lg text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800",
                              onclick: move |_| {
                                  let home = dirs::home_dir().unwrap_or_default();
                                  open::that(home).ok();
                              },
                              "Open Folder"
                          }
                          button {
                              class: "px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg text-sm font-medium",
                              disabled: selected_count == 0,
                              onclick: move |_| show_confirm.set(true),
                              "Delete Selected"
                          }
                      }
                  }

                  if *show_confirm.read() {
                      div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
                          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-6 w-full max-w-sm mx-4",
                              div { class: "flex items-center gap-3 mb-4",
                                  span { class: "material-symbols-rounded text-3xl text-red-500", "warning" }
                                  div {
                                      h3 { class: "text-lg font-bold", "Confirm Deletion" }
                                      p { class: "text-sm text-zinc-500", "Delete {selected_count} files?" }
                                  }
                              }
                              p { class: "text-sm text-zinc-600 dark:text-zinc-400 mb-5",
                                  "This will permanently delete {format_bytes(selected_size)} of files."
                              }
                              div { class: "flex gap-3 justify-end",
                                  button {
                                      class: "px-4 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800",
                                      onclick: move |_| show_confirm.set(false),
                                      "Cancel"
                                  }
                                  button {
                                      class: "px-4 py-2 rounded-lg bg-red-500 hover:bg-red-600 text-white text-sm font-medium",
                                      onclick: move |_| {
                                          show_confirm.set(false);
                                          status_msg.set("Delete not yet wired — implement storage_delete_files handler".to_string());
                                      },
                                      "Delete"
                                  }
                              }
                          }
                      }
                  }
              }
          }
      }
  }
}

#[derive(Debug, Clone, Props, PartialEq)]
struct LargeFileRowProps {
  file: LargeFile,
  selected: bool,
  on_toggle: Callback<String>,
}

#[component]
fn LargeFileRow(props: LargeFileRowProps) -> Element {
  let file_path = props.file.path.clone();
  let file_path_for_open = file_path.clone();
  rsx! {
      div { class: "flex items-center gap-3 px-4 py-3 hover:bg-zinc-50 dark:hover:bg-zinc-800/50",
          input {
              class: "w-4 h-4 rounded border-zinc-300 flex-shrink-0",
              r#type: "checkbox",
              checked: props.selected,
              onchange: move |_| props.on_toggle.call(file_path.clone()),
          }
          div { class: "flex items-center gap-2 flex-1 min-w-0",
              span { class: "material-symbols-rounded text-xl text-zinc-400 flex-shrink-0", "{file_icon(&props.file.extension)}" }
              div { class: "min-w-0",
                  p { class: "text-sm font-medium truncate", "{props.file.name}" }
                  p { class: "text-xs text-zinc-500 truncate", "{props.file.path}" }
              }
          }
          span { class: "w-24 text-sm text-zinc-500 text-right whitespace-nowrap", "{props.file.size_human}" }
          div { class: "w-16 flex items-center justify-center gap-1",
              button {
                  class: "p-1.5 rounded hover:bg-zinc-200 dark:hover:bg-zinc-700 text-zinc-500",
                  title: "Open containing folder",
                  onclick: move |_| {
                      if let Some(parent) = std::path::Path::new(&file_path_for_open).parent() {
                          open::that(parent).ok();
                      }
                  },
                  span { class: "material-symbols-rounded text-lg", "open_in_new" }
              }
          }
      }
  }
}
