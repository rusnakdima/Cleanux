export interface TableColumn {
  key: string;
  label: string;
  width?: string;
  align?: 'left' | 'center' | 'right';
  sortable?: boolean;
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
}
