import { Injectable, signal, computed, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import { FileService } from '@services/file.service';
import { BackupService } from '@services/backup.service';
import { PackageManagerService } from '@services/package-manager.service';
import { NotificationService } from '@services/notification.service';
import { ConfirmDialogService } from '@shared/confirm-dialog';
import { LoggerService } from '@services/logger.service';
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
  private api = inject(ApiService);
  private fileService = inject(FileService);
  private backupService = inject(BackupService);
  private packageManagerService = inject(PackageManagerService);
  private notification = inject(NotificationService);
  private confirmDialogService = inject(ConfirmDialogService);
  private logger = inject(LoggerService);

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

  readonly backupDir = signal<string>('');

  readonly cacheSize = computed(() => this.cacheData().reduce((sum, file) => sum + file.size, 0));
  readonly trashSize = computed(() => this.trashData().reduce((sum, file) => sum + file.size, 0));
  readonly logSize = computed(() => this.logData().reduce((sum, file) => sum + file.size, 0));
  readonly packageCacheSize = computed(() =>
    this.packageCacheData().reduce((sum, pkg) => sum + pkg.size, 0)
  );
  readonly totalJunk = computed(() => this.cacheSize() + this.trashSize() + this.logSize());

  readonly cacheCount = computed(() => this.cacheData().length);
  readonly trashCount = computed(() => this.trashData().length);
  readonly logCount = computed(() => this.logData().length);

  getItemCount(): number {
    const tab = this.activeTab();
    if (tab === 'cache') return this.filteredCacheData().length;
    if (tab === 'trash') return this.filteredTrashData().length;
    if (tab === 'logs') return this.filteredLogData().length;
    if (tab === 'packages') return this.packageCacheData().length;
    return 0;
  }

  getSelectedCount(): number {
    const tab = this.activeTab();
    if (tab === 'cache') return this.selectedCacheFiles().size;
    if (tab === 'trash') return this.selectedTrashFiles().size;
    if (tab === 'logs') return this.selectedLogFiles().size;
    return 0;
  }

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

  async loadActiveTabData(): Promise<void> {
    const tab = this.activeTab();

    if (tab === 'cache' && this.cacheData().length > 0) return;
    if (tab === 'trash' && this.trashData().length > 0) return;
    if (tab === 'logs' && this.logData().length > 0) return;
    if (tab === 'packages' && this.packageCacheData().length > 0) return;

    this.loading.set(true);
    try {
      if (tab === 'cache') {
        const result = await this.fileService.getCacheFiles(50, 0);
        this.cacheData.set(result.data);
        this.filteredCacheData.set(result.data);
        this.cacheHasMore.set(result.has_more);
        this.cacheTotal.set(result.total);
      } else if (tab === 'trash') {
        const trash = await this.fileService.getTrashFiles();
        this.trashData.set(trash);
        this.filteredTrashData.set(trash);
      } else if (tab === 'logs') {
        const logs = await this.fileService.getSystemLogs();
        this.logData.set(logs);
        this.filteredLogData.set(logs);
      } else if (tab === 'packages') {
        const packages = await this.packageManagerService.getPackageCacheInfo();
        this.packageCacheData.set(packages);
      }
    } catch (error: unknown) {
      this.logger.logError(
        'store',
        'CleanerStore',
        'loadActiveTabData',
        `Failed to load ${tab} data`,
        error as Error
      );
    } finally {
      this.loading.set(false);
    }
  }

  async loadMoreCache(): Promise<void> {
    if (!this.cacheHasMore() || this.loading()) return;

    this.loading.set(true);
    try {
      const offset = this.cacheData().length;
      const result = await this.fileService.getCacheFiles(50, offset);
      this.cacheData.update((current) => [...current, ...result.data]);
      this.filteredCacheData.update((current) => [...current, ...result.data]);
      this.cacheHasMore.set(result.has_more);
      this.cacheTotal.set(result.total);
    } catch (error: unknown) {
      this.logger.logError(
        'store',
        'CleanerStore',
        'loadMoreCache',
        'Failed to load more cache data',
        error as Error
      );
    } finally {
      this.loading.set(false);
    }
  }

  async forceReloadActiveTab(): Promise<void> {
    const tab = this.activeTab();
    if (tab === 'cache') this.cacheData.set([]);
    if (tab === 'trash') this.trashData.set([]);
    if (tab === 'logs') this.logData.set([]);
    await this.loadActiveTabData();
  }

  onSearchFiltered(rows: object[]): void {
    const tab = this.activeTab();
    if (tab === 'cache') this.filteredCacheData.set(rows as CacheFileItem[]);
    else if (tab === 'trash') this.filteredTrashData.set(rows as TrashFileItem[]);
    else this.filteredLogData.set(rows as LogFileItem[]);
  }

  searchDataForActiveTab(): object[] {
    const tab = this.activeTab();
    if (tab === 'cache') return this.cacheData() as object[];
    if (tab === 'trash') return this.trashData() as object[];
    return this.logData() as object[];
  }

  getSearchFields(): string[] {
    const tab = this.activeTab();
    if (tab === 'trash') return ['name', 'path'];
    return ['path'];
  }

  async clearSelectedForActiveTab(): Promise<void> {
    const tab = this.activeTab();
    let selectedKeys: Set<string>;

    if (tab === 'cache') selectedKeys = this.selectedCacheFiles();
    else if (tab === 'trash') selectedKeys = this.selectedTrashFiles();
    else selectedKeys = this.selectedLogFiles();

    const filesToClear = Array.from(selectedKeys);
    if (filesToClear.length === 0) return;

    const createBackup = await this.confirmDialogService.confirm({
      title: 'Clear Files',
      message: `Clear ${filesToClear.length} ${tab} item(s)?\n\nWould you like to create a backup before deleting?`,
    });
    if (!createBackup) return;

    this.loading.set(true);
    let backupCreated = false;
    try {
      let backupDir = this.backupDir();
      if (!backupDir) {
        backupDir = await this.backupService.getBackupDir();
        this.backupDir.set(backupDir);
      }

      const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
      const archivePath = `${backupDir}/backup_${tab}_${timestamp}.tar.gz`;

      await this.backupService.createBackup(filesToClear, archivePath);
      backupCreated = true;
    } catch (error) {
      this.logger.logWarn(
        'store',
        'cleaner',
        'createBackup',
        'Failed to create backup, proceeding with deletion',
        error instanceof Error ? error : new Error(String(error))
      );
    }

    try {
      if (tab === 'cache') {
        await this.fileService.clearSelectedCacheFiles(filesToClear);
        this.selectedCacheFiles.set(new Set());
      } else if (tab === 'trash') {
        await this.fileService.clearSelectedTrashFiles(filesToClear);
        this.selectedTrashFiles.set(new Set());
      } else if (tab === 'logs') {
        await this.fileService.clearSelectedLogFiles(filesToClear);
        this.selectedLogFiles.set(new Set());
      }
      await this.forceReloadActiveTab();
      if (backupCreated) {
        this.notification.success(
          `Successfully cleared ${filesToClear.length} ${tab} item(s). Backup created.`
        );
      }
    } catch (error: unknown) {
      this.notification.error(
        'Failed to clear files: ' + (error instanceof Error ? error.message : String(error)),
        error
      );
    } finally {
      this.loading.set(false);
    }
  }

  async cleanPackageCache(manager: string): Promise<void> {
    const confirmed = await this.confirmDialogService.confirm({
      title: 'Clean Package Cache',
      message: `Clean ${manager} package cache?`,
    });
    if (!confirmed) return;

    this.loading.set(true);
    try {
      await this.packageManagerService.cleanPackageCache(manager);
      await this.forceReloadActiveTab();
    } catch (error: unknown) {
      this.notification.error(
        'Failed to clean package cache: ' +
          (error instanceof Error ? error.message : String(error)),
        error
      );
    } finally {
      this.loading.set(false);
    }
  }

  async cleanAllPackageCaches(): Promise<void> {
    const confirmed = await this.confirmDialogService.confirm({
      title: 'Clean All Caches',
      message: 'Clean all package manager caches?',
    });
    if (!confirmed) return;

    this.loading.set(true);
    try {
      const results = await this.packageManagerService.cleanAllPackageCaches();
      await this.forceReloadActiveTab();
      this.notification.success(results.join('\n'));
    } catch (error: unknown) {
      this.notification.error(
        'Failed to clean package caches: ' +
          (error instanceof Error ? error.message : String(error)),
        error
      );
    } finally {
      this.loading.set(false);
    }
  }
}
