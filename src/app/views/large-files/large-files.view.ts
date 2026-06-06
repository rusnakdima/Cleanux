/* sys lib */
import {
  ChangeDetectionStrategy,
  Component,
  OnInit,
  signal,
  inject,
  computed,
} from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* services */
import { FileService, LargeFileItem } from '@services/file.service';
import { ThemeService } from '@services/theme.service';
import { NotificationService } from '@services/notification.service';
import { ConfirmDialogService } from '@shared/confirm-dialog';

/* components */
import { DataListComponent } from '@components/data-list/data-list.component';
import { FilePreviewComponent } from '@components/file-preview/file-preview.component';
import { FilePreviewData } from '@models/file-preview.model';
import { ListColumn, ListOptions } from '@models/data-list.model';
import { formatSize } from '@shared/utils/format.util';

@Component({
  selector: 'app-large-files-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    DataListComponent,
    FilePreviewComponent,
  ],
  templateUrl: './large-files.view.html',
})
export class LargeFilesView implements OnInit {
  private fileService = inject(FileService);
  private themeService = inject(ThemeService);
  private notification = inject(NotificationService);
  private confirmDialogService = inject(ConfirmDialogService);

  formatSize = formatSize;

  largeFiles = signal<LargeFileItem[]>([]);
  loading = signal(false);
  selectedFiles = signal<Set<string>>(new Set());

  previewData = signal<FilePreviewData | null>(null);

  isLoadingPreview = signal(false);

  errorPreview = signal<string | null>(null);

  hasMore = signal(false);
  total = signal(0);

  totalSize = computed(() => this.largeFiles().reduce((sum, file) => sum + file.size, 0));

  columns: ListColumn[] = [
    {
      key: 'name',
      primary: true,
      icon: 'description',
      secondaryKey: 'path',
      badge: 'sizeDisplay',
      actions: [
        {
          id: 'preview',
          icon: 'visibility',
          tooltip: 'Preview file',
        },
      ],
    },
  ];

  options: ListOptions = {
    showSearch: true,
    showCheckbox: true,
    checkboxKey: 'path',
    showActions: true,
    actionsPosition: 'right',
    showReloadButton: true,
    showSelectAll: true,
    searchPlaceholder: 'Search large files...',
    emptyMessage: 'No large files found',
  };

  async ngOnInit() {
    await this.loadData();
  }

  async loadData() {
    this.loading.set(true);
    try {
      const result = await this.fileService.getLargeFiles(50, 0);
      this.largeFiles.set(result.data);
      this.hasMore.set(result.has_more);
      this.total.set(result.total);
    } catch (error) {
      console.error('Failed to load large files:', error);
    } finally {
      this.loading.set(false);
    }
  }

  async loadMore() {
    if (!this.hasMore() || this.loading()) return;

    this.loading.set(true);
    try {
      const offset = this.largeFiles().length;
      const result = await this.fileService.getLargeFiles(50, offset);
      this.largeFiles.update((current) => [...current, ...result.data]);
      this.hasMore.set(result.has_more);
      this.total.set(result.total);
    } catch (error) {
      console.error('Failed to load more large files:', error);
    } finally {
      this.loading.set(false);
    }
  }

  onSelectionChange(keys: Set<string>): void {
    this.selectedFiles.set(keys);
  }

  async clearSelectedFiles() {
    const selectedKeys = this.selectedFiles();
    const filesToClear = Array.from(selectedKeys);

    if (filesToClear.length === 0) return;

    const confirmed = await this.confirmDialogService.confirm({
      title: 'Clear Files',
      message: `Clear ${filesToClear.length} large file(s)?`,
    });
    if (!confirmed) return;

    try {
      this.loading.set(true);
      await this.fileService.clearSelectedLargeFiles(filesToClear);
      this.selectedFiles.set(new Set());
      await this.loadData();
    } catch (error) {
      this.notification.error('Failed to clear files', error);
    } finally {
      this.loading.set(false);
    }
  }

  onRowAction(event: { action: string; item: LargeFileItem }): void {
    if (event.action === 'preview') {
      this.onPreview(event.item);
    }
  }

  async onPreview(file: LargeFileItem): Promise<void> {
    this.isLoadingPreview.set(true);
    this.errorPreview.set(null);
    this.previewData.set({ name: file.name, path: file.path, type: 'unknown' });

    try {
      const result = await this.fileService.previewFile(file.path);

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
        name: file.name,
        path: file.path,
        type: 'error',
        error: errorMessage,
      });
    } finally {
      this.isLoadingPreview.set(false);
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
      this.notification.error('Failed to open file', error);
    }
  }

  getAccentGradient(): string {
    return this.themeService.getAccentGradient();
  }
}
