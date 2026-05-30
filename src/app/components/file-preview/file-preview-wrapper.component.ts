import { Component, inject, ChangeDetectionStrategy } from '@angular/core';
import { FilePreviewService } from '@services/file-preview.service';
import { FilePreviewComponent } from './file-preview.component';

@Component({
  selector: 'app-file-preview-wrapper',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [FilePreviewComponent],
  template: `
    @if (previewService.isOpen()) {
      <app-file-preview
        [fileData]="previewService.previewData()"
        (close)="previewService.close()"
        (openInEditor)="previewService.openInEditor($event.path, $event.command)"
      />
    }
  `,
})
export class FilePreviewWrapperComponent {
  previewService = inject(FilePreviewService);
}
