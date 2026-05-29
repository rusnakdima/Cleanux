import { Injectable, signal, computed, inject } from '@angular/core';
import { TauriApiService } from '@api/tauri-api.service';
import { FileService } from '@services/file.service';
import {
  CacheFileItem,
  TrashFileItem,
  LogFileItem,
  LargeFileItem,
  ScanSummary,
  PaginatedData,
} from '@models/system.model';
import { PackageCacheInfo } from '@models/package-manager.model';
import { CleanerTabId } from '@models/cleaner.model';

@Injectable()
export class CleanerStore {
  private api = inject(TauriApiService);
  private fileService = inject(FileService);

  readonly cacheData = signal<CacheFileItem[]>([]);
  readonly filteredCacheData = signal<CacheFileItem[]>([]);
  readonly trashData = signal<TrashFileItem[]>([]);
  readonly filteredTrashData = signal<TrashFileItem[]>([]);
  readonly logData = signal<LogFileItem[]>([]);
  readonly filteredLogData = signal<LogFileItem[]>([]);
  readonly packageCacheData = signal<PackageCacheInfo[]>([]);
  readonly loading = signal(false);
  readonly activeTab = signal<CleanerTabId>('cache');

  readonly selectedCacheFiles = signal<Set<string>>(new Set());
  readonly selectedTrashFiles = signal<Set<string>>(new Set());
  readonly selectedLogFiles = signal<Set<string>>(new Set());

  readonly cacheHasMore = signal(false);
  readonly cacheOffset = signal(0);
  readonly cacheTotal = signal(0);
  readonly cacheLoading = signal(false);

  readonly largeFilesHasMore = signal(false);
  readonly largeFilesOffset = signal(0);
  readonly largeFilesTotal = signal(0);
  readonly largeFilesLoading = signal(false);
  readonly largeFilesData = signal<LargeFileItem[]>([]);

  readonly cacheSize = computed(() => this.cacheData().reduce((sum, file) => sum + file.size, 0));
  readonly trashSize = computed(() => this.trashData().reduce((sum, file) => sum + file.size, 0));
  readonly logSize = computed(() => this.logData().reduce((sum, file) => sum + file.size, 0));
  readonly packageCacheSize = computed(() =>
    this.packageCacheData().reduce((sum, pkg) => sum + pkg.size, 0)
  );
  readonly totalJunk = computed(() => this.cacheSize() + this.trashSize() + this.logSize());

  setActiveTab(tab: CleanerTabId): void {
    this.activeTab.set(tab);
  }

  async loadCacheFiles(
    limit: number = 50,
    offset: number = 0,
    reset: boolean = false
  ): Promise<PaginatedData<CacheFileItem>> {
    if (this.cacheLoading()) {
      return { data: [], has_more: false, total: this.cacheTotal() };
    }

    if (reset) {
      this.cacheData.set([]);
      this.cacheOffset.set(0);
      this.cacheTotal.set(0);
    }

    this.cacheLoading.set(true);

    try {
      const result = await this.fileService.getCacheFiles(limit, offset);

      if (reset) {
        this.cacheData.set(result.data);
      } else {
        this.cacheData.update((current) => [...current, ...result.data]);
      }

      this.cacheHasMore.set(result.has_more);
      this.cacheOffset.set(offset + result.data.length);
      this.cacheTotal.set(result.total);

      return result;
    } finally {
      this.cacheLoading.set(false);
    }
  }

  async loadLargeFiles(
    limit: number = 50,
    offset: number = 0,
    reset: boolean = false
  ): Promise<PaginatedData<LargeFileItem>> {
    if (this.largeFilesLoading()) {
      return { data: [], has_more: false, total: this.largeFilesTotal() };
    }

    if (reset) {
      this.largeFilesData.set([]);
      this.largeFilesOffset.set(0);
      this.largeFilesTotal.set(0);
    }

    this.largeFilesLoading.set(true);

    try {
      const result = await this.fileService.getLargeFiles(limit, offset);

      if (reset) {
        this.largeFilesData.set(result.data);
      } else {
        this.largeFilesData.update((current) => [...current, ...result.data]);
      }

      this.largeFilesHasMore.set(result.has_more);
      this.largeFilesOffset.set(offset + result.data.length);
      this.largeFilesTotal.set(result.total);

      return result;
    } finally {
      this.largeFilesLoading.set(false);
    }
  }

  async loadMoreCacheFiles(limit: number = 50): Promise<PaginatedData<CacheFileItem>> {
    if (!this.cacheHasMore() || this.cacheLoading()) {
      return { data: [], has_more: false, total: this.cacheTotal() };
    }
    return this.loadCacheFiles(limit, this.cacheOffset(), false);
  }

  async loadMoreLargeFiles(limit: number = 50): Promise<PaginatedData<LargeFileItem>> {
    if (!this.largeFilesHasMore() || this.largeFilesLoading()) {
      return { data: [], has_more: false, total: this.largeFilesTotal() };
    }
    return this.loadLargeFiles(limit, this.largeFilesOffset(), false);
  }

  async loadTrashFiles(limit?: number, offset?: number): Promise<TrashFileItem[]> {
    return await this.api.invoke<TrashFileItem[]>('getTrashFiles', {
      limit: limit ?? null,
      offset: offset ?? null,
    });
  }

  async loadSystemLogs(limit?: number, offset?: number): Promise<LogFileItem[]> {
    return await this.api.invoke<LogFileItem[]>('getSystemLogs', {
      limit: limit ?? null,
      offset: offset ?? null,
    });
  }

  async getCacheSummary(): Promise<ScanSummary> {
    return await this.api.invoke<ScanSummary>('getCacheSummary');
  }

  async getTrashSummary(): Promise<ScanSummary> {
    return await this.api.invoke<ScanSummary>('getTrashSummary');
  }

  async getLogSummary(): Promise<ScanSummary> {
    return await this.api.invoke<ScanSummary>('getLogSummary');
  }

  async getLargeFilesSummary(): Promise<ScanSummary> {
    return await this.api.invoke<ScanSummary>('getLargeFilesSummary');
  }

  async clearSelectedCacheFiles(paths: string[]): Promise<string> {
    return await this.api.invoke<string>('clearSelectedCacheFiles', { paths });
  }

  async clearSelectedTrashFiles(paths: string[]): Promise<string> {
    return await this.api.invoke<string>('clearSelectedTrashFiles', { paths });
  }

  async clearSelectedLogFiles(paths: string[]): Promise<string> {
    return await this.api.invoke<string>('clearSelectedLogFiles', { paths });
  }

  async clearSelectedLargeFiles(paths: string[]): Promise<string> {
    return await this.api.invoke<string>('clearSelectedLargeFiles', { paths });
  }

  async clearTrash(): Promise<string> {
    return await this.api.invoke<string>('clearTrash');
  }

  async clearCache(): Promise<string> {
    return await this.api.invoke<string>('clearCache');
  }

  async clearAllLogs(): Promise<string> {
    return await this.api.invoke<string>('clearAllLogs');
  }

  async clearAllLargeFiles(): Promise<string> {
    return await this.api.invoke<string>('clearAllLargeFiles');
  }
}
