import { Injectable, inject } from '@angular/core';
import { ApiService } from './api.service';
import { LoggerService } from '@services/logger.service';

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
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'RepairService', 'init', 'RepairService initialized');
  }

  async findBrokenSymlinks(): Promise<RepairItem[]> {
    this.logger.logInfo(
      'service',
      'RepairService',
      'findBrokenSymlinks',
      'Finding broken symlinks'
    );
    try {
      const result = await this.api.invoke<RepairItem[]>('find_broken_symlinks');
      this.logger.logInfo(
        'service',
        'RepairService',
        'findBrokenSymlinks',
        'Broken symlinks found',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'RepairService',
        'findBrokenSymlinks',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async findOrphanedPackages(): Promise<RepairItem[]> {
    this.logger.logInfo(
      'service',
      'RepairService',
      'findOrphanedPackages',
      'Finding orphaned packages'
    );
    try {
      const result = await this.api.invoke<RepairItem[]>('find_orphaned_packages');
      this.logger.logInfo(
        'service',
        'RepairService',
        'findOrphanedPackages',
        'Orphaned packages found',
        { count: result.length }
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'RepairService',
        'findOrphanedPackages',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanFontCache(): Promise<RepairResult> {
    this.logger.logInfo('service', 'RepairService', 'cleanFontCache', 'Cleaning font cache');
    try {
      const result = await this.api.invoke<RepairResult>('clean_font_cache');
      this.logger.logInfo('service', 'RepairService', 'cleanFontCache', 'Font cache cleaned');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'RepairService',
        'cleanFontCache',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanIconCache(): Promise<RepairResult> {
    this.logger.logInfo('service', 'RepairService', 'cleanIconCache', 'Cleaning icon cache');
    try {
      const result = await this.api.invoke<RepairResult>('clean_icon_cache');
      this.logger.logInfo('service', 'RepairService', 'cleanIconCache', 'Icon cache cleaned');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'RepairService',
        'cleanIconCache',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async repairPermissions(): Promise<RepairResult> {
    this.logger.logInfo('service', 'RepairService', 'repairPermissions', 'Repairing permissions');
    try {
      const result = await this.api.invoke<RepairResult>('repair_permissions');
      this.logger.logInfo('service', 'RepairService', 'repairPermissions', 'Permissions repaired');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'RepairService',
        'repairPermissions',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async removeBrokenSymlink(path: string): Promise<boolean> {
    this.logger.logInfo(
      'service',
      'RepairService',
      'removeBrokenSymlink',
      'Removing broken symlink',
      { path }
    );
    try {
      const result = await this.api.invoke<boolean>('remove_broken_symlink', { path });
      this.logger.logInfo(
        'service',
        'RepairService',
        'removeBrokenSymlink',
        'Broken symlink removed'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'RepairService',
        'removeBrokenSymlink',
        'Operation failed',
        error as Error,
        { path }
      );
      throw error;
    }
  }

  async removeOrphanedPackage(path: string): Promise<boolean> {
    this.logger.logInfo(
      'service',
      'RepairService',
      'removeOrphanedPackage',
      'Removing orphaned package',
      { path }
    );
    try {
      const result = await this.api.invoke<boolean>('remove_orphaned_package', { path });
      this.logger.logInfo(
        'service',
        'RepairService',
        'removeOrphanedPackage',
        'Orphaned package removed'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'RepairService',
        'removeOrphanedPackage',
        'Operation failed',
        error as Error,
        { path }
      );
      throw error;
    }
  }
}
