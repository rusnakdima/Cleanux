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
import { DataTableComponent } from '@components/data-table/data-table.component';
import { PaginationComponent } from '@components/pagination/pagination.component';

/* models */
import { TableColumn, TableOptions } from '@models/data-table.model';
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
    DataTableComponent,
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

  paginatedBackups = computed(() => {
    const data = this.backups();
    const start = (this.currentPage() - 1) * this.pageSize();
    return data.slice(start, start + this.pageSize());
  });

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
    const confirmed = confirm(
      `Restore backup "${backup.name}"?\n\nThis will extract files to their original location.`
    );
    if (!confirmed) return;

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
    const confirmed = confirm(`Delete backup "${backup.name}"?\n\nThis action cannot be undone.`);
    if (!confirmed) return;

    try {
      await this.backupService.deleteBackup(backup.path);
      await this.loadBackups();
    } catch (error) {
      this.notification.error('Failed to delete backup', error);
    }
  }

  isRestoring(path: string): boolean {
    return this.restoringId() === path;
  }

  onPageChange(page: number): void {
    this.currentPage.set(page);
  }

  onPageSizeChange(size: number): void {
    this.pageSize.set(size);
    this.currentPage.set(1);
  }

  columns: TableColumn[] = [
    { key: 'name', label: 'Name', width: 'flex-1', sortable: true },
    { key: 'size', label: 'Size', width: 'w-32', sortable: true, align: 'right' },
    { key: 'modified', label: 'Date', width: 'w-48', sortable: true, align: 'right' },
  ];

  getTableOptions(): TableOptions {
    return {
      showHeader: true,
      showCheckbox: false,
      hoverable: true,
      showReloadButton: true,
      showSelectedActions: false,
      showPreviewButton: false,
      showSearch: true,
      searchPlaceholder: 'Search backups...',
    };
  }

  onRestore(event: { action: string; item: BackupItem }): void {
    if (event.action === 'restore') {
      this.restoreBackup(event.item);
    }
  }

  onDelete(event: { action: string; item: BackupItem }): void {
    if (event.action === 'delete') {
      this.deleteBackup(event.item);
    }
  }

  trackByPath(index: number, backup: BackupItem): string {
    return backup.path;
  }
}
