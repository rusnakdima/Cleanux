export type ListCellFormat = 'size' | 'date' | 'number' | 'percentage';

export interface ListAction {
  id: string;
  icon: string;
  label?: string;
  class?: string;
  tooltip?: string;
  confirmMessage?: string;
  toggle?: boolean;
  toggleState?: (item: unknown) => boolean;
  disabled?: (item: unknown) => boolean;
}

export interface ListColumn {
  key: string;
  label?: string;
  width?: string;
  primary?: boolean;
  secondary?: string;
  secondaryKey?: string;
  icon?: string;
  iconKey?: string;
  badge?: string;
  badgeClass?: string | ((item: unknown) => string);
  timestamp?: string;
  format?: ListCellFormat;
  actions?: ListAction[];
  sortable?: boolean;
  align?: 'left' | 'center' | 'right';
  cellClass?: string | ((item: unknown) => string);
  truncate?: boolean;
}

export interface ListOptions {
  showHeader?: boolean;
  showCheckbox?: boolean;
  checkboxKey?: string;
  hoverable?: boolean;
  showReloadButton?: boolean;
  showSearch?: boolean;
  searchPlaceholder?: string;
  showPagination?: boolean;
  emptyMessage?: string;
  showActions?: boolean;
  actionsPosition?: 'right' | 'bottom';
  rowClass?: string | ((item: unknown) => string);
  showSelectAll?: boolean;
  searchTogglable?: boolean;
}

export interface ListActionEvent<T = unknown> {
  action: string;
  item: T;
}

export interface ListSelectionEvent {
  keys: Set<string>;
  items: unknown[];
}

export interface ListSortEvent {
  key: string;
  direction: 'asc' | 'desc';
}
