#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum JunkCategory {
  Browser,
  Thumbnails,
  Applications,
  System,
  Logs,
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct JunkItem {
  pub path: String,
  pub size: u64,
  pub category: JunkCategory,
  pub description: String,
  pub file_count: u32,
}
