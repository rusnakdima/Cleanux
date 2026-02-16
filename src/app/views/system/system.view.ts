/* sys lib */
import { Component, OnInit, signal, inject, computed } from '@angular/core';
import { CommonModule, DOCUMENT } from '@angular/common';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* services */
import { SystemService, SystemServiceItem } from '@services/system.service';

/* components */
import { DataTableComponent } from '@components/data-table/data-table.component';
import { HeaderComponent } from '@components/header/header.component';
import { SearchComponent } from '@components/search/search.component';

/* models */
import { TableColumn, TableOptions } from '@models/data-table.model';

@Component({
  selector: 'app-system-view',
  standalone: true,
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
  templateUrl: './system.view.html',
})
export class SystemView implements OnInit {
  private systemService = inject(SystemService);
  private document = inject(DOCUMENT);

  servicesData = signal<SystemServiceItem[]>([]);
  filteredData = signal<SystemServiceItem[]>([]);
  loading = signal(false);
  selectedServices = signal<Set<string>>(new Set());

  totalServices = computed(() => this.servicesData().length);
  runningServices = computed(() => this.servicesData().filter(s => s.isRunning).length);
  stoppedServices = computed(() => this.servicesData().filter(s => !s.isRunning).length);

  serviceColumns: TableColumn[] = [
    { key: 'name', label: 'Name', width: 'flex-1', sortable: true },
    { key: 'active', label: 'Active', width: 'w-32', sortable: true },
    { key: 'status', label: 'Status', align: 'right', width: 'w-32', sortable: true }
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
      const data = await this.systemService.getAllServices();
      this.servicesData.set(data);
      this.filteredData.set(data);
    } catch (error) {
      console.error('Failed to load system services:', error);
    } finally {
      this.loading.set(false);
    }
  }

  onFilteredData(data: SystemServiceItem[]): void {
    this.filteredData.set(data);
  }

  async stopSelectedServices() {
    const servicesToStop = Array.from(this.selectedServices());
    if (servicesToStop.length === 0) return;

    const confirmed = confirm(`Stop ${servicesToStop.length} service(s)?`);
    if (!confirmed) return;

    try {
      this.loading.set(true);
      await this.systemService.stopSelectedServices(servicesToStop);
      this.selectedServices.set(new Set());
      await this.loadData();
    } catch (error) {
      alert('Failed to stop services: ' + error);
    } finally {
      this.loading.set(false);
    }
  }

  async startSelectedServices() {
    const servicesToStart = Array.from(this.selectedServices());
    if (servicesToStart.length === 0) return;

    const confirmed = confirm(`Start ${servicesToStart.length} service(s)?`);
    if (!confirmed) return;

    try {
      this.loading.set(true);
      await this.systemService.enableSelectedServices(servicesToStart);
      this.selectedServices.set(new Set());
      await this.loadData();
    } catch (error) {
      alert('Failed to start services: ' + error);
    } finally {
      this.loading.set(false);
    }
  }

  async startService(service: string) {
    const confirmed = confirm(`Start ${service}?`);
    if (!confirmed) return;

    try {
      this.loading.set(true);
      await this.systemService.startService(service);
      await this.loadData();
    } catch (error) {
      alert('Failed to start service: ' + error);
    } finally {
      this.loading.set(false);
    }
  }

  getTableOptions(): TableOptions {
    return {
      showHeader: true,
      showCheckbox: true,
      checkboxKey: 'name',
      hoverable: true,
      showReloadButton: true,
      showSelectedActions: true,
      selectedActionText: 'Stop Selected',
    };
  }

  getStartTableOptions(): TableOptions {
    return {
      showHeader: true,
      showCheckbox: true,
      checkboxKey: 'name',
      hoverable: true,
      showReloadButton: true,
      showSelectedActions: true,
      selectedActionText: 'Start Selected',
    };
  }
}
