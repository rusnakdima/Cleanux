/* sys lib */
import { Component, OnInit, signal, inject, DOCUMENT } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* services */
import { MainService } from '@services/main.service';
import { CleanerViewStore } from '@services/cleaner-view.store';

/* components */
import { DataTableComponent } from '@components/data-table/data-table.component';
import { FilePreviewComponent } from '@components/file-preview/file-preview.component';
import { FilePreviewData } from '@models/file-preview.model';
import { HeaderComponent } from '@components/header/header.component';
import { SearchComponent } from '@components/search/search.component';

/* models */
import { TableColumn, TableOptions } from '@models/data-table.model';
import { CacheFileItem, LogFileItem, TrashFileItem } from '@services/system.service';

export type CleanerRow = CacheFileItem | TrashFileItem | LogFileItem;

@Component({
  selector: 'app-cleaner-view',
  standalone: true,
  providers: [CleanerViewStore],
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    DataTableComponent,
    FilePreviewComponent,
    HeaderComponent,
    SearchComponent,
  ],
  templateUrl: './cleaner.view.html',
})
export class CleanerView implements OnInit {
  readonly store = inject(CleanerViewStore);
  private mainService = inject(MainService);
  private document = inject(DOCUMENT);

  previewData = signal<FilePreviewData | null>(null);
  errorPreview = signal<string | null>(null);

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
    await this.store.loadActiveTabData();
  }

  async onTabChange(tab: 'cache' | 'trash' | 'logs'): Promise<void> {
    this.store.setActiveTab(tab);
    await this.store.loadActiveTabData();
  }

  getTableOptions(_tab: 'cache' | 'trash' | 'logs'): TableOptions {
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

  formatSize(size: number): string {
    const units = ['B', 'KB', 'MB', 'GB'];
    let i = 0;
    let s = size;
    while (s >= 1024 && i < units.length - 1) {
      s /= 1024;
      i++;
    }
    return `${s.toFixed(1)} ${units[i]}`;
  }

  async onPreview(item: CleanerRow): Promise<void> {
    const path = item.path;
    const displayName = 'name' in item && item.name ? item.name : path;
    if (!path) return;

    this.errorPreview.set(null);
    this.previewData.set({ name: displayName, path, type: 'unknown' });

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
    } catch (error: unknown) {
      const errorMessage =
        error instanceof Error ? error.message : 'Unable to preview file';
      this.previewData.set({
        name: displayName,
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
    } catch (error: unknown) {
      alert(
        'Failed to open file: ' +
          (error instanceof Error ? error.message : String(error))
      );
    }
  }
}
