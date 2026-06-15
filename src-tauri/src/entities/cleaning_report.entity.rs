use nosql_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleaningReportEntity {
  pub id: Option<String>,
  pub date: String,
  pub items_cleaned: i64,
  pub space_reclaimed: u64,
  pub duration: f64,
  pub categories: ReportCategoriesEntity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportCategoriesEntity {
  pub cache: i64,
  pub trash: i64,
  pub logs: i64,
  pub large_files: i64,
  pub duplicates: i64,
}

impl Entity for CleaningReportEntity {
  fn meta() -> EntityMeta {
    EntityMeta::new("cleaning_reports")
  }

  fn get_id(&self) -> Option<String> {
    self.id.clone()
  }

  fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }
}

impl CleaningReportEntity {
  pub fn new(
    date: String,
    items_cleaned: i64,
    space_reclaimed: u64,
    duration: f64,
    categories: ReportCategoriesEntity,
  ) -> Self {
    Self {
      id: None,
      date,
      items_cleaned,
      space_reclaimed,
      duration,
      categories,
    }
  }
}
