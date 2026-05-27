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

/* components */
import { DataTableComponent } from '@components/data-table/data-table.component';
import { FilePreviewComponent } from '@components/file-preview/file-preview.component';
import { FilePreviewData } from '@models/file-preview.model';
import { HeaderComponent } from '@components/header/header.component';
import { SearchComponent } from '@components/search/search.component';

/* models */
import { TableColumn, TableOptions } from '@models/data-table.model';
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
    DataTableComponent,
    FilePreviewComponent,
    HeaderComponent,
    SearchComponent,
  ],
  templateUrl: './large-files.view.html',
})
export class LargeFilesView implements OnInit {
  private fileService = inject(FileService);

  formatSize = formatSize;

  largeFiles = signal<LargeFileItem[]>([]);
  filteredFiles = signal<LargeFileItem[]>([]);
  loading = signal(false);
  selectedFiles = signal<Set<string>>(new Set());

  previewData = signal<FilePreviewData | null>(null);

  isLoadingPreview = signal(false);

  errorPreview = signal<string | null>(null);

  hasMore = signal(false);
  total = signal(0);

  totalSize = computed(() => this.largeFiles().reduce((sum, file) => sum + file.size, 0));

  columns: TableColumn[] = [
    { key: 'name', label: 'Name', width: 'w-64', sortable: true },
    { key: 'path', label: 'Path', width: 'flex-1', sortable: true },
    { key: 'size', label: 'Size', align: 'right', width: 'w-32', sortable: true },
    { key: 'modified', label: 'Modified', width: 'w-48', sortable: true },
  ];

  async ngOnInit() {
    await this.loadData();
  }

  async loadData() {
    this.loading.set(true);
    try {
      const result = await this.fileService.getLargeFiles(50, 0);
      this.largeFiles.set(result.data);
      this.filteredFiles.set(result.data);
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
      this.largeFiles.update(current => [...current, ...result.data]);
      this.filteredFiles.update(current => [...current, ...result.data]);
      this.hasMore.set(result.has_more);
      this.total.set(result.total);
    } catch (error) {
      console.error('Failed to load more large files:', error);
    } finally {
      this.loading.set(false);
    }
  }

  onFilteredData(files: object[]): void {
    this.filteredFiles.set(files as LargeFileItem[]);
  }

  async clearSelectedFiles() {
    const selectedKeys = this.selectedFiles();
    const filesToClear = Array.from(selectedKeys);

    if (filesToClear.length === 0) return;

    const confirmed = confirm(`Clear ${filesToClear.length} large file(s)?`);
    if (!confirmed) return;

    try {
      this.loading.set(true);
      await this.fileService.clearSelectedLargeFiles(filesToClear);
      this.selectedFiles.set(new Set());
      await this.loadData();
    } catch (error) {
      alert('Failed to clear files: ' + error);
    } finally {
      this.loading.set(false);
    }
  }

  getTableOptions(): TableOptions {
    return {
      showHeader: true,
      showCheckbox: true,
      checkboxKey: 'path',
      hoverable: true,
      showReloadButton: true,
      showSelectedActions: true,
      selectedActionText: 'Delete Selected',
      showPreviewButton: true,
    };
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
      alert('Failed to open file: ' + (error instanceof Error ? error.message : String(error)));
    }
  }
}
