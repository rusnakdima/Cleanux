import {
  Component,
  Input,
  Output,
  EventEmitter,
  signal,
  OnChanges,
  SimpleChanges,
  OnInit,
  OnDestroy,
  inject,
  ChangeDetectionStrategy,
} from '@angular/core';
import { FormControl } from '@angular/forms';
import { Subject, debounceTime, distinctUntilChanged, takeUntil } from 'rxjs';

import { ConfirmDialogService } from '@shared/confirm-dialog';

@Component({
  standalone: false,
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: '',
})
export abstract class BaseListComponent<T extends object = object> implements OnChanges, OnInit, OnDestroy {
  @Input() data: T[] = [];
  @Input() searchQuery: string = '';

  @Output() selectionChange = new EventEmitter<Set<string>>();
  @Output() searchChange = new EventEmitter<string>();

  _selectedKeys = new Set<string>();
  _originalData: T[] = [];
  _filteredData: T[] = [];

  allSelected = signal(false);
  indeterminate = signal(false);

  sortKey = signal<string | null>(null);
  sortDirection = signal<'asc' | 'desc'>('asc');
  searchQuerySignal = signal<string>('');
  toolbarExpanded = signal(false);
  toolbarLocked = signal(false);

  searchControl = new FormControl('');
  protected destroy$ = new Subject<void>();
  protected confirmDialogService: ConfirmDialogService = inject(ConfirmDialogService);

  ngOnChanges(changes: SimpleChanges): void {
    if (changes['data'] && changes['data'].currentValue) {
      this._originalData = [...changes['data'].currentValue];
      this.onSearch();
    }

    if (
      changes['searchQuery'] &&
      changes['searchQuery'].currentValue !== undefined &&
      changes['searchQuery'].currentValue !== null
    ) {
      this.searchQuerySignal.set(changes['searchQuery'].currentValue);
      this.searchControl.setValue(changes['searchQuery'].currentValue);
    }
  }

  ngOnInit(): void {
    this.searchControl.valueChanges
      .pipe(debounceTime(300), distinctUntilChanged(), takeUntil(this.destroy$))
      .subscribe((value) => {
        const query = value ?? '';
        this.searchQuerySignal.set(query);
        this.onSearch();
        this.searchChange.emit(query);
      });
  }

  ngOnDestroy(): void {
    this.destroy$.next();
    this.destroy$.complete();
  }

  protected rowRecord(item: T): Record<string, unknown> {
    return item as unknown as Record<string, unknown>;
  }

  cellValue(item: T, key: string): unknown {
    return this.rowRecord(item)[key];
  }

  sortData(data: T[], key: string, direction: 'asc' | 'desc'): T[] {
    return data.sort((a, b) => {
      const aVal = this.cellValue(a, key);
      const bVal = this.cellValue(b, key);

      if (aVal === bVal) return 0;
      if (aVal === null || aVal === undefined) return 1;
      if (bVal === null || bVal === undefined) return -1;

      let comparison: number;

      if (typeof aVal === 'number' && typeof bVal === 'number') {
        comparison = aVal - bVal;
      } else if (typeof aVal === 'boolean' && typeof bVal === 'boolean') {
        comparison = aVal === bVal ? 0 : aVal ? -1 : 1;
      } else {
        comparison = String(aVal).localeCompare(String(bVal));
      }

      return direction === 'asc' ? comparison : -comparison;
    });
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

  onSearch(): void {
    const query = this.searchQuerySignal().toLowerCase().trim();

    if (!query) {
      this._filteredData = [...this._originalData];
    } else {
      const filtered = this._originalData.filter((item) => {
        const rec = this.rowRecord(item);
        return Object.values(rec).some((value) => {
          if (typeof value === 'object' && value !== null) return false;
          return String(value).toLowerCase().includes(query);
        });
      });
      this._filteredData = filtered;
    }
  }

  toggleSelectAll(checked: boolean): void {
    if (checked) {
      this.data.forEach((item) => {
        this._selectedKeys.add(String(this.rowRecord(item)[this.checkboxKey]));
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

  isSelected(item: T): boolean {
    const key = String(this.rowRecord(item)[this.checkboxKey]);
    return this._selectedKeys.has(key);
  }

  clearSearch(): void {
    this.searchQuerySignal.set('');
    this.searchControl.setValue('');
    this.onSearch();
    this.searchChange.emit('');
  }

  onToolbarHoverEnter(): void {
    if (!this.toolbarLocked()) {
      this.toolbarExpanded.set(true);
    }
  }

  onToolbarHoverLeave(): void {
    if (!this.toolbarLocked()) {
      this.toolbarExpanded.set(false);
    }
  }

  toggleToolbar(): void {
    this.toolbarLocked.set(!this.toolbarLocked());
  }

  formatCell(columnKey: string, item: T): string {
    const v = this.cellValue(item, columnKey);
    if (v === null || v === undefined) return '';
    return String(v);
  }

  abstract get checkboxKey(): string;

  trackByFn(index: number, item: T): number {
    return index;
  }
}
