/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';

export interface BackupItem {
  name: string;
  path: string;
  size: number;
  modified: string;
}

@Injectable({
  providedIn: 'root',
})
export class BackupService {
  private api = inject(ApiService);

  async createBackup(paths: string[], archivePath: string): Promise<string> {
    return await this.api.invoke<string>('create_backup', { paths, archivePath });
  }

  async restoreBackup(archivePath: string, destination: string): Promise<string> {
    return await this.api.invoke<string>('restore_backup', { archivePath, destination });
  }

  async listBackups(): Promise<BackupItem[]> {
    return await this.api.invoke<BackupItem[]>('list_backups');
  }

  async deleteBackup(archivePath: string): Promise<string> {
    return await this.api.invoke<string>('delete_backup', { archivePath });
  }

  async getBackupDir(): Promise<string> {
    return await this.api.invoke<string>('get_backup_dir');
  }
}
