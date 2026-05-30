import { describe, it, expect, beforeEach } from 'vitest';
import { DataTableComponent } from './data-table.component';
import { TableColumn } from '@models/data-table.model';

interface TestItem {
  id: string;
  name: string;
  size: number;
  status: string;
}

describe('DataTableComponent', () => {
  let component: DataTableComponent<TestItem>;

  const mockColumns: TableColumn[] = [
    { key: 'id', label: 'ID', sortable: true },
    { key: 'name', label: 'Name', sortable: true },
    { key: 'size', label: 'Size', sortable: true, align: 'right' },
    { key: 'status', label: 'Status', align: 'center' },
  ];

  const mockData: TestItem[] = [
    { id: '1', name: 'File A', size: 1024, status: 'active' },
    { id: '2', name: 'File B', size: 2048, status: 'inactive' },
    { id: '3', name: 'File C', size: 512, status: 'active' },
  ];

  beforeEach(() => {
    component = new DataTableComponent<TestItem>();
    component.columns = mockColumns;
    component.data = mockData;
    component.pageSize = 15;
    component.currentPage = 1;
    component.ngOnChanges({
      data: {
        currentValue: mockData,
        previousValue: [],
        firstChange: false,
      } as any,
    });
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  describe('pagination', () => {
    it('should return all data when less than page size', () => {
      component.pageSize = 10;
      component.currentPage = 1;
      component.ngOnChanges({});

      const paginated = component.paginatedData;
      expect(paginated).toHaveLength(3);
    });

    it('should paginate correctly', () => {
      component.pageSize = 2;
      component.currentPage = 1;
      component.ngOnChanges({});

      const paginated = component.paginatedData;
      expect(paginated).toHaveLength(2);
      expect(paginated[0].name).toBe('File A');
    });

    it('should change page correctly', () => {
      component.pageSize = 2;
      component.currentPage = 2;
      component.ngOnChanges({});

      const paginated = component.paginatedData;
      expect(paginated).toHaveLength(1);
      expect(paginated[0].name).toBe('File C');
    });

    it('should emit pageChange event', () => {
      let emittedPage: number | null = null;
      component.pageChange.subscribe((page) => {
        emittedPage = page;
      });

      component.onPageChange(2);
      expect(emittedPage).toBe(2);
    });

    it('should emit newPageSize event', () => {
      let emittedSize: number | null = null;
      component.newPageSize.subscribe((size) => {
        emittedSize = size;
      });

      component.onPageSizeChange(50);
      expect(emittedSize).toBe(50);
    });
  });

  describe('selection', () => {
    it('should toggle select all', () => {
      let emittedKeys: Set<string> | null = null;
      component.selectionChange.subscribe((keys) => {
        emittedKeys = keys;
      });

      component.toggleSelectAll(true);

      expect(component.allSelected()).toBe(true);
      expect(component.indeterminate()).toBe(false);
      expect(emittedKeys).toHaveLength(3);
    });

    it('should deselect all', () => {
      component.toggleSelectAll(true);

      component.toggleSelectAll(false);

      expect(component.allSelected()).toBe(false);
      expect(component.indeterminate()).toBe(false);
    });

    it('should toggle individual item', () => {
      let emittedKeys: Set<string> | null = null;
      component.selectionChange.subscribe((keys) => {
        emittedKeys = keys;
      });

      component.toggleSelectItem(mockData[0], true);

      expect(emittedKeys?.has('1')).toBe(true);
    });

    it('should update indeterminate state correctly', () => {
      component.toggleSelectItem(mockData[0], true);

      expect(component.indeterminate()).toBe(true);
      expect(component.allSelected()).toBe(false);
    });

    it('should select all when all items selected', () => {
      component.toggleSelectAll(true);

      expect(component.allSelected()).toBe(true);
      expect(component.indeterminate()).toBe(false);
    });
  });

  describe('sorting', () => {
    it('should sort ascending by name', () => {
      component.sortKey.set('name');
      component.sortDirection.set('asc');

      const sorted = component.sortedFilteredData;
      expect(sorted[0].name).toBe('File A');
      expect(sorted[2].name).toBe('File C');
    });

    it('should sort descending by size', () => {
      component.sortKey.set('size');
      component.sortDirection.set('desc');

      const sorted = component.sortedFilteredData;
      expect(sorted[0].size).toBe(2048);
      expect(sorted[2].size).toBe(512);
    });

    it('should emit sortChange event', () => {
      let emittedSort: { key: string; direction: 'asc' | 'desc' } | null = null;
      component.sortChange.subscribe((sort) => {
        emittedSort = sort;
      });

      component.onSort(mockColumns[1]);

      expect(emittedSort?.key).toBe('name');
      expect(emittedSort?.direction).toBe('asc');
    });

    it('should toggle sort direction when same column', () => {
      component.sortKey.set('name');
      component.sortDirection.set('asc');

      component.onSort(mockColumns[1]);

      expect(component.sortDirection()).toBe('desc');
    });

    it('should get correct sort icon', () => {
      component.sortKey.set('name');
      component.sortDirection.set('asc');

      expect(component.getSortIcon(mockColumns[1])).toBe('arrow_upward');
      expect(component.getSortIcon(mockColumns[0])).toBe('unfold_more');
    });
  });

  describe('search', () => {
    it('should filter data by search query', () => {
      component.searchQuery.set('file a');
      component.ngOnChanges({
        data: { currentValue: mockData } as any,
        firstChange: false,
      } as any);

      expect(component.totalItems).toBe(1);
    });

    it('should clear search', () => {
      component.searchQuery.set('test');

      component.clearSearch();

      expect(component.searchQuery()).toBe('');
    });

    it('should emit searchChange event', () => {
      let emittedQuery: string | null = null;
      component.searchChange.subscribe((query) => {
        emittedQuery = query;
      });

      component.searchControl.setValue('test');
      component.ngOnInit();

      expect(emittedQuery).toBe('test');
    });
  });

  describe('cell formatting', () => {
    it('should get cell value', () => {
      const value = component.cellValue(mockData[0], 'name');
      expect(value).toBe('File A');
    });

    it('should format cell value as string', () => {
      const formatted = component.formatCell('size', mockData[0]);
      expect(formatted).toBe('1024');
    });

    it('should return empty string for null values', () => {
      const formatted = component.formatCell('nonexistent', mockData[0]);
      expect(formatted).toBe('');
    });

    it('should get size class correctly', () => {
      expect(component.getSizeClass({ id: '1', name: 'small', size: 100, status: '' })).toBe('');
      expect(
        component.getSizeClass({ id: '1', name: 'large', size: 600 * 1024 * 1024, status: '' })
      ).toBe('size-warning');
      expect(
        component.getSizeClass({ id: '1', name: 'huge', size: 2 * 1024 * 1024 * 1024, status: '' })
      ).toBe('size-danger');
    });

    it('should get status class correctly', () => {
      expect(component.getStatusClass({ id: '1', name: 'a', size: 0, status: 'running' })).toBe(
        'status-running'
      );
      expect(component.getStatusClass({ id: '1', name: 'a', size: 0, status: 'active' })).toBe(
        'status-running'
      );
      expect(component.getStatusClass({ id: '1', name: 'a', size: 0, status: 'stopped' })).toBe(
        'status-stopped'
      );
      expect(component.getStatusClass({ id: '1', name: 'a', size: 0, status: 'inactive' })).toBe(
        'status-stopped'
      );
    });
  });

  describe('row actions', () => {
    it('should emit rowClick event', () => {
      let emittedItem: TestItem | null = null;
      component.rowClick.subscribe((item) => {
        emittedItem = item;
      });

      component.onRowClick(mockData[0]);

      expect(emittedItem).toEqual(mockData[0]);
    });

    it('should emit rowDoubleClick event', () => {
      let emittedItem: TestItem | null = null;
      component.rowDoubleClick.subscribe((item) => {
        emittedItem = item;
      });

      component.onRowDoubleClick(mockData[0]);

      expect(emittedItem).toEqual(mockData[0]);
    });

    it('should emit preview event', () => {
      let emittedItem: TestItem | null = null;
      component.preview.subscribe((item) => {
        emittedItem = item;
      });

      component.onPreview(mockData[0]);

      expect(emittedItem).toEqual(mockData[0]);
    });

    it('should emit reload event', () => {
      let called = false;
      component.reload.subscribe(() => {
        called = true;
      });

      component.onReload();

      expect(called).toBe(true);
    });

    it('should emit selectedAction event', () => {
      let emittedKeys: Set<string> | null = null;
      component.selectedAction.subscribe((keys) => {
        emittedKeys = keys;
      });

      component.toggleSelectAll(true);
      component.onSelectedAction();

      expect(emittedKeys).toHaveLength(3);
    });
  });

  describe('alignment', () => {
    it('should return correct alignment class', () => {
      expect(component.getAlignClass({ key: 'test', label: 'Test' })).toBe('text-left');
      expect(component.getAlignClass({ key: 'test', label: 'Test', align: 'center' })).toBe(
        'text-center'
      );
      expect(component.getAlignClass({ key: 'test', label: 'Test', align: 'right' })).toBe(
        'text-right'
      );
    });
  });

  describe('utility methods', () => {
    it('should track by index', () => {
      expect(component.trackByFn(0, mockData[0])).toBe(0);
      expect(component.trackByFn(5, mockData[1])).toBe(5);
    });

    it('should get column width', () => {
      component.columnWidths.set({ name: '200px' });
      expect(component.getColumnWidth('name')).toBe('200px');
      expect(component.getColumnWidth('nonexistent')).toBe('flex-1');
    });

    it('should check if item is selected', () => {
      component._selectedKeys = new Set(['1', '2']);
      expect(component.isSelected(mockData[0])).toBe(true);
      expect(component.isSelected(mockData[2])).toBe(false);
    });
  });
});
