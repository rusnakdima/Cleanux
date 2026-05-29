import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
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
export class AutomationService {
  private api = inject(ApiService);

  async getQuickActions(): Promise<QuickAction[]> {
    return await this.api.invoke<QuickAction[]>('get_quick_actions');
  }

  async executeAction(actionId: string): Promise<string> {
    return await this.api.invoke<string>('execute_action', { actionId });
  }

  async getRecipes(): Promise<AutomationRecipe[]> {
    return await this.api.invoke<AutomationRecipe[]>('get_recipes');
  }

  async saveRecipe(recipe: AutomationRecipe): Promise<string> {
    return await this.api.invoke<string>('save_recipe', { recipe });
  }

  async deleteRecipe(recipeId: string): Promise<string> {
    return await this.api.invoke<string>('delete_recipe', { recipeId });
  }

  async executeRecipe(recipeId: string): Promise<string> {
    return await this.api.invoke<string>('execute_recipe', { recipeId });
  }

  async getExecutionHistory(): Promise<ExecutionHistoryEntry[]> {
    return await this.api.invoke<ExecutionHistoryEntry[]>('get_execution_history');
  }
}
