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
import { SystemService, SystemServiceItem } from '@features/system/services/system.service';
import { NotificationService } from '@services/notification.service';
import { getErrorMessage } from '@shared/utils/error.util';
import { ConfirmDialogService } from '@shared/confirm-dialog';

/* components */
import { DataListComponent } from '@components/data-list/data-list.component';

/* models */
import { ListColumn, ListOptions } from '@models/data-list.model';

@Component({
  selector: 'app-system-view',
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
  templateUrl: './system.view.html',
})
export class SystemView implements OnInit {
  private systemService = inject(SystemService);
  private document = inject(DOCUMENT);
  private notification = inject(NotificationService);
  private confirmDialogService = inject(ConfirmDialogService);

  servicesData = signal<SystemServiceItem[]>([]);
  loading = signal(false);
  selectedServices = signal<Set<string>>(new Set());

  totalServices = computed(() => this.servicesData().length);
  runningServices = computed(() => this.servicesData().filter((s) => s.isRunning).length);
  stoppedServices = computed(() => this.servicesData().filter((s) => !s.isRunning).length);

  columns: ListColumn[] = [
    {
      key: 'name',
      primary: true,
      icon: 'settings',
      badge: 'status',
      badgeClass: (item: unknown) => {
        const s = item as SystemServiceItem;
        return s.isRunning ? 'badge-success' : 'badge-error';
      },
      secondaryKey: 'active',
      actions: [
        {
          id: 'stop',
          icon: 'stop',
          tooltip: 'Stop service',
          confirmMessage: 'Stop this service?',
        },
        {
          id: 'start',
          icon: 'play_arrow',
          tooltip: 'Start service',
          confirmMessage: 'Start this service?',
        },
      ],
    },
  ];

  options: ListOptions = {
    showSearch: true,
    showCheckbox: true,
    checkboxKey: 'name',
    showActions: true,
    actionsPosition: 'right',
    showReloadButton: true,
    showSelectAll: true,
    searchPlaceholder: 'Search services...',
    emptyMessage: 'No services found',
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
      const data = await this.systemService.getAllServices();
      this.servicesData.set(data);
    } catch (error: unknown) {
      this.notification.cleanError('load system services', error);
    } finally {
      this.loading.set(false);
    }
  }

  onSelectionChange(keys: Set<string>): void {
    this.selectedServices.set(keys);
  }

  async stopSelectedServices() {
    const servicesToStop = Array.from(this.selectedServices());
    if (servicesToStop.length === 0) return;

    const confirmed = await this.confirmDialogService.confirm({
      title: 'Stop Services',
      message: `Stop ${servicesToStop.length} service(s)?`,
    });
    if (!confirmed) return;

    try {
      this.loading.set(true);
      await this.systemService.stopSelectedServices(servicesToStop);
      this.selectedServices.set(new Set());
      await this.loadData();
    } catch (error: unknown) {
      this.notification.error('Failed to stop services: ' + getErrorMessage(error), error);
    } finally {
      this.loading.set(false);
    }
  }

  async startSelectedServices() {
    const servicesToStart = Array.from(this.selectedServices());
    if (servicesToStart.length === 0) return;

    const confirmed = await this.confirmDialogService.confirm({
      title: 'Start Services',
      message: `Start ${servicesToStart.length} service(s)?`,
    });
    if (!confirmed) return;

    try {
      this.loading.set(true);
      await this.systemService.enableSelectedServices(servicesToStart);
      this.selectedServices.set(new Set());
      await this.loadData();
    } catch (error: unknown) {
      this.notification.error('Failed to start services: ' + getErrorMessage(error), error);
    } finally {
      this.loading.set(false);
    }
  }

  async startService(name: string) {
    const confirmed = await this.confirmDialogService.confirm({
      title: 'Start Service',
      message: `Start ${name}?`,
    });
    if (!confirmed) return;

    try {
      this.loading.set(true);
      await this.systemService.startService(name);
      await this.loadData();
    } catch (error: unknown) {
      this.notification.error('Failed to start service: ' + getErrorMessage(error), error);
    } finally {
      this.loading.set(false);
    }
  }

  onRowAction(event: { action: string; item: SystemServiceItem }): void {
    if (event.action === 'stop') {
      this.systemService.stopSelectedServices([event.item.name]).then(() => this.loadData());
    } else if (event.action === 'start') {
      this.startService(event.item.name);
    }
  }
}
