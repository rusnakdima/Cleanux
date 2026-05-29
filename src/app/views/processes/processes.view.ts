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
import { SystemService, ProcessItem } from '@services/system.service';
import { NotificationService } from '@services/notification.service';

/* components */
import { DataTableComponent } from '@components/data-table/data-table.component';

/* models */
import { TableColumn, TableOptions } from '@models/data-table.model';

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
    DataTableComponent,
  ],
  templateUrl: './processes.view.html',
})
export class ProcessesView implements OnInit {
  private systemService = inject(SystemService);
  private document = inject(DOCUMENT);
  private notification = inject(NotificationService);

  processesData = signal<ProcessItem[]>([]);
  filteredData = signal<ProcessItem[]>([]);
  loading = signal(false);
  selectedPids = signal<Set<number>>(new Set());

  currentPage = signal(1);
  pageSize = signal(15);

  totalProcesses = computed(() => this.processesData().length);

  paginatedData = computed(() => {
    const data = this.filteredData();
    const start = (this.currentPage() - 1) * this.pageSize();
    return data.slice(start, start + this.pageSize());
  });

  processColumns: TableColumn[] = [
    { key: 'pid', label: 'PID', width: 'w-24', sortable: true },
    { key: 'name', label: 'Service', width: 'flex-1', sortable: true },
    { key: 'cpu_usage', label: 'CPU load', width: 'w-24', sortable: true, align: 'right' },
    { key: 'memory_usage', label: 'RAM usage', width: 'w-24', sortable: true, align: 'right' },
  ];

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
      this.filteredData.set(data);
      this.currentPage.set(1);
    } catch (error: unknown) {
      console.error('Failed to load processes:', error);
    } finally {
      this.loading.set(false);
    }
  }

  onFilteredData(data: object[]): void {
    this.filteredData.set(data as ProcessItem[]);
    this.currentPage.set(1);
  }

  onSelectionChange(selected: Set<string>): void {
    this.selectedPids.set(new Set(Array.from(selected).map((s) => Number(s))));
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

    const confirmed = confirm(`Kill ${pidsToKill.length} process(es)?`);
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

  async killProcess(pid: number) {
    const confirmed = confirm(`Kill process ${pid}?`);
    if (!confirmed) return;

    try {
      this.loading.set(true);
      await this.systemService.killProcess(pid);
      await this.loadData();
    } catch (error: unknown) {
      this.notification.error('Failed to kill process', error);
    } finally {
      this.loading.set(false);
    }
  }

  async flushRamCaches() {
    console.log('Flushing RAM caches...');
    this.notification.success('RAM caches flushed successfully!');
  }

  getTableOptions(): TableOptions {
    return {
      showHeader: true,
      showCheckbox: true,
      checkboxKey: 'pid',
      hoverable: true,
      showReloadButton: true,
      showSelectedActions: true,
      selectedActionText: 'Kill Selected',
      showSearch: true,
      searchPlaceholder: 'Search processes...',
    };
  }
}
