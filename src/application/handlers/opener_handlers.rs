//! Filesystem Opener Platform Plugin handlers for Cleanux
//!
//! Provides cross-platform file/URL opener functionality.

use crate::error::AppError;
use crate::response::Response;

use std::path::Path;

/// Open a file with the OS default application
pub async fn opener_open(path: &str) -> Result<Response<bool>, AppError> {
  let path = Path::new(path);
  if !path.exists() {
    return Err(AppError::InvalidPath(format!(
      "path does not exist: {}",
      path.display()
    )));
  }

  #[cfg(target_os = "linux")]
  {
    open::that(path).map_err(|e| AppError::Internal(format!("failed to open: {}", e)))?;
  }
  #[cfg(target_os = "macos")]
  {
    open::that(path).map_err(|e| AppError::Internal(format!("failed to open: {}", e)))?;
  }
  #[cfg(target_os = "windows")]
  {
    open::that(path).map_err(|e| AppError::Internal(format!("failed to open: {}", e)))?;
  }
  #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
  {
    return Err(AppError::ValidationError(
      "unsupported platform".to_string(),
    ));
  }

  tracing::info!("Opened: {}", path.display());
  Ok(Response::success(
    true,
    Some(&format!("Opened: {}", path.display())),
  ))
}

/// Open a folder in the file manager and optionally select a file
pub async fn opener_open_folder(path: &str) -> Result<Response<bool>, AppError> {
  let path = Path::new(path);
  if !path.exists() {
    return Err(AppError::InvalidPath(format!(
      "path does not exist: {}",
      path.display()
    )));
  }

  #[cfg(target_os = "linux")]
  {
    // Use xdg-open on parent dir so file manager shows the folder
    let folder = if path.is_file() {
      path.parent().unwrap_or(path)
    } else {
      path
    };
    open::that(folder).map_err(|e| AppError::Internal(format!("failed to open folder: {}", e)))?;
  }
  #[cfg(target_os = "macos")]
  {
    open::that(path).map_err(|e| AppError::Internal(format!("failed to open: {}", e)))?;
  }
  #[cfg(target_os = "windows")]
  {
    open::that(path).map_err(|e| AppError::Internal(format!("failed to open: {}", e)))?;
  }
  #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
  {
    return Err(AppError::ValidationError(
      "unsupported platform".to_string(),
    ));
  }

  tracing::info!("Opened folder: {}", path.display());
  Ok(Response::success(
    true,
    Some(&format!("Opened folder: {}", path.display())),
  ))
}

/// Open a URL in the default browser
pub async fn opener_open_url(url: &str) -> Result<Response<bool>, AppError> {
  if !url.starts_with("http://") && !url.starts_with("https://") {
    return Err(AppError::ValidationError(format!("invalid URL: {}", url)));
  }

  open::that(url).map_err(|e| AppError::Internal(format!("failed to open URL: {}", e)))?;
  tracing::info!("Opened URL: {}", url);
  Ok(Response::success(
    true,
    Some(&format!("Opened URL: {}", url)),
  ))
}

/// Get MIME type for a file path
pub async fn opener_get_mime_type(path: &str) -> Result<Response<String>, AppError> {
  let path = Path::new(path);
  if !path.exists() {
    return Err(AppError::InvalidPath(format!(
      "path does not exist: {}",
      path.display()
    )));
  }

  let mime = mime_guess::from_path(path)
    .first()
    .map(|m| m.to_string())
    .unwrap_or_else(|| "application/octet-stream".to_string());

  Ok(Response::success(mime, Some("MIME type retrieved")))
}
