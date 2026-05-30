import { Injectable, signal, computed, inject } from '@angular/core';
import { FilePreviewData } from '@models/file-preview.model';
import { FileService } from '@services/file.service';

@Injectable({
  providedIn: 'root',
})
export class FilePreviewService {
  private fileService = inject(FileService);

  readonly previewData = signal<FilePreviewData | null>(null);
  readonly isOpen = signal(false);
  readonly isLoading = signal(false);

  readonly file = computed(() => this.previewData());

  async open(path: string): Promise<void> {
    this.isLoading.set(true);
    this.isOpen.set(true);
    this.previewData.set({
      name: path.split('/').pop() || 'unknown',
      path,
      type: 'unknown',
    });

    try {
      const result = await this.fileService.previewFile(path);
      this.previewData.set({
        name: result.name,
        path: result.path,
        type: result.type as 'image' | 'text' | 'binary' | 'unknown' | 'error',
        content: result.content,
        imageUrl: result.imageUrl,
      });
    } catch (error: unknown) {
      this.previewData.set({
        name: path.split('/').pop() || 'unknown',
        path,
        type: 'error',
        error: error instanceof Error ? error.message : 'Failed to load preview',
      });
    } finally {
      this.isLoading.set(false);
    }
  }

  close(): void {
    this.isOpen.set(false);
    this.previewData.set(null);
  }

  openInEditor(path: string, command?: string): void {
    this.fileService.openFile(path, command);
  }
}
