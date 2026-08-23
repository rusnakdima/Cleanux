use crate::domain::entities::automation_recipe::AutomationRecipe;
use crate::domain::entities::execution_history::{ExecutionHistory, ExecutionStatus};
use crate::infrastructure::json_storage::JsonStorage;
use chrono::Utc;
use std::sync::{Arc, RwLock};

const TABLE_AUTOMATION_RECIPES: &str = "automation_recipes";

pub struct RoutineService {
    storage: Arc<JsonStorage>,
    cache: RwLock<Vec<AutomationRecipe>>,
}

impl RoutineService {
    pub fn new(storage: Arc<JsonStorage>) -> Arc<Self> {
        let recipes = storage
            .load::<Vec<AutomationRecipe>>(TABLE_AUTOMATION_RECIPES)
            .unwrap_or_else(|_| Vec::new());
        Arc::new(Self {
            storage,
            cache: RwLock::new(recipes),
        })
    }

    fn persist(&self) -> Result<(), String> {
        let cache = self.cache.read().map_err(|e| e.to_string())?;
        self.storage
            .save(TABLE_AUTOMATION_RECIPES, &*cache)
            .map_err(|e| e.to_string())
    }

    pub fn save_routine(
        self: &Arc<Self>,
        mut recipe: AutomationRecipe,
    ) -> Result<AutomationRecipe, String> {
        if recipe.id.is_none() {
            recipe.id = Some(crate::domain::entities::automation_recipe::generate_id());
            recipe.created_at = Utc::now();
            recipe.last_run = None;
            let mut cache = self.cache.write().map_err(|e| e.to_string())?;
            cache.push(recipe.clone());
        } else {
            let id = recipe.id.as_ref().unwrap().clone();
            let mut cache = self.cache.write().map_err(|e| e.to_string())?;
            if let Some(idx) = cache.iter().position(|r| r.id.as_ref() == Some(&id)) {
                cache[idx] = recipe.clone();
            } else {
                cache.push(recipe.clone());
            }
        }
        self.persist()?;
        Ok(recipe)
    }

    pub fn get_routines(self: &Arc<Self>) -> Result<Vec<AutomationRecipe>, String> {
        let cache = self.cache.read().map_err(|e| e.to_string())?;
        Ok(cache.clone())
    }

    pub fn delete_routine(self: &Arc<Self>, id: &str) -> Result<(), String> {
        {
            let mut cache = self.cache.write().map_err(|e| e.to_string())?;
            let initial_len = cache.len();
            cache.retain(|r| r.id.as_deref() != Some(id));
            if cache.len() == initial_len {
                return Err(format!("Routine not found: {}", id));
            }
        }
        self.persist()?;
        Ok(())
    }

    pub fn get_routine_by_id(
        self: &Arc<Self>,
        id: &str,
    ) -> Result<Option<AutomationRecipe>, String> {
        let cache = self.cache.read().map_err(|e| e.to_string())?;
        Ok(cache.iter().find(|r| r.id.as_deref() == Some(id)).cloned())
    }

    pub fn execute_routine(self: &Arc<Self>, id: &str) -> Result<ExecutionHistory, String> {
        let history = {
            let mut cache = self.cache.write().map_err(|e| e.to_string())?;
            let recipe = cache
                .iter_mut()
                .find(|r| r.id.as_deref() == Some(id))
                .ok_or_else(|| format!("Routine not found: {}", id))?;
            recipe.last_run = Some(Utc::now());
            let recipe_name = recipe.name.clone();
            let recipe_id = recipe.id.clone();
            ExecutionHistory {
                id: Some(crate::domain::entities::automation_recipe::generate_id()),
                recipe_id,
                recipe_name,
                status: ExecutionStatus::Completed,
                started_at: Utc::now(),
                completed_at: Some(Utc::now()),
                items_affected: 0,
                space_reclaimed: 0,
                error_message: None,
            }
        };
        self.persist()?;
        Ok(history)
    }
}
