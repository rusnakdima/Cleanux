import {
  ChangeDetectionStrategy,
  Component,
  OnInit,
  signal,
  inject,
  computed,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { StartupService } from '@services/startup.service';
import { ThemeService } from '@services/theme.service';
import { LoadingErrorMixin } from '@views/mixins/loading-error.mixin';
import { StartupItem } from '@models/startup.model';
import { DataListComponent } from '@components/data-list/data-list.component';
import { ListColumn, ListOptions } from '@models/data-list.model';

@Component({
  selector: 'app-startup-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    DataListComponent,
  ],
  templateUrl: './startup.view.html',
})
export class StartupView extends LoadingErrorMixin implements OnInit {
  private startupService = inject(StartupService);
  private themeService = inject(ThemeService);

  startupData = signal<StartupItem[]>([]);

  currentPage = signal(1);
  pageSize = signal(15);

  totalItems = computed(() => this.startupData().length);
  enabledItems = computed(() => this.startupData().filter((s) => s.enabled).length);
  disabledItems = computed(() => this.startupData().filter((s) => !s.enabled).length);

  columns: ListColumn[] = [
    {
      key: 'name',
      primary: true,
      icon: 'apps',
      secondary: 'command',
      actions: [
        {
          id: 'toggle',
          icon: 'toggle_off',
          tooltip: 'Toggle enable/disable',
          toggle: true,
          toggleState: (item: unknown) => (item as StartupItem).enabled,
        },
      ],
    },
  ];

  options: ListOptions = {
    showSearch: true,
    showCheckbox: false,
    showActions: true,
    actionsPosition: 'right',
    showReloadButton: true,
    searchPlaceholder: 'Search startup items...',
    emptyMessage: 'No startup items found',
  };

  onPageChange(page: number) {
    this.currentPage.set(page);
  }

  onPageSizeChange(size: number) {
    this.pageSize.set(size);
    this.currentPage.set(1);
  }

  async ngOnInit() {
    await this.loadData();
  }

  async loadData() {
    this.currentPage.set(1);
    await this.runWithLoading(
      async () => {
        const data = await this.startupService.getStartupItems();
        this.startupData.set(data);
      },
      { errorMessage: 'Failed to load startup items' }
    );
  }

  async toggleItem(item: StartupItem) {
    if (item.enabled) {
      const confirmed = await this.confirmDialogService.confirm({
        title: 'Disable Startup Item',
        message: `Disable "${item.name}" from starting at login?`,
      });
      if (!confirmed) return;
      await this.runWithLoading(
        async () => {
          await this.startupService.disableStartupItem(item.path);
          await this.loadData();
        },
        { errorMessage: 'Failed to disable startup item', notificationKey: 'disable' }
      );
    } else {
      await this.runWithLoading(
        async () => {
          await this.startupService.enableStartupItem(item.path);
          await this.loadData();
        },
        { errorMessage: 'Failed to enable startup item', notificationKey: 'enable' }
      );
    }
  }

  onRowAction(event: { action: string; item: StartupItem }) {
    if (event.action === 'toggle') {
      this.toggleItem(event.item);
    }
  }

  onReload() {
    this.loadData();
  }

  getAccentGradient(): string {
    return this.themeService.getAccentGradient();
  }
}
