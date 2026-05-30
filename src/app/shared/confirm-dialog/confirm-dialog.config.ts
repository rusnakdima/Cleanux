export interface ConfirmDialogConfig {
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  confirmColor?: 'primary' | 'accent' | 'warn';
  dangerous?: boolean;
  requireYesToConfirm?: boolean;
  showCheckbox?: boolean;
  checkboxLabel?: string;
  closeOnClickOutside?: boolean;
  closeOnEscape?: boolean;
}

export const DEFAULT_DIALOG_CONFIG: Partial<ConfirmDialogConfig> = {
  confirmText: 'Confirm',
  cancelText: 'Cancel',
  confirmColor: 'primary',
  dangerous: false,
  requireYesToConfirm: false,
  showCheckbox: false,
  checkboxLabel: 'I understand this action cannot be undone',
  closeOnClickOutside: true,
  closeOnEscape: true,
};
