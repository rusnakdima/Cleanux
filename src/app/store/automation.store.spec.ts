import { describe, it, expect, beforeEach, vi } from 'vitest';
import { Injector, runInInjectionContext } from '@angular/core';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

const mockInvokeWrapperService = { invoke: vi.fn(), listen: vi.fn() };
vi.mock('@tauri-front/shared', () => ({
  InvokeWrapperService: mockInvokeWrapperService,
}));

vi.mock('@app/services/storage-entity.service', () => ({
  StorageEntityService: function() {
    this.findById = vi.fn();
    this.findMany = vi.fn();
    this.create = vi.fn();
    this.update = vi.fn();
    this.patch = vi.fn();
    this.delete = vi.fn();
    this.count = vi.fn();
    this.query = vi.fn();
  },
}));

vi.mock('@app/core/services/storage-query.service', () => ({
  CleanuxQueryService: function() {
    this.query = vi.fn();
    this.queryById = vi.fn();
    this.invalidate = vi.fn();
    this.invalidateAll = vi.fn();
  },
}));

vi.mock('@services/notification.service', () => ({
  NotificationService: function() {
    this.alert = vi.fn();
    this.confirm = vi.fn(() => true);
    this.error = vi.fn();
    this.success = vi.fn();
    this.cleanError = vi.fn();
  },
}));

vi.mock('./log-storage.service', () => ({
  LogStorageService: function() {
    this.init = vi.fn();
    this.addLog = vi.fn();
    this.getLogs = vi.fn();
    this.getStats = vi.fn();
    this.clearLogs = vi.fn();
    this.getErrors = vi.fn();
  },
}));

describe('AutomationStore', () => {
  let injector: Injector;
  let mockApi: { invoke: ReturnType<typeof vi.fn> };

  beforeEach(async () => {
    const { InvokeWrapperService } = await import('@tauri-front/shared');
    mockApi = { invoke: vi.fn(), listen: vi.fn() };
    injector = Injector.create({
      providers: [{ provide: InvokeWrapperService, useValue: mockApi }],
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
    const mockActions = [
      { id: 'qa1', name: 'Quick Clean', description: 'Clean', icon: 'broom', actions: [{ CleanCategory: { category: 'cache' } }] },
    ];
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
    const mockRecipes = [
      { id: 'rec1', name: 'Morning Cleanup', steps: [{ CleanCategory: { category: 'cache' } }], enabled: true, trigger: 'Scheduled' as const },
    ];
    mockApi.invoke.mockResolvedValue(mockRecipes);

    const result = await store.loadRecipes();

    expect(mockApi.invoke).toHaveBeenCalledWith('get_recipes');
    expect(result).toHaveLength(1);
  });

  it('should call save_recipe', async () => {
    const { AutomationStore } = await import('./automation.store');
    const store = runInInjectionContext(injector, () => new AutomationStore());
    mockApi.invoke.mockResolvedValue('Recipe saved');
    const recipe = { id: 'rec1', name: 'Morning Cleanup', steps: [{ CleanCategory: { category: 'cache' } }], enabled: true, trigger: 'Scheduled' as const };

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
    const mockHistory = [
      { id: 'exec1', name: 'Morning Cleanup', status: 'completed', startedAt: new Date(Date.now() - 3600000).toISOString(), completedAt: new Date(Date.now() - 3500000).toISOString(), stepsExecuted: 3, totalSteps: 3 },
    ];
    mockApi.invoke.mockResolvedValue(mockHistory);

    const result = await store.loadExecutionHistory();

    expect(mockApi.invoke).toHaveBeenCalledWith('get_execution_history');
    expect(result).toHaveLength(1);
  });
});
