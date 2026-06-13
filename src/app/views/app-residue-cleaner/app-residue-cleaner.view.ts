import { NotificationService } from '@services/notification.service';
import { ConfirmDialogService } from '@shared/confirm-dialog';
import {
  ChangeDetectionStrategy,
  Component,
  signal,
  inject,
  computed,
  WritableSignal,
} from '@angular/core';
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
  private confirmDialogService = inject(ConfirmDialogService);

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

  private selectionMap = new Map<ResidueTab, WritableSignal<Set<string>>>([
    ['configs', this.selectedConfigs],
    ['data', this.selectedData],
    ['caches', this.selectedCaches],
    ['orphaned', this.selectedOrphaned],
  ]);

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

  searchVisible = signal(false);
  searchQuery = signal('');
  searchFocused = signal(false);

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

  configsAllSelected = computed(() => {
    const total = this.configsData().length;
    const selected = this.selectedConfigs().size;
    return total > 0 && selected === total;
  });

  configsIndeterminate = computed(() => {
    const total = this.configsData().length;
    const selected = this.selectedConfigs().size;
    return selected > 0 && selected < total;
  });

  dataAllSelected = computed(() => {
    const total = this.dataData().length;
    const selected = this.selectedData().size;
    return total > 0 && selected === total;
  });

  dataIndeterminate = computed(() => {
    const total = this.dataData().length;
    const selected = this.selectedData().size;
    return selected > 0 && selected < total;
  });

  cachesAllSelected = computed(() => {
    const total = this.cachesData().length;
    const selected = this.selectedCaches().size;
    return total > 0 && selected === total;
  });

  cachesIndeterminate = computed(() => {
    const total = this.cachesData().length;
    const selected = this.selectedCaches().size;
    return selected > 0 && selected < total;
  });

  orphanedAllSelected = computed(() => {
    const total = this.orphanedData().length;
    const selected = this.selectedOrphaned().size;
    return total > 0 && selected === total;
  });

  orphanedIndeterminate = computed(() => {
    const total = this.orphanedData().length;
    const selected = this.selectedOrphaned().size;
    return selected > 0 && selected < total;
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

  private getSelectionSignal(): WritableSignal<Set<string>> {
    return this.selectionMap.get(this.activeTab()) ?? this.selectedConfigs;
  }

  isSelected(path: string): boolean {
    const sel = this.getSelectionSignal();
    return sel().has(path);
  }

  toggleSelection(path: string): void {
    const selection = this.getSelectionSignal();
    const current = new Set(selection());
    current.has(path) ? current.delete(path) : current.add(path);
    selection.set(current);
  }

  selectAll(items: AppResidue[], checked = true): void {
    const selection = this.getSelectionSignal();
    selection.set(checked ? new Set(items.map((i) => i.path)) : new Set());
  }

  deselectAll(): void {
    this.getSelectionSignal().set(new Set());
  }

  getCurrentItems(): AppResidue[] {
    const tab = this.activeTab();
    if (tab === 'configs') return this.configsData();
    if (tab === 'data') return this.dataData();
    if (tab === 'caches') return this.cachesData();
    return [];
  }

  getSelectedCount(): number {
    return this.getSelectionSignal()().size;
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

  showSearchInput(): void {
    this.searchVisible.set(true);
  }

  hideSearchInput(): void {
    if (!this.searchFocused()) {
      this.searchVisible.set(false);
    }
  }

  onSearchHoverEnter(): void {
    if (!this.searchVisible()) {
      this.searchVisible.set(true);
    }
  }

  onSearchHoverLeave(): void {
    this.hideSearchInput();
  }

  onSearchFocus(): void {
    this.searchFocused.set(true);
  }

  onSearchBlur(): void {
    this.searchFocused.set(false);
  }

  clearSearch(): void {
    this.searchQuery.set('');
  }

  onSearchChange(query: string): void {
    this.searchQuery.set(query.toLowerCase());
  }
}
