/* imports */
mod controllers;
mod models;
mod routes;
mod services;

/* tauri */
use tauri::command;

/* routes */
use routes::{
  cleaner_route::{
    clearAllLargeFiles, clearAllLogs, clearCache, clearSelectedCacheFiles, clearSelectedLargeFiles,
    clearSelectedLogFiles, clearSelectedTrashFiles, clearTrash, getCacheFiles, getLargeFiles,
    getSystemLogs, getTrashFiles, previewFile,
  },
  dashboard_route::{
    getCacheSummary, getLargeFilesSummary, getLogSummary, getSystemServices, getTrashSummary,
  },
  system_route::{
    enableSelectedServices, enableService, getAllServices, openFile, startService,
    stopSelectedServices, stopService,
  },
};

#[command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[allow(non_snake_case)]
pub fn run() {
  #[allow(unused_mut)]
  let mut builder = tauri::Builder::default();

  #[cfg(all(feature = "mcp-bridge", debug_assertions))]
  {
    builder = builder.plugin(tauri_plugin_mcp_bridge::init());
  }

  builder
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      greet,
      getSystemServices,
      getCacheFiles,
      getTrashFiles,
      getSystemLogs,
      getLargeFiles,
      getCacheSummary,
      getTrashSummary,
      getLogSummary,
      getLargeFilesSummary,
      clearSelectedCacheFiles,
      clearSelectedTrashFiles,
      clearSelectedLogFiles,
      clearSelectedLargeFiles,
      clearAllLogs,
      clearAllLargeFiles,
      stopSelectedServices,
      clearTrash,
      clearCache,
      stopService,
      previewFile,
      getAllServices,
      enableService,
      startService,
      enableSelectedServices,
      openFile
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
