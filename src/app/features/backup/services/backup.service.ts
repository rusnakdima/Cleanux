/* angular */
import { Injectable, inject } from '@angular/core';

/* services */
import { InvokeWrapperService } from '@tauri-front/shared';

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
  private api = inject(InvokeWrapperService);

  constructor() {}

  async createBackup(paths: string[], archivePath: string): Promise<string> {
    try {
      const result = await this.api.invoke<string>('create_backup', { paths, archivePath });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async restoreBackup(archivePath: string, destination: string): Promise<string> {
    try {
      const result = await this.api.invoke<string>('restore_backup', { archivePath, destination });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async listBackups(): Promise<BackupItem[]> {
    try {
      const result = await this.api.invoke<BackupItem[]>('list_backups');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async deleteBackup(archivePath: string): Promise<string> {
    try {
      const result = await this.api.invoke<string>('delete_backup', { archivePath });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async getBackupDir(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('get_backup_dir');
      return result;
    } catch (error) {
      throw error;
    }
  }
}
