import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';
import { LoggingService, getLoggingService } from '@tauri-apps/logger';

export interface RepairItem {
  path: string;
  issue_type: string;
  severity: 'info' | 'warning' | 'error';
  description?: string;
}

export interface RepairResult {
  removed?: number;
  failed?: string[];
  repaired?: number;
  cleanedPaths?: string[];
}

@Injectable({
  providedIn: 'root',
})
export class RepairService {
  private api = inject(ApiService);
  private loggingService = getLoggingService();

  constructor() {
    this.loggingService.info('RepairService initialized');
  }

  async findBrokenSymlinks(): Promise<RepairItem[]> {
    this.loggingService.info('Finding broken symlinks');
    try {
      const result = await this.api.invoke<RepairItem[]>('find_broken_symlinks');
      this.loggingService.info('Broken symlinks found', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async findOrphanedPackages(): Promise<RepairItem[]> {
    this.loggingService.info('Finding orphaned packages');
    try {
      const result = await this.api.invoke<RepairItem[]>('find_orphaned_packages');
      this.loggingService.info('Orphaned packages found', { count: result.length });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanFontCache(): Promise<RepairResult> {
    this.loggingService.info('Cleaning font cache');
    try {
      const result = await this.api.invoke<RepairResult>('clean_font_cache');
      this.loggingService.info('Font cache cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanIconCache(): Promise<RepairResult> {
    this.loggingService.info('Cleaning icon cache');
    try {
      const result = await this.api.invoke<RepairResult>('clean_icon_cache');
      this.loggingService.info('Icon cache cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async repairPermissions(): Promise<RepairResult> {
    this.loggingService.info('Repairing permissions');
    try {
      const result = await this.api.invoke<RepairResult>('repair_permissions');
      this.loggingService.info('Permissions repaired');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async removeBrokenSymlink(path: string): Promise<boolean> {
    this.loggingService.info('Removing broken symlink', { path });
    try {
      const result = await this.api.invoke<boolean>('remove_broken_symlink', { path });
      this.loggingService.info('Broken symlink removed');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { path });
      throw error;
    }
  }

  async removeOrphanedPackage(path: string): Promise<boolean> {
    this.loggingService.info('Removing orphaned package', { path });
    try {
      const result = await this.api.invoke<boolean>('remove_orphaned_package', { path });
      this.loggingService.info('Orphaned package removed');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { path });
      throw error;
    }
  }
}
