import { describe, it, expect, beforeEach, vi } from 'vitest';
import { Injector, runInInjectionContext } from '@angular/core';

vi.mock('@api/tauri-api.service');

describe('SystemStore', () => {
  let injector: Injector;
  let mockApi: { invoke: ReturnType<typeof vi.fn> };

  const createMockServices = () => [
    {
      name: 'service1.service',
      description: 'Test service 1',
      load: 'loaded',
      active: 'active',
      status: 'running',
      isRunning: true,
    },
  ];

  const createMockProcesses = () => [
    { pid: 1100, name: 'process_1', cpu_usage: 10.5, memory_usage: 25.3 },
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
    const { SystemStore } = await import('./system.store');
    const store = runInInjectionContext(injector, () => new SystemStore());

    expect(store.services()).toHaveLength(0);
    expect(store.processes()).toHaveLength(0);
    expect(store.loading()).toBe(false);
    expect(store.selectedServices()).toEqual(new Set());
    expect(store.selectedProcesses()).toEqual(new Set());
  });

  it('should call getSystemServices', async () => {
    const { SystemStore } = await import('./system.store');
    const store = runInInjectionContext(injector, () => new SystemStore());
    const mockServices = createMockServices();
    mockApi.invoke.mockResolvedValue(mockServices);

    const result = await store.loadSystemServices();

    expect(mockApi.invoke).toHaveBeenCalledWith('getSystemServices');
    expect(result).toHaveLength(1);
  });

  it('should call getAllServices', async () => {
    const { SystemStore } = await import('./system.store');
    const store = runInInjectionContext(injector, () => new SystemStore());
    const mockServices = createMockServices();
    mockApi.invoke.mockResolvedValue(mockServices);

    const result = await store.loadAllServices();

    expect(mockApi.invoke).toHaveBeenCalledWith('getAllServices');
    expect(result).toHaveLength(1);
  });

  it('should call stopService', async () => {
    const { SystemStore } = await import('./system.store');
    const store = runInInjectionContext(injector, () => new SystemStore());
    mockApi.invoke.mockResolvedValue('Service stopped');

    const result = await store.stopService('nginx.service');

    expect(mockApi.invoke).toHaveBeenCalledWith('stopService', { service: 'nginx.service' });
    expect(result).toBe('Service stopped');
  });

  it('should call stopSelectedServices', async () => {
    const { SystemStore } = await import('./system.store');
    const store = runInInjectionContext(injector, () => new SystemStore());
    mockApi.invoke.mockResolvedValue('Services stopped');

    const result = await store.stopSelectedServices(['service1.service', 'service2.service']);

    expect(mockApi.invoke).toHaveBeenCalledWith('stopSelectedServices', {
      services: ['service1.service', 'service2.service'],
    });
    expect(result).toBe('Services stopped');
  });

  it('should call startService', async () => {
    const { SystemStore } = await import('./system.store');
    const store = runInInjectionContext(injector, () => new SystemStore());
    mockApi.invoke.mockResolvedValue('Service started');

    const result = await store.startService('nginx.service');

    expect(mockApi.invoke).toHaveBeenCalledWith('startService', { service: 'nginx.service' });
    expect(result).toBe('Service started');
  });

  it('should call enableService', async () => {
    const { SystemStore } = await import('./system.store');
    const store = runInInjectionContext(injector, () => new SystemStore());
    mockApi.invoke.mockResolvedValue('Service enabled');

    const result = await store.enableService('nginx.service');

    expect(mockApi.invoke).toHaveBeenCalledWith('enableService', { service: 'nginx.service' });
    expect(result).toBe('Service enabled');
  });

  it('should call enableSelectedServices', async () => {
    const { SystemStore } = await import('./system.store');
    const store = runInInjectionContext(injector, () => new SystemStore());
    mockApi.invoke.mockResolvedValue('Services enabled');

    const result = await store.enableSelectedServices(['service1.service']);

    expect(mockApi.invoke).toHaveBeenCalledWith('enableSelectedServices', {
      services: ['service1.service'],
    });
    expect(result).toBe('Services enabled');
  });

  it('should call getProcesses', async () => {
    const { SystemStore } = await import('./system.store');
    const store = runInInjectionContext(injector, () => new SystemStore());
    const mockProcesses = createMockProcesses();
    mockApi.invoke.mockResolvedValue(mockProcesses);

    const result = await store.getProcesses();

    expect(mockApi.invoke).toHaveBeenCalledWith('getProcesses');
    expect(result).toHaveLength(1);
  });

  it('should call killProcess', async () => {
    const { SystemStore } = await import('./system.store');
    const store = runInInjectionContext(injector, () => new SystemStore());
    mockApi.invoke.mockResolvedValue('Process killed');

    const result = await store.killProcess(12345);

    expect(mockApi.invoke).toHaveBeenCalledWith('killProcess', { pid: 12345 });
    expect(result).toBe('Process killed');
  });

  it('should call killSelectedProcesses', async () => {
    const { SystemStore } = await import('./system.store');
    const store = runInInjectionContext(injector, () => new SystemStore());
    mockApi.invoke.mockResolvedValue('Processes killed');

    const result = await store.killSelectedProcesses([12345, 67890]);

    expect(mockApi.invoke).toHaveBeenCalledWith('killSelectedProcesses', { pids: [12345, 67890] });
    expect(result).toBe('Processes killed');
  });
});
