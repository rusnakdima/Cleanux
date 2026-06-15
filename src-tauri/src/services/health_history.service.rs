use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use log;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HealthSnapshot {
  pub id: Option<i64>,
  pub timestamp: String,
  pub health_score: f64,
  pub cache_size: u64,
  pub trash_size: u64,
  pub log_size: u64,
  pub large_files_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthTrend {
  pub trend: String,
  pub change_percent: f64,
  pub days_analyzed: u32,
}

pub struct HealthHistoryService {
  db_path: PathBuf,
}

impl HealthHistoryService {
  pub fn new() -> Self {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    let cleanux_dir = config_dir.join("cleanux");
    std::fs::create_dir_all(&cleanux_dir).ok();
    let db_path = cleanux_dir.join("health_history.db");
    log::info!(
      "HealthHistoryService initialized with db_path: {:?}",
      db_path
    );
    Self { db_path }
  }

  pub fn init_database(&self) -> SqlResult<()> {
    log::info!("Initializing health_history database");
    let conn = Connection::open(&self.db_path)?;
    conn.execute(
      "CREATE TABLE IF NOT EXISTS health_snapshots (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                health_score REAL NOT NULL,
                cache_size INTEGER NOT NULL,
                trash_size INTEGER NOT NULL,
                log_size INTEGER NOT NULL,
                large_files_count INTEGER NOT NULL
            )",
      [],
    )?;
    conn.execute(
      "CREATE INDEX IF NOT EXISTS idx_timestamp ON health_snapshots(timestamp)",
      [],
    )?;
    log::info!("Health_history database initialized successfully");
    Ok(())
  }

  pub fn save_health_snapshot(&self, snapshot: HealthSnapshot) -> SqlResult<i64> {
    log::info!(
      "Saving health snapshot with score: {}",
      snapshot.health_score
    );
    let conn = Connection::open(&self.db_path)?;
    conn.execute(
            "INSERT INTO health_snapshots (timestamp, health_score, cache_size, trash_size, log_size, large_files_count)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                snapshot.timestamp,
                snapshot.health_score,
                snapshot.cache_size as i64,
                snapshot.trash_size as i64,
                snapshot.log_size as i64,
                snapshot.large_files_count
            ],
        )?;
    let id = conn.last_insert_rowid();
    log::info!("Health snapshot saved with id: {}", id);
    Ok(id)
  }

  pub fn get_health_history(&self, days: u32) -> SqlResult<Vec<HealthSnapshot>> {
    log::info!("Fetching health history for last {} days", days);
    let conn = Connection::open(&self.db_path)?;
    let cutoff = chrono::Utc::now() - chrono::Duration::days(days as i64);
    let cutoff_str = cutoff.format("%Y-%m-%d %H:%M:%S").to_string();

    let mut stmt = conn.prepare(
      "SELECT id, timestamp, health_score, cache_size, trash_size, log_size, large_files_count
             FROM health_snapshots
             WHERE timestamp >= ?1
             ORDER BY timestamp ASC",
    )?;

    let snapshots = stmt.query_map([cutoff_str], |row| {
      Ok(HealthSnapshot {
        id: Some(row.get(0)?),
        timestamp: row.get(1)?,
        health_score: row.get(2)?,
        cache_size: row.get::<_, i64>(3)? as u64,
        trash_size: row.get::<_, i64>(4)? as u64,
        log_size: row.get::<_, i64>(5)? as u64,
        large_files_count: row.get(6)?,
      })
    })?;

    let mut result = Vec::new();
    for snapshot in snapshots {
      result.push(snapshot?);
    }
    log::info!("Retrieved {} health snapshots", result.len());
    Ok(result)
  }

  pub fn get_health_trends(&self, days: u32) -> SqlResult<HealthTrend> {
    log::info!("Calculating health trends for last {} days", days);
    let history = self.get_health_history(days)?;

    if history.len() < 2 {
      log::warn!(
        "Insufficient data for trend calculation, found {} snapshots",
        history.len()
      );
      return Ok(HealthTrend {
        trend: "insufficient_data".to_string(),
        change_percent: 0.0,
        days_analyzed: days,
      });
    }

    let first = &history[0];
    let last = &history[history.len() - 1];

    let change_percent = if first.health_score > 0.0 {
      ((last.health_score - first.health_score) / first.health_score) * 100.0
    } else {
      0.0
    };

    let trend = if change_percent > 5.0 {
      "improving"
    } else if change_percent < -5.0 {
      "declining"
    } else {
      "stable"
    };

    log::info!(
      "Health trend calculated: {} ({:.2}%)",
      trend,
      change_percent
    );
    Ok(HealthTrend {
      trend: trend.to_string(),
      change_percent,
      days_analyzed: days,
    })
  }
}

impl Default for HealthHistoryService {
  fn default() -> Self {
    Self::new()
  }
}
