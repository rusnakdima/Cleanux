import { Component, inject, ChangeDetectionStrategy } from '@angular/core';
import { FilePreviewService } from '@services/file-preview.service';
import { FilePreviewComponent } from './file-preview.component';

@Component({
  selector: 'app-file-preview-wrapper',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [FilePreviewComponent],
  templateUrl: './file-preview-wrapper.component.html',
})
export class FilePreviewWrapperComponent {
  previewService = inject(FilePreviewService);
}
