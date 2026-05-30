import { describe, it, expect, beforeEach, vi } from 'vitest';
import { Injector, runInInjectionContext } from '@angular/core';

vi.mock('@services/api.service');

describe('AutomationService', () => {
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
    const { ApiService } = await import('@services/api.service');
    mockApi = { invoke: vi.fn() };
    ApiService.prototype.api = mockApi;

    injector = Injector.create({
      providers: [{ provide: ApiService, useValue: mockApi }],
    });
  });

  it('should call get_quick_actions', async () => {
    const { AutomationService } = await import('@services/automation.service');
    const mockActions = createMockQuickActions();
    mockApi.invoke.mockResolvedValue(mockActions);

    const service = runInInjectionContext(injector, () => new AutomationService(mockApi as any));
    const result = await service.getQuickActions();

    expect(mockApi.invoke).toHaveBeenCalledWith('get_quick_actions');
    expect(result).toHaveLength(1);
    expect(result[0].id).toBe('qa1');
  });

  it('should call execute_action', async () => {
    const { AutomationService } = await import('@services/automation.service');
    mockApi.invoke.mockResolvedValue('Action executed');

    const service = runInInjectionContext(injector, () => new AutomationService(mockApi as any));
    const result = await service.executeAction('qa1');

    expect(mockApi.invoke).toHaveBeenCalledWith('execute_action', { actionId: 'qa1' });
    expect(result).toBe('Action executed');
  });

  it('should call get_recipes', async () => {
    const { AutomationService } = await import('@services/automation.service');
    const mockRecipes = createMockRecipes();
    mockApi.invoke.mockResolvedValue(mockRecipes);

    const service = runInInjectionContext(injector, () => new AutomationService(mockApi as any));
    const result = await service.getRecipes();

    expect(mockApi.invoke).toHaveBeenCalledWith('get_recipes');
    expect(result).toHaveLength(1);
    expect(result[0].id).toBe('rec1');
  });

  it('should call save_recipe', async () => {
    const { AutomationService } = await import('@services/automation.service');
    mockApi.invoke.mockResolvedValue('Recipe saved');
    const recipe = createMockRecipes()[0];

    const service = runInInjectionContext(injector, () => new AutomationService(mockApi as any));
    const result = await service.saveRecipe(recipe);

    expect(mockApi.invoke).toHaveBeenCalledWith('save_recipe', { recipe });
    expect(result).toBe('Recipe saved');
  });

  it('should call delete_recipe', async () => {
    const { AutomationService } = await import('@services/automation.service');
    mockApi.invoke.mockResolvedValue('Recipe deleted');

    const service = runInInjectionContext(injector, () => new AutomationService(mockApi as any));
    const result = await service.deleteRecipe('rec1');

    expect(mockApi.invoke).toHaveBeenCalledWith('delete_recipe', { recipeId: 'rec1' });
    expect(result).toBe('Recipe deleted');
  });

  it('should call execute_recipe', async () => {
    const { AutomationService } = await import('@services/automation.service');
    mockApi.invoke.mockResolvedValue('Recipe executed');

    const service = runInInjectionContext(injector, () => new AutomationService(mockApi as any));
    const result = await service.executeRecipe('rec1');

    expect(mockApi.invoke).toHaveBeenCalledWith('execute_recipe', { recipeId: 'rec1' });
    expect(result).toBe('Recipe executed');
  });

  it('should call get_execution_history', async () => {
    const { AutomationService } = await import('@services/automation.service');
    const mockHistory = createMockExecutionHistory();
    mockApi.invoke.mockResolvedValue(mockHistory);

    const service = runInInjectionContext(injector, () => new AutomationService(mockApi as any));
    const result = await service.getExecutionHistory();

    expect(mockApi.invoke).toHaveBeenCalledWith('get_execution_history');
    expect(result).toHaveLength(1);
    expect(result[0].status).toBe('completed');
  });
});
