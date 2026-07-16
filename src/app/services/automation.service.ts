/* angular */
import { Injectable, inject } from '@angular/core';

/* library */
import { InvokeWrapperService } from '@tauri-front/shared';

export interface QuickAction {
  id: string;
  name: string;
  description: string;
  icon: string;
  actions: Array<Record<string, unknown>>;
}

export interface AutomationRecipe {
  id?: string;
  name: string;
  steps: Array<Record<string, unknown>>;
  enabled: boolean;
  trigger: string;
}

export interface ExecutionHistoryEntry {
  id: string;
  name: string;
  status: string;
  startedAt: string;
  completedAt: string;
  stepsExecuted: number;
  totalSteps: number;
}

@Injectable({ providedIn: 'root' })
export class AutomationService {
  private api = inject(InvokeWrapperService);

  async getQuickActions(): Promise<QuickAction[]> {
    return this.api.invoke<QuickAction[]>('get_quick_actions');
  }

  async executeAction(actionId: string): Promise<string> {
    return this.api.invoke<string>('execute_action', { actionId });
  }

  async getRecipes(): Promise<AutomationRecipe[]> {
    return this.api.invoke<AutomationRecipe[]>('get_recipes');
  }

  async saveRecipe(recipe: AutomationRecipe): Promise<string> {
    return this.api.invoke<string>('save_recipe', { recipe });
  }

  async deleteRecipe(recipeId: string): Promise<string> {
    return this.api.invoke<string>('delete_recipe', { recipeId });
  }

  async executeRecipe(recipeId: string): Promise<string> {
    return this.api.invoke<string>('execute_recipe', { recipeId });
  }

  async getExecutionHistory(): Promise<ExecutionHistoryEntry[]> {
    return this.api.invoke<ExecutionHistoryEntry[]>('get_execution_history');
  }
}
