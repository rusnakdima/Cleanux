/* sys lib */
import { ChangeDetectionStrategy, Component, OnInit, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute } from '@angular/router';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* services */
import { FileService } from '@services/file.service';
import { CleanerStore } from '@stores/cleaner.store';
import { LogAnalyzerService } from '@services/log-analyzer.service';
import { NotificationService } from '@services/notification.service';
import { ConfirmDialogService } from '@shared/confirm-dialog';

/* components */
import { DataListComponent } from '@components/data-list/data-list.component';
import { FilePreviewComponent } from '@components/file-preview/file-preview.component';
import { FilePreviewData } from '@models/file-preview.model';

/* models */
import { ListColumn, ListOptions } from '@models/data-list.model';
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
  providers: [CleanerStore],
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    DataListComponent,
    FilePreviewComponent,
  ],
  templateUrl: './cleaner.view.html',
})
export class CleanerView implements OnInit {
  readonly store = inject(CleanerStore);
  private fileService = inject(FileService);
  private logAnalyzerService = inject(LogAnalyzerService);
  private notification = inject(NotificationService);
  private confirmDialogService = inject(ConfirmDialogService);
  private route = inject(ActivatedRoute);

  formatSize = formatSize;

  previewData = signal<FilePreviewData | null>(null);
  errorPreview = signal<string | null>(null);

  logSummary = signal<LogSummary | null>(null);
  selectedLogCategory = signal<string>('all');
  cleanDays = signal<number>(30);

  cacheColumns: ListColumn[] = [
    { key: 'path', primary: true, icon: 'cached', truncate: true, sortable: true },
    { key: 'size', format: 'size', align: 'right', sortable: true },
  ];

  trashColumns: ListColumn[] = [
    { key: 'name', primary: true, icon: 'delete_outline', sortable: true },
    { key: 'path', truncate: true, sortable: true },
    { key: 'size', format: 'size', align: 'right', sortable: true },
  ];

  logColumns: ListColumn[] = [
    { key: 'path', primary: true, icon: 'description', truncate: true, sortable: true },
    { key: 'size', format: 'size', align: 'right', sortable: true },
  ];

  packageColumns: ListColumn[] = [
    {
      key: 'name',
      primary: true,
      icon: 'package',
      sortable: true,
      actions: [{ id: 'clean', icon: 'delete', tooltip: 'Clean cache' }],
    },
    { key: 'description', truncate: true, sortable: true },
    { key: 'cachePath', truncate: true, sortable: true },
    { key: 'size', format: 'size', align: 'right', sortable: true },
  ];

  getCacheOptions(): ListOptions {
    return {
      showCheckbox: true,
      checkboxKey: 'path',
      hoverable: true,
      showReloadButton: true,
      showSearch: true,
      showSelectAll: true,
      searchPlaceholder: 'Search...',
    };
  }

  getTrashOptions(): ListOptions {
    return {
      showCheckbox: true,
      checkboxKey: 'path',
      hoverable: true,
      showReloadButton: true,
      showSearch: true,
      showSelectAll: true,
      searchPlaceholder: 'Search...',
    };
  }

  getLogOptions(): ListOptions {
    return {
      showCheckbox: true,
      checkboxKey: 'path',
      hoverable: true,
      showReloadButton: true,
      showSearch: true,
      showSelectAll: true,
      searchPlaceholder: 'Search...',
    };
  }

  getPackageOptions(): ListOptions {
    return {
      showCheckbox: false,
      hoverable: true,
      showReloadButton: true,
      showSearch: true,
      searchPlaceholder: 'Search...',
    };
  }

  async ngOnInit() {
    const tab = this.route.snapshot.queryParamMap.get('tab');
    if (tab && ['cache', 'trash', 'logs', 'packages'].includes(tab)) {
      await this.onTabChange(tab as 'cache' | 'trash' | 'logs' | 'packages');
    } else {
      await this.store.loadActiveTabData();
    }
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
    const confirmed = await this.confirmDialogService.confirm({
      title: 'Clear Old Logs',
      message: `Clear all logs older than ${days} days?`,
    });
    if (!confirmed) return;
    try {
      const result = await this.logAnalyzerService.cleanOldLogs(days);
      this.notification.success(result);
      await this.loadLogSummary();
    } catch (error: unknown) {
      this.notification.error(
        'Failed to clean logs: ' + (error instanceof Error ? error.message : String(error)),
        error
      );
    }
  }

  filteredLogEntries(): any[] {
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
      this.notification.error(
        'Failed to open file: ' + (error instanceof Error ? error.message : String(error)),
        error
      );
    }
  }
}
