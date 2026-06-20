use chrono::{DateTime, Utc};
use nosql_orm::Model;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Model)]
#[table_name("automation_recipes")]
#[index("name", 1)]
pub struct AutomationRecipeEntity {
  pub id: Option<String>,
  pub name: String,
  pub steps: Vec<ActionStep>,
  pub enabled: bool,
  pub trigger: RecipeTrigger,
  #[timestamp]
  pub created_at: DateTime<Utc>,
  #[timestamp]
  pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionStep {
  CleanCategory { category: String },
  RunProfile { profile_name: String },
  ExecuteCommand { command: String },
  Wait { seconds: u32 },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecipeTrigger {
  Manual,
  Scheduled,
  Event,
}
impl From<crate::services::automation_service::AutomationRecipe> for AutomationRecipeEntity {
  fn from(recipe: crate::services::automation_service::AutomationRecipe) -> Self {
    Self {
      id: Some(recipe.id),
      name: recipe.name,
      steps: recipe.steps.into_iter().map(|s| s.into()).collect(),
      enabled: recipe.enabled,
      trigger: match recipe.trigger {
        crate::services::automation_service::RecipeTrigger::Manual => RecipeTrigger::Manual,
        crate::services::automation_service::RecipeTrigger::Scheduled => RecipeTrigger::Scheduled,
        crate::services::automation_service::RecipeTrigger::Event => RecipeTrigger::Event,
      },
      created_at: chrono::Utc::now(),
      updated_at: chrono::Utc::now(),
    }
  }
}
impl From<crate::services::automation_service::ActionStep> for ActionStep {
  fn from(step: crate::services::automation_service::ActionStep) -> Self {
    match step {
      crate::services::automation_service::ActionStep::CleanCategory { category } => {
        ActionStep::CleanCategory { category }
      }
      crate::services::automation_service::ActionStep::RunProfile { profile_name } => {
        ActionStep::RunProfile { profile_name }
      }
      crate::services::automation_service::ActionStep::ExecuteCommand { command } => {
        ActionStep::ExecuteCommand { command }
      }
      crate::services::automation_service::ActionStep::Wait { seconds } => {
        ActionStep::Wait { seconds }
      }
    }
  }
}
