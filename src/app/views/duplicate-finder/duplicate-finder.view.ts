/* sys lib */
import {
  ChangeDetectionStrategy,
  Component,
  OnInit,
  signal,
  inject,
  DOCUMENT,
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

/* components */
import { FilePreviewComponent } from '@components/file-preview/file-preview.component';
import { FilePreviewData } from '@models/file-preview.model';
import { HeaderComponent } from '@components/header/header.component';
import { SearchComponent } from '@components/search/search.component';

/* models */
import { DuplicateGroup } from '@models/duplicate.model';
import { formatSize } from '@shared/utils/format.util';

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
    FilePreviewComponent,
    HeaderComponent,
    SearchComponent,
  ],
  templateUrl: './duplicate-finder.view.html',
})
export class DuplicateFinderView implements OnInit {
  private duplicateService = inject(DuplicateService);
  private fileService = inject(FileService);
  private document = inject(DOCUMENT);

  duplicateGroups = signal<DuplicateGroup[]>([]);
  filteredGroups = signal<DuplicateGroup[]>([]);
  loading = signal(false);
  selectedFiles = signal<Set<string>>(new Set());
  scanningPath = signal('');
  extensionFilter = signal('');
  showExtensionInput = signal(false);

  previewData = signal<FilePreviewData | null>(null);
  isLoadingPreview = signal(false);

  totalWastedSpace = signal(0);
  totalDuplicates = signal(0);

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
      this.filteredGroups.set(result.groups);
      this.totalWastedSpace.set(result.totalWastedSpace);
      this.totalDuplicates.set(result.totalDuplicates);
    } catch (error) {
      console.error('Failed to scan for duplicates:', error);
      alert('Failed to scan: ' + (error instanceof Error ? error.message : String(error)));
    } finally {
      this.loading.set(false);
    }
  }

  onFilteredData(groups: object[]): void {
    this.filteredGroups.set(groups as DuplicateGroup[]);
  }

  toggleExtensionInput(): void {
    this.showExtensionInput.set(!this.showExtensionInput());
    if (!this.showExtensionInput()) {
      this.extensionFilter.set('');
    }
  }

  async deleteSelectedFiles() {
    const selectedKeys = this.selectedFiles();
    const filesToDelete = Array.from(selectedKeys);

    if (filesToDelete.length === 0) return;

    const confirmed = confirm(`Delete ${filesToDelete.length} duplicate file(s)?`);
    if (!confirmed) return;

    try {
      this.loading.set(true);
      (await this.fileService.deleteFiles)
        ? await this.fileService.deleteFiles(filesToDelete)
        : await this.duplicateService.deleteFiles(filesToDelete);
      this.selectedFiles.set(new Set());
      await this.scanForDuplicates();
    } catch (error) {
      alert('Failed to delete files: ' + (error instanceof Error ? error.message : String(error)));
    } finally {
      this.loading.set(false);
    }
  }

  async onPreview(file: { path: string; name: string }): Promise<void> {
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
      alert('Failed to open file: ' + (error instanceof Error ? error.message : String(error)));
    }
  }

  formatSize(bytes: number): string {
    return formatSize(bytes);
  }

  toggleFileSelection(path: string): void {
    this.selectedFiles.update((set) => {
      const newSet = new Set(set);
      if (newSet.has(path)) {
        newSet.delete(path);
      } else {
        newSet.add(path);
      }
      return newSet;
    });
  }
}
