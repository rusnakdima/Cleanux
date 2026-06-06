export interface RepairItem {
  path: string;
  issue_type: string;
  severity: 'info' | 'warning' | 'error';
  description?: string;
}
