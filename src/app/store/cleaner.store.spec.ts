import { describe, it, expect, beforeEach, vi } from 'vitest';
import { Injector, runInInjectionContext } from '@angular/core';

vi.mock('@api/tauri-api.service');
vi.mock('@services/file.service');

describe('CleanerStore', () => {
  let injector: Injector;
  let mockApi: { invoke: ReturnType<typeof vi.fn> };
  let mockFileService: {
    getCacheFiles: ReturnType<typeof vi.fn>;
    getLargeFiles: ReturnType<typeof vi.fn>;
  };

  const createMockPaginatedData = <T>(items: T[], total: number, hasMore = false) => ({
    data: items,
    has_more: hasMore,
    total,
  });

  const createMockCacheFiles = (count = 5) =>
    Array.from({ length: count }, (_, i) => ({
      path: `/cache/file${i}.tmp`,
      size: (i + 1) * 1024,
      modified: new Date().toISOString(),
    }));

  beforeEach(async () => {
    const { TauriApiService } = await import('@api/tauri-api.service');
    const { FileService } = await import('@services/file.service');

    mockApi = { invoke: vi.fn(), listen: vi.fn().mockResolvedValue(() => {}) };
    mockFileService = {
      getCacheFiles: vi.fn(),
      getLargeFiles: vi.fn(),
    };

    TauriApiService.prototype.api = mockApi;
    FileService.prototype.api = mockFileService;

    injector = Injector.create({
      providers: [
        { provide: TauriApiService, useValue: mockApi },
        { provide: FileService, useValue: mockFileService },
      ],
    });
  });

  it('should have correct initial values', async () => {
    const { CleanerStore } = await import('./cleaner.store');
    const store = runInInjectionContext(
      injector,
      () => new CleanerStore(mockApi as any, mockFileService as any)
    );

    expect(store.cacheData()).toHaveLength(0);
    expect(store.trashData()).toHaveLength(0);
    expect(store.logData()).toHaveLength(0);
    expect(store.activeTab()).toBe('cache');
    expect(store.loading()).toBe(false);
    expect(store.cacheSize()).toBe(0);
    expect(store.trashSize()).toBe(0);
    expect(store.totalJunk()).toBe(0);
  });

  it('should update activeTab', async () => {
    const { CleanerStore } = await import('./cleaner.store');
    const store = runInInjectionContext(
      injector,
      () => new CleanerStore(mockApi as any, mockFileService as any)
    );

    store.setActiveTab('trash' as any);
    expect(store.activeTab()).toBe('trash');

    store.setActiveTab('logs' as any);
    expect(store.activeTab()).toBe('logs');
  });

  it('should load cache files and update state', async () => {
    const { CleanerStore } = await import('./cleaner.store');
    const store = runInInjectionContext(
      injector,
      () => new CleanerStore(mockApi as any, mockFileService as any)
    );
    const mockFiles = createMockCacheFiles(5);
    const mockPaginatedData = createMockPaginatedData(mockFiles, 5, false);
    mockFileService.getCacheFiles.mockResolvedValue(mockPaginatedData);

    const result = await store.loadCacheFiles(5, 0, true);

    expect(mockFileService.getCacheFiles).toHaveBeenCalledWith(5, 0);
    expect(store.cacheData()).toHaveLength(5);
    expect(store.cacheTotal()).toBe(5);
    expect(store.cacheHasMore()).toBe(false);
    expect(result.data).toHaveLength(5);
  });

  it('should append data when reset is false', async () => {
    const { CleanerStore } = await import('./cleaner.store');
    const store = runInInjectionContext(
      injector,
      () => new CleanerStore(mockApi as any, mockFileService as any)
    );
    const mockFiles1 = createMockCacheFiles(3);
    const mockFiles2 = createMockCacheFiles(2);
    mockFileService.getCacheFiles
      .mockResolvedValueOnce(createMockPaginatedData(mockFiles1, 5, true))
      .mockResolvedValueOnce(createMockPaginatedData(mockFiles2, 5, false));

    await store.loadCacheFiles(3, 0, true);
    await store.loadCacheFiles(2, 3, false);

    expect(store.cacheData()).toHaveLength(5);
  });

  it('should return early if already loading', async () => {
    const { CleanerStore } = await import('./cleaner.store');
    const store = runInInjectionContext(
      injector,
      () => new CleanerStore(mockApi as any, mockFileService as any)
    );
    store.cacheLoading.set(true);

    const result = await store.loadCacheFiles(5, 0, true);

    expect(result.data).toHaveLength(0);
    expect(mockFileService.getCacheFiles).not.toHaveBeenCalled();
  });

  it('should load large files', async () => {
    const { CleanerStore } = await import('./cleaner.store');
    const store = runInInjectionContext(
      injector,
      () => new CleanerStore(mockApi as any, mockFileService as any)
    );
    const mockFiles = [
      {
        name: 'video.mp4',
        path: '/home/video.mp4',
        size: 1024 * 1024 * 1024,
        modified: new Date().toISOString(),
      },
    ];
    mockFileService.getLargeFiles.mockResolvedValue(createMockPaginatedData(mockFiles, 1, false));

    const result = await store.loadLargeFiles(1, 0, true);

    expect(mockFileService.getLargeFiles).toHaveBeenCalledWith(1, 0);
    expect(store.largeFilesData()).toHaveLength(1);
    expect(store.largeFilesTotal()).toBe(1);
  });

  it('should calculate cacheSize correctly', async () => {
    const { CleanerStore } = await import('./cleaner.store');
    const store = runInInjectionContext(
      injector,
      () => new CleanerStore(mockApi as any, mockFileService as any)
    );
    const mockFiles = createMockCacheFiles(3);
    mockFileService.getCacheFiles.mockResolvedValue(createMockPaginatedData(mockFiles, 3, false));

    await store.loadCacheFiles(3, 0, true);

    const expectedSize = mockFiles.reduce((sum, f) => sum + f.size, 0);
    expect(store.cacheSize()).toBe(expectedSize);
  });

  it('should call getTrashFiles via api', async () => {
    const { CleanerStore } = await import('./cleaner.store');
    const store = runInInjectionContext(
      injector,
      () => new CleanerStore(mockApi as any, mockFileService as any)
    );
    const mockFiles = [
      {
        name: 'trash1.bin',
        path: '/trash/trash1.bin',
        size: 1024,
        deletedDate: new Date().toISOString(),
      },
    ];
    mockApi.invoke.mockResolvedValue(mockFiles);

    const result = await store.loadTrashFiles(10, 0);

    expect(mockApi.invoke).toHaveBeenCalledWith('getTrashFiles', { limit: 10, offset: 0 });
    expect(result).toHaveLength(1);
  });

  it('should call getCacheSummary via api', async () => {
    const { CleanerStore } = await import('./cleaner.store');
    const store = runInInjectionContext(
      injector,
      () => new CleanerStore(mockApi as any, mockFileService as any)
    );
    const mockSummary = { totalSize: 1024 * 1024, fileCount: 42 };
    mockApi.invoke.mockResolvedValue(mockSummary);

    const result = await store.getCacheSummary();

    expect(mockApi.invoke).toHaveBeenCalledWith('getCacheSummary');
    expect(result).toEqual(mockSummary);
  });

  it('should call clearSelectedCacheFiles via api', async () => {
    const { CleanerStore } = await import('./cleaner.store');
    const store = runInInjectionContext(
      injector,
      () => new CleanerStore(mockApi as any, mockFileService as any)
    );
    mockApi.invoke.mockResolvedValue('Cleared 5 files');

    const result = await store.clearSelectedCacheFiles(['/cache/file1.tmp']);

    expect(mockApi.invoke).toHaveBeenCalledWith('clearSelectedCacheFiles', {
      paths: ['/cache/file1.tmp'],
    });
    expect(result).toBe('Cleared 5 files');
  });

  it('should call clearTrash via api', async () => {
    const { CleanerStore } = await import('./cleaner.store');
    const store = runInInjectionContext(
      injector,
      () => new CleanerStore(mockApi as any, mockFileService as any)
    );
    mockApi.invoke.mockResolvedValue('Trash cleared');

    const result = await store.clearTrash();

    expect(mockApi.invoke).toHaveBeenCalledWith('clearTrash');
    expect(result).toBe('Trash cleared');
  });

  it('should call clearCache via api', async () => {
    const { CleanerStore } = await import('./cleaner.store');
    const store = runInInjectionContext(
      injector,
      () => new CleanerStore(mockApi as any, mockFileService as any)
    );
    mockApi.invoke.mockResolvedValue('Cache cleared');

    const result = await store.clearCache();

    expect(mockApi.invoke).toHaveBeenCalledWith('clearCache');
    expect(result).toBe('Cache cleared');
  });

  it('should call clearAllLogs via api', async () => {
    const { CleanerStore } = await import('./cleaner.store');
    const store = runInInjectionContext(
      injector,
      () => new CleanerStore(mockApi as any, mockFileService as any)
    );
    mockApi.invoke.mockResolvedValue('Logs cleared');

    const result = await store.clearAllLogs();

    expect(mockApi.invoke).toHaveBeenCalledWith('clearAllLogs');
    expect(result).toBe('Logs cleared');
  });

  it('should call clearAllLargeFiles via api', async () => {
    const { CleanerStore } = await import('./cleaner.store');
    const store = runInInjectionContext(
      injector,
      () => new CleanerStore(mockApi as any, mockFileService as any)
    );
    mockApi.invoke.mockResolvedValue('Large files cleared');

    const result = await store.clearAllLargeFiles();

    expect(mockApi.invoke).toHaveBeenCalledWith('clearAllLargeFiles');
    expect(result).toBe('Large files cleared');
  });
});
