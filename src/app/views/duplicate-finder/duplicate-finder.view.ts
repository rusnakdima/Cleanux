/* sys lib */
import {
  ChangeDetectionStrategy,
  Component,
  OnInit,
  signal,
  inject,
  DOCUMENT,
  computed,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* services */
import { DuplicateService } from '@services/duplicate.service';
import { FileService } from '@services/file.service';
import { NotificationService } from '@services/notification.service';

/* components */
import { DataListComponent } from '@components/data-list/data-list.component';
import { FilePreviewComponent } from '@components/file-preview/file-preview.component';
import { FilePreviewData } from '@models/file-preview.model';

/* models */
import { ListColumn, ListOptions } from '@models/data-list.model';
import { DuplicateGroup, DuplicateFile } from '@models/duplicate.model';
import { formatSize } from '@shared/utils/format.util';

interface FlattenedFile {
  name: string;
  path: string;
  size: number;
  hash: string;
  file_size: number;
  wasted_space: number;
  isOriginal: boolean;
}

@Component({
  selector: 'app-duplicate-finder-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    FormsModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    DataListComponent,
    FilePreviewComponent,
  ],
  templateUrl: './duplicate-finder.view.html',
})
export class DuplicateFinderView implements OnInit {
  private duplicateService = inject(DuplicateService);
  private fileService = inject(FileService);
  private document = inject(DOCUMENT);
  private notification = inject(NotificationService);

  formatSize = formatSize;

  duplicateGroups = signal<DuplicateGroup[]>([]);
  loading = signal(false);
  scanningPath = signal('');
  extensionFilter = signal('');

  previewData = signal<FilePreviewData | null>(null);
  isLoadingPreview = signal(false);
  selectedFiles = signal<Set<string>>(new Set());

  totalWastedSpace = signal(0);
  totalDuplicates = signal(0);

  columns: ListColumn[] = [
    {
      key: 'path',
      primary: true,
      icon: 'description',
      truncate: true,
      sortable: true,
    },
    {
      key: 'size',
      format: 'size',
      align: 'right',
      sortable: true,
    },
  ];

  options: ListOptions = {
    showCheckbox: true,
    checkboxKey: 'path',
    hoverable: true,
    showReloadButton: false,
    showSearch: true,
    searchPlaceholder: 'Search files...',
  };

  flattenedFiles = computed(() => {
    const files: FlattenedFile[] = [];
    for (const group of this.duplicateGroups()) {
      for (let i = 0; i < group.files.length; i++) {
        files.push({
          name: group.files[i].name,
          path: group.files[i].path,
          size: group.file_size,
          hash: group.hash,
          file_size: group.file_size,
          wasted_space: group.wasted_space,
          isOriginal: i === 0,
        });
      }
    }
    return files;
  });

  totalFiles = computed(() => {
    return this.flattenedFiles().length;
  });

  get isDark(): boolean {
    return this.document.body.classList.contains('dark');
  }

  async ngOnInit() {
    this.scanningPath.set('/home');
  }

  async scanForDuplicates() {
    if (!this.scanningPath()) return;

    this.loading.set(true);
    this.selectedFiles.set(new Set());

    try {
      const result = await this.duplicateService.findDuplicates(
        this.scanningPath(),
        this.extensionFilter() || undefined
      );
      this.duplicateGroups.set(result.groups);
      this.totalWastedSpace.set(result.totalWastedSpace);
      this.totalDuplicates.set(result.totalDuplicates);
    } catch (error) {
      this.notification.error('Failed to scan for duplicates', error);
    } finally {
      this.loading.set(false);
    }
  }

  onSelectionChange(keys: Set<string>): void {
    this.selectedFiles.set(keys);
  }

  onRowDoubleClick(file: FlattenedFile): void {
    this.onPreview(file as unknown as DuplicateFile);
  }

  async deleteSelectedFiles() {
    const selectedKeys = this.selectedFiles();
    const filesToDelete = Array.from(selectedKeys);

    if (filesToDelete.length === 0) return;

    const confirmed = confirm(`Delete ${filesToDelete.length} duplicate file(s)?`);
    if (!confirmed) return;

    try {
      this.loading.set(true);
      if (this.fileService.deleteFiles) {
        await this.fileService.deleteFiles(filesToDelete);
      } else {
        await this.duplicateService.deleteFiles(filesToDelete);
      }
      this.selectedFiles.set(new Set());
      await this.scanForDuplicates();
    } catch (error) {
      this.notification.error('Failed to delete files', error);
    } finally {
      this.loading.set(false);
    }
  }

  async onPreview(file: DuplicateFile): Promise<void> {
    this.isLoadingPreview.set(true);
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
      this.previewData.set({
        name: file.name,
        path: file.path,
        type: 'error',
        error: error instanceof Error ? error.message : 'Unable to preview file',
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
}
