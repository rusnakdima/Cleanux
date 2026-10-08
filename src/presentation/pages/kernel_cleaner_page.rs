//! Kernel Cleaner page — mirrors the template `data-page="kernel-cleaner"` section.

use crate::app::AppState;
use crate::bridge;
use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BootSpaceInfo {
  pub filesystem: String,
  pub total_bytes: u64,
  pub used_bytes: u64,
  pub available_bytes: u64,
  pub use_percent: u8,
  pub mounted_on: String,
}

impl BootSpaceInfo {
  fn total_mb(&self) -> u64 {
    self.total_bytes / (1024 * 1024)
  }
  fn used_mb(&self) -> u64 {
    self.used_bytes / (1024 * 1024)
  }
  fn available_mb(&self) -> u64 {
    self.available_bytes / (1024 * 1024)
  }
}

#[component]
pub fn KernelCleanerPage(state: AppState) -> Element {
  let mut current_kernel = use_signal(|| Option::<String>::None);
  let mut installed_kernels = use_signal(|| Vec::<String>::new());
  let mut old_kernels = use_signal(|| Vec::<String>::new());
  let mut boot_space_list = use_signal(|| Vec::<BootSpaceInfo>::new());
  let mut old_initramfs = use_signal(|| Vec::<String>::new());
  let mut selected_kernels = use_signal(|| std::collections::HashSet::<String>::new());
  let mut selected_initramfs = use_signal(|| std::collections::HashSet::<String>::new());
  let mut is_removing = use_signal(|| false);
  let mut is_refreshing_grub = use_signal(|| false);
  let mut status_message = use_signal(|| Option::<String>::None);
  let mut show_confirm = use_signal(|| false);
  let mut confirm_action = use_signal(|| String::new());
  let mut pending_kernel = use_signal(|| Option::<String>::None);
  let mut pending_count = use_signal(|| 0usize);

  // Load all kernel info on mount
  use_effect(move || {
    // Current kernel: returns Response<String> → data field is the string
    if let Ok(result) =
      bridge::invoke_app_command("system_get_current_kernel", &serde_json::json!({}))
    {
      if let Some(data) = result.get("data").and_then(|d| d.as_str()) {
        current_kernel.set(Some(data.to_string()));
      }
    }

    // Installed kernels: returns Response<Vec<String>>
    if let Ok(result) =
      bridge::invoke_app_command("system_get_installed_kernels", &serde_json::json!({}))
    {
      if let Some(data) = result.get("data") {
        if let Ok(kernels) = serde_json::from_value::<Vec<String>>(data.clone()) {
          installed_kernels.set(kernels);
        }
      }
    }

    // Old kernels: returns Response<Vec<String>>
    if let Ok(result) = bridge::invoke_app_command("system_get_old_kernels", &serde_json::json!({}))
    {
      if let Some(data) = result.get("data") {
        if let Ok(kernels) = serde_json::from_value::<Vec<String>>(data.clone()) {
          old_kernels.set(kernels);
        }
      }
    }

    // Boot space info: returns Response<Vec<BootSpaceInfo>>
    if let Ok(result) =
      bridge::invoke_app_command("system_get_boot_space_info", &serde_json::json!({}))
    {
      if let Some(data) = result.get("data") {
        if let Ok(infos) = serde_json::from_value::<Vec<BootSpaceInfo>>(data.clone()) {
          boot_space_list.set(infos);
        }
      }
    }

    // Old initramfs: returns Response<Vec<String>>
    if let Ok(result) =
      bridge::invoke_app_command("system_get_old_initramfs", &serde_json::json!({}))
    {
      if let Some(data) = result.get("data") {
        if let Ok(files) = serde_json::from_value::<Vec<String>>(data.clone()) {
          old_initramfs.set(files);
        }
      }
    }
  });

  // Pre-compute simple display values (no signal borrows needed for these)
  let current_kernel_display = current_kernel
    .read()
    .clone()
    .unwrap_or_else(|| "loading...".to_string());

  let installed_count = installed_kernels.read().len();
  let old_kernels_count = old_kernels.read().len();
  let initramfs_count = old_initramfs.read().len();
  let selected_kernels_count = selected_kernels.read().len();
  let selected_initramfs_count = selected_initramfs.read().len();
  let is_removing_val = *is_removing.read();
  let is_refreshing_grub_val = *is_refreshing_grub.read();
  let status_msg = status_message.read().clone();
  let show_confirm_val = *show_confirm.read();
  let confirm_act = confirm_action.read().clone();
  let _pending_ker = pending_kernel.read().clone().unwrap_or_default();
  let pending_cnt = *pending_count.read();

  // Boot infos: filter for boot partitions
  let boot_infos: Vec<BootSpaceInfo> = boot_space_list
    .read()
    .iter()
    .filter(|i| i.mounted_on == "/boot" || i.filesystem.contains("boot") || i.mounted_on == "/")
    .cloned()
    .collect();

  rsx! {
      section { "data-page": "kernel-cleaner",
          class: "space-y-6",

          // Header
          div { class: "flex items-center justify-between",
              div {
                  h1 { class: "text-xl font-bold", "Kernel Cleaner" }
                  p { class: "text-sm text-zinc-500 dark:text-zinc-400", "Manage and remove old kernels to free up disk space" }
              }
          }

          // Current Kernel Card
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              div { class: "flex items-center gap-4",
                  div { class: "w-12 h-12 rounded-xl bg-gradient-to-br from-cyan-500 to-blue-600 flex items-center justify-center",
                      span { class: "material-symbols-rounded text-white text-2xl", "kernel" }
                  }
                  div { class: "flex-1",
                      div { class: "text-xs font-medium text-cyan-600 dark:text-cyan-400 uppercase tracking-wider mb-1", "Currently Running" }
                      div { class: "text-lg font-semibold font-mono", "{current_kernel_display}" }
                  }
              }
          }

          // Installed Kernels + Old Kernels
          div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",

              // Installed kernels table
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "flex items-center justify-between mb-4",
                      h2 { class: "font-semibold", "Installed Kernels" }
                      span { class: "text-xs px-2.5 py-1 rounded-full bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-400",
                          "{installed_count} total"
                      }
                  }
                  if installed_kernels.read().is_empty() {
                      div { class: "flex flex-col items-center justify-center py-8 text-center",
                          span { class: "material-symbols-rounded text-4xl text-zinc-300 dark:text-zinc-600 mb-2", "loading" }
                          p { class: "text-sm text-zinc-400", "Loading..." }
                      }
                  } else {
                      div { class: "space-y-2 max-h-64 overflow-y-auto",
                          for kernel in installed_kernels.read().iter().cloned() {
                              div { class: "flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors",
                                  span { class: "material-symbols-rounded text-emerald-500", "check_circle" }
                                  span { class: "flex-1 font-mono text-sm", "{kernel}" }
                              }
                          }
                      }
                  }
              }

              // Old/Removable kernels
              div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
                  div { class: "flex items-center justify-between mb-4",
                      h2 { class: "font-semibold", "Old Kernels" }
                      span { class: "text-xs px-2.5 py-1 rounded-full bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-400",
                          if old_kernels_count > 0 {
                              "{old_kernels_count} removable"
                          } else {
                              "None found"
                          }
                      }
                  }
                  if old_kernels.read().is_empty() {
                      div { class: "flex flex-col items-center justify-center py-8 text-center",
                          span { class: "material-symbols-rounded text-4xl text-zinc-300 dark:text-zinc-600 mb-2", "check_circle" }
                          p { class: "text-sm text-zinc-500 dark:text-zinc-400", "No old kernels found" }
                      }
                  } else {
                      div { class: "space-y-2 max-h-64 overflow-y-auto",
                          for kernel in old_kernels.read().iter().cloned() {
                              div { class: "flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors",
                                  input {
                                      r#type: "checkbox",
                                      class: "w-4 h-4 rounded border-zinc-300 dark:border-zinc-600 text-rose-500 focus:ring-rose-500",
                                      checked: selected_kernels.read().contains(&kernel),
                                      onchange: move |_| {
                                          let mut sel = selected_kernels.read().clone();
                                          let k = kernel.clone();
                                          if sel.contains(&k) {
                                              sel.remove(&k);
                                          } else {
                                              sel.insert(k);
                                          }
                                          selected_kernels.set(sel);
                                      }
                                  }
                                  span { class: "flex-1 font-mono text-sm", "{kernel}" }
                              }
                          }
                      }
                  }
              }
          }

          // Boot Space Info Card
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              div { class: "flex items-center gap-4 mb-4",
                  div { class: "w-10 h-10 rounded-lg bg-violet-100 dark:bg-violet-900/30 flex items-center justify-center",
                      span { class: "material-symbols-rounded text-violet-600 dark:text-violet-400", "storage" }
                  }
                  div {
                      h2 { class: "font-semibold", "Boot Partition Space" }
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Space used by kernels and initramfs" }
                  }
              }
              if boot_infos.is_empty() {
                  div { class: "flex items-center justify-center py-6",
                      span { class: "text-sm text-zinc-400", "Loading boot space info..." }
                  }
              } else {
                  div { class: "space-y-4",
                      for info in boot_infos.iter() {
                          div { class: "space-y-2",
                              div { class: "flex items-center justify-between text-xs text-zinc-500 mb-1",
                                  span { class: "font-mono", "{info.filesystem}" }
                                  span { "{info.mounted_on}" }
                              }
                              div { class: "flex items-center justify-between text-sm",
                                  span { class: "text-zinc-600 dark:text-zinc-400", "Used" }
                                  span { class: "font-medium", "{info.used_mb()} MB / {info.total_mb()} MB" }
                              }
                              div { class: "w-full h-2 rounded-full bg-zinc-100 dark:bg-zinc-800 overflow-hidden",
                                  div {
                                      class: "h-full bg-violet-500 rounded-full transition-all",
                                      style: format!("width:{}%", info.use_percent)
                                  }
                              }
                              div { class: "flex items-center justify-between text-sm",
                                  span { class: "text-zinc-600 dark:text-zinc-400", "Available" }
                                  span { class: "font-medium", "{info.available_mb()} MB" }
                              }
                          }
                      }
                  }
              }
          }

          // Initramfs Cleanup Section
          div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-5",
              div { class: "flex items-center gap-4 mb-4",
                  div { class: "w-10 h-10 rounded-lg bg-amber-100 dark:bg-amber-900/30 flex items-center justify-center",
                      span { class: "material-symbols-rounded text-amber-600 dark:text-amber-400", "inventory_2" }
                  }
                  div { class: "flex-1",
                      h2 { class: "font-semibold", "Old initramfs Files" }
                      p { class: "text-xs text-zinc-500 dark:text-zinc-400", "Unused initial ramdisk images from removed kernels" }
                  }
                  if initramfs_count > 0 {
                      span { class: "text-xs px-2.5 py-1 rounded-full bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-400",
                          "{initramfs_count} files"
                      }
                  }
              }
              if old_initramfs.read().is_empty() {
                  div { class: "flex flex-col items-center justify-center py-8 text-center",
                      span { class: "material-symbols-rounded text-4xl text-zinc-300 dark:text-zinc-600 mb-2", "check_circle" }
                      p { class: "text-sm text-zinc-500 dark:text-zinc-400", "No old initramfs files" }
                  }
              } else {
                  div { class: "space-y-2 max-h-48 overflow-y-auto",
                      for entry in old_initramfs.read().iter().cloned() {
                          div { class: "flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors",
                              input {
                                  r#type: "checkbox",
                                  class: "w-4 h-4 rounded border-zinc-300 dark:border-zinc-600 text-rose-500 focus:ring-rose-500",
                                  checked: selected_initramfs.read().contains(&entry),
                                  onchange: move |_| {
                                      let mut sel = selected_initramfs.read().clone();
                                      let e = entry.clone();
                                      if sel.contains(&e) {
                                          sel.remove(&e);
                                      } else {
                                          sel.insert(e);
                                      }
                                      selected_initramfs.set(sel);
                                  }
                              }
                              span { class: "material-symbols-rounded text-amber-500", "description" }
                              span { class: "flex-1 font-mono text-sm truncate", "{entry}" }
                          }
                      }
                  }
              }
          }

          // Action Buttons
          div { class: "flex flex-wrap items-center gap-3",

              // Remove Kernel button
              button {
                  class: format!(
                      "px-4 py-2.5 rounded-xl font-medium text-sm transition-colors flex items-center gap-2 {}",
                      if selected_kernels_count > 0 && !is_removing_val {
                          "bg-rose-500 hover:bg-rose-600 text-white"
                      } else {
                          "bg-zinc-100 dark:bg-zinc-800 text-zinc-400 cursor-not-allowed"
                      }
                  ),
                  disabled: selected_kernels_count == 0 || is_removing_val,
                  onclick: move |_| {
                      if let Some(first) = selected_kernels.read().iter().next().cloned() {
                          pending_kernel.set(Some(first));
                          confirm_action.set("remove_kernel".to_string());
                          pending_count.set(selected_kernels.read().len());
                          show_confirm.set(true);
                      }
                  },
                  span { class: "material-symbols-rounded text-lg", "delete" }
                  "Remove Kernel"
                  if selected_kernels_count > 0 {
                      span { class: "ml-1 text-xs opacity-75", "({selected_kernels_count})" }
                  }
              }

              // Refresh GRUB button
              button {
                  class: format!(
                      "px-4 py-2.5 rounded-xl font-medium text-sm transition-colors flex items-center gap-2 {}",
                      if is_refreshing_grub_val {
                          "bg-zinc-200 dark:bg-zinc-700 text-zinc-400 cursor-not-allowed"
                      } else {
                          "bg-zinc-900 dark:bg-white text-white dark:text-zinc-900 hover:opacity-90"
                      }
                  ),
                  disabled: is_refreshing_grub_val,
                  onclick: move |_| {
                      is_refreshing_grub.set(true);
                      let _ = bridge::invoke_app_command("system_refresh_grub", &serde_json::json!({}));
                      is_refreshing_grub.set(false);
                      status_message.set(Some("GRUB configuration refreshed".to_string()));
                  },
                  span { class: "material-symbols-rounded text-lg", "sync" }
                  if is_refreshing_grub_val { "Refreshing..." } else { "Refresh GRUB" }
              }

              // Clear initramfs button
              button {
                  class: format!(
                      "px-4 py-2.5 rounded-xl font-medium text-sm transition-colors flex items-center gap-2 {}",
                      if selected_initramfs_count > 0 {
                          "bg-zinc-900 dark:bg-white text-white dark:text-zinc-900 hover:opacity-90"
                      } else {
                          "bg-zinc-100 dark:bg-zinc-800 text-zinc-400 cursor-not-allowed"
                      }
                  ),
                  disabled: selected_initramfs_count == 0,
                  onclick: move |_| {
                      confirm_action.set("clean_initramfs".to_string());
                      pending_count.set(selected_initramfs.read().len());
                      show_confirm.set(true);
                  },
                  span { class: "material-symbols-rounded text-lg", "cleaning_services" }
                  "Clean initramfs"
                  if selected_initramfs_count > 0 {
                      span { class: "ml-1 text-xs opacity-75", "({selected_initramfs_count})" }
                  }
              }

              // Status message
              if let Some(msg) = status_msg {
                  div { class: "text-sm text-emerald-600 dark:text-emerald-400 flex items-center gap-2 animate-pulse",
                      span { class: "material-symbols-rounded text-lg", "check_circle" }
                      "{msg}"
                  }
              }
          }

          // Confirmation Dialog
          if show_confirm_val {
              div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
                  div { class: "bg-white dark:bg-zinc-900 rounded-2xl border border-zinc-200 dark:border-zinc-800 p-6 max-w-sm w-full mx-4 shadow-2xl",
                      div { class: "flex items-center gap-3 mb-4",
                          div { class: "w-10 h-10 rounded-full bg-amber-100 dark:bg-amber-900/30 flex items-center justify-center",
                              span { class: "material-symbols-rounded text-amber-600 dark:text-amber-400", "warning" }
                          }
                          div {
                              h3 { class: "font-semibold", "Confirm Action" }
                              p { class: "text-sm text-zinc-500 dark:text-zinc-400", "This action cannot be undone" }
                          }
                      }
                      div { class: "flex items-center gap-3 p-3 rounded-lg bg-rose-50 dark:bg-rose-950/20 border border-rose-200 dark:border-rose-800 mb-4",
                          span { class: "material-symbols-rounded text-rose-500", "info" }
                          if confirm_act == "remove_kernel" {
                              p { class: "text-sm text-rose-700 dark:text-rose-300",
                                  "Remove {pending_cnt} kernel(s)?"
                              }
                          } else {
                              p { class: "text-sm text-rose-700 dark:text-rose-300",
                                  "Remove {pending_cnt} old initramfs file(s)?"
                              }
                          }
                      }
                      div { class: "flex gap-3",
                          button {
                              class: "flex-1 px-4 py-2 rounded-xl border border-zinc-200 dark:border-zinc-700 font-medium text-sm hover:bg-zinc-50 dark:hover:bg-zinc-800 transition-colors",
                              onclick: move |_| {
                                  show_confirm.set(false);
                                  confirm_action.set(String::new());
                                  pending_kernel.set(None);
                              },
                              "Cancel"
                          }
                          button {
                              class: "flex-1 px-4 py-2 rounded-xl bg-rose-500 hover:bg-rose-600 text-white font-medium text-sm transition-colors",
                              onclick: move |_| {
                                  show_confirm.set(false);
                                  let action = confirm_action.read().clone();
                                  if action == "remove_kernel" {
                                      let kernel_opt = pending_kernel.read().clone();
                                      if let Some(kernel) = kernel_opt {
                                          is_removing.set(true);
                                          let _ = bridge::invoke_app_command("system_remove_kernel", &serde_json::json!({ "kernel": &kernel }));
                                          // Refresh old kernels list
                                          if let Ok(result) = bridge::invoke_app_command("system_get_old_kernels", &serde_json::json!({})) {
                                              if let Some(data) = result.get("data") {
                                                  if let Ok(kernels) = serde_json::from_value::<Vec<String>>(data.clone()) {
                                                      old_kernels.set(kernels);
                                                  }
                                              }
                                          }
                                          is_removing.set(false);
                                          status_message.set(Some(format!("Kernel {} removal initiated", kernel)));
                                          pending_kernel.set(None);
                                          selected_kernels.set(std::collections::HashSet::new());
                                      }
                                  } else if action == "clean_initramfs" {
                                      let selected = selected_initramfs.read().clone();
                                      for file in selected.iter() {
                                          let _ = bridge::invoke_app_command("system_remove_initramfs", &serde_json::json!({ "initramfs": file }));
                                      }
                                      // Refresh initramfs list
                                      if let Ok(result) = bridge::invoke_app_command("system_get_old_initramfs", &serde_json::json!({})) {
                                          if let Some(data) = result.get("data") {
                                              if let Ok(files) = serde_json::from_value::<Vec<String>>(data.clone()) {
                                                  old_initramfs.set(files);
                                              }
                                          }
                                      }
                                      selected_initramfs.set(std::collections::HashSet::new());
                                      status_message.set(Some("Old initramfs files cleaned".to_string()));
                                  }
                                  confirm_action.set(String::new());
                              },
                              "Remove"
                          }
                      }
                  }
              }
          }
      }
  }
}
