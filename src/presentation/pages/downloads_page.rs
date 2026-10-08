//! Downloads Cleaner page — mirrors the template `data-page="downloads-cleaner"` section.

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

fn file_icon(ext: &str) -> &'static str {
  match ext.to_lowercase().as_str() {
    "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt" => "description",
    "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "ico" => "image",
    "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" => "movie",
    "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" => "audio_file",
    "zip" | "tar" | "gz" | "rar" | "7z" | "xz" | "bz2" => "folder_zip",
    "deb" | "rpm" | "AppImage" | "sh" => "terminal",
    "iso" | "img" => "disc_full",
    "exe" | "msi" => "download",
    _ => "insert_drive_file",
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DownloadFile {
  pub name: String,
  pub path: String,
  pub size: u64,
  pub size_human: String,
  pub modified: String,
  pub file_type: String,
  pub extension: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SortBy {
  Name,
  Size,
  Date,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum AgeFilter {
  All,
  Today,
  ThisWeek,
  ThisMonth,
  Older,
}

impl AgeFilter {
  #[allow(dead_code)]
  fn label(&self) -> &'static str {
    match self {
      AgeFilter::All => "All",
      AgeFilter::Today => "Today",
      AgeFilter::ThisWeek => "This Week",
      AgeFilter::ThisMonth => "This Month",
      AgeFilter::Older => "Older",
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TypeFilter {
  All,
  Documents,
  Images,
  Videos,
  Archives,
  Other,
}

impl TypeFilter {
  fn label(&self) -> &'static str {
    match self {
      TypeFilter::All => "All",
      TypeFilter::Documents => "Documents",
      TypeFilter::Images => "Images",
      TypeFilter::Videos => "Videos",
      TypeFilter::Archives => "Archives",
      TypeFilter::Other => "Other",
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadScanResult {
  pub files: Vec<DownloadFile>,
}

#[component]
pub fn DownloadsPage(state: AppState) -> Element {
  let mut files = use_signal(|| Vec::<DownloadFile>::new());
  let mut is_loading = use_signal(|| false);
  let mut status_msg = use_signal(|| String::new());
  let mut search_query = use_signal(|| String::new());
  let mut sort_by = use_signal(|| SortBy::Date);
  let mut _age_filter = use_signal(|| AgeFilter::All);
  let mut type_filter = use_signal(|| TypeFilter::All);
  let mut selected = use_signal(|| std::collections::HashSet::<String>::new());
  let mut show_confirm = use_signal(|| false);

  let downloads_path = dirs::download_dir()
    .unwrap_or_default()
    .to_string_lossy()
    .to_string();

  // Scan on mount
  use_effect(move || {
    is_loading.set(true);
    status_msg.set("Scanning downloads...".to_string());
    let path = downloads_path.clone();
    let payload = serde_json::json!({ "path": path });
    match bridge::invoke_app_command("storage_scan_directory", &payload) {
      Ok(resp) => {
        if let Ok(result) = serde_json::from_value::<DownloadScanResult>(resp) {
          let count = result.files.len();
          files.set(result.files);
          status_msg.set(format!("Found {} files", count));
        }
      }
      Err(e) => {
        status_msg.set(format!("Error: {}", e));
      }
    }
    is_loading.set(false);
  });

  // Filter and sort files
  let all_files = files.read();
  let q = search_query.read().to_lowercase();
  let tf = *type_filter.read();
  let sort = *sort_by.read();

  let mut filtered: Vec<DownloadFile> = all_files
    .iter()
    .filter(|f| {
      if !q.is_empty() && !f.name.to_lowercase().contains(&q) {
        return false;
      }
      if tf != TypeFilter::All {
        let matches = match tf {
          TypeFilter::Documents => matches!(
            f.extension.to_lowercase().as_str(),
            "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt"
          ),
          TypeFilter::Images => matches!(
            f.extension.to_lowercase().as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "ico"
          ),
          TypeFilter::Videos => matches!(
            f.extension.to_lowercase().as_str(),
            "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm"
          ),
          TypeFilter::Archives => matches!(
            f.extension.to_lowercase().as_str(),
            "zip" | "tar" | "gz" | "rar" | "7z" | "xz" | "bz2"
          ),
          TypeFilter::Other => !matches!(
            f.extension.to_lowercase().as_str(),
            "pdf"
              | "doc"
              | "docx"
              | "txt"
              | "rtf"
              | "odt"
              | "jpg"
              | "jpeg"
              | "png"
              | "gif"
              | "bmp"
              | "webp"
              | "svg"
              | "ico"
              | "mp4"
              | "mkv"
              | "avi"
              | "mov"
              | "wmv"
              | "flv"
              | "webm"
              | "zip"
              | "tar"
              | "gz"
              | "rar"
              | "7z"
              | "xz"
              | "bz2"
          ),
          _ => true,
        };
        if !matches {
          return false;
        }
      }
      true
    })
    .cloned()
    .collect();

  match sort {
    SortBy::Name => filtered.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
    SortBy::Size => filtered.sort_by(|a, b| b.size.cmp(&a.size)),
    SortBy::Date => filtered.sort_by(|a, b| b.modified.cmp(&a.modified)),
  }

  let total_size: u64 = filtered.iter().map(|f| f.size).sum();
  let selected_size: u64 = filtered
    .iter()
    .filter(|f| selected.read().contains(&f.path))
    .map(|f| f.size)
    .sum();
  let selected_count = selected.read().len();
  let all_selected =
    !filtered.is_empty() && filtered.iter().all(|f| selected.read().contains(&f.path));
  let filtered_count = filtered.len();
  let filtered_list = filtered;

  rsx! {
      section { "data-page": "downloads",
          div { class: "max-w-5xl mx-auto space-y-6",
              div { class: "flex items-center justify-between",
                  div {
                      h2 { class: "text-2xl font-bold", "Downloads" }
                      p { class: "text-sm text-zinc-500 dark:text-zinc-400", "Clean up your downloads folder" }
                  }
                  div { class: "flex gap-2",
                      span { class: "text-sm text-zinc-500", "{format_bytes(total_size)} total" }
                  }
              }

              // Controls
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-4 space-y-4",
                  div { class: "flex flex-col md:flex-row gap-3",
                      div { class: "flex-1",
                          input {
                              class: "w-full px-3 py-2 rounded-lg border border-zinc-300 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm focus:outline-none focus:ring-2 focus:ring-cyan-500",
                              r#type: "text",
                              placeholder: "Search files...",
                              value: "{search_query.read()}",
                              oninput: move |e| search_query.set(e.value().clone()),
                          }
                      }
                      button {
                          class: "px-4 py-2 bg-cyan-500 hover:bg-cyan-600 text-white rounded-lg text-sm font-medium",
                          onclick: move |_| {
                              is_loading.set(true);
                              let dl_path = dirs::download_dir().unwrap_or_default().to_string_lossy().to_string();
                              let payload = serde_json::json!({ "path": dl_path });
                              match bridge::invoke_app_command("storage_scan_directory", &payload) {
                                  Ok(resp) => {
                                      if let Ok(result) = serde_json::from_value::<DownloadScanResult>(resp) {
                                          files.set(result.files);
                                      }
                                  }
                                  Err(e) => { status_msg.set(format!("Error: {}", e)); }
                              }
                              is_loading.set(false);
                          },
                          "Refresh"
                      }
                  }
                  div { class: "flex flex-wrap gap-2",
                      div { class: "flex items-center gap-1 text-sm text-zinc-500",
                          "Sort:"
                          select {
                              class: "px-2 py-1 rounded border border-zinc-300 dark:border-zinc-700 bg-white dark:bg-zinc-800 text-sm",
                              onchange: move |e| {
                                  let v = e.value();
                                  sort_by.set(match v.as_str() {
                                      "name" => SortBy::Name,
                                      "size" => SortBy::Size,
                                      _ => SortBy::Date,
                                  });
                              },
                              option { value: "date", selected: *sort_by.read() == SortBy::Date, "Date" }
                              option { value: "size", selected: *sort_by.read() == SortBy::Size, "Size" }
                              option { value: "name", selected: *sort_by.read() == SortBy::Name, "Name" }
                          }
                      }
                      div { class: "flex items-center gap-1 text-sm text-zinc-500",
                          "Type:"
                          for tf in [TypeFilter::All, TypeFilter::Documents, TypeFilter::Images, TypeFilter::Videos, TypeFilter::Archives, TypeFilter::Other] {
                              button {
                                  class: format!(
                                      "px-2 py-1 rounded text-xs font-medium transition-colors {}",
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
              }

              // File List
              if *is_loading.read() {
                  div { class: "text-center py-16 text-zinc-400", p { "Scanning..." } }
              } else if filtered_count == 0 {
                  div { class: "text-center py-16 text-zinc-400",
                      span { class: "material-symbols-rounded text-6xl mb-4 block", "folder_open" }
                      p { class: "text-lg font-medium", "No files found" }
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
                                      let all_files = files.read();
                                      let tf = TypeFilter::All;
                                      let paths: std::collections::HashSet<String> = all_files.iter()
                                          .filter(|f| {
                                              match tf {
                                                  TypeFilter::All => true,
                                                  TypeFilter::Documents => matches!(f.extension.to_lowercase().as_str(), "pdf"|"doc"|"docx"|"txt"|"rtf"|"odt"),
                                                  TypeFilter::Images => matches!(f.extension.to_lowercase().as_str(), "jpg"|"jpeg"|"png"|"gif"|"bmp"|"webp"|"svg"|"ico"),
                                                  TypeFilter::Videos => matches!(f.extension.to_lowercase().as_str(), "mp4"|"mkv"|"avi"|"mov"|"wmv"|"flv"|"webm"),
                                                  TypeFilter::Archives => matches!(f.extension.to_lowercase().as_str(), "zip"|"tar"|"gz"|"rar"|"7z"|"xz"|"bz2"),
                                                  TypeFilter::Other => !matches!(f.extension.to_lowercase().as_str(), "pdf"|"doc"|"docx"|"txt"|"rtf"|"odt"|"jpg"|"jpeg"|"png"|"gif"|"bmp"|"webp"|"svg"|"ico"|"mp4"|"mkv"|"avi"|"mov"|"wmv"|"flv"|"webm"|"zip"|"tar"|"gz"|"rar"|"7z"|"xz"|"bz2"),
                                              }
                                          })
                                          .map(|f| f.path.clone())
                                          .collect();
                                      selected.set(paths);
                                  }
                              },
                          }
                          span { class: "flex-1 text-xs font-medium text-zinc-500 uppercase", "Name" }
                          span { class: "w-24 text-xs font-medium text-zinc-500 uppercase text-right", "Size" }
                          span { class: "w-32 text-xs font-medium text-zinc-500 uppercase", "Modified" }
                          span { class: "w-16 text-xs font-medium text-zinc-500 uppercase text-center", "Actions" }
                      }
                      div { class: "divide-y divide-zinc-100 dark:divide-zinc-800 max-h-96 overflow-y-auto",
                          for file in filtered_list {
                              DownloadFileRow {
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
              }

              // Action Bar
              if filtered_count > 0 {
                  div { class: "sticky bottom-20 bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 rounded-xl p-4 flex items-center justify-between",
                      div { class: "text-sm text-zinc-500",
                          "{selected_count} selected ({format_bytes(selected_size)})"
                      }
                      div { class: "flex gap-2",
                          button {
                              class: "px-4 py-2 border border-zinc-300 dark:border-zinc-700 rounded-lg text-sm font-medium hover:bg-zinc-100 dark:hover:bg-zinc-800",
                              onclick: move |_| {
                                  let home = dirs::home_dir().unwrap_or_default();
                                  open::that(home.join("Downloads")).ok();
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
                              "This will move {format_bytes(selected_size)} of files to trash."
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

              if !status_msg.read().is_empty() {
                  div { class: "text-sm text-zinc-500 dark:text-zinc-400", "{status_msg.read()}" }
              }
          }
      }
  }
}

#[derive(Debug, Clone, Props, PartialEq)]
struct DownloadFileRowProps {
  file: DownloadFile,
  selected: bool,
  on_toggle: Callback<String>,
}

#[component]
fn DownloadFileRow(props: DownloadFileRowProps) -> Element {
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
              span { class: "text-sm truncate", "{props.file.name}" }
          }
          span { class: "w-24 text-sm text-zinc-500 text-right whitespace-nowrap", "{props.file.size_human}" }
          span { class: "w-32 text-xs text-zinc-500 whitespace-nowrap", "{props.file.modified}" }
          div { class: "w-16 flex items-center justify-center gap-1",
              button {
                  class: "p-1.5 rounded hover:bg-zinc-200 dark:hover:bg-zinc-700 text-zinc-500",
                  title: "Open file",
                  onclick: move |_| {
                      open::that(&file_path_for_open).ok();
                  },
                  span { class: "material-symbols-rounded text-lg", "open_in_new" }
              }
          }
      }
  }
}
