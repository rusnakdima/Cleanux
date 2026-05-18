/* sys lib */
import { ChangeDetectionStrategy, Component, OnInit, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* services */
import { FileService } from '@services/file.service';
import { CleanerViewStore } from '@services/cleaner-view.store';
import { LogAnalyzerService } from '@services/log-analyzer.service';

/* components */
import { DataTableComponent } from '@components/data-table/data-table.component';
import { FilePreviewComponent } from '@components/file-preview/file-preview.component';
import { FilePreviewData } from '@models/file-preview.model';
import { HeaderComponent } from '@components/header/header.component';
import { SearchComponent } from '@components/search/search.component';

/* models */
import { TableColumn, TableOptions } from '@models/data-table.model';
import { CacheFileItem, LogFileItem, TrashFileItem } from '@services/file.service';
import { LogSummary, LogCategorySummary } from '@models/log-analyzer.model';
import { formatSize } from '@shared/utils/format.util';

export type CleanerRow = CacheFileItem | TrashFileItem | LogFileItem;

export type PackageManagerRow = {
  name: string;
  cachePath: string;
  size: number;
  description: string;
};

@Component({
  selector: 'app-cleaner-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
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
  private fileService = inject(FileService);
  private logAnalyzerService = inject(LogAnalyzerService);

  previewData = signal<FilePreviewData | null>(null);
  errorPreview = signal<string | null>(null);

  logSummary = signal<LogSummary | null>(null);
  selectedLogCategory = signal<string>('all');
  cleanDays = signal<number>(30);

  cacheColumns: TableColumn[] = [
    { key: 'path', label: 'Path', width: 'flex-1', sortable: true },
    { key: 'size', label: 'Size', align: 'right', width: 'w-32', sortable: true },
  ];

  trashColumns: TableColumn[] = [
    { key: 'name', label: 'Name', width: 'w-64', sortable: true },
    { key: 'path', label: 'Original Path', width: 'flex-1', sortable: true },
    { key: 'size', label: 'Size', align: 'right', width: 'w-32', sortable: true },
  ];

  logColumns: TableColumn[] = [
    { key: 'path', label: 'Path', width: 'flex-1', sortable: true },
    { key: 'size', label: 'Size', align: 'right', width: 'w-32', sortable: true },
    { key: 'modified', label: 'Modified', width: 'w-48', sortable: true },
  ];

  packageColumns: TableColumn[] = [
    { key: 'name', label: 'Package Manager', width: 'w-32', sortable: true },
    { key: 'description', label: 'Description', width: 'flex-1', sortable: true },
    { key: 'cachePath', label: 'Cache Path', width: 'flex-1', sortable: true },
    { key: 'size', label: 'Size', align: 'right', width: 'w-32', sortable: true },
  ];

  async ngOnInit() {
    await this.store.loadActiveTabData();
  }

  async onTabChange(tab: 'cache' | 'trash' | 'logs' | 'packages'): Promise<void> {
    this.store.setActiveTab(tab);
    await this.store.loadActiveTabData();
    if (tab === 'logs') {
      await this.loadLogSummary();
    }
  }

  async loadLogSummary(): Promise<void> {
    try {
      const summary = await this.logAnalyzerService.getLogSummary();
      this.logSummary.set(summary);
    } catch (error) {
      console.error('Failed to load log summary:', error);
    }
  }

  async cleanOldLogs(): Promise<void> {
    const days = this.cleanDays();
    if (days <= 0) return;
    const confirmed = confirm(`Clear all logs older than ${days} days?`);
    if (!confirmed) return;
    try {
      const result = await this.logAnalyzerService.cleanOldLogs(days);
      alert(result);
      await this.loadLogSummary();
    } catch (error: unknown) {
      alert('Failed to clean logs: ' + (error instanceof Error ? error.message : String(error)));
    }
  }

  getFilteredLogEntries(): any[] {
    const summary = this.logSummary();
    if (!summary) return [];
    if (this.selectedLogCategory() === 'all') {
      return [
        ...summary.system.entries,
        ...summary.application.entries,
        ...summary.security.entries,
        ...summary.hardware.entries,
      ];
    }
    const cat = summary[this.selectedLogCategory() as keyof LogSummary];
    return cat ? cat.entries : [];
  }

  getSeverityColor(severity: string): string {
    switch (severity) {
      case 'error':
        return 'text-red-500';
      case 'warning':
        return 'text-yellow-500';
      case 'info':
        return 'text-blue-500';
      default:
        return 'text-gray-500';
    }
  }

  getTableOptions(_tab: 'cache' | 'trash' | 'logs' | 'packages'): TableOptions {
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

  getPackageTableOptions(): TableOptions {
    return {
      showHeader: true,
      showCheckbox: false,
      hoverable: true,
      showReloadButton: true,
      showSelectedActions: false,
      showPreviewButton: false,
      showRowActions: true,
    };
  }

  onPackageAction(event: { action: string; item: PackageManagerRow }): void {
    if (event.action === 'clean') {
      this.store.cleanPackageCache(event.item.name);
    }
  }

  async onPreview(item: CleanerRow): Promise<void> {
    const path = item.path;
    const displayName = 'name' in item && item.name ? item.name : path;
    if (!path) return;

    this.errorPreview.set(null);
    this.previewData.set({ name: displayName, path, type: 'unknown' });

    try {
      const result = await this.fileService.previewFile(path);

      this.previewData.set({
        name: result.name,
        path: result.path,
        type: result.type as 'image' | 'text' | 'binary' | 'unknown',
        content: result.content,
        imageUrl: result.imageUrl,
      });
    } catch (error: unknown) {
      const errorMessage = error instanceof Error ? error.message : 'Unable to preview file';
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
      await this.fileService.openFile(event.path, event.command);
    } catch (error: unknown) {
      alert('Failed to open file: ' + (error instanceof Error ? error.message : String(error)));
    }
  }
}
