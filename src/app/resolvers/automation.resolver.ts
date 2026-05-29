import { inject } from '@angular/core';
import { ResolveFn } from '@angular/router';
import { TauriApiService } from '@api/tauri-api.service';
import { QuickAction, AutomationRecipe, ExecutionHistoryEntry } from '@stores/automation.store';

interface AutomationData {
  quickActions: QuickAction[];
  recipes: AutomationRecipe[];
  history: ExecutionHistoryEntry[];
}

export const automationResolver: ResolveFn<AutomationData> = async () => {
  const api = inject(TauriApiService);

  try {
    const [quickActions, recipes, history] = await Promise.all([
      api.invoke<QuickAction[]>('get_quick_actions'),
      api.invoke<AutomationRecipe[]>('get_recipes'),
      api.invoke<ExecutionHistoryEntry[]>('get_execution_history'),
    ]);
    return { quickActions, recipes, history };
  } catch (error) {
    console.error('Failed to resolve automation data:', error);
    return { quickActions: [], recipes: [], history: [] };
  }
};

export const quickActionsResolver: ResolveFn<QuickAction[]> = async () => {
  const api = inject(TauriApiService);

  try {
    return await api.invoke<QuickAction[]>('get_quick_actions');
  } catch (error) {
    console.error('Failed to resolve quick actions:', error);
    return [];
  }
};

export const recipesResolver: ResolveFn<AutomationRecipe[]> = async () => {
  const api = inject(TauriApiService);

  try {
    return await api.invoke<AutomationRecipe[]>('get_recipes');
  } catch (error) {
    console.error('Failed to resolve recipes:', error);
    return [];
  }
};
