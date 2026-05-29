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
import { NotificationService } from '@services/notification.service';
import { StartupItem } from '@models/startup.model';
import { SearchComponent } from '@components/search/search.component';
import { PaginationComponent } from '@components/pagination/pagination.component';

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
    SearchComponent,
    PaginationComponent,
  ],
  templateUrl: './startup.view.html',
})
export class StartupView implements OnInit {
  private startupService = inject(StartupService);
  private notification = inject(NotificationService);

  startupData = signal<StartupItem[]>([]);
  filteredData = signal<StartupItem[]>([]);
  loading = signal(false);

  currentPage = signal(1);
  pageSize = signal(15);

  totalItems = computed(() => this.startupData().length);
  enabledItems = computed(() => this.startupData().filter((s) => s.enabled).length);
  disabledItems = computed(() => this.startupData().filter((s) => !s.enabled).length);

  paginatedFilteredData = computed(() => {
    const start = (this.currentPage() - 1) * this.pageSize();
    return this.filteredData().slice(start, start + this.pageSize());
  });

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
    this.loading.set(true);
    this.currentPage.set(1);
    try {
      const data = await this.startupService.getStartupItems();
      this.startupData.set(data);
      this.filteredData.set(data);
    } catch (error: unknown) {
      console.error('Failed to load startup items:', error);
    } finally {
      this.loading.set(false);
    }
  }

  onFilteredData(data: object[]): void {
    this.filteredData.set(data as StartupItem[]);
    this.currentPage.set(1);
  }

  async toggleItem(item: StartupItem) {
    if (item.enabled) {
      const confirmed = confirm(`Disable "${item.name}" from starting at login?`);
      if (!confirmed) return;
      try {
        this.loading.set(true);
        await this.startupService.disableStartupItem(item.path);
        await this.loadData();
      } catch (error: unknown) {
        this.notification.error(
          'Failed to disable startup item: ' +
            (error instanceof Error ? error.message : String(error)),
          error
        );
      } finally {
        this.loading.set(false);
      }
    } else {
      try {
        this.loading.set(true);
        await this.startupService.enableStartupItem(item.path);
        await this.loadData();
      } catch (error: unknown) {
        this.notification.error(
          'Failed to enable startup item: ' +
            (error instanceof Error ? error.message : String(error)),
          error
        );
      } finally {
        this.loading.set(false);
      }
    }
  }
}
