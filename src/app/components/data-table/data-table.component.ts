/* sys lib */
import { Component, Input, Output, EventEmitter, signal, inject, computed, OnChanges, SimpleChanges } from '@angular/core';
import { CommonModule, DOCUMENT } from '@angular/common';

/* materials */
import { MatCheckboxModule } from '@angular/material/checkbox';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* components */
import { PaginationComponent } from '../pagination/pagination.component';

/* models */
import { TableColumn, TableOptions } from '@models/data-table.model';

@Component({
  selector: 'app-data-table',
  standalone: true,
  imports: [
    CommonModule,
    MatCheckboxModule,
    MatIconModule,
    MatButtonModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    PaginationComponent
  ],
  templateUrl: './data-table.component.html',
})
export class DataTableComponent implements OnChanges {
  @Input() columns: TableColumn[] = [];
  @Input() data: any[] = [];
  @Input() options: TableOptions = {};
  @Input() loading = false;
  @Input() currentPage: number = 1;
  @Input() pageSize: number = 15;
  @Input() pageSizeOptions: number[] = [10, 15, 25, 50, 100];
  @Input() showPageSizeSelector: boolean = true;

  @Input() set selectedKeys(keys: Set<string>) {
    this._selectedKeys = keys;
  }

  public p: number = 1;

  @Output() selectionChange = new EventEmitter<Set<string>>();
  @Output() rowClick = new EventEmitter<any>();
  @Output() rowDoubleClick = new EventEmitter<any>();
  @Output() reload = new EventEmitter<void>();
  @Output() selectedAction = new EventEmitter<Set<string>>();
  @Output() pageChange = new EventEmitter<number>();
  @Output() newPageSize = new EventEmitter<number>();
  @Output() sortChange = new EventEmitter<{ key: string; direction: 'asc' | 'desc' }>();
  @Output() preview = new EventEmitter<any>();

  private document = inject(DOCUMENT);

  _selectedKeys = new Set<string>();

  allSelected = signal(false);
  indeterminate = signal(false);

  sortKey = signal<string | null>(null);
  sortDirection = signal<'asc' | 'desc'>('asc');

  lastSelectedIndex = signal<number | null>(null);

  ngOnChanges(changes: SimpleChanges): void {
    if (changes['currentPage']) {
      this.p = this.currentPage;
    }
    if (changes['data'] && !changes['data'].firstChange) {
      this.p = 1;
    }
  }

  get paginatedData(): any[] {
    let sortedData = [...this.data];

    if (this.sortKey()) {
      const key = this.sortKey()!;
      const direction = this.sortDirection();
      sortedData.sort((a, b) => {
        const aVal = a[key];
        const bVal = b[key];

        if (aVal === bVal) return 0;
        if (aVal === null || aVal === undefined) return 1;
        if (bVal === null || bVal === undefined) return -1;

        let comparison: number;

        if (typeof aVal === 'number' && typeof bVal === 'number') {
          comparison = aVal - bVal;
        } else if (typeof aVal === 'boolean' && typeof bVal === 'boolean') {
          comparison = (aVal === bVal) ? 0 : aVal ? -1 : 1;
        } else {
          comparison = String(aVal).localeCompare(String(bVal));
        }

        return direction === 'asc' ? comparison : -comparison;
      });
    }

    const startIndex = (this.p - 1) * this.pageSize;
    const endIndex = startIndex + this.pageSize;
    return sortedData.slice(startIndex, endIndex);
  }

  get totalItems(): number {
    return this.data.length;
  }

  onPageChange(page: number): void {
    this.p = page;
    this.pageChange.emit(this.p);
  }

  get isDark(): boolean {
    return this.document.body.classList.contains('dark');
  }

  get showCheckbox(): boolean {
    return this.options.showCheckbox ?? false;
  }

  get checkboxKey(): string {
    return this.options.checkboxKey ?? 'id';
  }

  get selectedActionText(): string {
    return this.options.selectedActionText ?? 'Action';
  }

  get showReloadButton(): boolean {
    return this.options.showReloadButton ?? false;
  }

  get showSelectedActions(): boolean {
    return this.options.showSelectedActions ?? false;
  }

  isSelected(item: any): boolean {
    const key = item[this.checkboxKey];
    return this._selectedKeys.has(key);
  }

  toggleSelectAll(checked: boolean): void {
    if (checked) {
      this.data.forEach(item => {
        this._selectedKeys.add(item[this.checkboxKey]);
      });
      this.allSelected.set(true);
      this.indeterminate.set(false);
    } else {
      this._selectedKeys.clear();
      this.allSelected.set(false);
      this.indeterminate.set(false);
    }
    this.selectionChange.emit(new Set(this._selectedKeys));
  }

  toggleSelectItem(item: any, checked: boolean): void {
    const key = item[this.checkboxKey];
    if (checked) {
      this._selectedKeys.add(key);
    } else {
      this._selectedKeys.delete(key);
    }
    this.updateSelectionState();
    this.selectionChange.emit(new Set(this._selectedKeys));
  }

  updateSelectionState(): void {
    const total = this.data.length;
    const selected = this._selectedKeys.size;

    if (selected === 0) {
      this.allSelected.set(false);
      this.indeterminate.set(false);
    } else if (selected === total) {
      this.allSelected.set(true);
      this.indeterminate.set(false);
    } else {
      this.allSelected.set(false);
      this.indeterminate.set(true);
    }
  }

  getAlignClass(column: TableColumn): string {
    return {
      left: 'text-left',
      center: 'text-center',
      right: 'text-right'
    }[column.align || 'left'];
  }

  formatSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  onRowClick(item: any, event?: MouseEvent): void {
    const key = item[this.checkboxKey];
    const currentIndex = this.paginatedData.findIndex(d => d[this.checkboxKey] === key);
    const lastIndex = this.lastSelectedIndex();
    const isCurrentlySelected = this._selectedKeys.has(key);

    if (event && event.shiftKey && lastIndex !== null) {
      const start = Math.min(lastIndex, currentIndex);
      const end = Math.max(lastIndex, currentIndex);

      for (let i = start; i <= end; i++) {
        const rangeKey = this.paginatedData[i][this.checkboxKey];
        if (isCurrentlySelected) {
          this._selectedKeys.delete(rangeKey);
        } else {
          this._selectedKeys.add(rangeKey);
        }
      }
      this.updateSelectionState();
      this.selectionChange.emit(new Set(this._selectedKeys));
    } else {
      if (isCurrentlySelected) {
        this._selectedKeys.delete(key);
      } else {
        this._selectedKeys.add(key);
      }
      this.lastSelectedIndex.set(currentIndex);
      this.updateSelectionState();
      this.selectionChange.emit(new Set(this._selectedKeys));
    }

    this.rowClick.emit(item);
  }

  onPreviewClick(event: MouseEvent): void {
    event.stopPropagation();
  }

  onReload(): void {
    this.reload.emit();
  }

  onSelectedAction(): void {
    this.selectedAction.emit(new Set(this._selectedKeys));
  }

  onPageSizeChange(size: number): void {
    this.pageSize = size;
    this.p = 1; // Reset to first page
    this.newPageSize.emit(size);
    this.pageChange.emit(this.p);
  }

  onSort(column: TableColumn): void {
    if (!column.sortable) return;

    if (this.sortKey() === column.key) {
      this.sortDirection.update(d => d === 'asc' ? 'desc' : 'asc');
    } else {
      this.sortKey.set(column.key);
      this.sortDirection.set('asc');
    }
    this.sortChange.emit({ key: this.sortKey()!, direction: this.sortDirection() });
  }

  getSortIcon(column: TableColumn): string {
    if (!column.sortable) return '';
    if (this.sortKey() !== column.key) return 'unfold_more';
    return this.sortDirection() === 'asc' ? 'arrow_upward' : 'arrow_downward';
  }

  isSortActive(column: TableColumn): boolean {
    return this.sortKey() === column.key;
  }

  onRowDoubleClick(item: any): void {
    this.rowDoubleClick.emit(item);
  }

  onPreview(item: any): void {
    this.preview.emit(item);
  }

  get showPreviewButton(): boolean {
    return this.options.showPreviewButton ?? false;
  }
}
