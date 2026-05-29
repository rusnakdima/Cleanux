/* sys lib */
import { ChangeDetectionStrategy, Component, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { formatSize } from '@shared/utils/format.util';

/* materials */
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';

interface DownloadFile {
  path: string;
  name: string;
  size: number;
  age: string;
}

@Component({
  selector: 'app-downloads-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, FormsModule, MatIconModule, MatProgressSpinnerModule],
  templateUrl: './downloads.view.html',
})
export class DownloadsView {
  formatSize = formatSize;
  isScanning = signal(false);
  downloadsSize = signal(0);
  files = signal<DownloadFile[]>([]);
  selectedFiles = signal<Set<string>>(new Set());
  selectedAge = 'week';

  async scanDownloads() {
    this.isScanning.set(true);
    this.files.set([]);
    this.selectedFiles.set(new Set());

    await new Promise((resolve) => setTimeout(resolve, 1500));

    this.files.set([
      {
        path: '/home/user/Downloads/archive.zip',
        name: 'archive.zip',
        size: 156 * 1024 * 1024,
        age: '2 weeks ago',
      },
      {
        path: '/home/user/Downloads/installer.deb',
        name: 'installer.deb',
        size: 89 * 1024 * 1024,
        age: '3 weeks ago',
      },
      {
        path: '/home/user/Downloads/photo.jpg',
        name: 'photo.jpg',
        size: 4.5 * 1024 * 1024,
        age: '1 month ago',
      },
    ]);

    this.downloadsSize.set(250 * 1024 * 1024);
    this.isScanning.set(false);
  }

  toggleFile(path: string) {
    const selected = new Set(this.selectedFiles());
    if (selected.has(path)) {
      selected.delete(path);
    } else {
      selected.add(path);
    }
    this.selectedFiles.set(selected);
  }

  cleanSelected() {
    const count = this.selectedFiles().size;
    if (confirm(`Delete ${count} selected files? This cannot be undone.`)) {
      this.files.set([]);
      this.selectedFiles.set(new Set());
      this.downloadsSize.set(0);
    }
  }
}
