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
import {
  PackageDeepCleanService,
  PackageManagerSummary,
  OrphanedPackage,
} from '@services/package-deep-clean.service';

/* components */
import { HeaderComponent } from '@components/header/header.component';
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
    HeaderComponent,
  ],
  templateUrl: './package-deep-clean.view.html',
})
export class PackageDeepCleanView implements OnInit {
  private service = inject(PackageDeepCleanService);

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
      console.error('Failed to load package data:', error);
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
      alert(result.message);
      await this.loadData();
    } catch (error) {
      alert('Failed to clean APT cache');
    }
  }

  async aptAutoremove() {
    const confirmed = confirm('Run apt autoremove to remove unused packages?');
    if (!confirmed) return;
    try {
      const result = await this.service.aptAutoremove();
      alert(result);
      await this.loadData();
    } catch (error) {
      alert('Failed to run apt autoremove');
    }
  }

  async aptAutoclean() {
    try {
      const result = await this.service.aptAutoclean();
      alert(result.message);
      await this.loadData();
    } catch (error) {
      alert('Failed to run apt autoclean');
    }
  }

  async removeOrphanedPackage(pkg: OrphanedPackage) {
    const confirmed = confirm(`Remove orphaned package "${pkg.name}"?`);
    if (!confirmed) return;
    try {
      await this.service.removeOrphanedPackage(pkg.name);
      await this.loadData();
    } catch (error) {
      alert('Failed to remove orphaned package');
    }
  }

  async dnfCleanAll() {
    try {
      const result = await this.service.dnfCleanAll();
      alert(result.message);
      await this.loadData();
    } catch (error) {
      alert('Failed to clean DNF cache');
    }
  }

  async pacmanClean() {
    try {
      const result = await this.service.pacmanClean(this.pacmanKeepRecent());
      alert(result.message);
      await this.loadData();
    } catch (error) {
      alert('Failed to clean pacman cache');
    }
  }

  async pacmanFullClean() {
    const confirmed = confirm(
      'WARNING: Full clean (-Scc) will remove ALL cached packages including the current version!\n\nThis is a destructive operation. Are you sure?'
    );
    if (!confirmed) return;

    const doubleConfirmed = confirm(
      'This will remove ALL package cache, including those for currently installed packages. Type "YES" to confirm.'
    );
    if (!doubleConfirmed) return;

    try {
      const result = await this.service.pacmanFullClean();
      alert(result.message);
      await this.loadData();
    } catch (error) {
      alert('Failed to run pacman full clean');
    }
  }

  async zypperClean() {
    try {
      const result = await this.service.zypperClean();
      alert(result.message);
      await this.loadData();
    } catch (error) {
      alert('Failed to clean zypper cache');
    }
  }

  async deepCleanAll() {
    const confirmed = confirm('Run deep clean for all available package managers?');
    if (!confirmed) return;
    try {
      const result = await this.service.deepCleanAll();
      alert(`Deep clean completed. Total space freed: ${this.formatSize(result.totalSpaceFreed)}`);
      await this.loadData();
    } catch (error) {
      alert('Failed to run deep clean');
    }
  }
}
