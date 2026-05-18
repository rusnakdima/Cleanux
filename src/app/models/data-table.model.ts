export interface TableColumn {
  key: string;
  label: string;
  width?: string;
  align?: 'left' | 'center' | 'right';
  sortable?: boolean;
  actions?: TableAction[];
}

export interface TableAction {
  label: string;
  icon?: string;
  class?: string;
  callback: string;
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
}
