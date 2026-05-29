use crate::services::scan_state::get_scan_state;

#[tauri::command]
pub fn cancel_scan() -> Result<String, String> {
  let scan_state = get_scan_state();
  if scan_state.is_cancelled() {
    Ok("No active scan to cancel".to_string())
  } else {
    scan_state.cancel();
    Ok("Scan cancellation requested".to_string())
  }
}

#[tauri::command]
pub fn is_scan_cancelled() -> bool {
  get_scan_state().is_cancelled()
}