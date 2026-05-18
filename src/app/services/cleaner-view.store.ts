/* sys lib */
import { Injectable, inject, signal, computed } from '@angular/core';

/* services */
import { FileService, CacheFileItem, TrashFileItem, LogFileItem } from '@services/file.service';
import { BackupService } from '@services/backup.service';
import { PackageManagerService } from '@services/package-manager.service';
import { PackageCacheInfo } from '@models/package-manager.model';

export type CleanerTabId = 'cache' | 'trash' | 'logs' | 'packages';

@Injectable()
export class CleanerViewStore {
  private fileService = inject(FileService);
  private backupService = inject(BackupService);
  private packageManagerService = inject(PackageManagerService);

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

  readonly backupDir = signal<string>('');

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

  async loadActiveTabData(): Promise<void> {
    const tab = this.activeTab();

    if (tab === 'cache' && this.cacheData().length > 0) return;
    if (tab === 'trash' && this.trashData().length > 0) return;
    if (tab === 'logs' && this.logData().length > 0) return;
    if (tab === 'packages' && this.packageCacheData().length > 0) return;

    this.loading.set(true);
    try {
      if (tab === 'cache') {
        const cache = await this.fileService.getCacheFiles();
        this.cacheData.set(cache);
        this.filteredCacheData.set(cache);
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
      console.error(`Failed to load ${tab} data:`, error);
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

    const createBackup = confirm(
      `Clear ${filesToClear.length} ${tab} item(s)?\n\nWould you like to create a backup before deleting?`
    );
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
      console.warn('Failed to create backup, proceeding with deletion:', error);
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
        alert(`Successfully cleared ${filesToClear.length} ${tab} item(s). Backup created.`);
      }
    } catch (error: unknown) {
      alert('Failed to clear files: ' + (error instanceof Error ? error.message : String(error)));
    } finally {
      this.loading.set(false);
    }
  }

  async cleanPackageCache(manager: string): Promise<void> {
    const confirmed = confirm(`Clean ${manager} package cache?`);
    if (!confirmed) return;

    this.loading.set(true);
    try {
      await this.packageManagerService.cleanPackageCache(manager);
      await this.forceReloadActiveTab();
    } catch (error: unknown) {
      alert(
        'Failed to clean package cache: ' + (error instanceof Error ? error.message : String(error))
      );
    } finally {
      this.loading.set(false);
    }
  }

  async cleanAllPackageCaches(): Promise<void> {
    const confirmed = confirm('Clean all package manager caches?');
    if (!confirmed) return;

    this.loading.set(true);
    try {
      const results = await this.packageManagerService.cleanAllPackageCaches();
      await this.forceReloadActiveTab();
      alert(results.join('\n'));
    } catch (error: unknown) {
      alert(
        'Failed to clean package caches: ' +
          (error instanceof Error ? error.message : String(error))
      );
    } finally {
      this.loading.set(false);
    }
  }
}
