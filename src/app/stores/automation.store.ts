import { Injectable, signal, computed, inject } from '@angular/core';
import { UnifiedStorageService } from '@app/core/services/unified-storage.service';
import {
  QuickAction,
  ActionStep,
  AutomationRecipe,
  ExecutionHistoryEntry,
} from '@models/automation.model';

export type { QuickAction, ActionStep, AutomationRecipe, ExecutionHistoryEntry };

@Injectable({ providedIn: 'root' })
export class AutomationStore {
  private storage = inject(UnifiedStorageService);

  private _quickActions = signal<QuickAction[]>([]);
  private _recipes = signal<AutomationRecipe[]>([]);
  private _executionHistory = signal<ExecutionHistoryEntry[]>([]);
  private _loading = signal(false);
  private _error = signal<string | null>(null);

  readonly quickActions = this._quickActions.asReadonly();
  readonly recipes = this._recipes.asReadonly();
  readonly executionHistory = this._executionHistory.asReadonly();
  readonly loading = this._loading.asReadonly();
  readonly error = this._error.asReadonly();

  readonly hasRecipes = computed(() => this._recipes().length > 0);
  readonly hasExecutionHistory = computed(() => this._executionHistory().length > 0);

  async loadQuickActions(): Promise<QuickAction[]> {
    this._loading.set(true);
    this._error.set(null);
    try {
      const actions = await this.storage.entity.findMany<QuickAction>('quick_actions');
      this._quickActions.set(actions);
      return actions;
    } catch (e) {
      this._error.set(e instanceof Error ? e.message : 'Failed to load quick actions');
      return [];
    } finally {
      this._loading.set(false);
    }
  }

  async executeAction(actionId: string): Promise<string> {
    this._loading.set(true);
    this._error.set(null);
    try {
      return await this.storage.entity
        .findMany<{ result: string }>('execute_action', { actionId })
        .then(() => 'executed');
    } catch (e) {
      const message = e instanceof Error ? e.message : 'Failed to execute action';
      this._error.set(message);
      throw new Error(message);
    } finally {
      this._loading.set(false);
    }
  }

  async loadRecipes(): Promise<AutomationRecipe[]> {
    this._loading.set(true);
    this._error.set(null);
    try {
      const recipes = await this.storage.findMany<AutomationRecipe>('automation_recipes');
      this._recipes.set(recipes);
      return recipes;
    } catch (e) {
      this._error.set(e instanceof Error ? e.message : 'Failed to load recipes');
      return [];
    } finally {
      this._loading.set(false);
    }
  }

  async saveRecipe(recipe: AutomationRecipe): Promise<string> {
    this._loading.set(true);
    this._error.set(null);
    try {
      const saved = await this.storage.save<AutomationRecipe>(
        'automation_recipes',
        recipe,
        recipe.id || undefined
      );
      return saved.id || '';
    } catch (e) {
      const message = e instanceof Error ? e.message : 'Failed to save recipe';
      this._error.set(message);
      throw new Error(message);
    } finally {
      this._loading.set(false);
    }
  }

  async deleteRecipe(recipeId: string): Promise<string> {
    this._loading.set(true);
    this._error.set(null);
    try {
      await this.storage.delete('automation_recipes', recipeId);
      return 'deleted';
    } catch (e) {
      const message = e instanceof Error ? e.message : 'Failed to delete recipe';
      this._error.set(message);
      throw new Error(message);
    } finally {
      this._loading.set(false);
    }
  }

  async executeRecipe(recipeId: string): Promise<string> {
    this._loading.set(true);
    this._error.set(null);
    try {
      return await this.storage.entity
        .findMany<{ result: string }>('execute_recipe', { recipeId })
        .then(() => 'executed');
    } catch (e) {
      const message = e instanceof Error ? e.message : 'Failed to execute recipe';
      this._error.set(message);
      throw new Error(message);
    } finally {
      this._loading.set(false);
    }
  }

  async loadExecutionHistory(): Promise<ExecutionHistoryEntry[]> {
    this._loading.set(true);
    this._error.set(null);
    try {
      const history = await this.storage.findMany<ExecutionHistoryEntry>('execution_history');
      this._executionHistory.set(history);
      return history;
    } catch (e) {
      this._error.set(e instanceof Error ? e.message : 'Failed to load execution history');
      return [];
    } finally {
      this._loading.set(false);
    }
  }

  clearError(): void {
    this._error.set(null);
  }
}
