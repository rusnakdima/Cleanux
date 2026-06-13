import { Injectable, signal, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import {
  QuickAction,
  ActionStep,
  AutomationRecipe,
  ExecutionHistoryEntry,
} from '@models/automation.model';

export type { QuickAction, ActionStep, AutomationRecipe, ExecutionHistoryEntry };

@Injectable()
export class AutomationStore {
  private api = inject(ApiService);

  async loadQuickActions(): Promise<QuickAction[]> {
    return await this.api.invoke<QuickAction[]>('get_quick_actions');
  }

  async executeAction(actionId: string): Promise<string> {
    return await this.api.invoke<string>('execute_action', { actionId });
  }

  async loadRecipes(): Promise<AutomationRecipe[]> {
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

  async loadExecutionHistory(): Promise<ExecutionHistoryEntry[]> {
    return await this.api.invoke<ExecutionHistoryEntry[]>('get_execution_history');
  }
}
