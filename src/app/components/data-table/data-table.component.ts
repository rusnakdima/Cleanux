/* sys lib */
import {
  ChangeDetectionStrategy,
  Component,
  effect,
  inject,
  Input,
  Output,
  EventEmitter,
  OnChanges,
  OnDestroy,
  SimpleChanges,
  signal,
  computed,
  untracked,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { FormControl } from '@angular/forms';
import { Subject } from 'rxjs';
import { debounceTime, distinctUntilChanged, takeUntil } from 'rxjs/operators';

import { CdkVirtualScrollViewport, ScrollingModule } from '@angular/cdk/scrolling';
import { MatCheckboxModule } from '@angular/material/checkbox';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* components */
import { PaginationComponent } from '../pagination/pagination.component';

/* models */
import { TableColumn, TableOptions, TableAction, RowActionEvent } from '@models/data-table.model';
import { formatSize } from '@shared/utils/format.util';
import { CheckboxComponent } from '@shared/checkbox';
import { ConfirmDialogService } from '@shared/confirm-dialog';

@Component({
  selector: 'app-data-table',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    ScrollingModule,
    MatCheckboxModule,
    MatIconModule,
    MatButtonModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    FormsModule,
    ReactiveFormsModule,
    PaginationComponent,
    CheckboxComponent,
  ],
  templateUrl: './data-table.component.html',
})
export class DataTableComponent<T extends object = object> implements OnChanges {
  @Input() columns: TableColumn[] = [];
  @Input() options: TableOptions = {};
  @Input() loading = false;
  @Input() currentPage: number = 1;
  @Input() pageSize: number = 15;
  @Input() pageSizeOptions: number[] = [10, 15, 25, 50, 100];
  @Input() showPageSizeSelector: boolean = true;

  @Input() set selectedKeys(keys: Set<string>) {
    this._selectedKeys = keys;
  }

  @Output() rowClick = new EventEmitter<T>();
  @Output() rowDoubleClick = new EventEmitter<T>();
  @Output() reload = new EventEmitter<void>();
  @Output() selectedAction = new EventEmitter<Set<string>>();
  @Output() pageChange = new EventEmitter<number>();
  @Output() newPageSize = new EventEmitter<number>();
  @Output() sortChange = new EventEmitter<{ key: string; direction: 'asc' | 'desc' }>();
  @Output() preview = new EventEmitter<T>();
  @Output() rowAction = new EventEmitter<RowActionEvent<T>>();
  @Output() searchChange = new EventEmitter<string>();
  @Output() selectionChange = new EventEmitter<Set<string>>();

  _selectedKeys = new Set<string>();
  _data: T[] = [];
  _filteredData: T[] = [];

  allSelected = computed(() => {
    if (this._data.length === 0) return false;
    return this._data.every((item) => this._selectedKeys.has(this.getKey(item)));
  });

  indeterminate = computed(() => {
    const selectedCount = this._selectedKeys.size;
    return selectedCount > 0 && selectedCount < this._data.length;
  });

  sortKey = signal<string | null>(null);
  sortDirection = signal<'asc' | 'desc'>('asc');
  searchQuery = signal<string>('');

  lastSelectedIndex = signal<number | null>(null);

  searchControl = new FormControl('');

  formatSize = formatSize;

  columnWidths = signal<Record<string, string>>({});
  resizingColumn = signal<string | null>(null);
  resizeStartX = signal<number>(0);
  resizeStartWidth = signal<number>(0);

  private resizeColumnKey = signal<string | null>(null);

  toolbarExpanded = signal(false);
  toolbarLocked = signal(false);

  public p: number = 1;

  readonly VIRTUAL_SCROLL_THRESHOLD = 100;
  readonly DEFAULT_ROW_HEIGHT = 48;
  private destroy$ = new Subject<void>();

  private confirmDialogService = inject(ConfirmDialogService);

  @Input() set data(value: T[]) {
    this._data = value;
    this._filteredData = [...value];
  }

  get rowHeight(): number {
    return this.options.rowHeight ?? this.DEFAULT_ROW_HEIGHT;
  }

  get useVirtualScroll(): boolean {
    return (this.options.virtualScroll ?? false) && this.totalItems > this.VIRTUAL_SCROLL_THRESHOLD;
  }

  get checkboxKey(): string {
    return this.options.checkboxKey ?? 'id';
  }

  constructor() {
    effect(() => {
      this.searchControl.valueChanges
        .pipe(debounceTime(300), distinctUntilChanged(), takeUntil(this.destroy$))
        .subscribe((value: string | null) => {
          const query = value ?? '';
          this.searchQuery.set(query);
          untracked(() => this.onSearch());
          this.searchChange.emit(query);
        });
    });
  }

  ngOnDestroy(): void {
    this.destroy$.next();
    this.destroy$.complete();
  }

  onSearch(): void {
    const query = this.searchQuery().toLowerCase().trim();
    if (!query) {
      this._filteredData = [...this._data];
      return;
    }

    this._filteredData = this._data.filter((item) => {
      const record = this.rowRecord(item);
      return Object.values(record).some((val) => {
        if (val === null || val === undefined) return false;
        return String(val).toLowerCase().includes(query);
      });
    });
  }

  ngOnChanges(changes: SimpleChanges): void {
    if (changes['currentPage']) {
      this.p = this.currentPage;
    }

    if (changes['data'] && !changes['data'].firstChange) {
      const currLength = changes['data'].currentValue?.length ?? 0;
      const prevLength = changes['data'].previousValue?.length ?? 0;

      if (currLength < prevLength) {
        this.p = 1;
      }
    }

    if (changes['data'] && changes['data'].currentValue) {
      this.initColumnWidths();
    }

    if (
      changes['searchQuery'] &&
      changes['searchQuery'].currentValue !== undefined &&
      changes['searchQuery'].currentValue !== null
    ) {
      this.searchQuery.set(changes['searchQuery'].currentValue);
      this.searchControl.setValue(changes['searchQuery'].currentValue);
    }
  }

  rowRecord(item: T): Record<string, unknown> {
    return item as unknown as Record<string, unknown>;
  }

  private initColumnWidths(): void {
    const widths: Record<string, string> = {};
    for (const col of this.columns) {
      if (col.width) {
        widths[col.key] = col.width;
      }
    }
    this.columnWidths.set(widths);
  }

  getColumnWidth(key: string): string {
    return this.columnWidths()[key] ?? 'flex-1';
  }

  onResizeStart(event: MouseEvent, column: TableColumn): void {
    if (!column.resizable) return;
    event.preventDefault();
    event.stopPropagation();

    this.resizingColumn.set(column.key);
    this.resizeStartX.set(event.clientX);

    const currentWidth = this.columnWidths()[column.key] || this.getColumnWidth(column.key);
    const numericWidth = this.parseWidth(currentWidth);
    this.resizeStartWidth.set(numericWidth);

    this.resizeColumnKey.set(column.key);

    document.addEventListener('mousemove', this.onResizeMove);
    document.addEventListener('mouseup', this.onResizeEnd);
  }

  private onResizeMove = (event: MouseEvent): void => {
    const key = this.resizeColumnKey();
    if (!key) return;

    const delta = event.clientX - this.resizeStartX();
    let newWidth = this.resizeStartWidth() + delta;

    const column = this.columns.find((c) => c.key === key);
    if (column) {
      if (column.minWidth) {
        const minWidth = this.parseWidth(column.minWidth);
        newWidth = Math.max(newWidth, minWidth);
      }
      if (column.maxWidth) {
        const maxWidth = this.parseWidth(column.maxWidth);
        newWidth = Math.min(newWidth, maxWidth);
      }
    }

    newWidth = Math.max(newWidth, 40);

    const newWidthStr = `${newWidth}px`;
    this.columnWidths.update((w: Record<string, string>) => ({ ...w, [key]: newWidthStr }));
  };

  private onResizeEnd = (): void => {
    this.resizingColumn.set(null);
    this.resizeColumnKey.set(null);
    document.removeEventListener('mousemove', this.onResizeMove);
    document.removeEventListener('mouseup', this.onResizeEnd);
  };

  private parseWidth(width: string): number {
    if (width.endsWith('px')) {
      return parseInt(width, 10);
    }
    if (width.endsWith('%')) {
      return parseInt(width, 10);
    }
    if (width === 'flex-1' || width === 'auto') {
      return 100;
    }
    return parseInt(width, 10) || 100;
  }

  get paginatedData(): T[] {
    if (this.useVirtualScroll) {
      return this.sortedFilteredData;
    }

    let sortedData = [...this._filteredData];

    if (this.sortKey()) {
      const key = this.sortKey()!;
      const direction = this.sortDirection();
      sortedData = this.sortData(sortedData, key, direction);
    }

    const startIndex = (this.p - 1) * this.pageSize;
    const endIndex = startIndex + this.pageSize;
    return sortedData.slice(startIndex, endIndex);
  }

  get sortedFilteredData(): T[] {
    if (!this.sortKey()) {
      return [...this._filteredData];
    }
    return this.sortData([...this._filteredData], this.sortKey()!, this.sortDirection());
  }

  rowStatus(item: T): string {
    const s = this.rowRecord(item)['status'];
    return typeof s === 'string' ? s : '';
  }

  get totalItems(): number {
    return this._filteredData.length;
  }

  get virtualItemSize(): number {
    return this.rowHeight;
  }

  onPageChange(page: number): void {
    this.p = page;
    this.pageChange.emit(this.p);
  }

  get showCheckbox(): boolean {
    return this.options.showCheckbox ?? false;
  }

  get selectedActionText(): string {
    return this.options.selectedActionText ?? 'Action';
  }

  get showReloadButton(): boolean {
    return this.options.showReloadButton ?? false;
  }

  get searchTogglable(): boolean {
    return this.options.searchTogglable ?? false;
  }

  get showSelectedActions(): boolean {
    return this.options.showSelectedActions ?? false;
  }

  getAlignClass(column: TableColumn): string {
    return {
      left: 'text-left',
      center: 'text-center',
      right: 'text-right',
    }[column.align || 'left'];
  }

  getCellClass(column: TableColumn, item: T): string {
    if (!column.cellClass) return '';

    if (typeof column.cellClass === 'function') {
      return column.cellClass(item) || '';
    }

    return column.cellClass;
  }

  getFormattedValue(column: TableColumn, item: T): unknown {
    const value = this.cellValue(item, column.key);
    if (value === null || value === undefined) return '';
    return value;
  }

  getSizeClass(item: T): string {
    const size = this.sizeFromItem(item);
    if (size >= 1073741824) return 'size-danger';
    if (size >= 524288000) return 'size-warning';
    return '';
  }

  getStatusClass(item: T): string {
    const status = this.rowStatus(item).toLowerCase();
    if (status === 'running' || status === 'active') return 'status-running';
    if (status === 'stopped' || status === 'exited' || status === 'inactive' || status === 'dead')
      return 'status-stopped';
    return '';
  }

  onRowClick(item: T, event?: MouseEvent): void {
    const key = String(this.rowRecord(item)[this.checkboxKey]);
    const currentIndex = this.useVirtualScroll
      ? this.sortedFilteredData.findIndex(
          (d) => String(this.rowRecord(d)[this.checkboxKey]) === key
        )
      : this.paginatedData.findIndex((d) => String(this.rowRecord(d)[this.checkboxKey]) === key);
    const lastIndex = this.lastSelectedIndex();
    const isCurrentlySelected = this._selectedKeys.has(key);

    if (event && event.shiftKey && lastIndex !== null) {
      const data = this.useVirtualScroll ? this.sortedFilteredData : this.paginatedData;
      const start = Math.min(lastIndex, currentIndex);
      const end = Math.max(lastIndex, currentIndex);

      for (let i = start; i <= end; i++) {
        const rangeKey = String(this.rowRecord(data[i])[this.checkboxKey]);
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

  toggleSelectItem(item: T, checked: boolean): void {
    const key = String(this.rowRecord(item)[this.checkboxKey]);
    if (checked) {
      this._selectedKeys.add(key);
    } else {
      this._selectedKeys.delete(key);
    }
    this.updateSelectionState();
    this.selectionChange.emit(new Set(this._selectedKeys));
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
    this.p = 1;
    this.newPageSize.emit(size);
    this.pageChange.emit(this.p);
  }

  onSort(column: TableColumn): void {
    if (!column.sortable) return;

    if (this.sortKey() === column.key) {
      this.sortDirection.update((d) => (d === 'asc' ? 'desc' : 'asc'));
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

  onRowDoubleClick(item: T): void {
    this.rowDoubleClick.emit(item);
  }

  onPreview(item: T): void {
    this.preview.emit(item);
  }

  async onRowAction(action: TableAction, item: T, event: MouseEvent): Promise<void> {
    event.stopPropagation();

    if (action.confirmMessage) {
      const confirmed = await this.confirmDialogService.confirm({
        title: 'Confirm Action',
        message: action.confirmMessage,
      });
      if (!confirmed) {
        return;
      }
    }

    this.rowAction.emit({ action: action.id, item });
  }

  get showPreviewButton(): boolean {
    return this.options.showPreviewButton ?? false;
  }

  get showRowActions(): boolean {
    return this.options.showRowActions ?? false;
  }

  get hasRowActions(): boolean {
    return this.rowActions.length > 0;
  }

  get rowActions(): TableAction[] {
    for (const col of this.columns) {
      if (col.actions && col.actions.length > 0) {
        return col.actions;
      }
    }
    return [];
  }

  sizeFromItem(item: T): number {
    const s = this.rowRecord(item)['size'];
    return typeof s === 'number' ? s : 0;
  }

  updateSelectionState(): void {}

  cellValue(item: T, key: string): unknown {
    const keys = key.split('.');
    let value: unknown = item;
    for (const k of keys) {
      if (value && typeof value === 'object' && k in value) {
        value = (value as Record<string, unknown>)[k];
      } else {
        return undefined;
      }
    }
    return value;
  }

  sortData(data: T[], key: string, direction: 'asc' | 'desc'): T[] {
    return [...data].sort((a, b) => {
      const aVal = this.cellValue(a, key);
      const bVal = this.cellValue(b, key);

      if (aVal === null || aVal === undefined) return direction === 'asc' ? 1 : -1;
      if (bVal === null || bVal === undefined) return direction === 'asc' ? -1 : 1;

      if (typeof aVal === 'number' && typeof bVal === 'number') {
        return direction === 'asc' ? aVal - bVal : bVal - aVal;
      }

      const aStr = String(aVal).toLowerCase();
      const bStr = String(bVal).toLowerCase();

      if (aStr < bStr) return direction === 'asc' ? -1 : 1;
      if (aStr > bStr) return direction === 'asc' ? 1 : -1;
      return 0;
    });
  }

  clearSearch(): void {
    this.searchControl.setValue('');
  }

  toggleToolbar(): void {
    this.toolbarExpanded.update((v) => !v);
  }

  onToolbarHoverEnter(): void {
    if (!this.searchTogglable) return;
    this.toolbarExpanded.set(true);
  }

  onToolbarHoverLeave(): void {
    if (!this.searchTogglable) return;
    if (!this.toolbarLocked()) {
      this.toolbarExpanded.set(false);
    }
  }

  toggleSelectAll(checked: boolean): void {
    if (checked) {
      this._data.forEach((item) => {
        this._selectedKeys.add(this.getKey(item));
      });
    } else {
      this._selectedKeys.clear();
    }
    this.updateSelectionState();
    this.selectionChange.emit(new Set(this._selectedKeys));
  }

  isSelected(item: T): boolean {
    return this._selectedKeys.has(this.getKey(item));
  }

  formatCell(key: string, item: T): string {
    const value = this.cellValue(item, key);
    if (value === null || value === undefined) return '';
    return String(value);
  }

  trackByFn(index: number, item: T): number {
    return index;
  }

  private getKey(item: T): string {
    return String(this.rowRecord(item)[this.checkboxKey]);
  }
}
