import { describe, it, expect, beforeEach, vi } from 'vitest';
import { Injector, runInInjectionContext } from '@angular/core';

vi.mock('@services/api.service');

describe('FileService', () => {
  let injector: Injector;
  let mockApi: { invoke: ReturnType<typeof vi.fn> };

  const createMockPaginatedData = <T>(items: T[], total: number, hasMore = false) => ({
    data: items,
    has_more: hasMore,
    total,
  });

  beforeEach(async () => {
    const { ApiService } = await import('@services/api.service');
    mockApi = { invoke: vi.fn() };
    ApiService.prototype.api = mockApi;

    injector = Injector.create({
      providers: [{ provide: ApiService, useValue: mockApi }],
    });
  });

  it('should call getCacheFiles with pagination', async () => {
    const { FileService } = await import('@services/file.service');
    const mockFiles = Array.from({ length: 5 }, (_, i) => ({
      path: `/cache/file${i}.tmp`,
      size: (i + 1) * 1024,
      modified: new Date().toISOString(),
    }));
    const mockData = createMockPaginatedData(mockFiles, 10, true);
    mockApi.invoke.mockResolvedValue(mockData);

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    const result = await service.getCacheFiles(5, 0);

    expect(mockApi.invoke).toHaveBeenCalledWith('getCacheFiles', {
      limit: 5,
      offset: 0,
      signal: expect.any(AbortSignal),
    });
    expect(result.data).toHaveLength(5);
    expect(result.has_more).toBe(true);
    expect(result.total).toBe(10);
  });

  it('should call getTrashFiles', async () => {
    const { FileService } = await import('@services/file.service');
    const mockFiles = [
      {
        name: 'trash1.bin',
        path: '/trash/trash1.bin',
        size: 1024,
        deletedDate: new Date().toISOString(),
      },
    ];
    mockApi.invoke.mockResolvedValue(mockFiles);

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    const result = await service.getTrashFiles(10, 0);

    expect(mockApi.invoke).toHaveBeenCalledWith('getTrashFiles', { limit: 10, offset: 0 });
    expect(result).toHaveLength(1);
  });

  it('should call getSystemLogs', async () => {
    const { FileService } = await import('@services/file.service');
    const mockLogs = [{ path: '/var/log/syslog', size: 1024, modified: new Date().toISOString() }];
    mockApi.invoke.mockResolvedValue(mockLogs);

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    const result = await service.getSystemLogs();

    expect(mockApi.invoke).toHaveBeenCalledWith('getSystemLogs', { limit: null, offset: null });
    expect(result).toHaveLength(1);
  });

  it('should call getLargeFiles', async () => {
    const { FileService } = await import('@services/file.service');
    const mockFiles = [
      {
        name: 'video.mp4',
        path: '/home/video.mp4',
        size: 1024 * 1024 * 1024,
        modified: new Date().toISOString(),
      },
    ];
    const mockData = createMockPaginatedData(mockFiles, 3, false);
    mockApi.invoke.mockResolvedValue(mockData);

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    const result = await service.getLargeFiles(3, 0);

    expect(mockApi.invoke).toHaveBeenCalledWith('getLargeFiles', {
      limit: 3,
      offset: 0,
      signal: expect.any(AbortSignal),
    });
    expect(result.data).toHaveLength(1);
  });

  it('should call getCacheSummary', async () => {
    const { FileService } = await import('@services/file.service');
    const mockSummary = { totalSize: 1024 * 1024, fileCount: 42 };
    mockApi.invoke.mockResolvedValue(mockSummary);

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    const result = await service.getCacheSummary();

    expect(mockApi.invoke).toHaveBeenCalledWith('getCacheSummary');
    expect(result).toEqual(mockSummary);
  });

  it('should call getTrashSummary', async () => {
    const { FileService } = await import('@services/file.service');
    const mockSummary = { totalSize: 512 * 1024, fileCount: 15 };
    mockApi.invoke.mockResolvedValue(mockSummary);

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    const result = await service.getTrashSummary();

    expect(mockApi.invoke).toHaveBeenCalledWith('getTrashSummary');
    expect(result).toEqual(mockSummary);
  });

  it('should call getLogSummary', async () => {
    const { FileService } = await import('@services/file.service');
    const mockSummary = { totalSize: 256 * 1024, fileCount: 8 };
    mockApi.invoke.mockResolvedValue(mockSummary);

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    const result = await service.getLogSummary();

    expect(mockApi.invoke).toHaveBeenCalledWith('getLogSummary');
    expect(result).toEqual(mockSummary);
  });

  it('should call getLargeFilesSummary', async () => {
    const { FileService } = await import('@services/file.service');
    const mockSummary = { totalSize: 5 * 1024 * 1024 * 1024, fileCount: 3 };
    mockApi.invoke.mockResolvedValue(mockSummary);

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    const result = await service.getLargeFilesSummary();

    expect(mockApi.invoke).toHaveBeenCalledWith('getLargeFilesSummary');
    expect(result).toEqual(mockSummary);
  });

  it('should call clearSelectedCacheFiles', async () => {
    const { FileService } = await import('@services/file.service');
    mockApi.invoke.mockResolvedValue('Cleared 5 files');

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    const result = await service.clearSelectedCacheFiles(['/cache/file1.tmp']);

    expect(mockApi.invoke).toHaveBeenCalledWith('clearSelectedCacheFiles', {
      paths: ['/cache/file1.tmp'],
    });
    expect(result).toBe('Cleared 5 files');
  });

  it('should call clearTrash', async () => {
    const { FileService } = await import('@services/file.service');
    mockApi.invoke.mockResolvedValue('Trash cleared');

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    const result = await service.clearTrash();

    expect(mockApi.invoke).toHaveBeenCalledWith('clearTrash');
    expect(result).toBe('Trash cleared');
  });

  it('should call clearCache', async () => {
    const { FileService } = await import('@services/file.service');
    mockApi.invoke.mockResolvedValue('Cache cleared');

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    const result = await service.clearCache();

    expect(mockApi.invoke).toHaveBeenCalledWith('clearCache');
    expect(result).toBe('Cache cleared');
  });

  it('should call clearAllLogs', async () => {
    const { FileService } = await import('@services/file.service');
    mockApi.invoke.mockResolvedValue('Logs cleared');

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    const result = await service.clearAllLogs();

    expect(mockApi.invoke).toHaveBeenCalledWith('clearAllLogs');
    expect(result).toBe('Logs cleared');
  });

  it('should call clearAllLargeFiles', async () => {
    const { FileService } = await import('@services/file.service');
    mockApi.invoke.mockResolvedValue('Large files cleared');

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    const result = await service.clearAllLargeFiles();

    expect(mockApi.invoke).toHaveBeenCalledWith('clearAllLargeFiles');
    expect(result).toBe('Large files cleared');
  });

  it('should call previewFile', async () => {
    const { FileService } = await import('@services/file.service');
    const mockResult = { name: 'test.txt', path: '/tmp/test.txt', type: 'text' };
    mockApi.invoke.mockResolvedValue(mockResult);

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    const result = await service.previewFile('/tmp/test.txt');

    expect(mockApi.invoke).toHaveBeenCalledWith('previewFile', { path: '/tmp/test.txt' });
    expect(result).toEqual(mockResult);
  });

  it('should call openFile', async () => {
    const { FileService } = await import('@services/file.service');
    mockApi.invoke.mockResolvedValue(undefined);

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    await service.openFile('/tmp/test.txt', 'xdg-open');

    expect(mockApi.invoke).toHaveBeenCalledWith('openFile', {
      path: '/tmp/test.txt',
      command: 'xdg-open',
    });
  });

  it('should call deleteFiles', async () => {
    const { FileService } = await import('@services/file.service');
    mockApi.invoke.mockResolvedValue('Deleted 3 files');

    const service = runInInjectionContext(injector, () => new FileService(mockApi as any));
    const result = await service.deleteFiles(['/tmp/file1', '/tmp/file2']);

    expect(mockApi.invoke).toHaveBeenCalledWith('deleteFiles', {
      paths: ['/tmp/file1', '/tmp/file2'],
    });
    expect(result).toBe('Deleted 3 files');
  });
});
