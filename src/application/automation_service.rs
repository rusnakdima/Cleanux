//! AutomationService - orchestrates automation recipes

use crate::domain::{AutomationRecipe, ExecutionHistory};

pub trait AutomationServiceTrait {
    fn get_recipes(&self) -> Result<Vec<AutomationRecipe>, String>;
    fn get_recipe(&self, id: &str) -> Result<Option<AutomationRecipe>, String>;
    fn create_recipe(&mut self, recipe: AutomationRecipe) -> Result<AutomationRecipe, String>;
    fn update_recipe(
        &mut self,
        id: &str,
        recipe: AutomationRecipe,
    ) -> Result<AutomationRecipe, String>;
    fn delete_recipe(&mut self, id: &str) -> Result<(), String>;
    fn execute_recipe(&mut self, id: &str) -> Result<ExecutionHistory, String>;
}
