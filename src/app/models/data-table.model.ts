export type CellFormatType = 'size' | 'date' | 'status' | 'percentage' | 'bytes';

export interface TableColumn {
  key: string;
  label: string;
  width?: string;
  minWidth?: string;
  maxWidth?: string;
  align?: 'left' | 'center' | 'right';
  sortable?: boolean;
  resizable?: boolean;
  cellClass?: string | ((item: unknown) => string);
  format?: CellFormatType;
  actions?: TableAction[];
  hidden?: boolean;
}

export interface TableAction {
  id: string;
  label: string;
  icon?: string;
  class?: string;
  tooltip?: string;
  confirmMessage?: string;
}

export interface TableOptions {
  showHeader?: boolean;
  showCheckbox?: boolean;
  checkboxKey?: string;
  hoverable?: boolean;
  showReloadButton?: boolean;
  showSelectedActions?: boolean;
  selectedActionText?: string;
  showPreviewButton?: boolean;
  showRowActions?: boolean;
  showSearch?: boolean;
  searchPlaceholder?: string;
  virtualScroll?: boolean;
  rowHeight?: number;
  emptyMessage?: string;
  persistKey?: string;
  searchTogglable?: boolean;
}

export interface SortEvent {
  key: string;
  direction: 'asc' | 'desc';
}

export interface PageEvent {
  page: number;
  pageSize: number;
}

export interface RowActionEvent<T = unknown> {
  action: string;
  item: T;
}

export interface SelectionEvent {
  keys: Set<string>;
  items: unknown[];
}
