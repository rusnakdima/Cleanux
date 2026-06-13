import { ChangeDetectionStrategy, Component, input, output, signal, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { OrphanedConfig } from '@services/app-residue.service';
import { PaginationComponent } from '@components/pagination/pagination.component';

@Component({
  selector: 'app-orphaned-tab',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    PaginationComponent,
  ],
  template: `
    <div class="pt-4">
      <div class="flex items-center justify-between mb-4">
        <div class="flex items-center gap-4">
          <button (click)="refresh.emit()" class="btn btn-ghost" matTooltip="Refresh">
            <mat-icon fontIcon="refresh" />
          </button>
        </div>
        @if (selectedCount() > 0) {
          <button (click)="cleanSelected.emit()" class="btn btn-danger">Clean Selected</button>
        }
      </div>

      @if (loading()) {
        <div class="flex items-center justify-center py-12">
          <mat-progress-spinner mode="indeterminate" diameter="48"></mat-progress-spinner>
        </div>
      } @else if (data().length === 0) {
        <div
          class="flex flex-col items-center justify-center py-12 text-zinc-500 dark:text-zinc-400"
        >
          <mat-icon class="icon-4xl mb-4" fontIcon="inventory_2" />
          <p class="text-lg font-medium">No orphaned configs found</p>
        </div>
      } @else {
        <div class="max-h-[calc(100vh-500px)] overflow-auto">
          <div class="space-y-2">
            @for (item of paginatedData(); track item.path) {
              <div
                class="glass-card p-4 flex items-center justify-between hover:bg-zinc-700 dark:hover:bg-zinc-200 transition-colors cursor-pointer"
                [class.bg-indigo-500/20]="isSelected(item.path)"
                (click)="toggleSelectionEvent.emit(item.path)"
              >
                <div class="flex items-center gap-4 flex-1 min-w-0">
                  <mat-icon class="text-zinc-500 dark:text-zinc-400" fontIcon="settings" />
                  <div class="flex flex-col min-w-0 flex-1">
                    <span
                      class="font-medium text-zinc-900 dark:text-white truncate"
                      [matTooltip]="item.package_name"
                      >{{ item.package_name }}</span
                    >
                    <span
                      class="text-xs text-zinc-400 dark:text-zinc-500 truncate"
                      [matTooltip]="item.path"
                      >{{ item.path }}</span
                    >
                  </div>
                </div>
                <div class="flex items-center gap-4">
                  <span
                    class="px-2 py-1 rounded text-xs bg-slate-500/10 text-zinc-400 dark:text-zinc-500"
                  >
                    {{ item.config_type }}
                  </span>
                  <mat-icon
                    class="text-success"
                    [class.opacity-0]="!isSelected(item.path)"
                    fontIcon="check_circle"
                  />
                </div>
              </div>
            }
          </div>
          <app-pagination
            [totalItems]="data().length"
            [currentPage]="currentPage()"
            [pageSize]="pageSize()"
            (pageChange)="pageChange.emit($event)"
            (pageSizeChange)="pageSizeChange.emit($event)"
          />
        </div>
      }
    </div>
  `,
})
export class OrphanedTabComponent {
  data = input.required<OrphanedConfig[]>();
  loading = input.required<boolean>();
  selectedKeys = input.required<Set<string>>();
  currentPage = input.required<number>();
  pageSize = input.required<number>();

  refresh = output<void>();
  cleanSelected = output<void>();
  pageChange = output<number>();
  pageSizeChange = output<number>();
  toggleSelectionEvent = output<string>();

  paginatedData = computed(() => {
    const start = (this.currentPage() - 1) * this.pageSize();
    return this.data().slice(start, start + this.pageSize());
  });

  selectedCount = computed(() => this.selectedKeys().size);

  isSelected(path: string): boolean {
    return this.selectedKeys().has(path);
  }
}
