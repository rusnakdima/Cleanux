//! Frontend Shell Pages handlers for Cleanux
//!
//! Lists available UI pages/screens rendered from the app's UI layer.

use crate::response::Response;
use serde::{Deserialize, Serialize};

/// Page information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
  pub id: String,
  pub name: String,
  pub route: String,
  pub description: String,
}

/// Shell pages list output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellPagesOutput {
  pub pages: Vec<PageInfo>,
  pub active_page: String,
}

/// List all available shell pages
pub fn shell_pages_list() -> Response<ShellPagesOutput> {
  let pages = vec![
    PageInfo {
      id: "dashboard".to_string(),
      name: "Dashboard".to_string(),
      route: "/".to_string(),
      description: "System overview and health summary".to_string(),
    },
    PageInfo {
      id: "clean".to_string(),
      name: "Clean".to_string(),
      route: "/clean".to_string(),
      description: "Junk scanning and cleaning operations".to_string(),
    },
    PageInfo {
      id: "files".to_string(),
      name: "Files".to_string(),
      route: "/files".to_string(),
      description: "File analysis, duplicates, and large files".to_string(),
    },
    PageInfo {
      id: "power".to_string(),
      name: "Power".to_string(),
      route: "/power".to_string(),
      description: "Power management and thermal controls".to_string(),
    },
    PageInfo {
      id: "settings".to_string(),
      name: "Settings".to_string(),
      route: "/settings".to_string(),
      description: "Application settings and preferences".to_string(),
    },
  ];

  Response::success(
    ShellPagesOutput {
      pages,
      active_page: "dashboard".to_string(),
    },
    Some("Shell pages retrieved"),
  )
}

/// Get active page
pub fn shell_get_active_page() -> Response<String> {
  Response::success("dashboard".to_string(), Some("Active page retrieved"))
}
