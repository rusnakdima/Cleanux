/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';
import { LoggerService } from '@services/logger.service';

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
  private loggingService = new LoggerService();

  constructor() {
    this.loggingService.info('BackupService initialized');
  }

  async createBackup(paths: string[], archivePath: string): Promise<string> {
    this.loggingService.info('Creating backup', { pathsCount: paths.length, archivePath });
    try {
      const result = await this.api.invoke<string>('create_backup', { paths, archivePath });
      this.loggingService.info('Backup created');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async restoreBackup(archivePath: string, destination: string): Promise<string> {
    this.loggingService.info('Restoring backup', { archivePath, destination });
    try {
      const result = await this.api.invoke<string>('restore_backup', { archivePath, destination });
      this.loggingService.info('Backup restored');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { archivePath, destination });
      throw error;
    }
  }

  async listBackups(): Promise<BackupItem[]> {
    this.loggingService.info('Listing backups');
    try {
      const result = await this.api.invoke<BackupItem[]>('list_backups');
      this.loggingService.info('Backups listed', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async deleteBackup(archivePath: string): Promise<string> {
    this.loggingService.info('Deleting backup', { archivePath });
    try {
      const result = await this.api.invoke<string>('delete_backup', { archivePath });
      this.loggingService.info('Backup deleted');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { archivePath });
      throw error;
    }
  }

  async getBackupDir(): Promise<string> {
    this.loggingService.info('Getting backup directory');
    try {
      const result = await this.api.invoke<string>('get_backup_dir');
      this.loggingService.info('Backup directory retrieved', { path: result });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }
}
