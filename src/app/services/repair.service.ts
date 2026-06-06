import { Injectable } from '@angular/core';
import { BaseApiService } from './base-api.service';
import { OperationResult } from '@models/cleaner.models';
import { RepairItem } from '@models/repair.model';

@Injectable({
  providedIn: 'root',
})
export class RepairService extends BaseApiService {
  async findBrokenSymlinks(): Promise<RepairItem[]> {
    return await this.call<RepairItem[]>('find_broken_symlinks');
  }

  async findOrphanedPackages(): Promise<RepairItem[]> {
    return await this.call<RepairItem[]>('find_orphaned_packages');
  }

  async cleanFontCache(): Promise<OperationResult> {
    return await this.call<OperationResult>('clean_font_cache');
  }

  async cleanIconCache(): Promise<OperationResult> {
    return await this.call<OperationResult>('clean_icon_cache');
  }

  async repairPermissions(): Promise<OperationResult> {
    return await this.call<OperationResult>('repair_permissions');
  }

  async removeBrokenSymlink(path: string): Promise<boolean> {
    return await this.call<boolean>('remove_broken_symlink', { path });
  }

  async removeOrphanedPackage(path: string): Promise<boolean> {
    return await this.call<boolean>('remove_orphaned_package', { path });
  }
}
