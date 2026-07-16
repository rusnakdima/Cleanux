import { describe, it, expect, beforeEach, vi } from 'vitest';
import { Injector, runInInjectionContext } from '@angular/core';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

const mockInvokeWrapperService = { invoke: vi.fn(), listen: vi.fn() };
vi.mock('@tauri-front/shared', () => ({
  InvokeWrapperService: mockInvokeWrapperService,
}));

describe('SchedulerService', () => {
  let injector: Injector;
  let mockApi: { invoke: ReturnType<typeof vi.fn> };

  beforeEach(async () => {
    const { InvokeWrapperService } = await import('@tauri-front/shared');
    mockApi = { invoke: vi.fn(), listen: vi.fn() };

    injector = Injector.create({
      providers: [{ provide: InvokeWrapperService, useValue: mockApi }],
    });
  });

  it('should call get_schedule_config and return config', async () => {
    const { SchedulerService } = await import('@services/scheduler.service');
    const mockConfig = {
      enabled: true,
      interval_hours: 24,
      cleaning_type: 'all',
      paths: [],
      last_run: '2026-07-01T00:00:00Z',
      next_run: '2026-07-02T00:00:00Z',
    };
    mockApi.invoke.mockResolvedValue(mockConfig);

    const service = runInInjectionContext(injector, () => new SchedulerService());
    const result = await service.getScheduleConfig();

    expect(mockApi.invoke).toHaveBeenCalledWith('get_schedule_config');
    expect(result).toEqual(mockConfig);
  });

  it('should return null when get_schedule_config throws', async () => {
    const { SchedulerService } = await import('@services/scheduler.service');
    mockApi.invoke.mockRejectedValue(new Error('Not found'));

    const service = runInInjectionContext(injector, () => new SchedulerService());
    const result = await service.getScheduleConfig();

    expect(result).toBeNull();
  });

  it('should call save_schedule_config with config', async () => {
    const { SchedulerService } = await import('@services/scheduler.service');
    const config = {
      enabled: true,
      interval_hours: 12,
      cleaning_type: 'cache',
      paths: ['/tmp'],
      last_run: null,
      next_run: null,
    };
    mockApi.invoke.mockResolvedValue(undefined);

    const service = runInInjectionContext(injector, () => new SchedulerService());
    await service.saveScheduleConfig(config);

    expect(mockApi.invoke).toHaveBeenCalledWith('save_schedule_config', { config });
  });

  it('should call delete_schedule_config', async () => {
    const { SchedulerService } = await import('@services/scheduler.service');
    mockApi.invoke.mockResolvedValue(undefined);

    const service = runInInjectionContext(injector, () => new SchedulerService());
    await service.deleteScheduleConfig();

    expect(mockApi.invoke).toHaveBeenCalledWith('delete_schedule_config');
  });

  it('should call run_cleaning_now with cleaningType', async () => {
    const { SchedulerService } = await import('@services/scheduler.service');
    mockApi.invoke.mockResolvedValue(undefined);

    const service = runInInjectionContext(injector, () => new SchedulerService());
    await service.runCleaningNow('cache');

    expect(mockApi.invoke).toHaveBeenCalledWith('run_cleaning_now', { cleaningType: 'cache' });
  });

  it('should call run_cleaning_now with trash cleaningType', async () => {
    const { SchedulerService } = await import('@services/scheduler.service');
    mockApi.invoke.mockResolvedValue(undefined);

    const service = runInInjectionContext(injector, () => new SchedulerService());
    await service.runCleaningNow('trash');

    expect(mockApi.invoke).toHaveBeenCalledWith('run_cleaning_now', { cleaningType: 'trash' });
  });
});
