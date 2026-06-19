/* sys lib */
import { ChangeDetectionStrategy, Component, OnInit, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTabsModule } from '@angular/material/tabs';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatTooltipModule } from '@angular/material/tooltip';

/* services */
import { PackageDeepCleanService } from '@features/package-deep-clean/services/package-deep-clean.service';
import { PackageManagerSummary, OrphanedPackage } from '@entities/package.model';
import { NotificationService } from '@services/notification.service';
import { ConfirmDialogService } from '@shared/confirm-dialog';

import { formatSize } from '@shared/utils/format.util';

@Component({
  selector: 'app-package-deep-clean-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTabsModule,
    MatInputModule,
    MatFormFieldModule,
    MatTooltipModule,
  ],
  templateUrl: './package-deep-clean.view.html',
})
export class PackageDeepCleanView implements OnInit {
  private service = inject(PackageDeepCleanService);
  private notification = inject(NotificationService);
  private confirmDialogService = inject(ConfirmDialogService);

  formatSize = formatSize;

  summary = signal<PackageManagerSummary | null>(null);
  orphanedPackages = signal<OrphanedPackage[]>([]);
  partialDownloads = signal<string[]>([]);
  loading = signal(false);

  pacmanKeepRecent = signal(2);

  activeTabIndex = signal(0);

  async ngOnInit() {
    await this.loadData();
  }

  async loadData() {
    this.loading.set(true);
    try {
      const summary = await this.service.getPackageSummary();
      this.summary.set(summary);

      const orphans = await this.service.getOrphanedPackages();
      this.orphanedPackages.set(orphans);

      const partials = await this.service.getPartialDownloads();
      this.partialDownloads.set(partials);

      const tabs = this.getAvailableTabs();
      if (tabs.length > 0) {
        this.activeTabIndex.set(tabs[0].index);
      }
    } catch (error) {
      this.notification.error('Failed to load package data', error);
    } finally {
      this.loading.set(false);
    }
  }

  getAvailableTabs(): { name: string; index: number }[] {
    const summary = this.summary();
    const tabs: { name: string; index: number }[] = [];
    let idx = 0;

    if (summary?.aptAvailable) {
      tabs.push({ name: 'APT', index: idx++ });
    }
    if (summary?.dnfAvailable) {
      tabs.push({ name: 'DNF/YUM', index: idx++ });
    }
    if (summary?.pacmanAvailable) {
      tabs.push({ name: 'Pacman', index: idx++ });
    }
    if (summary?.zypperAvailable) {
      tabs.push({ name: 'Zypper', index: idx++ });
    }

    return tabs;
  }

  getActiveTabName(): string {
    const tabs = this.getAvailableTabs();
    const active = tabs.find((t) => t.index === this.activeTabIndex());
    return active?.name || '';
  }

  async aptClean() {
    try {
      const result = await this.service.aptClean();
      this.notification.success(result.message);
      await this.loadData();
    } catch (error) {
      this.notification.error('Failed to clean APT cache', error);
    }
  }

  async aptAutoremove() {
    const confirmed = await this.confirmDialogService.confirm({
      title: 'APT Autoremove',
      message: 'Run apt autoremove to remove unused packages?',
    });
    if (!confirmed) return;
    try {
      const result = await this.service.aptAutoremove();
      this.notification.success(result);
      await this.loadData();
    } catch (error) {
      this.notification.error('Failed to run apt autoremove', error);
    }
  }

  async aptAutoclean() {
    try {
      const result = await this.service.aptAutoclean();
      this.notification.success(result.message);
      await this.loadData();
    } catch (error) {
      this.notification.error('Failed to run apt autoclean', error);
    }
  }

  async removeOrphanedPackage(pkg: OrphanedPackage) {
    const confirmed = await this.confirmDialogService.confirm({
      title: 'Remove Package',
      message: `Remove orphaned package "${pkg.name}"?`,
      dangerous: true,
    });
    if (!confirmed) return;
    try {
      await this.service.removeOrphanedPackage(pkg.name);
      await this.loadData();
    } catch (error) {
      this.notification.error('Failed to remove orphaned package', error);
    }
  }

  async dnfCleanAll() {
    try {
      const result = await this.service.dnfCleanAll();
      this.notification.success(result.message);
      await this.loadData();
    } catch (error) {
      this.notification.error('Failed to clean DNF cache', error);
    }
  }

  async pacmanClean() {
    try {
      const result = await this.service.pacmanClean(this.pacmanKeepRecent());
      this.notification.success(result.message);
      await this.loadData();
    } catch (error) {
      this.notification.error('Failed to clean pacman cache', error);
    }
  }

  async pacmanFullClean() {
    const confirmed = await this.confirmDialogService.confirm({
      title: 'Pacman Full Clean',
      message:
        'WARNING: Full clean (-Scc) will remove ALL cached packages including the current version!\n\nThis is a destructive operation. Are you sure?',
      dangerous: true,
    });
    if (!confirmed) return;

    const doubleConfirmed = await this.confirmDialogService.confirm({
      title: 'Confirm Full Clean',
      message:
        'This will remove ALL package cache, including those for currently installed packages. Type "YES" to confirm.',
      requireYesToConfirm: true,
    });
    if (!doubleConfirmed) return;

    try {
      const result = await this.service.pacmanFullClean();
      this.notification.success(result.message);
      await this.loadData();
    } catch (error) {
      this.notification.error('Failed to run pacman full clean', error);
    }
  }

  async zypperClean() {
    try {
      const result = await this.service.zypperClean();
      this.notification.success(result.message);
      await this.loadData();
    } catch (error) {
      this.notification.error('Failed to clean zypper cache', error);
    }
  }

  async deepCleanAll() {
    const confirmed = await this.confirmDialogService.confirm({
      title: 'Deep Clean',
      message: 'Run deep clean for all available package managers?',
    });
    if (!confirmed) return;
    try {
      const result = await this.service.deepCleanAll();
      this.notification.success(
        `Deep clean completed. Total space freed: ${this.formatSize(result.totalSpaceFreed)}`
      );
      await this.loadData();
    } catch (error) {
      this.notification.error('Failed to run deep clean', error);
    }
  }
}
