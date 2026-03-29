/* sys lib */
import { Injectable, inject, signal, computed } from '@angular/core';

/* services */
import { SystemService, CacheFileItem, TrashFileItem, LogFileItem } from '@services/system.service';

export type CleanerTabId = 'cache' | 'trash' | 'logs';

@Injectable()
export class CleanerViewStore {
  private systemService = inject(SystemService);

  readonly cacheData = signal<CacheFileItem[]>([]);
  readonly filteredCacheData = signal<CacheFileItem[]>([]);
  readonly trashData = signal<TrashFileItem[]>([]);
  readonly filteredTrashData = signal<TrashFileItem[]>([]);
  readonly logData = signal<LogFileItem[]>([]);
  readonly filteredLogData = signal<LogFileItem[]>([]);
  readonly loading = signal(false);
  readonly activeTab = signal<CleanerTabId>('cache');

  readonly selectedCacheFiles = signal<Set<string>>(new Set());
  readonly selectedTrashFiles = signal<Set<string>>(new Set());
  readonly selectedLogFiles = signal<Set<string>>(new Set());

  readonly cacheSize = computed(() =>
    this.cacheData().reduce((sum, file) => sum + file.size, 0)
  );
  readonly trashSize = computed(() =>
    this.trashData().reduce((sum, file) => sum + file.size, 0)
  );
  readonly logSize = computed(() =>
    this.logData().reduce((sum, file) => sum + file.size, 0)
  );
  readonly totalJunk = computed(
    () => this.cacheSize() + this.trashSize() + this.logSize()
  );

  setActiveTab(tab: CleanerTabId): void {
    this.activeTab.set(tab);
  }

  async loadActiveTabData(): Promise<void> {
    const tab = this.activeTab();

    if (tab === 'cache' && this.cacheData().length > 0) return;
    if (tab === 'trash' && this.trashData().length > 0) return;
    if (tab === 'logs' && this.logData().length > 0) return;

    this.loading.set(true);
    try {
      if (tab === 'cache') {
        const cache = await this.systemService.getCacheFiles();
        this.cacheData.set(cache);
        this.filteredCacheData.set(cache);
      } else if (tab === 'trash') {
        const trash = await this.systemService.getTrashFiles();
        this.trashData.set(trash);
        this.filteredTrashData.set(trash);
      } else if (tab === 'logs') {
        const logs = await this.systemService.getSystemLogs();
        this.logData.set(logs);
        this.filteredLogData.set(logs);
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

    const confirmed = confirm(`Clear ${filesToClear.length} ${tab} item(s)?`);
    if (!confirmed) return;

    this.loading.set(true);
    try {
      if (tab === 'cache') {
        await this.systemService.clearSelectedCacheFiles(filesToClear);
        this.selectedCacheFiles.set(new Set());
      } else if (tab === 'trash') {
        await this.systemService.clearSelectedTrashFiles(filesToClear);
        this.selectedTrashFiles.set(new Set());
      } else if (tab === 'logs') {
        await this.systemService.clearSelectedLogFiles(filesToClear);
        this.selectedLogFiles.set(new Set());
      }
      await this.forceReloadActiveTab();
    } catch (error: unknown) {
      alert(
        'Failed to clear files: ' +
          (error instanceof Error ? error.message : String(error))
      );
    } finally {
      this.loading.set(false);
    }
  }
}
