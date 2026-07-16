import { describe, it, expect, beforeEach, vi } from 'vitest';
import { Injector, runInInjectionContext } from '@angular/core';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

const mockInvokeWrapperService = { invoke: vi.fn(), listen: vi.fn() };
vi.mock('@tauri-front/shared', () => ({
  InvokeWrapperService: mockInvokeWrapperService,
}));

describe('HealthHistoryService', () => {
  let injector: Injector;
  let mockApi: { invoke: ReturnType<typeof vi.fn> };

  beforeEach(async () => {
    const { InvokeWrapperService } = await import('@tauri-front/shared');
    mockApi = { invoke: vi.fn(), listen: vi.fn() };

    injector = Injector.create({
      providers: [{ provide: InvokeWrapperService, useValue: mockApi }],
    });
  });

  it('should call save_health_snapshot with correct params', async () => {
    const { HealthHistoryService } = await import('@services/health-history.service');
    const mockResult = { id: 42 };
    mockApi.invoke.mockResolvedValue(mockResult);

    const service = runInInjectionContext(injector, () => new HealthHistoryService());
    const result = await service.saveHealthSnapshot(85, 1024 * 1024, 512 * 1024, 256 * 1024, 5);

    expect(mockApi.invoke).toHaveBeenCalledWith('save_health_snapshot', {
      health_score: 85,
      cache_size: 1024 * 1024,
      trash_size: 512 * 1024,
      log_size: 256 * 1024,
      large_files_count: 5,
    });
    expect(result).toEqual({ id: 42 });
  });

  it('should call get_health_history with days parameter', async () => {
    const { HealthHistoryService } = await import('@services/health-history.service');
    const mockHistory = [
      {
        id: 1,
        timestamp: '2026-07-01T00:00:00Z',
        health_score: 80,
        cache_size: 1024,
        trash_size: 512,
        log_size: 256,
        large_files_count: 3,
      },
      {
        id: 2,
        timestamp: '2026-07-02T00:00:00Z',
        health_score: 85,
        cache_size: 512,
        trash_size: 256,
        log_size: 128,
        large_files_count: 2,
      },
    ];
    mockApi.invoke.mockResolvedValue(mockHistory);

    const service = runInInjectionContext(injector, () => new HealthHistoryService());
    const result = await service.getHealthHistory(7);

    expect(mockApi.invoke).toHaveBeenCalledWith('get_health_history', { days: 7 });
    expect(result).toHaveLength(2);
    expect(result[0].health_score).toBe(80);
  });

  it('should call get_health_trends with days parameter', async () => {
    const { HealthHistoryService } = await import('@services/health-history.service');
    const mockTrend = {
      trend: 'improving',
      change_percent: 12.5,
      days_analyzed: 30,
    };
    mockApi.invoke.mockResolvedValue(mockTrend);

    const service = runInInjectionContext(injector, () => new HealthHistoryService());
    const result = await service.getHealthTrends(30);

    expect(mockApi.invoke).toHaveBeenCalledWith('get_health_trends', { days: 30 });
    expect(result.trend).toBe('improving');
    expect(result.change_percent).toBe(12.5);
  });

  it('should handle get_health_trends declining trend', async () => {
    const { HealthHistoryService } = await import('@services/health-history.service');
    const mockTrend = {
      trend: 'declining',
      change_percent: -8.3,
      days_analyzed: 14,
    };
    mockApi.invoke.mockResolvedValue(mockTrend);

    const service = runInInjectionContext(injector, () => new HealthHistoryService());
    const result = await service.getHealthTrends(14);

    expect(result.trend).toBe('declining');
    expect(result.change_percent).toBe(-8.3);
  });
});
