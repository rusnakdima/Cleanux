import { signal, inject } from '@angular/core';
import { FileService } from '@services/file.service';
import { NotificationService } from '@services/notification.service';
import { FilePreviewData } from '@models/file-preview.model';

export interface PreviewableItem {
  path: string;
  name?: string;
}

export abstract class FilePreviewMixin {
  protected fileService = inject(FileService);
  protected notification = inject(NotificationService);

  isLoadingPreview = signal(false);
  previewData = signal<FilePreviewData | null>(null);
  errorPreview = signal<string | null>(null);

  async onPreview(item: PreviewableItem): Promise<void> {
    const path = item.path;
    const displayName = item.name ?? path.split('/').pop() ?? 'unknown';
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
