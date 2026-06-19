import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';

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

  constructor() {}

  async findBrokenSymlinks(): Promise<RepairItem[]> {
    try {
      const result = await this.api.invoke<RepairItem[]>('find_broken_symlinks');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async findOrphanedPackages(): Promise<RepairItem[]> {
    try {
      const result = await this.api.invoke<RepairItem[]>('find_orphaned_packages');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanFontCache(): Promise<RepairResult> {
    try {
      const result = await this.api.invoke<RepairResult>('clean_font_cache');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanIconCache(): Promise<RepairResult> {
    try {
      const result = await this.api.invoke<RepairResult>('clean_icon_cache');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async repairPermissions(): Promise<RepairResult> {
    try {
      const result = await this.api.invoke<RepairResult>('repair_permissions');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async removeBrokenSymlink(path: string): Promise<boolean> {
    try {
      const result = await this.api.invoke<boolean>('remove_broken_symlink', { path });
      return result;
    } catch (error) {
      throw error;
    }
  }

  async removeOrphanedPackage(path: string): Promise<boolean> {
    try {
      const result = await this.api.invoke<boolean>('remove_orphaned_package', { path });
      return result;
    } catch (error) {
      throw error;
    }
  }
}
