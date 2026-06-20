use crate::crud_create_command;
use crate::crud_delete_command;
use crate::crud_get_all_command;
use crate::crud_get_command;
use crate::crud_update_command;
crud_get_command!(get_automation_recipe, "automation_recipes");
crud_get_all_command!(get_automation_recipes, "automation_recipes");
crud_create_command!(create_automation_recipe, "automation_recipes");
crud_update_command!(update_automation_recipe, "automation_recipes");
crud_delete_command!(delete_automation_recipe, "automation_recipes");
use crate::models::{Response, Status};
use crate::AppState;
use tauri::State;
#[tauri::command(rename_all = "camelCase")]
pub async fn crud_get_execution_history(
  state: State<'_, AppState>,
  page: Option<u64>,
  limit: Option<u64>,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  let docs = state
    .data
    .repository_service
    .find_many(
      "execution_history",
      None,
      page,
      limit,
      Some("started_at"),
      false,
    )
    .await
    .map_err(|e| Response::error(Status::Error, e.to_string()))?;
  Ok(Response::success(
    "Execution history retrieved".to_string(),
    serde_json::to_value(docs).unwrap_or(serde_json::Value::Null),
  ))
}
#[tauri::command(rename_all = "camelCase")]
pub async fn crud_get_quick_actions(
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  use crate::services::automation_service::AutomationService;
  let actions = AutomationService::get_quick_actions_list();
  Ok(Response::success(
    "Quick actions retrieved".to_string(),
    serde_json::to_value(actions).unwrap_or_default(),
  ))
}
#[tauri::command(rename_all = "camelCase")]
pub async fn crud_execute_action(
  action_id: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  use crate::services::automation_service::AutomationService;
  AutomationService::execute_action(action_id)
}
#[tauri::command(rename_all = "camelCase")]
pub async fn crud_execute_recipe(
  state: State<'_, AppState>,
  recipe_id: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  use crate::services::automation_service::AutomationService;
  let filter = serde_json::json!({
      "id": recipe_id
  });
  let filter = nosql_orm::query::Filter::from_json(&filter)
    .map_err(|e| Response::error(Status::Error, e.to_string()))?;
  let recipes = state
    .data
    .repository_service
    .find_many(
      "automation_recipes",
      Some(filter),
      None,
      Some(1),
      None,
      true,
    )
    .await
    .map_err(|e| Response::error(Status::Error, e.to_string()))?;
  let recipe = recipes
    .into_iter()
    .next()
    .ok_or_else(|| Response::error(Status::Error, "Recipe not found".to_string()))?;
  AutomationService::execute_recipe_from_entity(recipe)
    .map_err(|e| Response::error(Status::Error, e.to_string()))
}
