import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';

export interface QuickAction {
  id: string;
  name: string;
  description: string;
  icon: string;
  actions: ActionStep[];
}

export interface ActionStep {
  CleanCategory?: { category: string };
  RunProfile?: { profileName: string };
  ExecuteCommand?: { command: string };
  Wait?: { seconds: number };
}

export interface AutomationRecipe {
  id: string;
  name: string;
  steps: ActionStep[];
  enabled: boolean;
  trigger: 'Manual' | 'Scheduled' | 'Event';
}

export interface ExecutionHistoryEntry {
  id: string;
  name: string;
  status: string;
  startedAt: string;
  completedAt?: string;
  stepsExecuted: number;
  totalSteps: number;
}

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
