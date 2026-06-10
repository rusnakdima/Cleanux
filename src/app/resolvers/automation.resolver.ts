import { inject } from '@angular/core';
import { ResolveFn } from '@angular/router';
import { ApiService } from '@services/api.service';
import { QuickAction, AutomationRecipe, ExecutionHistoryEntry } from '@stores/automation.store';

interface AutomationData {
  quickActions: QuickAction[];
  recipes: AutomationRecipe[];
  history: ExecutionHistoryEntry[];
}

export const automationResolver: ResolveFn<AutomationData> = async () => {
  const api = inject(ApiService);

  try {
    const [quickActions, recipes, history] = await Promise.all([
      api.invoke<QuickAction[]>('get_quick_actions'),
      api.invoke<AutomationRecipe[]>('get_recipes'),
      api.invoke<ExecutionHistoryEntry[]>('get_execution_history'),
    ]);
    return { quickActions, recipes, history };
  } catch (error) {
    throw error;
  }
};

export const quickActionsResolver: ResolveFn<QuickAction[]> = async () => {
  const api = inject(ApiService);

  try {
    return await api.invoke<QuickAction[]>('get_quick_actions');
  } catch (error) {
    throw error;
  }
};

export const recipesResolver: ResolveFn<AutomationRecipe[]> = async () => {
  const api = inject(ApiService);

  try {
    return await api.invoke<AutomationRecipe[]>('get_recipes');
  } catch (error) {
    throw error;
  }
};
