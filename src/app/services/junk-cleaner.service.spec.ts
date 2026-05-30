import { describe, it, expect, beforeEach, vi } from 'vitest';
import { Injector, runInInjectionContext } from '@angular/core';

vi.mock('@services/api.service');

describe('JunkCleanerService', () => {
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

  it('should call get_junk_summary', async () => {
    const { JunkCleanerService } = await import('@services/junk-cleaner.service');
    const mockSummary = [
      { category: 'browser_cache', size: 1024, count: 10 },
      { category: 'thumbnail_cache', size: 512, count: 5 },
    ];
    mockApi.invoke.mockResolvedValue(mockSummary);

    const service = runInInjectionContext(injector, () => new JunkCleanerService(mockApi as any));
    const result = await service.getJunkSummary();

    expect(mockApi.invoke).toHaveBeenCalledWith('get_junk_summary');
    expect(result).toEqual(mockSummary);
  });

  it('should call scan_browser_caches', async () => {
    const { JunkCleanerService } = await import('@services/junk-cleaner.service');
    const mockItems = Array.from({ length: 3 }, (_, i) => ({
      path: `/cache/file${i}.tmp`,
      size: (i + 1) * 1024,
      modified: new Date().toISOString(),
    }));
    mockApi.invoke.mockResolvedValue(mockItems);

    const service = runInInjectionContext(injector, () => new JunkCleanerService(mockApi as any));
    const result = await service.scanBrowserCaches();

    expect(mockApi.invoke).toHaveBeenCalledWith('scan_browser_caches');
    expect(result).toHaveLength(3);
  });

  it('should call scan_thumbnail_caches', async () => {
    const { JunkCleanerService } = await import('@services/junk-cleaner.service');
    const mockItems = [{ path: '/thumb/1.jpg', size: 512, modified: new Date().toISOString() }];
    mockApi.invoke.mockResolvedValue(mockItems);

    const service = runInInjectionContext(injector, () => new JunkCleanerService(mockApi as any));
    const result = await service.scanThumbnailCaches();

    expect(mockApi.invoke).toHaveBeenCalledWith('scan_thumbnail_caches');
    expect(result).toHaveLength(1);
  });

  it('should call scan_application_caches', async () => {
    const { JunkCleanerService } = await import('@services/junk-cleaner.service');
    const mockItems = [{ path: '/app/cache1', size: 1024, modified: new Date().toISOString() }];
    mockApi.invoke.mockResolvedValue(mockItems);

    const service = runInInjectionContext(injector, () => new JunkCleanerService(mockApi as any));
    const result = await service.scanApplicationCaches();

    expect(mockApi.invoke).toHaveBeenCalledWith('scan_application_caches');
    expect(result).toHaveLength(1);
  });

  it('should call scan_system_temp', async () => {
    const { JunkCleanerService } = await import('@services/junk-cleaner.service');
    const mockItems = [{ path: '/tmp/test', size: 1024, modified: new Date().toISOString() }];
    mockApi.invoke.mockResolvedValue(mockItems);

    const service = runInInjectionContext(injector, () => new JunkCleanerService(mockApi as any));
    const result = await service.scanSystemTemp();

    expect(mockApi.invoke).toHaveBeenCalledWith('scan_system_temp');
    expect(result).toHaveLength(1);
  });

  it('should call scan_log_rotations', async () => {
    const { JunkCleanerService } = await import('@services/junk-cleaner.service');
    const mockItems = [
      { path: '/var/log/syslog.1', size: 1024, modified: new Date().toISOString() },
    ];
    mockApi.invoke.mockResolvedValue(mockItems);

    const service = runInInjectionContext(injector, () => new JunkCleanerService(mockApi as any));
    const result = await service.scanLogRotations();

    expect(mockApi.invoke).toHaveBeenCalledWith('scan_log_rotations');
    expect(result).toHaveLength(1);
  });

  it('should call clean_junk_category', async () => {
    const { JunkCleanerService } = await import('@services/junk-cleaner.service');
    mockApi.invoke.mockResolvedValue('Cleaned 10 files');

    const service = runInInjectionContext(injector, () => new JunkCleanerService(mockApi as any));
    const result = await service.cleanJunkCategory('browser_cache');

    expect(mockApi.invoke).toHaveBeenCalledWith('clean_junk_category', {
      category: 'browser_cache',
    });
    expect(result).toBe('Cleaned 10 files');
  });
});
