import { NotificationService } from '@services/notification.service';
import { ChangeDetectionStrategy, Component, signal, inject, computed } from '@angular/core';
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
} from '@services/app-residue.service';
import { formatSize } from '@shared/utils/format.util';
import { PaginationComponent } from '@components/pagination/pagination.component';

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
    PaginationComponent,
  ],
  templateUrl: './app-residue-cleaner.view.html',
})
export class AppResidueCleanerView {
  private residueService = inject(AppResidueService);
  private notification = inject(NotificationService);

  formatSize = formatSize;

  activeTab = signal<ResidueTab>('configs');
  loading = signal(false);
  showBackupWarning = signal(true);

  configsData = signal<AppResidue[]>([]);
  dataData = signal<AppResidue[]>([]);
  cachesData = signal<AppResidue[]>([]);
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

  configsCount = () => this.configsData().length;
  dataCount = () => this.dataData().length;
  cachesCount = () => this.cachesData().length;
  orphanedCount = () => this.orphanedData().length;

  paginatedConfigs = computed(() => {
    const start = (this.configsPage() - 1) * this.configsPageSize();
    return this.configsData().slice(start, start + this.configsPageSize());
  });

  paginatedData = computed(() => {
    const start = (this.dataPage() - 1) * this.dataPageSize();
    return this.dataData().slice(start, start + this.dataPageSize());
  });

  paginatedCaches = computed(() => {
    const start = (this.cachesPage() - 1) * this.cachesPageSize();
    return this.cachesData().slice(start, start + this.cachesPageSize());
  });

  paginatedOrphaned = computed(() => {
    const start = (this.orphanedPage() - 1) * this.orphanedPageSize();
    return this.orphanedData().slice(start, start + this.orphanedPageSize());
  });

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

  toggleSelection(path: string): void {
    const tab = this.activeTab();
    switch (tab) {
      case 'configs': {
        const current = new Set(this.selectedConfigs());
        current.has(path) ? current.delete(path) : current.add(path);
        this.selectedConfigs.set(current);
        break;
      }
      case 'data': {
        const current = new Set(this.selectedData());
        current.has(path) ? current.delete(path) : current.add(path);
        this.selectedData.set(current);
        break;
      }
      case 'caches': {
        const current = new Set(this.selectedCaches());
        current.has(path) ? current.delete(path) : current.add(path);
        this.selectedCaches.set(current);
        break;
      }
      case 'orphaned': {
        const current = new Set(this.selectedOrphaned());
        current.has(path) ? current.delete(path) : current.add(path);
        this.selectedOrphaned.set(current);
        break;
      }
    }
  }

  selectAll(items: AppResidue[]): void {
    const tab = this.activeTab();
    const allPaths = new Set(items.map((i) => i.path));
    switch (tab) {
      case 'configs':
        this.selectedConfigs.set(allPaths);
        break;
      case 'data':
        this.selectedData.set(allPaths);
        break;
      case 'caches':
        this.selectedCaches.set(allPaths);
        break;
      case 'orphaned':
        this.selectedOrphaned.set(allPaths);
        break;
    }
  }

  deselectAll(): void {
    const tab = this.activeTab();
    switch (tab) {
      case 'configs':
        this.selectedConfigs.set(new Set());
        break;
      case 'data':
        this.selectedData.set(new Set());
        break;
      case 'caches':
        this.selectedCaches.set(new Set());
        break;
      case 'orphaned':
        this.selectedOrphaned.set(new Set());
        break;
    }
  }

  getCurrentItems(): AppResidue[] {
    const tab = this.activeTab();
    switch (tab) {
      case 'configs':
        return this.configsData();
      case 'data':
        return this.dataData();
      case 'caches':
        return this.cachesData();
    }
    return null as any;
  }

  getSelectedCount(): number {
    const tab = this.activeTab();
    switch (tab) {
      case 'configs':
        return this.selectedConfigs().size;
      case 'data':
        return this.selectedData().size;
      case 'caches':
        return this.selectedCaches().size;
      case 'orphaned':
        return this.selectedOrphaned().size;
    }
    return 0;
  }

  async loadSummary(): Promise<void> {
    try {
      const summary = await this.residueService.getResidueSummary();
      this.summary.set(summary);
    } catch (error) {
      console.error('Failed to load summary:', error);
    }
  }

  async loadConfigs(): Promise<void> {
    this.loading.set(true);
    try {
      const data = await this.residueService.scanUserConfigs();
      this.configsData.set(data);
    } catch (error) {
      console.error('Failed to load configs:', error);
    } finally {
      this.loading.set(false);
    }
  }

  async loadData(): Promise<void> {
    this.loading.set(true);
    try {
      const data = await this.residueService.scanUserData();
      this.dataData.set(data);
    } catch (error) {
      console.error('Failed to load data:', error);
    } finally {
      this.loading.set(false);
    }
  }

  async loadCaches(): Promise<void> {
    this.loading.set(true);
    try {
      const data = await this.residueService.scanUserCaches();
      this.cachesData.set(data);
    } catch (error) {
      console.error('Failed to load caches:', error);
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
      console.error('Failed to load orphaned configs:', error);
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

    const confirmed = confirm(
      `Are you sure you want to clean ${paths.length} item(s)?\n\nThis action cannot be undone. Consider backing up your data first.`
    );
    if (!confirmed) return;

    this.loading.set(true);
    try {
      const result = await this.residueService.cleanMultipleResidues(paths);
      this.notification.success(
        `Cleaned ${result.removed} items${result.failed.length > 0 ? `, ${result.failed.length} failed` : ''}`
      );
      await this.refreshCurrentTab();
    } catch (error) {
      this.notification.error(
        'Failed to clean residues: ' + (error instanceof Error ? error.message : String(error)),
        error
      );
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
        if (this.configsData().length === 0) await this.loadConfigs();
        break;
      case 'data':
        if (this.dataData().length === 0) await this.loadData();
        break;
      case 'caches':
        if (this.cachesData().length === 0) await this.loadCaches();
        break;
      case 'orphaned':
        if (this.orphanedData().length === 0) await this.loadOrphaned();
        break;
    }
  }

  dismissBackupWarning(): void {
    this.showBackupWarning.set(false);
  }

  getResidueTypeLabel(type: string): string {
    switch (type) {
      case 'Config':
        return 'Config';
      case 'Data':
        return 'Data';
      case 'Cache':
        return 'Cache';
      case 'Both':
        return 'Config/Data';
      default:
        return type;
    }
  }

  getResidueTypeColor(residue: AppResidue): string {
    switch (residue.residue_type) {
      case 'Config':
        return 'bg-blue-500/10 text-blue-600';
      case 'Data':
        return 'bg-green-500/10 text-green-600';
      case 'Cache':
        return 'bg-orange-500/10 text-orange-600';
      case 'Both':
        return 'bg-purple-500/10 text-purple-600';
      default:
        return 'bg-slate-500/10 text-slate-600';
    }
  }
}
