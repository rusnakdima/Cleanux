//! Media Cleaner page — mirrors the template `data-page="media-cleaner"` section.
//!
//! Cleans Steam shader/download cache, Spotify, VLC, thumbnails, and media artwork.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaCacheItem {
  pub name: String,
  pub path: String,
  #[serde(rename = "sizeBytes")]
  pub size_bytes: u64,
  #[serde(rename = "sizeHuman")]
  pub size_human: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaCacheSummary {
  #[serde(rename = "steamShaderBytes")]
  pub steam_shader_bytes: u64,
  #[serde(rename = "steamDownloadBytes")]
  pub steam_download_bytes: u64,
  #[serde(rename = "spotifyBytes")]
  pub spotify_bytes: u64,
  #[serde(rename = "vlcBytes")]
  pub vlc_bytes: u64,
  #[serde(rename = "thumbnailBytes")]
  pub thumbnail_bytes: u64,
  #[serde(rename = "mediaArtBytes")]
  pub media_art_bytes: u64,
  #[serde(rename = "totalBytes")]
  pub total_bytes: u64,
  #[serde(rename = "totalHuman")]
  pub total_human: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
  Steam,
  Spotify,
  VLC,
  Thumbnails,
  Icons,
}

impl Tab {
  fn label(&self) -> &'static str {
    match self {
      Tab::Steam => "Steam",
      Tab::Spotify => "Spotify",
      Tab::VLC => "VLC",
      Tab::Thumbnails => "Thumbnails",
      Tab::Icons => "Icons",
    }
  }
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
struct MediaCardProps {
  label: String,
  size: String,
  path: String,
  description: String,
  on_clean: Callback<()>,
}

#[component]
fn MediaCard(props: MediaCardProps) -> Element {
  rsx! {
      div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-700 p-5 hover:shadow-md transition-all",
          div { class: "flex items-start justify-between mb-3",
              div {
                  p { class: "font-semibold text-sm", "{props.label}" }
                  p { class: "text-xs text-zinc-500 dark:text-zinc-400 mt-0.5 font-mono max-w-48 truncate",
                      title: "{props.path}",
                      "{props.path}"
                  }
              }
              p { class: "text-xl font-bold text-red-500", "{props.size}" }
          }
          p { class: "text-xs text-zinc-400 dark:text-zinc-500 mb-4", "{props.description}" }
          button {
              class: "w-full px-3 py-2 rounded-lg bg-red-500 hover:bg-red-600 text-white text-xs font-semibold transition-colors",
              onclick: move |_| props.on_clean.call(()),
              "Clean"
          }
      }
  }
}

#[component]
pub fn MediaCleanerPage(state: AppState) -> Element {
  let mut active_tab = use_signal(|| Tab::Steam);
  let mut summary = use_signal(|| MediaCacheSummary {
    steam_shader_bytes: 0,
    steam_download_bytes: 0,
    spotify_bytes: 0,
    vlc_bytes: 0,
    thumbnail_bytes: 0,
    media_art_bytes: 0,
    total_bytes: 0,
    total_human: "0 B".to_string(),
  });
  let mut last_freed = use_signal(|| String::new());

  // Load summary on mount
  use_effect(move || {
    let result = bridge::invoke_app_command("media_cache_get_summary", &serde_json::json!({}));
    if let Ok(val) = result {
      if let Ok(s) = serde_json::from_value::<MediaCacheSummary>(val) {
        summary.set(s);
      }
    }
  });

  let tab_items: Vec<(Tab, &str)> = vec![
    (Tab::Steam, "Steam"),
    (Tab::Spotify, "Spotify"),
    (Tab::VLC, "VLC"),
    (Tab::Thumbnails, "Thumbnails"),
    (Tab::Icons, "Icons"),
  ];

  let (steam_shader, steam_download, spotify, vlc, thumbnails, media_art) = {
    let s = summary.read();
    (
      s.steam_shader_bytes,
      s.steam_download_bytes,
      s.spotify_bytes,
      s.vlc_bytes,
      s.thumbnail_bytes,
      s.media_art_bytes,
    )
  };

  rsx! {
      section { "data-page": "media-cleaner",
          class: "space-y-6",

          // Header
          div { class: "flex items-center justify-between",
              div {
                  h1 { class: "text-2xl font-bold", "Media Caches" }
                  p { class: "text-sm text-zinc-500 dark:text-zinc-400 mt-1",
                      "Steam, Spotify, VLC, thumbnails and icon caches"
                  }
              }
              div { class: "text-right",
                  p { class: "text-2xl font-bold text-red-500", "{summary.read().total_human}" }
                  p { class: "text-xs text-zinc-500 dark:text-zinc-400", "total reclaimable" }
              }
          }

          // Status banner
          if !last_freed.read().is_empty() {
              div { class: "bg-green-50 dark:bg-green-950/30 border border-green-200 dark:border-green-800 rounded-xl p-4",
                  p { class: "text-sm text-green-700 dark:text-green-300",
                      "✅ Cleaned {last_freed.read()} of media cache files"
                  }
              }
          }

          // Tab bar
          div { class: "flex gap-1 bg-zinc-100 dark:bg-zinc-800/50 rounded-xl p-1",
              for (tab, label) in tab_items {
                  button {
                      class: format!(
                          "flex-1 px-4 py-2 rounded-lg text-xs font-medium transition-all {}",
                          if *active_tab.read() == tab {
                              "bg-white dark:bg-zinc-700 shadow-sm text-zinc-900 dark:text-white"
                          } else {
                              "text-zinc-500 dark:text-zinc-400 hover:text-zinc-700 dark:hover:text-zinc-200"
                          }
                      ),
                      onclick: move |_| active_tab.set(tab),
                      "{label}"
                  }
              }
          }

          // Content based on active tab
          div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
              match *active_tab.read() {
                  Tab::Steam => {
                      rsx! {
                          MediaCard {
                              label: "Shader Cache".to_string(),
                              size: format_bytes(steam_shader),
                              path: "~/.steam/steamapps/shadercache".to_string(),
                              description: "Steam shader pre-compilation cache".to_string(),
                              on_clean: move |_| {
                                  let _ = bridge::invoke_app_command("media_cache_clean_steam_shader", &serde_json::json!({}));
                                  let result = bridge::invoke_app_command("media_cache_get_summary", &serde_json::json!({}));
                                  if let Ok(val) = result {
                                      if let Ok(s) = serde_json::from_value::<MediaCacheSummary>(val) {
                                          summary.set(s.clone());
                                          last_freed.set(s.total_human);
                                      }
                                  }
                              },
                          }
                          MediaCard {
                              label: "Download Cache".to_string(),
                              size: format_bytes(steam_download),
                              path: "~/.steam/steamapps/download".to_string(),
                              description: "Steam download cache".to_string(),
                              on_clean: move |_| {
                                  let _ = bridge::invoke_app_command("media_cache_clean_steam_download", &serde_json::json!({}));
                                  let result = bridge::invoke_app_command("media_cache_get_summary", &serde_json::json!({}));
                                  if let Ok(val) = result {
                                      if let Ok(s) = serde_json::from_value::<MediaCacheSummary>(val) {
                                          summary.set(s.clone());
                                          last_freed.set(s.total_human);
                                      }
                                  }
                              },
                          }
                      }
                  }
                  Tab::Spotify => {
                      rsx! {
                          MediaCard {
                              label: "Spotify Cache".to_string(),
                              size: format_bytes(spotify),
                              path: "~/.cache/spotify".to_string(),
                              description: "Spotify streamed audio and image cache".to_string(),
                              on_clean: move |_| {
                                  let _ = bridge::invoke_app_command("media_cache_clean_spotify", &serde_json::json!({}));
                                  let result = bridge::invoke_app_command("media_cache_get_summary", &serde_json::json!({}));
                                  if let Ok(val) = result {
                                      if let Ok(s) = serde_json::from_value::<MediaCacheSummary>(val) {
                                          summary.set(s.clone());
                                          last_freed.set(s.total_human);
                                      }
                                  }
                              },
                          }
                      }
                  }
                  Tab::VLC => {
                      rsx! {
                          MediaCard {
                              label: "VLC Cache".to_string(),
                              size: format_bytes(vlc),
                              path: "~/.cache/vlc".to_string(),
                              description: "VLC media player cache".to_string(),
                              on_clean: move |_| {
                                  let _ = bridge::invoke_app_command("media_cache_clean_vlc", &serde_json::json!({}));
                                  let result = bridge::invoke_app_command("media_cache_get_summary", &serde_json::json!({}));
                                  if let Ok(val) = result {
                                      if let Ok(s) = serde_json::from_value::<MediaCacheSummary>(val) {
                                          summary.set(s.clone());
                                          last_freed.set(s.total_human);
                                      }
                                  }
                              },
                          }
                      }
                  }
                  Tab::Thumbnails => {
                      rsx! {
                          MediaCard {
                              label: "Thumbnail Cache".to_string(),
                              size: format_bytes(thumbnails),
                              path: "~/.cache/thumbnails".to_string(),
                              description: "File manager thumbnail cache".to_string(),
                              on_clean: move |_| {
                                  let _ = bridge::invoke_app_command("media_cache_clean_thumbnails", &serde_json::json!({}));
                                  let result = bridge::invoke_app_command("media_cache_get_summary", &serde_json::json!({}));
                                  if let Ok(val) = result {
                                      if let Ok(s) = serde_json::from_value::<MediaCacheSummary>(val) {
                                          summary.set(s.clone());
                                          last_freed.set(s.total_human);
                                      }
                                  }
                              },
                          }
                      }
                  }
                  Tab::Icons => {
                      rsx! {
                          MediaCard {
                              label: "Media Art Cache".to_string(),
                              size: format_bytes(media_art),
                              path: "~/.cache/media-art".to_string(),
                              description: "Album artwork and media metadata cache".to_string(),
                              on_clean: move |_| {
                                  let _ = bridge::invoke_app_command("media_cache_clean_media_art", &serde_json::json!({}));
                                  let result = bridge::invoke_app_command("media_cache_get_summary", &serde_json::json!({}));
                                  if let Ok(val) = result {
                                      if let Ok(s) = serde_json::from_value::<MediaCacheSummary>(val) {
                                          summary.set(s.clone());
                                          last_freed.set(s.total_human);
                                      }
                                  }
                              },
                          }
                      }
                  }
              }
          }

          // Clean All button
          div { class: "flex justify-end gap-3",
              button {
                  class: "px-6 py-2.5 rounded-xl bg-red-500 hover:bg-red-600 text-white font-semibold text-sm transition-colors shadow-sm",
                  onclick: move |_| {
                      // Clean all media caches
                      let _ = bridge::invoke_app_command("media_cache_clean_steam_shader", &serde_json::json!({}));
                      let _ = bridge::invoke_app_command("media_cache_clean_steam_download", &serde_json::json!({}));
                      let _ = bridge::invoke_app_command("media_cache_clean_spotify", &serde_json::json!({}));
                      let _ = bridge::invoke_app_command("media_cache_clean_vlc", &serde_json::json!({}));
                      let _ = bridge::invoke_app_command("media_cache_clean_thumbnails", &serde_json::json!({}));
                      let _ = bridge::invoke_app_command("media_cache_clean_media_art", &serde_json::json!({}));
                      let result = bridge::invoke_app_command("media_cache_get_summary", &serde_json::json!({}));
                      if let Ok(val) = result {
                          if let Ok(s) = serde_json::from_value::<MediaCacheSummary>(val) {
                              summary.set(s);
                          }
                      }
                  },
                  "Clean All Media Caches"
              }
          }
      }
  }
}
