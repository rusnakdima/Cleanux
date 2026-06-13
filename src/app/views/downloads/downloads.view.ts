/* sys lib */
import { ChangeDetectionStrategy, Component, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { formatSize } from '@shared/utils/format.util';
import { DOWNLOAD_DELAY_MS } from '@shared/utils/constants';

import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { ConfirmDialogService } from '@shared/confirm-dialog';

import { DataListComponent } from '@components/data-list/data-list.component';
import { ListColumn, ListOptions } from '@models/data-list.model';

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
  imports: [CommonModule, FormsModule, MatIconModule, MatProgressSpinnerModule, DataListComponent],
  templateUrl: './downloads.view.html',
})
export class DownloadsView {
  private confirmDialogService = inject(ConfirmDialogService);

  formatSize = formatSize;
  isScanning = signal(false);
  downloadsSize = signal(0);
  files = signal<DownloadFile[]>([]);
  selectedFiles = signal<Set<string>>(new Set());

  columns: ListColumn[] = [
    {
      key: 'name',
      primary: true,
      icon: 'description',
      secondary: 'path',
      timestamp: 'age',
      format: 'number',
    },
  ];

  options: ListOptions = {
    showSearch: false,
    showCheckbox: true,
    checkboxKey: 'path',
    showActions: false,
    emptyMessage: 'No files found in Downloads',
  };

  async scanDownloads() {
    this.isScanning.set(true);
    this.files.set([]);
    this.selectedFiles.set(new Set());

    await new Promise((resolve) => setTimeout(resolve, DOWNLOAD_DELAY_MS));

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

  onSelectionChange(keys: Set<string>) {
    this.selectedFiles.set(keys);
  }

  async cleanSelected() {
    const count = this.selectedFiles().size;
    if (
      await this.confirmDialogService.confirm({
        title: 'Delete Files',
        message: `Delete ${count} selected files? This cannot be undone.`,
        dangerous: true,
      })
    ) {
      this.files.set([]);
      this.selectedFiles.set(new Set());
      this.downloadsSize.set(0);
    }
  }
}
