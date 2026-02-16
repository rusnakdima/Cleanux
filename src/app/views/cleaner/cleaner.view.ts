/* sys lib */
import { Component, OnInit, signal, inject, computed } from '@angular/core';
import { CommonModule, DOCUMENT } from '@angular/common';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { MatTabsModule } from '@angular/material/tabs';

/* services */
import { SystemService, CacheFileItem, TrashFileItem, LogFileItem } from '@services/system.service';
import { MainService } from '@services/main.service';

/* components */
import { DataTableComponent } from '@components/data-table/data-table.component';
import { FilePreviewComponent } from '@components/file-preview/file-preview.component';
import { FilePreviewData } from '@models/file-preview.model';
import { HeaderComponent } from '@components/header/header.component';
import { SearchComponent } from '@components/search/search.component';

/* models */
import { TableColumn, TableOptions } from '@models/data-table.model';

@Component({
  selector: 'app-cleaner-view',
  standalone: true,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    MatTabsModule,
    DataTableComponent,
    FilePreviewComponent,
    HeaderComponent,
    SearchComponent,
  ],
  templateUrl: './cleaner.view.html',
})
export class CleanerView implements OnInit {
  private systemService = inject(SystemService);
  private mainService = inject(MainService);
  private document = inject(DOCUMENT);

  cacheData = signal<CacheFileItem[]>([]);
  filteredCacheData = signal<CacheFileItem[]>([]);
  trashData = signal<TrashFileItem[]>([]);
  filteredTrashData = signal<TrashFileItem[]>([]);
  logData = signal<LogFileItem[]>([]);
  filteredLogData = signal<LogFileItem[]>([]);
  loading = signal(false);
  activeTab = signal<'cache' | 'trash' | 'logs'>('cache');

  selectedCacheFiles = signal<Set<string>>(new Set());
  selectedTrashFiles = signal<Set<string>>(new Set());
  selectedLogFiles = signal<Set<string>>(new Set());

  previewData = signal<FilePreviewData | null>(null);

  errorPreview = signal<string | null>(null);

  cacheSize = computed(() => this.cacheData().reduce((sum, file) => sum + file.size, 0));
  trashSize = computed(() => this.trashData().reduce((sum, file) => sum + file.size, 0));
  logSize = computed(() => this.logData().reduce((sum, file) => sum + file.size, 0));
  totalJunk = computed(() => this.cacheSize() + this.trashSize() + this.logSize());

  cacheColumns: TableColumn[] = [
    { key: 'path', label: 'Path', width: 'flex-1', sortable: true },
    { key: 'size', label: 'Size', align: 'right', width: 'w-32', sortable: true }
  ];

  trashColumns: TableColumn[] = [
    { key: 'name', label: 'Name', width: 'w-64', sortable: true },
    { key: 'path', label: 'Original Path', width: 'flex-1', sortable: true },
    { key: 'size', label: 'Size', align: 'right', width: 'w-32', sortable: true }
  ];

  logColumns: TableColumn[] = [
    { key: 'path', label: 'Path', width: 'flex-1', sortable: true },
    { key: 'size', label: 'Size', align: 'right', width: 'w-32', sortable: true },
    { key: 'modified', label: 'Modified', width: 'w-48', sortable: true }
  ];

  get isDark(): boolean {
    return this.document.body.classList.contains('dark');
  }

  async ngOnInit() {
    await this.loadActiveTabData();
  }

  async onTabChange(tab: 'cache' | 'trash' | 'logs') {
    this.activeTab.set(tab);
    await this.loadActiveTabData();
  }

  async loadActiveTabData() {
    const tab = this.activeTab();

    // Check if we already have data to avoid unnecessary re-loading unless forced
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
    } catch (error) {
      console.error(`Failed to load ${tab} data:`, error);
    } finally {
      this.loading.set(false);
    }
  }

  onFilteredCacheData(files: CacheFileItem[]): void {
    this.filteredCacheData.set(files);
  }

  onFilteredTrashData(files: TrashFileItem[]): void {
    this.filteredTrashData.set(files);
  }

  onFilteredLogData(files: LogFileItem[]): void {
    this.filteredLogData.set(files);
  }

  getCurrentFilteredData(): any[] {
    const tab = this.activeTab();
    if (tab === 'cache') return this.filteredCacheData();
    if (tab === 'trash') return this.filteredTrashData();
    return this.filteredLogData();
  }

  getSearchFields(): string[] {
    const tab = this.activeTab();
    if (tab === 'trash') return ['name', 'path'];
    return ['path'];
  }

  async loadData() {
    // Force refresh active tab
    const tab = this.activeTab();
    if (tab === 'cache') this.cacheData.set([]);
    if (tab === 'trash') this.trashData.set([]);
    if (tab === 'logs') this.logData.set([]);
    await this.loadActiveTabData();
  }

  async clearSelectedFiles() {
    const tab = this.activeTab();
    let selectedKeys: Set<string>;

    if (tab === 'cache') selectedKeys = this.selectedCacheFiles();
    else if (tab === 'trash') selectedKeys = this.selectedTrashFiles();
    else selectedKeys = this.selectedLogFiles();

    const filesToClear = Array.from(selectedKeys);

    if (filesToClear.length === 0) return;

    const confirmed = confirm(`Clear ${filesToClear.length} ${tab} item(s)?`);
    if (!confirmed) return;

    try {
      this.loading.set(true);
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
      await this.loadData();
    } catch (error) {
      alert('Failed to clear files: ' + error);
    } finally {
      this.loading.set(false);
    }
  }

  formatSize(size: number): string {
    const units = ['B', 'KB', 'MB', 'GB'];
    let i = 0;
    while (size >= 1024 && i < units.length - 1) {
      size /= 1024;
      i++;
    }
    return `${size.toFixed(1)} ${units[i]}`;
  }

  getTableOptions(tab: 'cache' | 'trash' | 'logs'): TableOptions {
    return {
      showHeader: true,
      showCheckbox: true,
      checkboxKey: 'path',
      hoverable: true,
      showReloadButton: true,
      showSelectedActions: true,
      selectedActionText: 'Clear Selected',
      showPreviewButton: true,
    };
  }

  async clearSelectedFilesForTab(tab: 'cache' | 'trash' | 'logs') {
    let selectedKeys: Set<string>;

    if (tab === 'cache') selectedKeys = this.selectedCacheFiles();
    else if (tab === 'trash') selectedKeys = this.selectedTrashFiles();
    else selectedKeys = this.selectedLogFiles();

    const filesToClear = Array.from(selectedKeys);

    if (filesToClear.length === 0) return;

    const confirmed = confirm(`Clear ${filesToClear.length} ${tab} item(s)?`);
    if (!confirmed) return;

    try {
      this.loading.set(true);
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
      await this.loadData();
    } catch (error) {
      alert('Failed to clear files: ' + error);
    } finally {
      this.loading.set(false);
    }
  }

  get showPreviewButton(): boolean {
    return true;
  }

  async onPreview(item: any): Promise<void> {
    const path = item.path || item.name;
    if (!path) return;

    this.errorPreview.set(null);
    this.previewData.set({ name: item.name || path, path, type: 'unknown' });

    try {
      const result = await this.mainService.previewFile<{
        name: string;
        path: string;
        type: string;
        content?: string;
        imageUrl?: string;
      }>(path);

      this.previewData.set({
        name: result.name,
        path: result.path,
        type: result.type as 'image' | 'text' | 'binary' | 'unknown',
        content: result.content,
        imageUrl: result.imageUrl,
      });
    } catch (error: any) {
      const errorMessage = error?.message || 'Unable to preview file';
      this.previewData.set({
        name: item.name || path,
        path: path,
        type: 'error',
        error: errorMessage,
      });
    }
  }

  onClosePreview(): void {
    this.previewData.set(null);
  }

  async onOpenInEditor(event: { path: string; command?: string }): Promise<void> {
    if (!event.path) return;
    try {
      await this.mainService.openFile(event.path, event.command);
    } catch (error: any) {
      alert('Failed to open file: ' + (error?.message || error));
    }
  }
}
