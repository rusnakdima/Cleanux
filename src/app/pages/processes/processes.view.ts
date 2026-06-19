/* sys lib */
import {
  ChangeDetectionStrategy,
  Component,
  OnInit,
  signal,
  inject,
  computed,
} from '@angular/core';
import { CommonModule, DOCUMENT } from '@angular/common';
import { FormsModule } from '@angular/forms';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* services */
import { SystemService, ProcessItem } from '@features/system/services/system.service';
import { NotificationService } from '@services/notification.service';
import { ConfirmDialogService } from '@shared/confirm-dialog';

/* components */
import { DataListComponent } from '@components/data-list/data-list.component';

/* models */
import { ListColumn, ListOptions } from '@entities/data-list.model';

@Component({
  selector: 'app-processes-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    FormsModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    DataListComponent,
  ],
  templateUrl: './processes.view.html',
})
export class ProcessesView implements OnInit {
  private systemService = inject(SystemService);
  private document = inject(DOCUMENT);
  private notification = inject(NotificationService);
  private confirmDialogService = inject(ConfirmDialogService);

  processesData = signal<ProcessItem[]>([]);
  loading = signal(false);
  selectedPids = signal<Set<number>>(new Set());

  currentPage = signal(1);
  pageSize = signal(15);

  totalProcesses = computed(() => this.processesData().length);

  columns: ListColumn[] = [
    {
      key: 'name',
      primary: true,
      icon: 'memory',
      badge: 'pid',
      badgeClass: 'badge-primary',
      secondaryKey: 'cpu_usage',
      format: 'number',
      actions: [
        {
          id: 'kill',
          icon: 'stop',
          tooltip: 'Kill process',
          confirmMessage: 'Kill this process?',
        },
      ],
    },
  ];

  options: ListOptions = {
    showSearch: true,
    showCheckbox: true,
    checkboxKey: 'pid',
    showActions: true,
    actionsPosition: 'right',
    showReloadButton: true,
    searchTogglable: true,
    showSelectAll: true,
    searchPlaceholder: 'Search processes...',
    emptyMessage: 'No processes found',
  };

  get isDark(): boolean {
    return this.document.body.classList.contains('dark');
  }

  async ngOnInit() {
    await this.loadData();
  }

  async loadData() {
    this.loading.set(true);
    try {
      const data = await this.systemService.getProcesses();
      this.processesData.set(data);
      this.currentPage.set(1);
    } catch (error: unknown) {
      this.notification.error('Failed to load processes', error);
    } finally {
      this.loading.set(false);
    }
  }

  onSelectionChange(keys: Set<string>): void {
    this.selectedPids.set(new Set(Array.from(keys).map((s) => Number(s))));
  }

  onPageChange(page: number): void {
    this.currentPage.set(page);
  }

  onPageSizeChange(size: number): void {
    this.pageSize.set(size);
    this.currentPage.set(1);
  }

  async killSelectedProcesses() {
    const pidsToKill = Array.from(this.selectedPids());
    if (pidsToKill.length === 0) return;

    const confirmed = await this.confirmDialogService.confirm({
      title: 'Kill Processes',
      message: `Kill ${pidsToKill.length} process(es)?`,
      dangerous: true,
    });
    if (!confirmed) return;

    try {
      this.loading.set(true);
      await this.systemService.killSelectedProcesses(pidsToKill);
      this.selectedPids.set(new Set());
      await this.loadData();
    } catch (error: unknown) {
      this.notification.error('Failed to kill processes', error);
    } finally {
      this.loading.set(false);
    }
  }

  onRowAction(event: { action: string; item: ProcessItem }): void {
    if (event.action === 'kill') {
      this.systemService.killProcess(event.item.pid).then(() => this.loadData());
    }
  }

  async flushRamCaches() {
    this.notification.success('RAM caches flushed successfully!');
  }
}
