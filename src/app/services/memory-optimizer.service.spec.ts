import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';
import { Injector, runInInjectionContext } from '@angular/core';

vi.mock('@services/api.service');

describe('MemoryOptimizerService', () => {
  let injector: Injector;
  let mockApi: { invoke: ReturnType<typeof vi.fn> };

  beforeEach(async () => {
    const { ApiService } = await import('@services/api.service');
    mockApi = { invoke: vi.fn() };
    ApiService.prototype.api = mockApi;

    injector = Injector.create({
      providers: [{ provide: ApiService, useValue: mockApi }],
    });
  });

  it('should call get_memory_info', async () => {
    const { MemoryOptimizerService } = await import('@services/memory-optimizer.service');

    const service = runInInjectionContext(injector, () => new MemoryOptimizerService());
    const mockInfo = {
      total: 16 * 1024 * 1024 * 1024,
      used: 8 * 1024 * 1024 * 1024,
      available: 8 * 1024 * 1024 * 1024,
      cached: 4 * 1024 * 1024 * 1024,
      buffers: 1 * 1024 * 1024 * 1024,
    };
    mockApi.invoke.mockResolvedValue(mockInfo);

    const result = await service.getMemoryInfo();

    expect(mockApi.invoke).toHaveBeenCalledWith('get_memory_info');
    expect(result.total).toBe(16 * 1024 * 1024 * 1024);
  });

  it('should call get_swap_info', async () => {
    const { MemoryOptimizerService } = await import('@services/memory-optimizer.service');

    const service = runInInjectionContext(injector, () => new MemoryOptimizerService());
    const mockInfo = { total: 8 * 1024 * 1024 * 1024, used: 2 * 1024 * 1024 * 1024 };
    mockApi.invoke.mockResolvedValue(mockInfo);

    const result = await service.getSwapInfo();

    expect(mockApi.invoke).toHaveBeenCalledWith('get_swap_info');
    expect(result.total).toBe(8 * 1024 * 1024 * 1024);
  });

  it('should call get_process_memory', async () => {
    const { MemoryOptimizerService } = await import('@services/memory-optimizer.service');

    const service = runInInjectionContext(injector, () => new MemoryOptimizerService());
    const mockProcesses = [
      { pid: 1234, name: 'chrome', memory_mb: 100, cpu_percent: 5.2 },
      { pid: 5678, name: 'firefox', memory_mb: 200, cpu_percent: 3.1 },
    ];
    mockApi.invoke.mockResolvedValue(mockProcesses);

    const result = await service.getProcessMemory();

    expect(mockApi.invoke).toHaveBeenCalledWith('get_process_memory');
    expect(result).toHaveLength(2);
  });

  it('should call optimize_memory', async () => {
    const { MemoryOptimizerService } = await import('@services/memory-optimizer.service');

    const service = runInInjectionContext(injector, () => new MemoryOptimizerService());
    mockApi.invoke.mockResolvedValue(true);

    const result = await service.optimizeMemory();

    expect(mockApi.invoke).toHaveBeenCalledWith('optimize_memory');
    expect(result).toBe(true);
  });
});
