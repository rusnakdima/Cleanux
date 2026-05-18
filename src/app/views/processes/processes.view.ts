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

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* services */
import { SystemService, ProcessItem } from '@services/system.service';

/* components */
import { DataTableComponent } from '@components/data-table/data-table.component';
import { HeaderComponent } from '@components/header/header.component';
import { SearchComponent } from '@components/search/search.component';

/* models */
import { TableColumn, TableOptions } from '@models/data-table.model';

@Component({
  selector: 'app-processes-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    DataTableComponent,
    HeaderComponent,
    SearchComponent,
  ],
  templateUrl: './processes.view.html',
  styleUrl: './processes.view.css',
})
export class ProcessesView implements OnInit {
  private systemService = inject(SystemService);
  private document = inject(DOCUMENT);

  processesData = signal<ProcessItem[]>([]);
  filteredData = signal<ProcessItem[]>([]);
  loading = signal(false);
  selectedPids = signal<Set<number>>(new Set());

  selectedPidsAsStrings = computed(() => {
    return new Set(Array.from(this.selectedPids()).map((pid) => String(pid)));
  });

  totalProcesses = computed(() => this.processesData().length);

  processColumns: TableColumn[] = [
    { key: 'name', label: 'Name', width: 'flex-1', sortable: true },
    { key: 'pid', label: 'PID', width: 'w-24', sortable: true },
    { key: 'cpu_usage', label: 'CPU %', width: 'w-24', sortable: true, align: 'right' },
    { key: 'memory_usage', label: 'Memory', width: 'w-24', sortable: true, align: 'right' },
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
    } catch (error: unknown) {
      console.error('Failed to load processes:', error);
    } finally {
      this.loading.set(false);
    }
  }

  onFilteredData(data: object[]): void {
    this.filteredData.set(data as ProcessItem[]);
  }

  onSelectionChange(selected: Set<string>): void {
    this.selectedPids.set(new Set(Array.from(selected).map((s) => Number(s))));
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
      alert(
        'Failed to kill processes: ' + (error instanceof Error ? error.message : String(error))
      );
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
      alert('Failed to kill process: ' + (error instanceof Error ? error.message : String(error)));
    } finally {
      this.loading.set(false);
    }
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
    };
  }
}
