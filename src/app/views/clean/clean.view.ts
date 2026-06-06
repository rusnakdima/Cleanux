/* sys lib */
import { ChangeDetectionStrategy, Component, signal, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';

/* router */
import { RouterLink } from '@angular/router';

/* services */
import { FileService } from '@services/file.service';
import { NotificationService } from '@services/notification.service';
import { ConfirmDialogService } from '@shared/confirm-dialog';
import { formatSize } from '@shared/utils/format.util';

type CleanCategory = 'cache' | 'trash' | 'logs' | 'packages';

@Component({
  selector: 'app-clean-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, MatProgressSpinnerModule, RouterLink],
  templateUrl: './clean.view.html',
})
export class CleanView implements OnInit {
  private fileService = inject(FileService);
  private notification = inject(NotificationService);
  private confirmDialogService = inject(ConfirmDialogService);

  formatSize = formatSize;

  activeCategory = signal<CleanCategory>('cache');
  isScanning = signal(false);
  isCleaning = signal(false);

  cacheSize = signal(0);
  cacheCount = signal(0);
  trashSize = signal(0);
  trashCount = signal(0);
  logSize = signal(0);
  logCount = signal(0);
  packageCacheSize = signal(0);
  packageManagerCount = signal(0);

  selectedItems = signal<string[]>([]);

  categories = [
    { id: 'cache' as CleanCategory, label: 'Cache', icon: 'cached', color: 'cyan' },
    { id: 'trash' as CleanCategory, label: 'Trash', icon: 'delete_outline', color: 'error' },
    { id: 'logs' as CleanCategory, label: 'Logs', icon: 'description', color: 'info' },
    { id: 'packages' as CleanCategory, label: 'Packages', icon: 'package', color: 'warning' },
  ];

  async ngOnInit() {
    await this.loadStats();
  }

  async loadStats() {
    this.isScanning.set(true);
    try {
      const [cache, trash, logs] = await Promise.all([
        this.fileService.getCacheSummary(),
        this.fileService.getTrashSummary(),
        this.fileService.getLogSummary(),
      ]);

      this.cacheSize.set(cache.totalSize);
      this.cacheCount.set(cache.fileCount);
      this.trashSize.set(trash.totalSize);
      this.trashCount.set(trash.fileCount);
      this.logSize.set(logs.totalSize);
      this.logCount.set(logs.fileCount);
      this.packageCacheSize.set(0);
      this.packageManagerCount.set(0);
    } catch (e) {
      console.error('Failed to load stats:', e);
    } finally {
      this.isScanning.set(false);
    }
  }

  getCategorySize(category: CleanCategory): number {
    switch (category) {
      case 'cache':
        return this.cacheSize();
      case 'trash':
        return this.trashSize();
      case 'logs':
        return this.logSize();
      case 'packages':
        return this.packageCacheSize();
    }
  }

  getCategoryCount(category: CleanCategory): number {
    switch (category) {
      case 'cache':
        return this.cacheCount();
      case 'trash':
        return this.trashCount();
      case 'logs':
        return this.logCount();
      case 'packages':
        return this.packageManagerCount();
    }
  }

  getCategoryColor(category: CleanCategory): string {
    const cat = this.categories.find((c) => c.id === category);
    return cat?.color || 'cyan';
  }

  async onCleanCategory(category: CleanCategory) {
    const confirmed = await this.confirmDialogService.confirm({
      title: 'Clean Category',
      message: `Clean all ${category}? This action cannot be undone.`,
      dangerous: true,
    });
    if (!confirmed) return;

    this.isCleaning.set(true);
    try {
      switch (category) {
        case 'cache':
          await this.fileService.clearCache();
          this.cacheSize.set(0);
          this.cacheCount.set(0);
          break;
        case 'trash':
          await this.fileService.clearTrash();
          this.trashSize.set(0);
          this.trashCount.set(0);
          break;
        case 'logs':
          await this.fileService.clearAllLogs();
          this.logSize.set(0);
          this.logCount.set(0);
          break;
        case 'packages':
          break;
      }
    } catch (e) {
      console.error('Clean failed:', e);
      this.notification.error('Clean operation failed', e as Error);
    } finally {
      this.isCleaning.set(false);
    }
  }

  async onCleanAll() {
    const totalSize = this.cacheSize() + this.trashSize() + this.logSize();
    if (totalSize === 0) return;

    const confirmed = await this.confirmDialogService.confirm({
      title: 'Clean All',
      message: `Clean all items (${this.formatSize(totalSize)})? This action cannot be undone.`,
      dangerous: true,
    });
    if (!confirmed) return;

    this.isCleaning.set(true);
    try {
      await Promise.all([
        this.fileService.clearCache(),
        this.fileService.clearTrash(),
        this.fileService.clearAllLogs(),
      ]);

      this.cacheSize.set(0);
      this.cacheCount.set(0);
      this.trashSize.set(0);
      this.trashCount.set(0);
      this.logSize.set(0);
      this.logCount.set(0);
    } catch (e) {
      console.error('Clean all failed:', e);
      this.notification.error('Clean operation failed', e as Error);
    } finally {
      this.isCleaning.set(false);
    }
  }

  get totalCleanableSize(): number {
    return this.cacheSize() + this.trashSize() + this.logSize();
  }

  get totalItemCount(): number {
    return this.cacheCount() + this.trashCount() + this.logCount();
  }
}
