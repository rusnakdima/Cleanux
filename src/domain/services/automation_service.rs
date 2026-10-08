//! AutomationServiceTrait — domain trait for automation recipe management.
//!
//! Defines the contract for recipe CRUD and execution.
//! Implementation lives in `infrastructure`.

use crate::domain::entities::automation_recipe::AutomationRecipe;
use crate::domain::entities::execution_history::ExecutionHistory;
use crate::error::Result;

pub trait AutomationServiceTrait: Send + Sync {
    fn get_recipes(&self) -> Result<Vec<AutomationRecipe>>;
    fn get_recipe(&self, id: &str) -> Result<Option<AutomationRecipe>>;
    fn create_recipe(&mut self, recipe: AutomationRecipe) -> Result<AutomationRecipe>;
    fn update_recipe(
        &mut self,
        id: &str,
        recipe: AutomationRecipe,
    ) -> Result<AutomationRecipe>;
    fn delete_recipe(&mut self, id: &str) -> Result<()>;
    fn execute_recipe(&mut self, id: &str) -> Result<ExecutionHistory>;
}
