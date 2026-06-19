/* sys lib */
import {
  ChangeDetectionStrategy,
  Component,
  inject,
  Input,
  Output,
  EventEmitter,
  OnChanges,
  OnDestroy,
  SimpleChanges,
  signal,
  untracked,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { FormControl } from '@angular/forms';
import { Subject } from 'rxjs';
import { debounceTime, distinctUntilChanged, takeUntil } from 'rxjs/operators';

import { MatCheckboxModule } from '@angular/material/checkbox';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

/* components */
import { PaginationComponent } from '../pagination/pagination.component';

/* models */
import { ListColumn, ListOptions, ListActionEvent } from '@entities/data-list.model';
import { formatSize } from '@shared/utils/format.util';
import { ConfirmDialogService } from '@shared/confirm-dialog';
import { sortData, filterData, getCellValue, rowRecord } from '@shared/utils/list.helpers';

@Component({
  selector: 'app-data-list',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    MatCheckboxModule,
    MatIconModule,
    MatButtonModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    FormsModule,
    ReactiveFormsModule,
    PaginationComponent,
  ],
  templateUrl: './data-list.component.html',
})
export class DataListComponent<T extends object = object> implements OnChanges {
  @Input() columns: ListColumn[] = [];
  @Input() options: ListOptions = {};
  @Input() loading = false;
  @Input() currentPage = 1;
  @Input() pageSize = 15;
  @Input() pageSizeOptions: number[] = [10, 15, 25, 50, 100];
  @Input() showPageSizeSelector = true;
  @Input() actionLabel?: string;
  @Input() checkboxKey = 'id';
  @Input() data: T[] = [];

  @Input() set selectedKeys(keys: Set<string>) {
    this._selectedKeys = keys;
  }

  @Output() rowClick = new EventEmitter<T>();
  @Output() rowDoubleClick = new EventEmitter<T>();
  @Output() reload = new EventEmitter<void>();
  @Output() rowAction = new EventEmitter<ListActionEvent<T>>();
  @Output() pageChange = new EventEmitter<number>();
  @Output() newPageSize = new EventEmitter<number>();
  @Output() sortChange = new EventEmitter<{ key: string; direction: 'asc' | 'desc' }>();
  @Output() searchChange = new EventEmitter<string>();
  @Output() selectionChange = new EventEmitter<Set<string>>();

  _selectedKeys = new Set<string>();
  _originalData: T[] = [];
  _filteredData: T[] = [];

  sortKey = signal<string | null>(null);
  sortDirection = signal<'asc' | 'desc'>('asc');
  searchQuery = signal<string>('');

  toolbarExpanded = signal(false);
  toolbarLocked = signal(false);

  searchControl = new FormControl('');
  private destroy$ = new Subject<void>();

  formatSize = formatSize;

  private confirmDialogService = inject(ConfirmDialogService);

  p = 1;

  constructor() {
    this.searchControl.valueChanges
      .pipe(debounceTime(300), distinctUntilChanged(), takeUntil(this.destroy$))
      .subscribe((value: string | null) => {
        const query = value ?? '';
        this.searchQuery.set(query);
        untracked(() => this.onSearch());
        this.searchChange.emit(query);
      });
  }

  ngOnDestroy(): void {
    this.destroy$.next();
    this.destroy$.complete();
  }

  ngOnChanges(changes: SimpleChanges): void {
    if (changes['currentPage']) {
      this.p = this.currentPage;
    }

    if (changes['data'] && changes['data'].currentValue) {
      this._originalData = [...changes['data'].currentValue];
      this.onSearch();
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

  onSearch(): void {
    this._filteredData = filterData(this._originalData, this.searchQuery(), rowRecord);
  }

  rowRecord = rowRecord;

  get paginatedData(): T[] {
    const startIndex = (this.p - 1) * this.pageSize;
    const endIndex = startIndex + this.pageSize;
    return this.sortedFilteredData.slice(startIndex, endIndex);
  }

  get sortedFilteredData(): T[] {
    if (!this.sortKey()) {
      return [...this._filteredData];
    }
    return sortData(this._filteredData, this.sortKey()!, this.sortDirection(), (item, key) =>
      this.cellValue(item, key)
    );
  }

  get totalItems(): number {
    return this._filteredData.length;
  }

  onPageChange(page: number): void {
    this.p = page;
    this.pageChange.emit(page);
  }

  get showCheckbox(): boolean {
    return this.options.showCheckbox ?? false;
  }

  get showSelectAll(): boolean {
    return this.options.showSelectAll ?? this.showCheckbox;
  }

  get showReloadButton(): boolean {
    return this.options.showReloadButton ?? false;
  }

  get showSearch(): boolean {
    return this.options.showSearch ?? true;
  }

  get searchTogglable(): boolean {
    return this.options.searchTogglable ?? false;
  }

  get showPagination(): boolean {
    return this.options.showPagination ?? true;
  }

  get showActions(): boolean {
    return this.options.showActions ?? false;
  }

  get actionsPosition(): 'right' | 'bottom' {
    return this.options.actionsPosition ?? 'right';
  }

  get rowClassFn(): ((item: T) => string) | null {
    if (!this.options.rowClass) return null;
    if (typeof this.options.rowClass === 'function') {
      return this.options.rowClass as (item: T) => string;
    }
    return () => this.options.rowClass as string;
  }

  getRowClass(item: T): string {
    const fn = this.rowClassFn;
    return fn ? fn(item) : '';
  }

  getCellClass(column: ListColumn, item: T): string {
    if (!column.cellClass) return '';

    if (typeof column.cellClass === 'function') {
      return column.cellClass(item) || '';
    }

    return column.cellClass;
  }

  getBadgeClass(badge: string, item: T): string {
    const badgeClass = this.columns.find((c) => c.badge === badge)?.badgeClass;
    if (!badgeClass) return 'badge-primary';

    if (typeof badgeClass === 'function') {
      return badgeClass(item) || 'badge-primary';
    }

    return badgeClass;
  }

  getItemIcon(item: T): string {
    const iconCol = this.columns.find((c) => c.iconKey);
    if (iconCol) {
      const iconVal = this.cellValue(item, iconCol.iconKey!);
      if (typeof iconVal === 'string') {
        const iconMap: Record<string, string> = {
          text: 'text_snippet',
          link: 'link',
          image: 'image',
          folder: 'folder',
          file: 'description',
          running: 'play_circle',
          stopped: 'stop_circle',
          active: 'check_circle',
          inactive: 'cancel',
          true: 'check_circle',
          false: 'cancel',
        };
        return iconMap[iconVal] ?? iconVal;
      }
    }
    const staticIcon = this.columns.find((c) => c.icon);
    return staticIcon?.icon ?? 'description';
  }

  getTimestamp(item: T): string {
    const tsCol = this.columns.find((c) => c.timestamp);
    if (!tsCol) return '';
    const tsVal = this.cellValue(item, tsCol.timestamp!);
    return typeof tsVal === 'string' ? tsVal : String(tsVal ?? '');
  }

  getSecondaryValue(item: T): string {
    const col = this.primaryColumn;
    if (!col) return '';
    if (col.secondaryKey) {
      const v = this.cellValue(item, col.secondaryKey);
      return typeof v === 'string' ? v : String(v ?? '');
    }
    return col.secondary ? this.formatCell(col.secondary, item) : '';
  }

  getFormattedValue(column: ListColumn, item: T): string {
    const value = this.cellValue(item, column.key);
    if (value === null || value === undefined) return '';

    if (column.format === 'size' && typeof value === 'number') {
      return formatSize(value);
    }

    return String(value);
  }

  onRowClick(item: T, event?: MouseEvent): void {
    if (this.showCheckbox) {
      const key = String(this.rowRecord(item)[this.checkboxKey]);
      const isCurrentlySelected = this._selectedKeys.has(key);

      if (isCurrentlySelected) {
        this._selectedKeys.delete(key);
      } else {
        this._selectedKeys.add(key);
      }
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

  onRowDoubleClick(item: T): void {
    this.rowDoubleClick.emit(item);
  }

  onReload(): void {
    this.reload.emit();
  }

  onPageSizeChange(size: number): void {
    this.pageSize = size;
    this.p = 1;
    this.newPageSize.emit(size);
    this.pageChange.emit(this.p);
  }

  onSort(column: ListColumn): void {
    if (!column.sortable) return;

    if (this.sortKey() === column.key) {
      this.sortDirection.update((d) => (d === 'asc' ? 'desc' : 'asc'));
    } else {
      this.sortKey.set(column.key);
      this.sortDirection.set('asc');
    }
    this.sortChange.emit({ key: this.sortKey()!, direction: this.sortDirection() });
  }

  getSortIcon(column: ListColumn): string {
    if (!column.sortable) return '';
    if (this.sortKey() !== column.key) return 'unfold_more';
    return this.sortDirection() === 'asc' ? 'arrow_upward' : 'arrow_downward';
  }

  isSortActive(column: ListColumn): boolean {
    return this.sortKey() === column.key;
  }

  async onRowAction(action: ListActionEvent<T>): Promise<void> {
    const act = this.rowActions.find((a) => a.id === action.action);
    if (act?.confirmMessage) {
      const confirmed = await this.confirmDialogService.confirm({
        title: 'Confirm Action',
        message: act.confirmMessage,
      });
      if (!confirmed) {
        return;
      }
    }
    this.rowAction.emit(action);
  }

  isActionDisabled(actionId: string, item: T): boolean {
    const act = this.rowActions.find((a) => a.id === actionId);
    if (!act?.disabled) return false;
    return act.disabled(item);
  }

  get hasRowActions(): boolean {
    return this.columns.some((c) => c.actions && c.actions.length > 0);
  }

  get rowActions() {
    return this.columns.flatMap((c) => c.actions ?? []);
  }

  isActionToggle(action: ListActionEvent<T>): boolean {
    const act = this.rowActions.find((a) => a.id === action.action);
    return act?.toggle ?? false;
  }

  getActionToggleState(actionId: string, item: T): boolean {
    const act = this.rowActions.find((a) => a.id === actionId);
    if (!act?.toggleState) return false;
    return act.toggleState(item);
  }

  get primaryColumn(): ListColumn | undefined {
    return this.columns[0];
  }

  get hasSecondaryContent(): boolean {
    const col = this.primaryColumn;
    return !!(col && (col.secondary || col.secondaryKey || col.timestamp));
  }

  getActionClass(action: ListActionEvent<T>, item: T): string {
    const act = this.rowActions.find((a) => a.id === action.action);
    let cls = act?.class ?? 'action-btn';

    if (this.isActionToggle(action) && this.getActionToggleState(action.action, item)) {
      cls += ' text-success';
    }

    return cls;
  }

  updateSelectionState(): void {}

  cellValue = getCellValue;

  formatCell(key: string, item: T): string {
    const value = this.cellValue(item, key);
    if (value === null || value === undefined) return '';
    return String(value);
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
      this._originalData.forEach((item) => {
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

  trackByFn(index: number, item: T): number {
    return index;
  }

  private getKey(item: T): string {
    return String(this.rowRecord(item)[this.checkboxKey]);
  }
}
