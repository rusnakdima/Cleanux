import { Injectable, inject } from '@angular/core';
import { ApiService } from './api.service';

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

  async findBrokenSymlinks(): Promise<RepairItem[]> {
    return await this.api.invoke<RepairItem[]>('find_broken_symlinks');
  }

  async findOrphanedPackages(): Promise<RepairItem[]> {
    return await this.api.invoke<RepairItem[]>('find_orphaned_packages');
  }

  async cleanFontCache(): Promise<RepairResult> {
    return await this.api.invoke<RepairResult>('clean_font_cache');
  }

  async cleanIconCache(): Promise<RepairResult> {
    return await this.api.invoke<RepairResult>('clean_icon_cache');
  }

  async repairPermissions(): Promise<RepairResult> {
    return await this.api.invoke<RepairResult>('repair_permissions');
  }

  async removeBrokenSymlink(path: string): Promise<boolean> {
    return await this.api.invoke<boolean>('remove_broken_symlink', { path });
  }

  async removeOrphanedPackage(path: string): Promise<boolean> {
    return await this.api.invoke<boolean>('remove_orphaned_package', { path });
  }
}
