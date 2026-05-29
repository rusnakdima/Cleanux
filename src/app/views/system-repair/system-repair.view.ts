import { ChangeDetectionStrategy, Component, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { MatTabsModule } from '@angular/material/tabs';
import { RepairService, RepairItem } from '@services/repair.service';
import { NotificationService } from '@services/notification.service';

type RepairTab = 'symlinks' | 'packages' | 'cache' | 'permissions';

@Component({
  selector: 'app-system-repair-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    MatTabsModule,
  ],
  templateUrl: './system-repair.view.html',
})
export class SystemRepairView {
  private repairService = inject(RepairService);
  private notification = inject(NotificationService);

  activeTab = signal<RepairTab>('symlinks');
  loading = signal(false);

  symlinksData = signal<RepairItem[]>([]);
  packagesData = signal<RepairItem[]>([]);
  fontCacheResult = signal<string | null>(null);
  iconCacheResult = signal<string | null>(null);
  permissionsResult = signal<string | null>(null);

  symlinkCount = () => this.symlinksData().length;
  packagesCount = () => this.packagesData().length;

  async loadSymlinks() {
    this.loading.set(true);
    try {
      const data = await this.repairService.findBrokenSymlinks();
      this.symlinksData.set(data);
    } catch (error: unknown) {
      console.error('Failed to load symlinks:', error);
    } finally {
      this.loading.set(false);
    }
  }

  async loadPackages() {
    this.loading.set(true);
    try {
      const data = await this.repairService.findOrphanedPackages();
      this.packagesData.set(data);
    } catch (error: unknown) {
      console.error('Failed to load packages:', error);
    } finally {
      this.loading.set(false);
    }
  }

  async cleanFontCache() {
    this.loading.set(true);
    try {
      const result = await this.repairService.cleanFontCache();
      this.fontCacheResult.set(`Font cache cleaned: ${result.removed} files removed`);
    } catch (error: unknown) {
      this.fontCacheResult.set('Failed to clean font cache');
    } finally {
      this.loading.set(false);
    }
  }

  async cleanIconCache() {
    this.loading.set(true);
    try {
      const result = await this.repairService.cleanIconCache();
      this.iconCacheResult.set(`Icon cache cleaned: ${result.removed} files removed`);
    } catch (error: unknown) {
      this.iconCacheResult.set('Failed to clean icon cache');
    } finally {
      this.loading.set(false);
    }
  }

  async repairPermissions() {
    this.loading.set(true);
    try {
      const result = await this.repairService.repairPermissions();
      this.permissionsResult.set(`Permissions repaired: ${result.repaired} items fixed`);
    } catch (error: unknown) {
      this.permissionsResult.set('Failed to repair permissions');
    } finally {
      this.loading.set(false);
    }
  }

  async fixSymlink(item: RepairItem) {
    const confirmed = confirm(`Remove broken symlink: ${item.path}?`);
    if (!confirmed) return;
    try {
      await this.repairService.removeBrokenSymlink(item.path);
      await this.loadSymlinks();
    } catch (error: unknown) {
      this.notification.error('Failed to remove symlink', error);
    }
  }

  async fixAllSymlinks() {
    const items = this.symlinksData();
    if (items.length === 0) return;
    const confirmed = confirm(`Remove all ${items.length} broken symlinks?`);
    if (!confirmed) return;
    for (const item of items) {
      try {
        await this.repairService.removeBrokenSymlink(item.path);
      } catch (error) {
        console.error('Failed to remove:', item.path, error);
      }
    }
    await this.loadSymlinks();
  }

  async fixPackage(item: RepairItem) {
    const confirmed = confirm(`Purge orphaned package: ${item.path}?`);
    if (!confirmed) return;
    try {
      await this.repairService.removeOrphanedPackage(item.path);
      await this.loadPackages();
    } catch (error: unknown) {
      this.notification.error('Failed to purge package', error);
    }
  }

  async fixAllPackages() {
    const items = this.packagesData();
    if (items.length === 0) return;
    const confirmed = confirm(`Purge all ${items.length} orphaned packages?`);
    if (!confirmed) return;
    for (const item of items) {
      try {
        await this.repairService.removeOrphanedPackage(item.path);
      } catch (error) {
        console.error('Failed to remove:', item.path, error);
      }
    }
    await this.loadPackages();
  }

  getSeverityIcon(severity: string): string {
    switch (severity) {
      case 'error':
        return 'error';
      case 'warning':
        return 'warning';
      case 'info':
        return 'info';
      default:
        return 'help';
    }
  }

  getSeverityColor(severity: string): string {
    switch (severity) {
      case 'error':
        return 'text-red-500';
      case 'warning':
        return 'text-yellow-500';
      case 'info':
        return 'text-blue-500';
      default:
        return 'text-gray-500';
    }
  }

  async onTabChange(index: number) {
    const tabs: RepairTab[] = ['symlinks', 'packages', 'cache', 'permissions'];
    this.activeTab.set(tabs[index]);
    this.fontCacheResult.set(null);
    this.iconCacheResult.set(null);
    this.permissionsResult.set(null);

    switch (tabs[index]) {
      case 'symlinks':
        if (this.symlinksData().length === 0) await this.loadSymlinks();
        break;
      case 'packages':
        if (this.packagesData().length === 0) await this.loadPackages();
        break;
    }
  }
}
