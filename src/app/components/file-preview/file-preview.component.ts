/* sys lib */
import { Component, Input, Output, EventEmitter, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DomSanitizer, SafeUrl } from '@angular/platform-browser';
import { FormsModule } from '@angular/forms';

/* materials */
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatMenuModule } from '@angular/material/menu';

/* models */
import { FilePreviewData } from '@models/file-preview.model';

@Component({
  selector: 'app-file-preview',
  standalone: true,
  imports: [CommonModule, MatIconModule, MatButtonModule, MatMenuModule, FormsModule],
  templateUrl: './file-preview.component.html',
})
export class FilePreviewComponent {
  @Input() set fileData(data: FilePreviewData | null) {
    this._file.set(data);
    if (data) {
      this.isLoading.set(false);
      this.showOpenWith.set(false);
    }
  }

  @Output() close = new EventEmitter<void>();
  @Output() openInEditor = new EventEmitter<{ path: string; command?: string }>();

  private sanitizer = inject(DomSanitizer);

  _file = signal<FilePreviewData | null>(null);
  isLoading = signal(true);
  showOpenWith = signal(false);
  customCommand = signal('');

  editors = [
    { name: 'VS Code', command: 'code', icon: 'code' },
    { name: 'Cursor', command: 'cursor', icon: 'terminal' },
    { name: 'Sublime Text', command: 'subl', icon: 'edit' },
    { name: 'Gedit', command: 'gedit', icon: 'description' },
    { name: 'System Default', command: '', icon: 'open_in_new' },
  ];

  file() {
    return this._file();
  }

  getFileIcon(): string {
    const file = this._file();
    if (!file) return 'description';

    switch (file.type) {
      case 'image': return 'image';
      case 'text': return 'description';
      case 'binary': return 'code';
      case 'error': return 'error_outline';
      default: return 'insert_drive_file';
    }
  }

  getSanitizedImageUrl(): SafeUrl | string {
    const file = this._file();
    if (!file?.imageUrl) return '';
    return this.sanitizer.bypassSecurityTrustUrl(file.imageUrl);
  }

  onClose(): void {
    this.close.emit();
  }

  toggleOpenWith(): void {
    this.showOpenWith.update(v => !v);
  }

  openWith(command?: string): void {
    const file = this._file();
    if (file) {
      this.openInEditor.emit({ path: file.path, command });
      this.showOpenWith.set(false);
    }
  }

  openCustom(): void {
    if (this.customCommand()) {
      this.openWith(this.customCommand());
    }
  }
}
