import { ChangeDetectionStrategy, Component, input, output, signal, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { AppResidue } from '@services/app-residue.service';
import { formatSize } from '@shared/utils/format.util';
import { PaginationComponent } from '@components/pagination/pagination.component';

type ResidueTabType = 'configs' | 'data' | 'caches';

@Component({
  selector: 'app-residue-tab',
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
          <div
            class="search-controls-bar"
            (mouseenter)="onSearchHoverEnter()"
            (mouseleave)="onSearchHoverLeave()"
          >
            @if (!searchVisible()) {
              <button class="search-toggle-btn" (click)="showSearchInput()">
                <mat-icon class="toggle-icon" fontIcon="search" />
              </button>
            } @else {
              <div class="search-input-wrapper">
                <mat-icon class="search-icon" fontIcon="search" />
                <input
                  type="text"
                  class="search-input"
                  [placeholder]="'Search ' + tabType() + '...'"
                  [value]="searchQuery()"
                  (input)="onSearchChange($any($event.target).value)"
                  (focus)="onSearchFocus()"
                  (blur)="onSearchBlur()"
                />
                @if (searchQuery()) {
                  <button (click)="clearSearch()" class="search-clear-btn">
                    <mat-icon class="clear-icon" fontIcon="close" />
                  </button>
                }
                <button (click)="hideSearchInput()" class="search-close-btn">
                  <mat-icon class="close-icon" fontIcon="close" />
                </button>
              </div>
            }
          </div>

          @if (data().length > 0) {
            <label
              class="custom-checkbox select-all-checkbox"
              [matTooltip]="allSelected() ? 'Deselect All' : 'Select All'"
            >
              <input
                type="checkbox"
                [checked]="allSelected()"
                (change)="selectAllEvent.emit($any($event.target).checked)"
              />
              <span class="checkmark" [class.indeterminate]="indeterminate()">
                @if (indeterminate()) {
                  <mat-icon class="check-icon" fontIcon="remove" />
                } @else {
                  <mat-icon class="check-icon" fontIcon="check" />
                }
              </span>
            </label>
          }

          <button (click)="refresh.emit()" class="btn btn-ghost" matTooltip="Refresh">
            <mat-icon fontIcon="refresh" />
          </button>
        </div>
        @if (selectedCount() > 0) {
          <div class="flex items-center gap-2">
            <span class="text-sm text-zinc-400 dark:text-zinc-500"
              >{{ selectedCount() }} selected ({{ formatSize(selectedSize()) }})</span
            >
            <button (click)="previewClean.emit()" class="btn btn-primary">Preview Clean</button>
            <button (click)="cleanSelected.emit()" class="btn btn-danger">Clean Selected</button>
          </div>
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
          <mat-icon class="icon-4xl mb-4" [fontIcon]="emptyIcon()" />
          <p class="text-lg font-medium">No {{ tabType() }} residues found</p>
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
                  <mat-icon class="text-zinc-500 dark:text-zinc-400" fontIcon="folder" />
                  <div class="flex flex-col min-w-0 flex-1">
                    <span
                      class="font-medium text-white dark:text-zinc-900 truncate"
                      [matTooltip]="item.app_name"
                      >{{ item.app_name }}</span
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
                    class="px-2 py-1 rounded text-xs font-medium"
                    [class]="getResidueTypeColor(item)"
                  >
                    {{ getResidueTypeLabel(item.residue_type) }}
                  </span>
                  @if (item.detected_as_uninstalled) {
                    <span class="px-2 py-1 rounded text-xs bg-error/10 text-error"
                      >Uninstalled</span
                    >
                  }
                  <span class="text-sm font-medium text-zinc-400 dark:text-zinc-500">{{
                    formatSize(item.size)
                  }}</span>
                  <mat-icon
                    class="text-success"
                    [class.opacity-0]="!isSelected(item.path)"
                    fontIcon="check_circle"
                  />
                </div>
              </div>
            }
          </div>
        </div>
        <app-pagination
          [totalItems]="data().length"
          [currentPage]="currentPage()"
          [pageSize]="pageSize()"
          (pageChange)="pageChange.emit($event)"
          (pageSizeChange)="pageSizeChange.emit($event)"
        />
      }
    </div>
  `,
})
export class ResidueTabComponent {
  tabType = input.required<ResidueTabType>();
  data = input.required<AppResidue[]>();
  loading = input.required<boolean>();
  selectedKeys = input.required<Set<string>>();
  currentPage = input.required<number>();
  pageSize = input.required<number>();
  totalItems = input.required<number>();

  refresh = output<void>();
  previewClean = output<void>();
  cleanSelected = output<void>();
  pageChange = output<number>();
  pageSizeChange = output<number>();
  toggleSelectionEvent = output<string>();
  selectAllEvent = output<boolean>();

  formatSize = formatSize;

  searchVisible = signal(false);
  searchQuery = signal('');
  searchFocused = signal(false);

  allSelected = computed(() => {
    const total = this.data().length;
    const selected = this.selectedKeys().size;
    return total > 0 && selected === total;
  });

  indeterminate = computed(() => {
    const total = this.data().length;
    const selected = this.selectedKeys().size;
    return selected > 0 && selected < total;
  });

  paginatedData = computed(() => {
    const start = (this.currentPage() - 1) * this.pageSize();
    return this.data().slice(start, start + this.pageSize());
  });

  selectedCount = computed(() => this.selectedKeys().size);

  selectedSize = computed(() => {
    return this.data()
      .filter((item) => this.selectedKeys().has(item.path))
      .reduce((sum, item) => sum + item.size, 0);
  });

  emptyIcon = computed(() => {
    switch (this.tabType()) {
      case 'configs':
        return 'folder_open';
      case 'data':
        return 'storage';
      case 'caches':
        return 'cached';
    }
  });

  isSelected(path: string): boolean {
    return this.selectedKeys().has(path);
  }

  showSearchInput(): void {
    this.searchVisible.set(true);
  }

  hideSearchInput(): void {
    if (!this.searchFocused()) {
      this.searchVisible.set(false);
    }
  }

  onSearchHoverEnter(): void {
    if (!this.searchVisible()) {
      this.searchVisible.set(true);
    }
  }

  onSearchHoverLeave(): void {
    this.hideSearchInput();
  }

  onSearchFocus(): void {
    this.searchFocused.set(true);
  }

  onSearchBlur(): void {
    this.searchFocused.set(false);
  }

  clearSearch(): void {
    this.searchQuery.set('');
  }

  onSearchChange(query: string): void {
    this.searchQuery.set(query.toLowerCase());
  }

  getResidueTypeLabel(type: string): string {
    switch (type) {
      case 'Config':
        return 'Config';
      case 'Data':
        return 'Data';
      case 'Cache':
        return 'Cache';
      case 'Both':
        return 'Config/Data';
      default:
        return type;
    }
  }

  getResidueTypeColor(residue: AppResidue): string {
    switch (residue.residue_type) {
      case 'Config':
        return 'bg-blue-500/10 text-blue-600';
      case 'Data':
        return 'bg-green-500/10 text-green-600';
      case 'Cache':
        return 'bg-orange-500/10 text-orange-600';
      case 'Both':
        return 'bg-purple-500/10 text-purple-600';
      default:
        return 'bg-slate-500/10 text-slate-600';
    }
  }
}
