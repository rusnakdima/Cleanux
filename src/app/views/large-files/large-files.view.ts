/* sys lib */
import { Component, OnInit, signal, inject, computed } from '@angular/core';
import { CommonModule, DOCUMENT } from '@angular/common';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* services */
import { SystemService, LargeFileItem } from '@services/system.service';
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
  selector: 'app-large-files-view',
  standalone: true,
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
  private systemService = inject(SystemService);
  private mainService = inject(MainService);
  private document = inject(DOCUMENT);

  largeFiles = signal<LargeFileItem[]>([]);
  filteredFiles = signal<LargeFileItem[]>([]);
  loading = signal(false);
  selectedFiles = signal<Set<string>>(new Set());

  previewData = signal<FilePreviewData | null>(null);

  isLoadingPreview = signal(false);

  errorPreview = signal<string | null>(null);

  totalSize = computed(() => this.largeFiles().reduce((sum, file) => sum + file.size, 0));

  columns: TableColumn[] = [
    { key: 'name', label: 'Name', width: 'w-64', sortable: true },
    { key: 'path', label: 'Path', width: 'flex-1', sortable: true },
    { key: 'size', label: 'Size', align: 'right', width: 'w-32', sortable: true },
    { key: 'modified', label: 'Modified', width: 'w-48', sortable: true }
  ];

  get isDark(): boolean {
    return this.document.body.classList.contains('dark');
  }

  async ngOnInit() {
    await this.loadData();
  }

  async loadData() {
    this.loading.set(true);
    try {
      const files = await this.systemService.getLargeFiles();
      this.largeFiles.set(files);
      this.filteredFiles.set(files);
    } catch (error) {
      console.error('Failed to load large files:', error);
    } finally {
      this.loading.set(false);
    }
  }

  onFilteredData(files: LargeFileItem[]): void {
    this.filteredFiles.set(files);
  }

  async clearSelectedFiles() {
    const selectedKeys = this.selectedFiles();
    const filesToClear = Array.from(selectedKeys);

    if (filesToClear.length === 0) return;

    const confirmed = confirm(`Clear ${filesToClear.length} large file(s)?`);
    if (!confirmed) return;

    try {
      this.loading.set(true);
      await this.systemService.clearSelectedLargeFiles(filesToClear);
      this.selectedFiles.set(new Set());
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
      const result = await this.mainService.previewFile<{
        name: string;
        path: string;
        type: string;
        content?: string;
        imageUrl?: string;
      }>(file.path);

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
      await this.mainService.openFile(event.path, event.command);
    } catch (error: any) {
      alert('Failed to open file: ' + (error?.message || error));
    }
  }
}
