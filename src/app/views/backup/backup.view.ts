/* sys lib */
import { Component, OnInit, signal, inject, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* services */
import { BackupService, BackupItem } from '@services/backup.service';
import { NotificationService } from '@services/notification.service';

/* components */
import { DataListComponent } from '@components/data-list/data-list.component';

/* models */
import { ListColumn, ListOptions } from '@models/data-list.model';
import { formatSize } from '@shared/utils/format.util';

@Component({
  selector: 'app-backup-view',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    DataListComponent,
  ],
  templateUrl: './backup.view.html',
})
export class BackupView implements OnInit {
  private backupService = inject(BackupService);
  private notification = inject(NotificationService);

  formatSize = formatSize;

  backups = signal<BackupItem[]>([]);
  loading = signal(false);
  restoringId = signal<string | null>(null);

  currentPage = signal(1);
  pageSize = signal(15);

  columns: ListColumn[] = [
    {
      key: 'name',
      primary: true,
      icon: 'backup',
      secondaryKey: 'modified',
      actions: [
        {
          id: 'restore',
          icon: 'restore',
          tooltip: 'Restore backup',
          confirmMessage: 'Restore backup? Files will be extracted to original location.',
        },
        {
          id: 'delete',
          icon: 'delete',
          tooltip: 'Delete backup',
          class: 'action-btn text-error',
          confirmMessage: 'Delete backup? This action cannot be undone.',
        },
      ],
    },
  ];

  options: ListOptions = {
    showSearch: true,
    showCheckbox: false,
    showActions: true,
    actionsPosition: 'right',
    showReloadButton: true,
    searchPlaceholder: 'Search backups...',
    emptyMessage: 'No backups found',
  };

  async ngOnInit() {
    await this.loadBackups();
  }

  async loadBackups() {
    this.loading.set(true);
    try {
      const backups = await this.backupService.listBackups();
      this.backups.set(backups);
      this.currentPage.set(1);
    } catch (error) {
      this.notification.error('Failed to load backups', error);
    } finally {
      this.loading.set(false);
    }
  }

  formatDate(dateStr: string): string {
    if (!dateStr) return 'Unknown';
    try {
      return new Date(dateStr).toLocaleString();
    } catch {
      return dateStr;
    }
  }

  async restoreBackup(backup: BackupItem) {
    this.restoringId.set(backup.path);
    try {
      await this.backupService.restoreBackup(backup.path, '/tmp/cleanux_restore');
      this.notification.success('Backup restored successfully to /tmp/cleanux_restore');
    } catch (error) {
      this.notification.error('Failed to restore backup', error);
    } finally {
      this.restoringId.set(null);
    }
  }

  async deleteBackup(backup: BackupItem) {
    try {
      await this.backupService.deleteBackup(backup.path);
      await this.loadBackups();
    } catch (error) {
      this.notification.error('Failed to delete backup', error);
    }
  }

  onPageChange(page: number): void {
    this.currentPage.set(page);
  }

  onPageSizeChange(size: number): void {
    this.pageSize.set(size);
    this.currentPage.set(1);
  }

  onRowAction(event: { action: string; item: BackupItem }): void {
    if (event.action === 'restore') {
      this.restoreBackup(event.item);
    } else if (event.action === 'delete') {
      this.deleteBackup(event.item);
    }
  }
}
