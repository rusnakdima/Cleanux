//! ReportService — generates cleaning reports from history and exports them.

use crate::domain::entities::cleaning_report::{CleaningReport, ReportCategories};
use crate::error::AppError;
use crate::global_state::get_history_entries;

pub type Result<T, E = AppError> = std::result::Result<T, E>;

/// ReportService for generating and exporting cleaning reports.
pub struct ReportService;

impl ReportService {
  /// Generate a cleaning report from execution history.
  pub fn generate_report() -> Result<CleaningReport> {
    let history = get_history_entries();

    let items_cleaned: i64 = history.iter().map(|h| h.items_affected).sum();
    let space_reclaimed: u64 = history.iter().map(|h| h.space_reclaimed).sum();
    let duration: f64 = history
      .iter()
      .filter_map(|h| {
        h.completed_at
          .map(|completed| (completed - h.started_at).num_seconds() as f64)
      })
      .sum();

    let categories = ReportCategories {
      cache: 0,
      trash: 0,
      logs: 0,
      large_files: 0,
      duplicates: 0,
    };

    let date = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    Ok(CleaningReport {
      id: Some(uuid::Uuid::new_v4().to_string()),
      date,
      items_cleaned,
      space_reclaimed,
      duration,
      categories,
    })
  }

  /// Export the report in the specified format ("pdf", "csv", or "json").
  pub fn export_report(&self, format: &str) -> Result<String> {
    let report = Self::generate_report()?;

    match format {
      "pdf" => {
        // Generate a simple text-based report that could be converted to PDF
        let text = format!(
                    "Cleaning Report\n==============\nDate: {}\nItems Cleaned: {}\nSpace Reclaimed: {} bytes\nDuration: {:.2}s\n\nCategories:\n  Cache: {}\n  Trash: {}\n  Logs: {}\n  Large Files: {}\n  Duplicates: {}",
                    report.date,
                    report.items_cleaned,
                    report.space_reclaimed,
                    report.duration,
                    report.categories.cache,
                    report.categories.trash,
                    report.categories.logs,
                    report.categories.large_files,
                    report.categories.duplicates
                );
        Ok(text)
      }
      "csv" => {
        let csv = format!(
                    "date,items_cleaned,space_reclaimed,duration,cache,trash,logs,large_files,duplicates\n{},{},{},{},{},{},{},{},{}",
                    report.date,
                    report.items_cleaned,
                    report.space_reclaimed,
                    report.duration,
                    report.categories.cache,
                    report.categories.trash,
                    report.categories.logs,
                    report.categories.large_files,
                    report.categories.duplicates
                );
        Ok(csv)
      }
      _ => {
        serde_json::to_string_pretty(&report).map_err(|e| AppError::Internal(e.to_string().into()))
      }
    }
  }
}
