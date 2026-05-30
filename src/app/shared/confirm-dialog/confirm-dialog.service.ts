import { Injectable, signal, computed } from '@angular/core';
import { ConfirmDialogConfig, DEFAULT_DIALOG_CONFIG } from './confirm-dialog.config';

@Injectable({
  providedIn: 'root',
})
export class ConfirmDialogService {
  private isOpen = signal(false);
  private config = signal<ConfirmDialogConfig>({} as ConfirmDialogConfig);
  private resolveCallback: ((result: boolean) => void) | null = null;

  readonly dialogOpen = this.isOpen.asReadonly();
  readonly dialogConfig = this.config.asReadonly();

  confirm(config: ConfirmDialogConfig): Promise<boolean> {
    return new Promise((resolve) => {
      this.config.set({ ...DEFAULT_DIALOG_CONFIG, ...config });
      this.isOpen.set(true);
      this.resolveCallback = resolve;
    });
  }

  confirmDelete(itemName: string): Promise<boolean> {
    return this.confirm({
      title: 'Confirm Delete',
      message: `Are you sure you want to delete "${itemName}"? This action cannot be undone.`,
      confirmText: 'Delete',
      cancelText: 'Cancel',
      confirmColor: 'warn',
      dangerous: true,
    });
  }

  confirmDangerous(action: string, requireYes: boolean = false): Promise<boolean> {
    return this.confirm({
      title: 'Dangerous Action',
      message: `${action}\n\nThis action cannot be undone.`,
      confirmText: 'Confirm',
      cancelText: 'Cancel',
      confirmColor: 'warn',
      dangerous: true,
      requireYesToConfirm: requireYes,
      closeOnClickOutside: !requireYes,
      closeOnEscape: !requireYes,
    });
  }

  confirmCheckbox(
    message: string,
    checkboxLabel: string = 'I understand this action cannot be undone'
  ): Promise<boolean> {
    return this.confirm({
      title: 'Confirm Action',
      message,
      confirmText: 'Confirm',
      cancelText: 'Cancel',
      confirmColor: 'primary',
      showCheckbox: true,
      checkboxLabel,
    });
  }

  resolve(result: boolean): void {
    if (this.resolveCallback) {
      this.resolveCallback(result);
      this.resolveCallback = null;
    }
    this.isOpen.set(false);
    this.config.set({} as ConfirmDialogConfig);
  }
}
