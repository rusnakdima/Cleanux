import { describe, it, expect, beforeEach, vi } from 'vitest';
import { Injector, runInInjectionContext } from '@angular/core';

vi.mock('@services/api.service');

describe('PackageManagerService', () => {
  let injector: Injector;
  let mockApi: { invoke: ReturnType<typeof vi.fn> };

  beforeEach(async () => {
    const { InvokeWrapperService } = await import('@tauri-front/shared');
    mockApi = { invoke: vi.fn(), listen: vi.fn() };

    injector = Injector.create({
      providers: [{ provide: InvokeWrapperService, useValue: mockApi }],
    });
  });

  it('should call get_package_cache_info and update cacheInfo signal', async () => {
    const { PackageManagerService } = await import('@services/package-manager.service');
    const mockResponse = {
      status: 'success',
      message: 'OK',
      data: [
        { name: 'npm', cachePath: '/home/.npm', size: 1024 * 1024, description: 'NPM cache' },
        { name: 'yarn', cachePath: '/home/.yarn', size: 512 * 1024, description: 'Yarn cache' },
      ],
    };
    mockApi.invoke.mockResolvedValue(mockResponse);

    const service = runInInjectionContext(injector, () => new PackageManagerService());
    const result = await service.getPackageCacheInfo();

    expect(mockApi.invoke).toHaveBeenCalledWith('get_package_cache_info');
    expect(result).toHaveLength(2);
    expect(result[0].name).toBe('npm');
    expect(service.cacheInfo()).toHaveLength(2);
  });

  it('should set loading signal during getPackageCacheInfo', async () => {
    const { PackageManagerService } = await import('@services/package-manager.service');
    const mockResponse = { status: 'success', message: 'OK', data: [] };
    mockApi.invoke.mockResolvedValue(mockResponse);

    const service = runInInjectionContext(injector, () => new PackageManagerService());

    let loadingDuringCall = false;
    service.loading.subscribe((v) => {
      if (v) loadingDuringCall = true;
    });

    await service.getPackageCacheInfo();

    expect(loadingDuringCall).toBe(true);
    expect(service.loading()).toBe(false);
  });

  it('should call clean_package_cache with manager name', async () => {
    const { PackageManagerService } = await import('@services/package-manager.service');
    const mockResponse = { status: 'success', message: 'Cleaned npm cache' };
    const mockCacheInfo = {
      status: 'success',
      message: 'OK',
      data: [{ name: 'npm', cachePath: '/npm', size: 100, description: 'desc' }],
    };
    mockApi.invoke.mockResolvedValueOnce(mockCacheInfo).mockResolvedValueOnce(mockResponse);

    const service = runInInjectionContext(injector, () => new PackageManagerService());
    await service.getPackageCacheInfo();
    const result = await service.cleanPackageCache('npm');

    expect(mockApi.invoke).toHaveBeenCalledWith('clean_package_cache', { manager: 'npm' });
    expect(result).toBe('Cleaned npm cache');
  });

  it('should throw error when clean_package_cache fails', async () => {
    const { PackageManagerService } = await import('@services/package-manager.service');
    const mockCacheInfo = {
      status: 'success',
      message: 'OK',
      data: [{ name: 'npm', cachePath: '/npm', size: 100, description: 'desc' }],
    };
    mockApi.invoke
      .mockResolvedValueOnce(mockCacheInfo)
      .mockRejectedValueOnce(new Error('Permission denied'));

    const service = runInInjectionContext(injector, () => new PackageManagerService());
    await service.getPackageCacheInfo();

    await expect(service.cleanPackageCache('npm')).rejects.toThrow('Permission denied');
  });

  it('should cleanAllPackageCaches for all cached managers', async () => {
    const { PackageManagerService } = await import('@services/package-manager.service');
    const mockCacheInfo = {
      status: 'success',
      message: 'OK',
      data: [
        { name: 'npm', cachePath: '/npm', size: 100, description: 'desc' },
        { name: 'yarn', cachePath: '/yarn', size: 100, description: 'desc' },
      ],
    };
    const mockCleanResponse = { status: 'success', message: 'Cleaned' };
    mockApi.invoke
      .mockResolvedValueOnce(mockCacheInfo)
      .mockResolvedValueOnce(mockCleanResponse)
      .mockResolvedValueOnce(mockCleanResponse);

    const service = runInInjectionContext(injector, () => new PackageManagerService());
    await service.getPackageCacheInfo();
    const results = await service.cleanAllPackageCaches();

    expect(results).toHaveLength(2);
    expect(results[0]).toContain('npm');
    expect(results[1]).toContain('yarn');
  });
});
