/* sys lib */
import {
  ChangeDetectionStrategy,
  Component,
  OnInit,
  OnDestroy,
  signal,
  inject,
  NgZone,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* services */
import { MemoryOptimizerService } from '@features/memory-optimizer/services/memory-optimizer.service';
import { MemoryInfo, SwapInfo, ProcessMemory } from '@models/memory.model';
import { LoadingErrorMixin } from '@views/mixins/loading-error.mixin';
import { MEMORY_REFRESH_INTERVAL_MS } from '@shared/constants/timeout.constants';

/* components */
import { DataListComponent } from '@components/data-list/data-list.component';

/* models */
import { ListColumn, ListOptions } from '@models/data-list.model';
import { formatSize } from '@shared/utils/format.util';

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
    DataListComponent,
  ],
  templateUrl: './memory-optimizer.view.html',
})
export class MemoryOptimizerView extends LoadingErrorMixin implements OnInit, OnDestroy {
  private memoryService = inject(MemoryOptimizerService);
  private ngZone = inject(NgZone);
  private refreshInterval: ReturnType<typeof setInterval> | null = null;

  formatSize = formatSize;

  memoryInfo = signal<MemoryInfo | null>(null);
  swapInfo = signal<SwapInfo | null>(null);
  processes = signal<ProcessMemory[]>([]);

  optimizing = signal(false);
  currentPage = signal(1);
  pageSize = signal(15);

  columns: ListColumn[] = [
    {
      key: 'name',
      primary: true,
      icon: 'memory',
      badge: 'pid',
      badgeClass: 'badge-primary',
      secondaryKey: 'memory_mb',
      format: 'number',
      sortable: true,
    },
  ];

  options: ListOptions = {
    showSearch: true,
    showCheckbox: false,
    showActions: false,
    showReloadButton: true,
    searchPlaceholder: 'Search processes...',
    emptyMessage: 'No processes found',
  };

  ngOnInit() {
    this.loadData();
    this.ngZone.runOutsideAngular(() => {
      this.refreshInterval = setInterval(() => {
        this.ngZone.run(() => this.loadData());
      }, MEMORY_REFRESH_INTERVAL_MS);
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
      this.processes.set(procData);
    } catch (error) {
      throw error;
    }
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
      throw error;
    } finally {
      this.optimizing.set(false);
    }
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
}
