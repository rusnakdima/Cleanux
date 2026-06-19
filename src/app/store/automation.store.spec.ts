import { describe, it, expect, beforeEach, vi } from 'vitest';
import { Injector, runInInjectionContext } from '@angular/core';

vi.mock('@api/tauri-api.service');

describe('AutomationStore', () => {
  let injector: Injector;
  let mockApi: { invoke: ReturnType<typeof vi.fn> };

  const createMockQuickActions = () => [
    {
      id: 'qa1',
      name: 'Quick Clean',
      description: 'Clean cache and temp files',
      icon: 'broom',
      actions: [{ CleanCategory: { category: 'cache' } }],
    },
  ];

  const createMockRecipes = () => [
    {
      id: 'rec1',
      name: 'Morning Cleanup',
      steps: [{ CleanCategory: { category: 'cache' } }],
      enabled: true,
      trigger: 'Scheduled' as const,
    },
  ];

  const createMockExecutionHistory = () => [
    {
      id: 'exec1',
      name: 'Morning Cleanup',
      status: 'completed',
      startedAt: new Date(Date.now() - 3600000).toISOString(),
      completedAt: new Date(Date.now() - 3500000).toISOString(),
      stepsExecuted: 3,
      totalSteps: 3,
    },
  ];

  beforeEach(async () => {
    const { TauriApiService } = await import('@api/tauri-api.service');
    mockApi = { invoke: vi.fn(), listen: vi.fn() };
    TauriApiService.prototype.api = mockApi;

    injector = Injector.create({
      providers: [{ provide: TauriApiService, useValue: mockApi }],
    });
  });

  it('should have correct initial values', async () => {
    const { AutomationStore } = await import('./automation.store');
    const store = runInInjectionContext(injector, () => new AutomationStore());

    expect(store.quickActions()).toHaveLength(0);
    expect(store.recipes()).toHaveLength(0);
    expect(store.executionHistory()).toHaveLength(0);
    expect(store.loading()).toBe(false);
  });

  it('should call get_quick_actions', async () => {
    const { AutomationStore } = await import('./automation.store');
    const store = runInInjectionContext(injector, () => new AutomationStore());
    const mockActions = createMockQuickActions();
    mockApi.invoke.mockResolvedValue(mockActions);

    const result = await store.loadQuickActions();

    expect(mockApi.invoke).toHaveBeenCalledWith('get_quick_actions');
    expect(result).toHaveLength(1);
  });

  it('should call execute_action', async () => {
    const { AutomationStore } = await import('./automation.store');
    const store = runInInjectionContext(injector, () => new AutomationStore());
    mockApi.invoke.mockResolvedValue('Action executed');

    const result = await store.executeAction('qa1');

    expect(mockApi.invoke).toHaveBeenCalledWith('execute_action', { actionId: 'qa1' });
    expect(result).toBe('Action executed');
  });

  it('should call get_recipes', async () => {
    const { AutomationStore } = await import('./automation.store');
    const store = runInInjectionContext(injector, () => new AutomationStore());
    const mockRecipes = createMockRecipes();
    mockApi.invoke.mockResolvedValue(mockRecipes);

    const result = await store.loadRecipes();

    expect(mockApi.invoke).toHaveBeenCalledWith('get_recipes');
    expect(result).toHaveLength(1);
  });

  it('should call save_recipe', async () => {
    const { AutomationStore } = await import('./automation.store');
    const store = runInInjectionContext(injector, () => new AutomationStore());
    mockApi.invoke.mockResolvedValue('Recipe saved');
    const recipe = createMockRecipes()[0];

    const result = await store.saveRecipe(recipe);

    expect(mockApi.invoke).toHaveBeenCalledWith('save_recipe', { recipe });
    expect(result).toBe('Recipe saved');
  });

  it('should call delete_recipe', async () => {
    const { AutomationStore } = await import('./automation.store');
    const store = runInInjectionContext(injector, () => new AutomationStore());
    mockApi.invoke.mockResolvedValue('Recipe deleted');

    const result = await store.deleteRecipe('rec1');

    expect(mockApi.invoke).toHaveBeenCalledWith('delete_recipe', { recipeId: 'rec1' });
    expect(result).toBe('Recipe deleted');
  });

  it('should call execute_recipe', async () => {
    const { AutomationStore } = await import('./automation.store');
    const store = runInInjectionContext(injector, () => new AutomationStore());
    mockApi.invoke.mockResolvedValue('Recipe executed');

    const result = await store.executeRecipe('rec1');

    expect(mockApi.invoke).toHaveBeenCalledWith('execute_recipe', { recipeId: 'rec1' });
    expect(result).toBe('Recipe executed');
  });

  it('should call get_execution_history', async () => {
    const { AutomationStore } = await import('./automation.store');
    const store = runInInjectionContext(injector, () => new AutomationStore());
    const mockHistory = createMockExecutionHistory();
    mockApi.invoke.mockResolvedValue(mockHistory);

    const result = await store.loadExecutionHistory();

    expect(mockApi.invoke).toHaveBeenCalledWith('get_execution_history');
    expect(result).toHaveLength(1);
  });
});
