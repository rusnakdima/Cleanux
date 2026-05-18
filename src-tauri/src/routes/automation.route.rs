use crate::models::ResponseModel;
use crate::services::automation_service::{AutomationRecipe, AutomationService};

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_quick_actions() -> Result<ResponseModel, ResponseModel> {
  AutomationService::get_quick_actions()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn execute_action(action_id: String) -> Result<ResponseModel, ResponseModel> {
  AutomationService::execute_action(action_id)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_recipes() -> Result<ResponseModel, ResponseModel> {
  AutomationService::get_recipes()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn save_recipe(recipe: AutomationRecipe) -> Result<ResponseModel, ResponseModel> {
  AutomationService::save_recipe(recipe)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn delete_recipe(recipe_id: String) -> Result<ResponseModel, ResponseModel> {
  AutomationService::delete_recipe(recipe_id)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn execute_recipe(recipe_id: String) -> Result<ResponseModel, ResponseModel> {
  AutomationService::execute_recipe(recipe_id)
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn get_execution_history() -> Result<ResponseModel, ResponseModel> {
  AutomationService::get_execution_history_list()
}
