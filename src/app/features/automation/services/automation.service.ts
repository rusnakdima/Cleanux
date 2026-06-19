import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import {
  QuickAction,
  ActionStep,
  AutomationRecipe,
  ExecutionHistoryEntry,
} from '@entities/automation.model';

export type { QuickAction, ActionStep, AutomationRecipe, ExecutionHistoryEntry };

@Injectable({
  providedIn: 'root',
})
export class AutomationService {
  private api = inject(ApiService);

  constructor() {}

  async getQuickActions(): Promise<QuickAction[]> {
    try {
      const result = await this.api.invoke<QuickAction[]>('get_quick_actions');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async executeAction(actionId: string): Promise<string> {
    try {
      const result = await this.api.invoke<string>('execute_action', { actionId });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getRecipes(): Promise<AutomationRecipe[]> {
    try {
      const result = await this.api.invoke<AutomationRecipe[]>('get_recipes');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async saveRecipe(recipe: AutomationRecipe): Promise<string> {
    try {
      const result = await this.api.invoke<string>('save_recipe', { recipe });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async deleteRecipe(recipeId: string): Promise<string> {
    try {
      const result = await this.api.invoke<string>('delete_recipe', { recipeId });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async executeRecipe(recipeId: string): Promise<string> {
    try {
      const result = await this.api.invoke<string>('execute_recipe', { recipeId });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getExecutionHistory(): Promise<ExecutionHistoryEntry[]> {
    try {
      const result = await this.api.invoke<ExecutionHistoryEntry[]>('get_execution_history');
      return result;
    } catch (error) {
      throw error;
    }
  }
}
