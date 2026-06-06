import { Injectable } from '@angular/core';
import { BaseApiService } from '@services/base-api.service';
import {
  QuickAction,
  ActionStep,
  AutomationRecipe,
  ExecutionHistoryEntry,
} from '@models/automation.model';

export type { QuickAction, ActionStep, AutomationRecipe, ExecutionHistoryEntry };

@Injectable({
  providedIn: 'root',
})
export class AutomationService extends BaseApiService {
  async getQuickActions(): Promise<QuickAction[]> {
    return await this.call<QuickAction[]>('get_quick_actions');
  }

  async executeAction(actionId: string): Promise<string> {
    return await this.call<string>('execute_action', { actionId });
  }

  async getRecipes(): Promise<AutomationRecipe[]> {
    return await this.call<AutomationRecipe[]>('get_recipes');
  }

  async saveRecipe(recipe: AutomationRecipe): Promise<string> {
    return await this.call<string>('save_recipe', { recipe });
  }

  async deleteRecipe(recipeId: string): Promise<string> {
    return await this.call<string>('delete_recipe', { recipeId });
  }

  async executeRecipe(recipeId: string): Promise<string> {
    return await this.call<string>('execute_recipe', { recipeId });
  }

  async getExecutionHistory(): Promise<ExecutionHistoryEntry[]> {
    return await this.call<ExecutionHistoryEntry[]>('get_execution_history');
  }
}
