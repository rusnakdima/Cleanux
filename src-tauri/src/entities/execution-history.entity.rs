use nosql_orm::Model;
use nosql_orm::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Model, Validate)]
#[table_name("execution_history")]
#[index("started_at", 1)]
pub struct ExecutionHistoryEntity {
  pub id: Option<String>,
  pub name: String,
  pub status: String,
  pub started_at: String,
  pub completed_at: Option<String>,
  pub steps_executed: u32,
  pub total_steps: u32,
}

impl From<crate::services::automation_service::ExecutionHistoryEntry> for ExecutionHistoryEntity {
  fn from(entry: crate::services::automation_service::ExecutionHistoryEntry) -> Self {
    Self {
      id: Some(entry.id),
      name: entry.name,
      status: entry.status,
      started_at: entry.started_at,
      completed_at: entry.completed_at,
      steps_executed: entry.steps_executed,
      total_steps: entry.total_steps,
    }
  }
}
