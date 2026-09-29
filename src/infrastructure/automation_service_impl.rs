//! AutomationServiceImpl — implementation of AutomationServiceTrait
//!
//! Delegates recipe management to RoutineService and executes recipe actions
//! by dispatching to cleaner_handlers.

use crate::application::automation_service::AutomationServiceTrait;
use crate::application::handlers::cleaner_handlers;
use crate::domain::entities::automation_recipe::{Action, AutomationRecipe};
use crate::domain::entities::execution_history::{ExecutionHistory, ExecutionStatus};
use crate::global_state::{add_history_entry, routine_service};
use crate::infrastructure::memory_service::MemoryService;
use chrono::Utc;
use dioxus_shared::AppError;
use dioxus_shared::Result;

pub struct AutomationServiceImpl;

impl AutomationServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AutomationServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl AutomationServiceTrait for AutomationServiceImpl {
    fn get_recipes(&self) -> Result<Vec<AutomationRecipe>> {
        routine_service().get_routines().map_err(AppError::from)
    }

    fn get_recipe(&self, id: &str) -> Result<Option<AutomationRecipe>> {
        routine_service().get_routine_by_id(id).map_err(AppError::from)
    }

    fn create_recipe(&mut self, recipe: AutomationRecipe) -> Result<AutomationRecipe> {
        routine_service().save_routine(recipe).map_err(AppError::from)
    }

    fn update_recipe(
        &mut self,
        id: &str,
        recipe: AutomationRecipe,
    ) -> Result<AutomationRecipe> {
        let mut updated = recipe;
        updated.id = Some(id.to_string());
        routine_service().save_routine(updated).map_err(AppError::from)
    }

    fn delete_recipe(&mut self, id: &str) -> Result<()> {
        routine_service().delete_routine(id).map_err(AppError::from)
    }

    fn execute_recipe(&mut self, id: &str) -> Result<ExecutionHistory> {
        let recipe = routine_service()
            .get_routine_by_id(id)
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound(format!("recipe '{}' not found", id).into()))?;

        let started_at = Utc::now();
        let mut items_affected: i64 = 0;
        let mut space_reclaimed: u64 = 0;
        let mut error_msg: Option<String> = None;

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| AppError::Internal(format!("failed to build runtime: {}", e)))?;

        for action in &recipe.actions {
            let result = execute_action(&rt, action);
            match result {
                Ok((affected, reclaimed)) => {
                    items_affected += affected;
                    space_reclaimed += reclaimed;
                }
                Err(e) => {
                    error_msg = Some(e.to_string());
                    break;
                }
            }
        }

        let status = if error_msg.is_some() {
            ExecutionStatus::Failed
        } else {
            ExecutionStatus::Completed
        };

        let history_entry = ExecutionHistory {
            id: Some(uuid::Uuid::new_v4().to_string()),
            recipe_id: recipe.id.clone(),
            recipe_name: recipe.name.clone(),
            started_at,
            completed_at: Some(Utc::now()),
            status,
            items_affected,
            space_reclaimed,
            error_message: error_msg,
        };

        add_history_entry(history_entry.clone());
        Ok(history_entry)
    }
}

/// Execute a single automation action.
fn execute_action(
    rt: &tokio::runtime::Runtime,
    action: &Action,
) -> Result<(i64, u64)> {
    match action.action_type.as_str() {
        "scan_cache" => {
            let result = rt.block_on(cleaner_handlers::scan_cache())?;
            Ok((result.data.map(|v| v.len() as i64).unwrap_or(0), 0))
        }
        "clean_cache" => {
            let result = rt.block_on(cleaner_handlers::clean_junk_category("cache"))?;
            Ok((1, result.data.unwrap_or(0)))
        }
        "scan_trash" => {
            let result = rt.block_on(cleaner_handlers::scan_trash())?;
            Ok((result.data.map(|v| v.len() as i64).unwrap_or(0), 0))
        }
        "clean_trash" => {
            let result = rt.block_on(cleaner_handlers::clean_junk_category("trash"))?;
            Ok((1, result.data.unwrap_or(0)))
        }
        "scan_logs" => {
            let result = rt.block_on(cleaner_handlers::scan_logs())?;
            Ok((result.data.map(|v| v.len() as i64).unwrap_or(0), 0))
        }
        "clean_logs" => {
            let result = rt.block_on(cleaner_handlers::clean_junk_category("logs"))?;
            Ok((1, result.data.unwrap_or(0)))
        }
        "docker_prune" => {
            let result = rt.block_on(cleaner_handlers::docker_system_prune())?;
            Ok((1, result.data.unwrap_or(0)))
        }
        "optimize_memory" => {
            let result = rt.block_on(crate::application::handlers::optimize_memory())?;
            let _optimized = result.data.unwrap_or(false);
            Ok((1, 0)) // Memory optimization doesn't return bytes reclaimed
        }
        "custom" => {
            if let Some(cmd) = action.params.get("command").and_then(|v| v.as_str()) {
                let output = std::process::Command::new("sh")
                    .args(["-c", cmd])
                    .output()
                    .map_err(|e| AppError::Internal(format!("command failed: {}", e)))?;
                let reclaimed = if output.status.success() { 0 } else { 0 };
                Ok((1, reclaimed))
            } else {
                Err(AppError::ValidationError("custom action missing 'command' param".into()).into())
            }
        }
        _ => Err(AppError::ValidationError(format!("unknown action type: {}", action.action_type)).into()),
    }
}
