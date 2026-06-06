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

  resolve(result: boolean): void {
    if (this.resolveCallback) {
      this.resolveCallback(result);
      this.resolveCallback = null;
    }
    this.isOpen.set(false);
    this.config.set({} as ConfirmDialogConfig);
  }
}
