use crate::helpers::{data_string, success_response};
use crate::models::AppError;
use crate::models::ResponseModel;
use crate::services::profile_service::ProfileService;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn get_timestamp_u64() -> u64 {
  use std::time::{SystemTime, UNIX_EPOCH};
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .map_err(|_| AppError::Unknown("Time went backwards".to_string()))
    .unwrap_or_default()
    .as_secs()
}

fn generate_uuid() -> String {
  let now = get_timestamp_u64();
  let random = rand_simple();
  format!("{:x}-{:x}", now, random)
}

fn rand_simple() -> u128 {
  use std::collections::hash_map::RandomState;
  use std::hash::{BuildHasher, Hasher};
  let hasher = RandomState::new().build_hasher();
  hasher.finish() as u128
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuickAction {
  pub id: String,
  pub name: String,
  pub description: String,
  pub icon: String,
  pub actions: Vec<ActionStep>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ActionStep {
  CleanCategory { category: String },
  RunProfile { profile_name: String },
  ExecuteCommand { command: String },
  Wait { seconds: u32 },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RecipeTrigger {
  Manual,
  Scheduled,
  Event,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AutomationRecipe {
  pub id: String,
  pub name: String,
  pub steps: Vec<ActionStep>,
  pub enabled: bool,
  pub trigger: RecipeTrigger,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionHistoryEntry {
  pub id: String,
  pub name: String,
  pub status: String,
  pub started_at: String,
  pub completed_at: Option<String>,
  pub steps_executed: u32,
  pub total_steps: u32,
}

pub struct AutomationService;

fn get_recipes_path() -> PathBuf {
  let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
  let cleanux_dir = config_dir.join("cleanux");
  if !cleanux_dir.exists() {
    let _ = fs::create_dir_all(&cleanux_dir);
  }
  cleanux_dir.join("recipes.json")
}

fn get_history_path() -> PathBuf {
  let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
  let cleanux_dir = config_dir.join("cleanux");
  if !cleanux_dir.exists() {
    let _ = fs::create_dir_all(&cleanux_dir);
  }
  cleanux_dir.join("execution_history.json")
}

fn get_predefined_quick_actions() -> Vec<QuickAction> {
  vec![
    QuickAction {
      id: "quick-cache-clear".to_string(),
      name: "Quick Cache Clear".to_string(),
      description: "Clear all system caches quickly".to_string(),
      icon: "cached".to_string(),
      actions: vec![ActionStep::CleanCategory {
        category: "cache".to_string(),
      }],
    },
    QuickAction {
      id: "quick-deep-clean".to_string(),
      name: "Deep Clean".to_string(),
      description: "Clear cache, trash, and logs in one go".to_string(),
      icon: "cleaning_services".to_string(),
      actions: vec![
        ActionStep::CleanCategory {
          category: "cache".to_string(),
        },
        ActionStep::CleanCategory {
          category: "trash".to_string(),
        },
        ActionStep::CleanCategory {
          category: "logs".to_string(),
        },
      ],
    },
    QuickAction {
      id: "quick-morning-refresh".to_string(),
      name: "Morning Refresh".to_string(),
      description: "Clear logs and cache to free up space".to_string(),
      icon: "wb_sunny".to_string(),
      actions: vec![
        ActionStep::CleanCategory {
          category: "logs".to_string(),
        },
        ActionStep::Wait { seconds: 2 },
        ActionStep::CleanCategory {
          category: "cache".to_string(),
        },
      ],
    },
    QuickAction {
      id: "quick-large-files".to_string(),
      name: "Large Files Scan".to_string(),
      description: "Clear files larger than 100MB".to_string(),
      icon: "file_upload".to_string(),
      actions: vec![ActionStep::CleanCategory {
        category: "largefiles".to_string(),
      }],
    },
  ]
}

fn execute_action_step(step: &ActionStep) -> Result<String, AppError> {
  use crate::services::{
    cache_cleaning_service::CacheCleaningService,
    large_file_cleaning_service::LargeFileCleaningService,
    log_cleaning_service::LogCleaningService, trash_cleaning_service::TrashCleaningService,
  };
  match step {
    ActionStep::CleanCategory { category } => match category.as_str() {
      "cache" => {
        let _ = CacheCleaningService.clearCache();
        Ok("Cache cleared".to_string())
      }
      "trash" => {
        let _ = TrashCleaningService.clearTrash();
        Ok("Trash cleared".to_string())
      }
      "logs" => {
        let _ = LogCleaningService.clearAllLogs();
        Ok("Logs cleared".to_string())
      }
      "largefiles" => {
        let _ = LargeFileCleaningService.clearAllLargeFiles();
        Ok("Large files cleared".to_string())
      }
      _ => Err(AppError::message(format!("Unknown category: {}", category))),
    },
    ActionStep::RunProfile { profile_name } => match ProfileService.apply_profile(profile_name) {
      Ok(result) => Ok(format!("Profile '{}' applied: {}", profile_name, result)),
      Err(e) => Err(AppError::message(format!("Failed to run profile: {}", e))),
    },
    ActionStep::ExecuteCommand { command } => {
      let allowed_commands = ["sync", "shutdown", "reboot", "systemctl", "service"];
      let is_allowed = allowed_commands.iter().any(|c| command.starts_with(c));
      if !is_allowed {
        return Err(AppError::InvalidPath(format!(
          "Command '{}' is not in the allowed list",
          command
        )));
      }
      use std::process::Command;
      let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| AppError::message(format!("Failed to execute command: {}", e)))?;
      if output.status.success() {
        Ok(format!("Command executed: {}", command))
      } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(AppError::message(format!("Command failed: {}", stderr)))
      }
    }
    ActionStep::Wait { seconds } => {
      thread::sleep(Duration::from_secs(*seconds as u64));
      Ok(format!("Waited {} seconds", seconds))
    }
  }
}

fn add_to_history(entry: ExecutionHistoryEntry) -> Result<(), AppError> {
  let mut history = get_execution_history().unwrap_or_default();
  history.insert(0, entry);
  if history.len() > 100 {
    history.truncate(100);
  }
  let json = serde_json::to_string_pretty(&history)
    .map_err(|e| AppError::message(format!("Failed to serialize history: {}", e)))?;
  fs::write(get_history_path(), json)
    .map_err(|e| AppError::message(format!("Failed to write history: {}", e)))?;
  Ok(())
}

fn get_execution_history() -> Result<Vec<ExecutionHistoryEntry>, AppError> {
  let path = get_history_path();
  if !path.exists() {
    return Ok(Vec::new());
  }
  let content = fs::read_to_string(&path)
    .map_err(|e| AppError::message(format!("Failed to read history: {}", e)))?;
  let history: Vec<ExecutionHistoryEntry> = serde_json::from_str(&content)
    .map_err(|e| AppError::message(format!("Failed to parse history: {}", e)))?;
  Ok(history)
}

#[allow(non_snake_case)]
impl AutomationService {
  pub fn get_quick_actions() -> Result<ResponseModel, ResponseModel> {
    Self::get_quick_actions_inner().map_err(|e| e.into_response())
  }

  fn get_quick_actions_inner() -> Result<ResponseModel, AppError> {
    let actions = get_predefined_quick_actions();
    let json = serde_json::to_value(&actions)
      .map_err(|e| AppError::message(format!("Failed to serialize actions: {}", e)))?;
    let data = serde_json::from_value(json)
      .map_err(|e| AppError::message(format!("Failed to deserialize actions: {}", e)))?;
    Ok(success_response("Quick actions retrieved", data))
  }

  pub fn execute_action(action_id: String) -> Result<ResponseModel, ResponseModel> {
    Self::execute_action_inner(action_id).map_err(|e| e.into_response())
  }

  fn execute_action_inner(action_id: String) -> Result<ResponseModel, AppError> {
    let actions = get_predefined_quick_actions();
    let action = actions
      .into_iter()
      .find(|a| a.id == action_id)
      .ok_or_else(|| AppError::message(format!("Action not found: {}", action_id)))?;

    let start_time = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let total_steps = action.actions.len() as u32;

    let mut steps_executed = 0u32;
    for step in &action.actions {
      execute_action_step(step)?;
      steps_executed += 1;
    }

    let entry = ExecutionHistoryEntry {
      id: generate_uuid(),
      name: action.name.clone(),
      status: "completed".to_string(),
      started_at: start_time,
      completed_at: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
      steps_executed,
      total_steps,
    };
    let _ = add_to_history(entry);

    Ok(success_response(
      format!("Action '{}' executed successfully", action.name),
      data_string("executed"),
    ))
  }

  pub fn get_recipes() -> Result<ResponseModel, ResponseModel> {
    Self::get_recipes_inner().map_err(|e| e.into_response())
  }

  fn get_recipes_inner() -> Result<ResponseModel, AppError> {
    let path = get_recipes_path();
    if !path.exists() {
      return Ok(success_response("No recipes saved", data_string("[]")));
    }
    let content = fs::read_to_string(&path)
      .map_err(|e| AppError::message(format!("Failed to read recipes: {}", e)))?;
    let recipes: Vec<AutomationRecipe> = serde_json::from_str(&content)
      .map_err(|e| AppError::message(format!("Failed to parse recipes: {}", e)))?;
    let json = serde_json::to_value(&recipes)
      .map_err(|e| AppError::message(format!("Failed to serialize recipes: {}", e)))?;
    let data = serde_json::from_value(json)
      .map_err(|e| AppError::message(format!("Failed to deserialize recipes: {}", e)))?;
    Ok(success_response("Recipes retrieved", data))
  }

  pub fn save_recipe(recipe: AutomationRecipe) -> Result<ResponseModel, ResponseModel> {
    Self::save_recipe_inner(recipe).map_err(|e| e.into_response())
  }

  fn save_recipe_inner(mut recipe: AutomationRecipe) -> Result<ResponseModel, AppError> {
    if recipe.id.is_empty() {
      recipe.id = generate_uuid();
    }
    let path = get_recipes_path();
    let mut recipes: Vec<AutomationRecipe> = if path.exists() {
      let content = fs::read_to_string(&path)
        .map_err(|e| AppError::message(format!("Failed to read recipes: {}", e)))?;
      serde_json::from_str(&content)
        .map_err(|e| AppError::message(format!("Failed to parse recipes: {}", e)))?
    } else {
      Vec::new()
    };

    if let Some(pos) = recipes.iter().position(|r| r.id == recipe.id) {
      recipes[pos] = recipe;
    } else {
      recipes.push(recipe);
    }

    let json = serde_json::to_string_pretty(&recipes)
      .map_err(|e| AppError::message(format!("Failed to serialize recipes: {}", e)))?;
    fs::write(&path, json)
      .map_err(|e| AppError::message(format!("Failed to write recipes: {}", e)))?;

    Ok(success_response(
      "Recipe saved successfully",
      data_string("saved"),
    ))
  }

  pub fn delete_recipe(recipe_id: String) -> Result<ResponseModel, ResponseModel> {
    Self::delete_recipe_inner(recipe_id).map_err(|e| e.into_response())
  }

  fn delete_recipe_inner(recipe_id: String) -> Result<ResponseModel, AppError> {
    let path = get_recipes_path();
    if !path.exists() {
      return Ok(success_response(
        "No recipes to delete",
        data_string("deleted"),
      ));
    }
    let content = fs::read_to_string(&path)
      .map_err(|e| AppError::message(format!("Failed to read recipes: {}", e)))?;
    let mut recipes: Vec<AutomationRecipe> = serde_json::from_str(&content)
      .map_err(|e| AppError::message(format!("Failed to parse recipes: {}", e)))?;

    recipes.retain(|r| r.id != recipe_id);

    let json = serde_json::to_string_pretty(&recipes)
      .map_err(|e| AppError::message(format!("Failed to serialize recipes: {}", e)))?;
    fs::write(&path, json)
      .map_err(|e| AppError::message(format!("Failed to write recipes: {}", e)))?;

    Ok(success_response("Recipe deleted", data_string("deleted")))
  }

  pub fn execute_recipe(recipe_id: String) -> Result<ResponseModel, ResponseModel> {
    Self::execute_recipe_inner(recipe_id).map_err(|e| e.into_response())
  }

  fn execute_recipe_inner(recipe_id: String) -> Result<ResponseModel, AppError> {
    let path = get_recipes_path();
    if !path.exists() {
      return Err(AppError::message("No recipes found"));
    }
    let content = fs::read_to_string(&path)
      .map_err(|e| AppError::message(format!("Failed to read recipes: {}", e)))?;
    let recipes: Vec<AutomationRecipe> = serde_json::from_str(&content)
      .map_err(|e| AppError::message(format!("Failed to parse recipes: {}", e)))?;

    let recipe = recipes
      .into_iter()
      .find(|r| r.id == recipe_id)
      .ok_or_else(|| AppError::message(format!("Recipe not found: {}", recipe_id)))?;

    let start_time = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let total_steps = recipe.steps.len() as u32;

    let mut steps_executed = 0u32;
    for step in &recipe.steps {
      execute_action_step(step)?;
      steps_executed += 1;
    }

    let entry = ExecutionHistoryEntry {
      id: generate_uuid(),
      name: recipe.name.clone(),
      status: "completed".to_string(),
      started_at: start_time,
      completed_at: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
      steps_executed,
      total_steps,
    };
    let _ = add_to_history(entry);

    Ok(success_response(
      format!("Recipe '{}' executed successfully", recipe.name),
      data_string("executed"),
    ))
  }

  pub fn get_execution_history_list() -> Result<ResponseModel, ResponseModel> {
    Self::get_execution_history_inner().map_err(|e| e.into_response())
  }

  fn get_execution_history_inner() -> Result<ResponseModel, AppError> {
    let history = get_execution_history()?;
    let json = serde_json::to_value(&history)
      .map_err(|e| AppError::message(format!("Failed to serialize history: {}", e)))?;
    let data = serde_json::from_value(json)
      .map_err(|e| AppError::message(format!("Failed to deserialize history: {}", e)))?;
    Ok(success_response("History retrieved", data))
  }
}
