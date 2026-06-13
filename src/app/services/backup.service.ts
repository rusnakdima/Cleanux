/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from './api.service';
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
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'BackupService', 'init', 'BackupService initialized');
  }

  async createBackup(paths: string[], archivePath: string): Promise<string> {
    this.logger.logInfo('service', 'BackupService', 'createBackup', 'Creating backup', {
      pathsCount: paths.length,
      archivePath,
    });
    try {
      const result = await this.api.invoke<string>('create_backup', { paths, archivePath });
      this.logger.logInfo('service', 'BackupService', 'createBackup', 'Backup created');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'BackupService',
        'createBackup',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async restoreBackup(archivePath: string, destination: string): Promise<string> {
    this.logger.logInfo('service', 'BackupService', 'restoreBackup', 'Restoring backup', {
      archivePath,
      destination,
    });
    try {
      const result = await this.api.invoke<string>('restore_backup', { archivePath, destination });
      this.logger.logInfo('service', 'BackupService', 'restoreBackup', 'Backup restored');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'BackupService',
        'restoreBackup',
        'Operation failed',
        error as Error,
        { archivePath, destination }
      );
      throw error;
    }
  }

  async listBackups(): Promise<BackupItem[]> {
    this.logger.logInfo('service', 'BackupService', 'listBackups', 'Listing backups');
    try {
      const result = await this.api.invoke<BackupItem[]>('list_backups');
      this.logger.logInfo('service', 'BackupService', 'listBackups', 'Backups listed', {
        count: result.length,
      });
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'BackupService',
        'listBackups',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async deleteBackup(archivePath: string): Promise<string> {
    this.logger.logInfo('service', 'BackupService', 'deleteBackup', 'Deleting backup', {
      archivePath,
    });
    try {
      const result = await this.api.invoke<string>('delete_backup', { archivePath });
      this.logger.logInfo('service', 'BackupService', 'deleteBackup', 'Backup deleted');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'BackupService',
        'deleteBackup',
        'Operation failed',
        error as Error,
        { archivePath }
      );
      throw error;
    }
  }

  async getBackupDir(): Promise<string> {
    this.logger.logInfo('service', 'BackupService', 'getBackupDir', 'Getting backup directory');
    try {
      const result = await this.api.invoke<string>('get_backup_dir');
      this.logger.logInfo(
        'service',
        'BackupService',
        'getBackupDir',
        'Backup directory retrieved',
        { path: result }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'BackupService',
        'getBackupDir',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }
}
