/* sys lib */
import { ChangeDetectionStrategy, Component, signal, inject, computed } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';

/* router */
import { RouterLink } from '@angular/router';

/* services */
import { MonitorStore } from '@store/monitor.store';
import { FileService } from '@services/file.service';
import { formatSize } from '@shared/utils/format.util';

@Component({
  selector: 'app-files-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, RouterLink],
  templateUrl: './files.view.html',
})
export class FilesView {
  private fileService = inject(FileService);
  protected monitorStore = inject(MonitorStore);

  formatSize = formatSize;

  cacheSize = signal(0);
  trashSize = signal(0);
  largeFileSize = signal(0);

  fileTools = [
    {
      id: 'large-files',
      label: 'Large Files',
      icon: 'file_present',
      route: '/large-files',
      color: 'cyan',
    },
    {
      id: 'duplicate-finder',
      label: 'Duplicates',
      icon: 'content_copy',
      route: '/duplicate-finder',
      color: 'violet',
    },
    {
      id: 'disk-usage',
      label: 'Disk Usage',
      icon: 'pie_chart',
      route: '/disk-usage',
      color: 'emerald',
    },
  ];

  constructor() {
    this.loadStats();
  }

  async loadStats() {
    try {
      const cache = await this.fileService.getCacheSummary();
      this.cacheSize.set(cache.totalSize);
    } catch {}

    try {
      const trash = await this.fileService.getTrashSummary();
      this.trashSize.set(trash.totalSize);
    } catch {}

    try {
      const large = await this.fileService.getLargeFilesSummary();
      this.largeFileSize.set(large.totalSize);
    } catch {}
  }
}
