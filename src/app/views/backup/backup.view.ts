/* sys lib */
import { Component, OnInit, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* services */
import { BackupService, BackupItem } from '@services/backup.service';

/* components */
import { HeaderComponent } from '@components/header/header.component';

/* models */
import { formatSize } from '@shared/utils/format.util';

@Component({
  selector: 'app-backup-view',
  standalone: true,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    HeaderComponent,
  ],
  templateUrl: './backup.view.html',
})
export class BackupView implements OnInit {
  private backupService = inject(BackupService);

  formatSize = formatSize;

  backups = signal<BackupItem[]>([]);
  loading = signal(false);
  restoringId = signal<string | null>(null);

  async ngOnInit() {
    await this.loadBackups();
  }

  async loadBackups() {
    this.loading.set(true);
    try {
      const backups = await this.backupService.listBackups();
      this.backups.set(backups);
    } catch (error) {
      console.error('Failed to load backups:', error);
      alert('Failed to load backups: ' + (error instanceof Error ? error.message : String(error)));
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
      alert('Backup restored successfully to /tmp/cleanux_restore');
    } catch (error) {
      alert(
        'Failed to restore backup: ' + (error instanceof Error ? error.message : String(error))
      );
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
      alert('Failed to delete backup: ' + (error instanceof Error ? error.message : String(error)));
    }
  }

  isRestoring(path: string): boolean {
    return this.restoringId() === path;
  }
}
