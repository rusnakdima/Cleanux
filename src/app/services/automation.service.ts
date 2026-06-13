import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import {
  QuickAction,
  ActionStep,
  AutomationRecipe,
  ExecutionHistoryEntry,
} from '@models/automation.model';
import { LoggerService } from '@services/logger.service';

export type { QuickAction, ActionStep, AutomationRecipe, ExecutionHistoryEntry };

@Injectable({
  providedIn: 'root',
})
export class AutomationService {
  private api = inject(ApiService);
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'AutomationService', 'init', 'AutomationService initialized');
  }

  async getQuickActions(): Promise<QuickAction[]> {
    this.logger.logInfo('service', 'AutomationService', 'getQuickActions', 'Getting quick actions');
    try {
      const result = await this.api.invoke<QuickAction[]>('get_quick_actions');
      this.logger.logInfo(
        'service',
        'AutomationService',
        'getQuickActions',
        'Quick actions retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'AutomationService',
        'getQuickActions',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async executeAction(actionId: string): Promise<string> {
    this.logger.logInfo('service', 'AutomationService', 'executeAction', 'Executing action', {
      actionId,
    });
    try {
      const result = await this.api.invoke<string>('execute_action', { actionId });
      this.logger.logInfo('service', 'AutomationService', 'executeAction', 'Action executed');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'AutomationService',
        'executeAction',
        'Operation failed',
        error as Error,
        { actionId }
      );
      throw error;
    }
  }

  async getRecipes(): Promise<AutomationRecipe[]> {
    this.logger.logInfo('service', 'AutomationService', 'getRecipes', 'Getting recipes');
    try {
      const result = await this.api.invoke<AutomationRecipe[]>('get_recipes');
      this.logger.logInfo('service', 'AutomationService', 'getRecipes', 'Recipes retrieved', {
        count: result.length,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'AutomationService',
        'getRecipes',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async saveRecipe(recipe: AutomationRecipe): Promise<string> {
    this.logger.logInfo('service', 'AutomationService', 'saveRecipe', 'Saving recipe', {
      name: recipe.name,
    });
    try {
      const result = await this.api.invoke<string>('save_recipe', { recipe });
      this.logger.logInfo('service', 'AutomationService', 'saveRecipe', 'Recipe saved');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'AutomationService',
        'saveRecipe',
        'Operation failed',
        error as Error,
        { name: recipe.name }
      );
      throw error;
    }
  }

  async deleteRecipe(recipeId: string): Promise<string> {
    this.logger.logInfo('service', 'AutomationService', 'deleteRecipe', 'Deleting recipe', {
      recipeId,
    });
    try {
      const result = await this.api.invoke<string>('delete_recipe', { recipeId });
      this.logger.logInfo('service', 'AutomationService', 'deleteRecipe', 'Recipe deleted');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'AutomationService',
        'deleteRecipe',
        'Operation failed',
        error as Error,
        { recipeId }
      );
      throw error;
    }
  }

  async executeRecipe(recipeId: string): Promise<string> {
    this.logger.logInfo('service', 'AutomationService', 'executeRecipe', 'Executing recipe', {
      recipeId,
    });
    try {
      const result = await this.api.invoke<string>('execute_recipe', { recipeId });
      this.logger.logInfo('service', 'AutomationService', 'executeRecipe', 'Recipe executed');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'AutomationService',
        'executeRecipe',
        'Operation failed',
        error as Error,
        { recipeId }
      );
      throw error;
    }
  }

  async getExecutionHistory(): Promise<ExecutionHistoryEntry[]> {
    this.logger.logInfo(
      'service',
      'AutomationService',
      'getExecutionHistory',
      'Getting execution history'
    );
    try {
      const result = await this.api.invoke<ExecutionHistoryEntry[]>('get_execution_history');
      this.logger.logInfo(
        'service',
        'AutomationService',
        'getExecutionHistory',
        'Execution history retrieved',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'AutomationService',
        'getExecutionHistory',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }
}
