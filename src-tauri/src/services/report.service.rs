use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CleaningReport {
  pub id: Option<i64>,
  pub date: String,
  pub items_cleaned: i64,
  pub space_reclaimed: u64,
  pub duration: f64,
  pub categories: ReportCategories,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportCategories {
  pub cache: i64,
  pub trash: i64,
  pub logs: i64,
  pub large_files: i64,
  pub duplicates: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotComparison {
  pub before_id: i64,
  pub after_id: i64,
  pub space_reclaimed: u64,
  pub items_cleaned: i64,
  pub health_improvement: f64,
  pub details: ComparisonDetails,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ComparisonDetails {
  pub cache_change: i64,
  pub trash_change: i64,
  pub log_change: i64,
  pub large_file_change: i64,
}
pub struct ReportService {
  db_path: PathBuf,
  reports_dir: PathBuf,
}
impl ReportService {
  pub fn new() -> Self {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    let cleanux_dir = config_dir.join("cleanux");
    std::fs::create_dir_all(&cleanux_dir).ok();
    let reports_dir = cleanux_dir.join("reports");
    std::fs::create_dir_all(&reports_dir).ok();
    let db_path = cleanux_dir.join("reports.db");
    Self {
      db_path,
      reports_dir,
    }
  }
  pub fn init_database(&self) -> SqlResult<()> {
    let conn = Connection::open(&self.db_path)?;
    conn.execute(
      "CREATE TABLE IF NOT EXISTS cleaning_reports (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL,
                items_cleaned INTEGER NOT NULL,
                space_reclaimed INTEGER NOT NULL,
                duration REAL NOT NULL,
                cache_items INTEGER NOT NULL,
                trash_items INTEGER NOT NULL,
                log_items INTEGER NOT NULL,
                large_file_items INTEGER NOT NULL,
                duplicate_items INTEGER NOT NULL
            )",
      [],
    )?;
    conn.execute(
      "CREATE INDEX IF NOT EXISTS idx_report_date ON cleaning_reports(date)",
      [],
    )?;
    Ok(())
  }
  pub fn generate_cleaning_report(
    &self,
    items_cleaned: i64,
    space_reclaimed: u64,
    duration: f64,
    categories: ReportCategories,
  ) -> SqlResult<i64> {
    let conn = Connection::open(&self.db_path)?;
    let date = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
            "INSERT INTO cleaning_reports (date, items_cleaned, space_reclaimed, duration, cache_items, trash_items, log_items, large_file_items, duplicate_items)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                date,
                items_cleaned,
                space_reclaimed as i64,
                duration,
                categories.cache,
                categories.trash,
                categories.logs,
                categories.large_files,
                categories.duplicates
            ],
        )?;
    let report_id = conn.last_insert_rowid();
    let report = CleaningReport {
      id: Some(report_id),
      date,
      items_cleaned,
      space_reclaimed,
      duration,
      categories,
    };
    let filename = format!("report_{}.json", report_id);
    let filepath = self.reports_dir.join(filename);
    let json = serde_json::to_string_pretty(&report).map_err(|e| {
      rusqlite::Error::InvalidParameterName(format!("JSON serialization error: {}", e))
    })?;
    std::fs::write(filepath, json)
      .map_err(|e| rusqlite::Error::InvalidParameterName(format!("File write error: {}", e)))?;
    Ok(report_id)
  }
  pub fn get_cleaning_history(&self, limit: Option<i64>) -> SqlResult<Vec<CleaningReport>> {
    let conn = Connection::open(&self.db_path)?;
    let mut result = Vec::new();
    match limit {
      Some(l) => {
        let mut stmt = conn.prepare(
                    "SELECT id, date, items_cleaned, space_reclaimed, duration, cache_items, trash_items, log_items, large_file_items, duplicate_items
                     FROM cleaning_reports ORDER BY date DESC LIMIT ?1"
                )?;
        let reports = stmt.query_map([l], |row| self.row_to_report(row))?;
        for report in reports {
          result.push(report?);
        }
      }
      None => {
        let mut stmt = conn.prepare(
                    "SELECT id, date, items_cleaned, space_reclaimed, duration, cache_items, trash_items, log_items, large_file_items, duplicate_items
                     FROM cleaning_reports ORDER BY date DESC"
                )?;
        let reports = stmt.query_map([], |row| self.row_to_report(row))?;
        for report in reports {
          result.push(report?);
        }
      }
    }
    Ok(result)
  }
  pub fn get_report_by_id(&self, report_id: i64) -> SqlResult<Option<CleaningReport>> {
    let conn = Connection::open(&self.db_path)?;
    let mut stmt = conn.prepare(
            "SELECT id, date, items_cleaned, space_reclaimed, duration, cache_items, trash_items, log_items, large_file_items, duplicate_items
             FROM cleaning_reports WHERE id = ?1"
        )?;
    let mut rows = stmt.query([report_id])?;
    if let Some(row) = rows.next()? {
      Ok(Some(self.row_to_report(row)?))
    } else {
      Ok(None)
    }
  }
  fn row_to_report(&self, row: &rusqlite::Row) -> SqlResult<CleaningReport> {
    Ok(CleaningReport {
      id: Some(row.get(0)?),
      date: row.get(1)?,
      items_cleaned: row.get(2)?,
      space_reclaimed: row.get::<_, i64>(3)? as u64,
      duration: row.get(4)?,
      categories: ReportCategories {
        cache: row.get(5)?,
        trash: row.get(6)?,
        logs: row.get(7)?,
        large_files: row.get(8)?,
        duplicates: row.get(9)?,
      },
    })
  }
  pub fn export_to_html(&self, report_id: i64) -> SqlResult<String> {
    let report = self.get_report_by_id(report_id)?;
    match report {
      Some(r) => Ok(self.generate_html(&r)),
      None => Err(rusqlite::Error::InvalidParameterName(
        "Report not found".to_string(),
      )),
    }
  }
  fn generate_html(&self, report: &CleaningReport) -> String {
    let space_reclaimed_gb = report.space_reclaimed as f64 / (1024.0 * 1024.0 * 1024.0);
    let formatted_space = if space_reclaimed_gb >= 1.0 {
      format!("{:.2} GB", space_reclaimed_gb)
    } else {
      let space_reclaimed_mb = report.space_reclaimed as f64 / (1024.0 * 1024.0);
      format!("{:.2} MB", space_reclaimed_mb)
    };
    format!(
      r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Cleaning Report - {date}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%); min-height: 100vh; padding: 40px; color: #e0e0e0; }}
        .container {{ max-width: 900px; margin: 0 auto; background: rgba(255,255,255,0.05); border-radius: 20px; padding: 40px; backdrop-filter: blur(10px); border: 1px solid rgba(255,255,255,0.1); }}
        h1 {{ color: #00d4ff; margin-bottom: 10px; font-size: 2.5em; }}
        .subtitle {{ color: #888; margin-bottom: 40px; }}
        .stats {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin-bottom: 40px; }}
        .stat-card {{ background: linear-gradient(135deg, rgba(0,212,255,0.1) 0%, rgba(0,150,200,0.05) 100%); border-radius: 15px; padding: 25px; text-align: center; border: 1px solid rgba(0,212,255,0.2); }}
        .stat-value {{ font-size: 2.5em; font-weight: bold; color: #00d4ff; }}
        .stat-label {{ color: #888; margin-top: 8px; font-size: 0.9em; text-transform: uppercase; letter-spacing: 1px; }}
        .categories {{ background: rgba(255,255,255,0.03); border-radius: 15px; padding: 30px; margin-bottom: 30px; }}
        .categories h2 {{ color: #00d4ff; margin-bottom: 20px; }}
        .category-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 15px; }}
        .category {{ background: rgba(0,212,255,0.08); padding: 15px; border-radius: 10px; text-align: center; }}
        .category-name {{ color: #888; font-size: 0.85em; margin-bottom: 5px; }}
        .category-value {{ font-size: 1.5em; color: #fff; }}
        .footer {{ text-align: center; color: #666; margin-top: 40px; padding-top: 20px; border-top: 1px solid rgba(255,255,255,0.1); }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Cleaning Report</h1>
        <p class="subtitle">Generated on {date}</p>
        <div class="stats">
            <div class="stat-card">
                <div class="stat-value">{items_cleaned}</div>
                <div class="stat-label">Items Cleaned</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{space}</div>
                <div class="stat-label">Space Reclaimed</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{duration:.1}s</div>
                <div class="stat-label">Duration</div>
            </div>
        </div>
        <div class="categories">
            <h2>Categories Breakdown</h2>
            <div class="category-grid">
                <div class="category">
                    <div class="category-name">Cache</div>
                    <div class="category-value">{cache}</div>
                </div>
                <div class="category">
                    <div class="category-name">Trash</div>
                    <div class="category-value">{trash}</div>
                </div>
                <div class="category">
                    <div class="category-name">Logs</div>
                    <div class="category-value">{logs}</div>
                </div>
                <div class="category">
                    <div class="category-name">Large Files</div>
                    <div class="category-value">{large_files}</div>
                </div>
                <div class="category">
                    <div class="category-name">Duplicates</div>
                    <div class="category-value">{duplicates}</div>
                </div>
            </div>
        </div>
        <div class="footer">
            <p>Generated by Cleanux</p>
        </div>
    </div>
</body>
</html>"#,
      date = report.date,
      items_cleaned = report.items_cleaned,
      space = formatted_space,
      duration = report.duration,
      cache = report.categories.cache,
      trash = report.categories.trash,
      logs = report.categories.logs,
      large_files = report.categories.large_files,
      duplicates = report.categories.duplicates
    )
  }
  pub fn compare_snapshots(&self, before_id: i64, after_id: i64) -> SqlResult<SnapshotComparison> {
    let before = self.get_report_by_id(before_id)?;
    let after = self.get_report_by_id(after_id)?;
    match (before, after) {
      (Some(b), Some(a)) => {
        let space_reclaimed = a.space_reclaimed.saturating_sub(b.space_reclaimed);
        let items_cleaned = a.items_cleaned.saturating_sub(b.items_cleaned);
        let health_improvement = 0.0;
        Ok(SnapshotComparison {
          before_id,
          after_id,
          space_reclaimed,
          items_cleaned,
          health_improvement,
          details: ComparisonDetails {
            cache_change: a.categories.cache - b.categories.cache,
            trash_change: a.categories.trash - b.categories.trash,
            log_change: a.categories.logs - b.categories.logs,
            large_file_change: a.categories.large_files - b.categories.large_files,
          },
        })
      }
      _ => Err(rusqlite::Error::InvalidParameterName(
        "One or both reports not found".to_string(),
      )),
    }
  }
}
impl Default for ReportService {
  fn default() -> Self {
    Self::new()
  }
}
