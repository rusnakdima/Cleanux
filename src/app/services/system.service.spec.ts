import { describe, it, expect, beforeEach, vi } from 'vitest';
import { Injector, runInInjectionContext } from '@angular/core';

vi.mock('@services/api.service');

describe('SystemService', () => {
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
    const { ApiService } = await import('@services/api.service');
    mockApi = { invoke: vi.fn() };
    ApiService.prototype.api = mockApi;

    injector = Injector.create({
      providers: [{ provide: ApiService, useValue: mockApi }],
    });
  });

  it('should call getSystemServices', async () => {
    const { SystemService } = await import('@services/system.service');
    const mockServices = createMockServices();
    mockApi.invoke.mockResolvedValue(mockServices);

    const service = runInInjectionContext(injector, () => new SystemService(mockApi as any));
    const result = await service.getSystemServices();

    expect(mockApi.invoke).toHaveBeenCalledWith('getSystemServices');
    expect(result).toHaveLength(1);
  });

  it('should call getAllServices', async () => {
    const { SystemService } = await import('@services/system.service');
    const mockServices = createMockServices();
    mockApi.invoke.mockResolvedValue(mockServices);

    const service = runInInjectionContext(injector, () => new SystemService(mockApi as any));
    const result = await service.getAllServices();

    expect(mockApi.invoke).toHaveBeenCalledWith('getAllServices');
    expect(result).toHaveLength(1);
  });

  it('should call stopService', async () => {
    const { SystemService } = await import('@services/system.service');
    mockApi.invoke.mockResolvedValue('Service stopped');

    const service = runInInjectionContext(injector, () => new SystemService(mockApi as any));
    const result = await service.stopService('nginx.service');

    expect(mockApi.invoke).toHaveBeenCalledWith('stopService', { service: 'nginx.service' });
    expect(result).toBe('Service stopped');
  });

  it('should call stopSelectedServices', async () => {
    const { SystemService } = await import('@services/system.service');
    mockApi.invoke.mockResolvedValue('Services stopped');

    const service = runInInjectionContext(injector, () => new SystemService(mockApi as any));
    const result = await service.stopSelectedServices(['service1.service', 'service2.service']);

    expect(mockApi.invoke).toHaveBeenCalledWith('stopSelectedServices', {
      services: ['service1.service', 'service2.service'],
    });
    expect(result).toBe('Services stopped');
  });

  it('should call startService', async () => {
    const { SystemService } = await import('@services/system.service');
    mockApi.invoke.mockResolvedValue('Service started');

    const service = runInInjectionContext(injector, () => new SystemService(mockApi as any));
    const result = await service.startService('nginx.service');

    expect(mockApi.invoke).toHaveBeenCalledWith('startService', { service: 'nginx.service' });
    expect(result).toBe('Service started');
  });

  it('should call enableService', async () => {
    const { SystemService } = await import('@services/system.service');
    mockApi.invoke.mockResolvedValue('Service enabled');

    const service = runInInjectionContext(injector, () => new SystemService(mockApi as any));
    const result = await service.enableService('nginx.service');

    expect(mockApi.invoke).toHaveBeenCalledWith('enableService', { service: 'nginx.service' });
    expect(result).toBe('Service enabled');
  });

  it('should call enableSelectedServices', async () => {
    const { SystemService } = await import('@services/system.service');
    mockApi.invoke.mockResolvedValue('Services enabled');

    const service = runInInjectionContext(injector, () => new SystemService(mockApi as any));
    const result = await service.enableSelectedServices(['service1.service']);

    expect(mockApi.invoke).toHaveBeenCalledWith('enableSelectedServices', {
      services: ['service1.service'],
    });
    expect(result).toBe('Services enabled');
  });

  it('should call getProcesses', async () => {
    const { SystemService } = await import('@services/system.service');
    const mockProcesses = createMockProcesses();
    mockApi.invoke.mockResolvedValue(mockProcesses);

    const service = runInInjectionContext(injector, () => new SystemService(mockApi as any));
    const result = await service.getProcesses();

    expect(mockApi.invoke).toHaveBeenCalledWith('getProcesses');
    expect(result).toHaveLength(1);
  });

  it('should call killProcess', async () => {
    const { SystemService } = await import('@services/system.service');
    mockApi.invoke.mockResolvedValue('Process killed');

    const service = runInInjectionContext(injector, () => new SystemService(mockApi as any));
    const result = await service.killProcess(12345);

    expect(mockApi.invoke).toHaveBeenCalledWith('killProcess', { pid: 12345 });
    expect(result).toBe('Process killed');
  });

  it('should call killSelectedProcesses', async () => {
    const { SystemService } = await import('@services/system.service');
    mockApi.invoke.mockResolvedValue('Processes killed');

    const service = runInInjectionContext(injector, () => new SystemService(mockApi as any));
    const result = await service.killSelectedProcesses([12345, 67890]);

    expect(mockApi.invoke).toHaveBeenCalledWith('killSelectedProcesses', { pids: [12345, 67890] });
    expect(result).toBe('Processes killed');
  });
});
