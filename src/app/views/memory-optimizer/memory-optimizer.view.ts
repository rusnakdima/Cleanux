/* sys lib */
import {
  ChangeDetectionStrategy,
  Component,
  OnInit,
  OnDestroy,
  signal,
  inject,
  NgZone,
  computed,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* services */
import {
  MemoryOptimizerService,
  MemoryInfo,
  SwapInfo,
  ProcessMemory,
} from '@services/memory-optimizer.service';

/* components */
import { DataTableComponent } from '@components/data-table/data-table.component';
import { PaginationComponent } from '@components/pagination/pagination.component';

/* models */
import { TableColumn, TableOptions } from '@models/data-table.model';

@Component({
  selector: 'app-memory-optimizer-view',
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
  templateUrl: './memory-optimizer.view.html',
})
export class MemoryOptimizerView implements OnInit, OnDestroy {
  private memoryService = inject(MemoryOptimizerService);
  private ngZone = inject(NgZone);
  private refreshInterval: ReturnType<typeof setInterval> | null = null;

  memoryInfo = signal<MemoryInfo | null>(null);
  swapInfo = signal<SwapInfo | null>(null);
  processes = signal<ProcessMemory[]>([]);
  loading = signal(false);
  optimizing = signal(false);
  sortColumn = signal<string>('memory_mb');
  sortDirection = signal<'asc' | 'desc'>('desc');

  currentPage = signal(1);
  pageSize = signal(15);

  paginatedProcesses = computed(() => {
    const data = this.processes();
    const start = (this.currentPage() - 1) * this.pageSize();
    return data.slice(start, start + this.pageSize());
  });

  ngOnInit() {
    this.loadData();
    this.ngZone.runOutsideAngular(() => {
      this.refreshInterval = setInterval(() => {
        this.ngZone.run(() => this.loadData());
      }, 3000);
    });
  }

  ngOnDestroy() {
    if (this.refreshInterval) {
      clearInterval(this.refreshInterval);
    }
  }

  async loadData() {
    try {
      const [memory, swap, procData] = await Promise.all([
        this.memoryService.getMemoryInfo(),
        this.memoryService.getSwapInfo(),
        this.memoryService.getProcessMemory(),
      ]);
      this.memoryInfo.set(memory);
      this.swapInfo.set(swap);
      this.processes.set(this.sortProcesses(procData));
    } catch (error) {
      console.error('Failed to load memory data:', error);
    }
  }

  sortProcesses(processes: ProcessMemory[]): ProcessMemory[] {
    const col = this.sortColumn();
    const dir = this.sortDirection();
    return [...processes].sort((a, b) => {
      const aVal = (a as any)[col] ?? '';
      const bVal = (b as any)[col] ?? '';
      if (typeof aVal === 'number' && typeof bVal === 'number') {
        return dir === 'asc' ? aVal - bVal : bVal - aVal;
      } else {
        const aStr = String(aVal).toLowerCase();
        const bStr = String(bVal).toLowerCase();
        return dir === 'asc' ? aStr.localeCompare(bStr) : bStr.localeCompare(aStr);
      }
    });
  }

  onSort(column: string) {
    if (this.sortColumn() === column) {
      this.sortDirection.update((d) => (d === 'asc' ? 'desc' : 'asc'));
    } else {
      this.sortColumn.set(column);
      this.sortDirection.set('desc');
    }
    this.processes.set(this.sortProcesses(this.processes()));
  }

  onPageChange(page: number): void {
    this.currentPage.set(page);
  }

  onPageSizeChange(size: number): void {
    this.pageSize.set(size);
    this.currentPage.set(1);
  }

  async optimizeMemory() {
    this.optimizing.set(true);
    try {
      await this.memoryService.optimizeMemory();
      await this.loadData();
    } catch (error) {
      console.error('Failed to optimize memory:', error);
    } finally {
      this.optimizing.set(false);
    }
  }

  formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  get memoryUsagePercent(): number {
    const mem = this.memoryInfo();
    if (!mem || mem.total === 0) return 0;
    return (mem.used / mem.total) * 100;
  }

  get swapUsagePercent(): number {
    const swap = this.swapInfo();
    if (!swap || swap.total === 0) return 0;
    return (swap.used / swap.total) * 100;
  }

  processColumns: TableColumn[] = [
    { key: 'name', label: 'Name', width: 'flex-1', sortable: true },
    { key: 'pid', label: 'PID', width: 'w-24', sortable: true },
    { key: 'memory_mb', label: 'Memory (MB)', width: 'w-32', sortable: true, align: 'right' },
    { key: 'cpu_percent', label: 'CPU %', width: 'w-24', sortable: true, align: 'right' },
  ];

  trackByPid(index: number, proc: ProcessMemory): number {
    return proc.pid;
  }

  getTableOptions(): TableOptions {
    return {
      showHeader: true,
      showCheckbox: false,
      hoverable: true,
      showReloadButton: true,
      showSelectedActions: false,
      showPreviewButton: false,
      showSearch: true,
      searchPlaceholder: 'Search processes...',
    };
  }
}
