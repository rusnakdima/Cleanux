import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import {
  QuickAction,
  ActionStep,
  AutomationRecipe,
  ExecutionHistoryEntry,
} from '@models/automation.model';
import { LoggingService, getLoggingService } from '@tauri-apps/logger';

export type { QuickAction, ActionStep, AutomationRecipe, ExecutionHistoryEntry };

@Injectable({
  providedIn: 'root',
})
export class AutomationService {
  private api = inject(ApiService);
  private loggingService = getLoggingService();

  constructor() {
    this.loggingService.info('AutomationService initialized');
  }

  async getQuickActions(): Promise<QuickAction[]> {
    this.loggingService.info('Getting quick actions');
    try {
      const result = await this.api.invoke<QuickAction[]>('get_quick_actions');
      this.loggingService.info('Quick actions retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async executeAction(actionId: string): Promise<string> {
    this.loggingService.info('Executing action', { actionId });
    try {
      const result = await this.api.invoke<string>('execute_action', { actionId });
      this.loggingService.info('Action executed');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { actionId });
      throw error;
    }
  }

  async getRecipes(): Promise<AutomationRecipe[]> {
    this.loggingService.info('Getting recipes');
    try {
      const result = await this.api.invoke<AutomationRecipe[]>('get_recipes');
      this.loggingService.info('Recipes retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async saveRecipe(recipe: AutomationRecipe): Promise<string> {
    this.loggingService.info('Saving recipe', { name: recipe.name });
    try {
      const result = await this.api.invoke<string>('save_recipe', { recipe });
      this.loggingService.info('Recipe saved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { name: recipe.name });
      throw error;
    }
  }

  async deleteRecipe(recipeId: string): Promise<string> {
    this.loggingService.info('Deleting recipe', { recipeId });
    try {
      const result = await this.api.invoke<string>('delete_recipe', { recipeId });
      this.loggingService.info('Recipe deleted');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { recipeId });
      throw error;
    }
  }

  async executeRecipe(recipeId: string): Promise<string> {
    this.loggingService.info('Executing recipe', { recipeId });
    try {
      const result = await this.api.invoke<string>('execute_recipe', { recipeId });
      this.loggingService.info('Recipe executed');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { recipeId });
      throw error;
    }
  }

  async getExecutionHistory(): Promise<ExecutionHistoryEntry[]> {
    this.loggingService.info('Getting execution history');
    try {
      const result = await this.api.invoke<ExecutionHistoryEntry[]>('get_execution_history');
      this.loggingService.info('Execution history retrieved', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }
}
