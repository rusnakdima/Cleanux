/* sys lib */
import { Injectable } from '@angular/core';

/* services */
import { BaseApiService } from './base-api.service';
import { BackupItem } from '@models/backup.model';

@Injectable({
  providedIn: 'root',
})
export class BackupService extends BaseApiService {
  async createBackup(paths: string[], archivePath: string): Promise<string> {
    return await this.call<string>('create_backup', { paths, archivePath });
  }

  async restoreBackup(archivePath: string, destination: string): Promise<string> {
    return await this.call<string>('restore_backup', { archivePath, destination });
  }

  async listBackups(): Promise<BackupItem[]> {
    return await this.call<BackupItem[]>('list_backups');
  }

  async deleteBackup(archivePath: string): Promise<string> {
    return await this.call<string>('delete_backup', { archivePath });
  }

  async getBackupDir(): Promise<string> {
    return await this.call<string>('get_backup_dir');
  }
}
