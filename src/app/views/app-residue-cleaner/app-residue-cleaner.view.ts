import { NotificationService } from '@services/notification.service';
import { ConfirmDialogService } from '@shared/confirm-dialog';
import { ChangeDetectionStrategy, Component, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { MatTabsModule } from '@angular/material/tabs';
import {
  AppResidueService,
  AppResidue,
  OrphanedConfig,
  AppResidueSummary,
} from '@features/app-residue-cleaner/services/app-residue.service';
import { formatSize } from '@shared/utils/format.util';
import { getErrorMessage } from '@shared/utils/error.util';
import { ResidueTabComponent } from './residue-tab.component';
import { OrphanedTabComponent } from './orphaned-tab.component';

type ResidueTab = 'configs' | 'data' | 'caches' | 'orphaned';

@Component({
  selector: 'app-app-residue-cleaner-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    MatTabsModule,
    ResidueTabComponent,
    OrphanedTabComponent,
  ],
  templateUrl: './app-residue-cleaner.view.html',
})
export class AppResidueCleanerView {
  private residueService = inject(AppResidueService);
  private notification = inject(NotificationService);
  private confirmDialogService = inject(ConfirmDialogService);

  formatSize = formatSize;

  activeTab = signal<ResidueTab>('configs');
  loading = signal(false);
  showBackupWarning = signal(true);

  configs = signal<AppResidue[]>([]);
  data = signal<AppResidue[]>([]);
  caches = signal<AppResidue[]>([]);
  orphanedData = signal<OrphanedConfig[]>([]);

  selectedConfigs = signal<Set<string>>(new Set());
  selectedData = signal<Set<string>>(new Set());
  selectedCaches = signal<Set<string>>(new Set());
  selectedOrphaned = signal<Set<string>>(new Set());

  previewMode = signal(false);
  previewItems = signal<AppResidue[]>([]);

  summary = signal<AppResidueSummary | null>(null);

  configsPage = signal(1);
  configsPageSize = signal(15);
  dataPage = signal(1);
  dataPageSize = signal(15);
  cachesPage = signal(1);
  cachesPageSize = signal(15);
  orphanedPage = signal(1);
  orphanedPageSize = signal(15);

  configsCount = () => this.configs().length;
  dataCount = () => this.data().length;
  cachesCount = () => this.caches().length;
  orphanedCount = () => this.orphanedData().length;

  onConfigsPageChange(page: number) {
    this.configsPage.set(page);
  }

  onConfigsPageSizeChange(size: number) {
    this.configsPageSize.set(size);
    this.configsPage.set(1);
  }

  onDataPageChange(page: number) {
    this.dataPage.set(page);
  }

  onDataPageSizeChange(size: number) {
    this.dataPageSize.set(size);
    this.dataPage.set(1);
  }

  onCachesPageChange(page: number) {
    this.cachesPage.set(page);
  }

  onCachesPageSizeChange(size: number) {
    this.cachesPageSize.set(size);
    this.cachesPage.set(1);
  }

  onOrphanedPageChange(page: number) {
    this.orphanedPage.set(page);
  }

  onOrphanedPageSizeChange(size: number) {
    this.orphanedPageSize.set(size);
    this.orphanedPage.set(1);
  }

  onConfigsToggle(path: string) {
    const current = new Set(this.selectedConfigs());
    current.has(path) ? current.delete(path) : current.add(path);
    this.selectedConfigs.set(current);
  }

  onConfigsSelectAll(checked: boolean) {
    this.selectedConfigs.set(checked ? new Set(this.configs().map((i) => i.path)) : new Set());
  }

  onDataToggle(path: string) {
    const current = new Set(this.selectedData());
    current.has(path) ? current.delete(path) : current.add(path);
    this.selectedData.set(current);
  }

  onDataSelectAll(checked: boolean) {
    this.selectedData.set(checked ? new Set(this.data().map((i) => i.path)) : new Set());
  }

  onCachesToggle(path: string) {
    const current = new Set(this.selectedCaches());
    current.has(path) ? current.delete(path) : current.add(path);
    this.selectedCaches.set(current);
  }

  onCachesSelectAll(checked: boolean) {
    this.selectedCaches.set(checked ? new Set(this.caches().map((i) => i.path)) : new Set());
  }

  onOrphanedToggle(path: string) {
    const current = new Set(this.selectedOrphaned());
    current.has(path) ? current.delete(path) : current.add(path);
    this.selectedOrphaned.set(current);
  }

  getSelectedSize(items: AppResidue[]): number {
    return items
      .filter((item) => this.isSelected(item.path))
      .reduce((sum, item) => sum + item.size, 0);
  }

  isSelected(path: string): boolean {
    const tab = this.activeTab();
    switch (tab) {
      case 'configs':
        return this.selectedConfigs().has(path);
      case 'data':
        return this.selectedData().has(path);
      case 'caches':
        return this.selectedCaches().has(path);
      case 'orphaned':
        return this.selectedOrphaned().has(path);
    }
  }

  async loadSummary(): Promise<void> {
    try {
      const summary = await this.residueService.getResidueSummary();
      this.summary.set(summary);
    } catch (error) {
      this.notification.error('Failed to load summary', error);
    }
  }

  async loadConfigs(): Promise<void> {
    this.loading.set(true);
    try {
      const data = await this.residueService.scanUserConfigs();
      this.configs.set(data);
    } catch (error) {
      this.notification.error('Failed to load configs', error);
    } finally {
      this.loading.set(false);
    }
  }

  async loadData(): Promise<void> {
    this.loading.set(true);
    try {
      const data = await this.residueService.scanUserData();
      this.data.set(data);
    } catch (error) {
      this.notification.error('Failed to load data', error);
    } finally {
      this.loading.set(false);
    }
  }

  async loadCaches(): Promise<void> {
    this.loading.set(true);
    try {
      const data = await this.residueService.scanUserCaches();
      this.caches.set(data);
    } catch (error) {
      this.notification.error('Failed to load caches', error);
    } finally {
      this.loading.set(false);
    }
  }

  async loadOrphaned(): Promise<void> {
    this.loading.set(true);
    try {
      const data = await this.residueService.getOrphanedConfigs();
      this.orphanedData.set(data);
    } catch (error) {
      this.notification.error('Failed to load orphaned configs', error);
    } finally {
      this.loading.set(false);
    }
  }

  async onPreviewClean(): Promise<void> {
    const items = this.getCurrentItems().filter((item) => this.isSelected(item.path));
    this.previewItems.set(items);
    this.previewMode.set(true);
  }

  async onCleanSelected(): Promise<void> {
    const tab = this.activeTab();
    let paths: string[];

    if (tab === 'orphaned') {
      paths = Array.from(this.selectedOrphaned());
    } else {
      paths = this.getCurrentItems()
        .filter((item) => this.isSelected(item.path))
        .map((item) => item.path);
    }

    if (paths.length === 0) return;

    const confirmed = await this.confirmDialogService.confirm({
      title: 'Clean Residues',
      message: `Are you sure you want to clean ${paths.length} item(s)?\n\nThis action cannot be undone. Consider backing up your data first.`,
      dangerous: true,
    });
    if (!confirmed) return;

    this.loading.set(true);
    try {
      const result = await this.residueService.cleanMultipleResidues(paths);
      this.notification.success(
        `Cleaned ${result.removed} items${result.failed.length > 0 ? `, ${result.failed.length} failed` : ''}`
      );
      await this.refreshCurrentTab();
    } catch (error) {
      this.notification.error('Failed to clean residues: ' + getErrorMessage(error), error);
    } finally {
      this.loading.set(false);
    }
  }

  async refreshCurrentTab(): Promise<void> {
    this.deselectAll();
    switch (this.activeTab()) {
      case 'configs':
        await this.loadConfigs();
        break;
      case 'data':
        await this.loadData();
        break;
      case 'caches':
        await this.loadCaches();
        break;
      case 'orphaned':
        await this.loadOrphaned();
        break;
    }
    await this.loadSummary();
  }

  async onTabChange(index: number): Promise<void> {
    const tabs: ResidueTab[] = ['configs', 'data', 'caches', 'orphaned'];
    this.activeTab.set(tabs[index]);
    this.previewMode.set(false);
    this.previewItems.set([]);

    switch (tabs[index]) {
      case 'configs':
        if (this.configs().length === 0) await this.loadConfigs();
        break;
      case 'data':
        if (this.data().length === 0) await this.loadData();
        break;
      case 'caches':
        if (this.caches().length === 0) await this.loadCaches();
        break;
      case 'orphaned':
        if (this.orphanedData().length === 0) await this.loadOrphaned();
        break;
    }
  }

  dismissBackupWarning(): void {
    this.showBackupWarning.set(false);
  }

  private deselectAll(): void {
    this.selectedConfigs.set(new Set());
    this.selectedData.set(new Set());
    this.selectedCaches.set(new Set());
    this.selectedOrphaned.set(new Set());
  }

  private getCurrentItems(): AppResidue[] {
    const tab = this.activeTab();
    switch (tab) {
      case 'configs':
        return this.configs();
      case 'data':
        return this.data();
      case 'caches':
        return this.caches();
    }
    return [];
  }
}
