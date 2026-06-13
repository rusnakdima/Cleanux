import { Injectable } from '@angular/core';
import { ToastService } from '../shared/toast/toast.service';
import { ConfirmDialogService } from '../shared/confirm-dialog/confirm-dialog.service';

@Injectable({
  providedIn: 'root',
})
export class NotificationService {
  constructor(
    private toastService: ToastService,
    private confirmDialogService: ConfirmDialogService
  ) {}

  alert(message: string): void {
    this.toastService.show(message, 'info');
  }

  async confirm(message: string): Promise<boolean> {
    return this.confirmDialogService.confirm({ title: 'Confirm', message });
  }

  error(title: string, error: unknown): void {
    const message = error instanceof Error ? error.message : String(error);
    this.alert(`${title}: ${message}`);
  }

  success(message: string): void {
    this.toastService.show(message, 'success');
  }

  /**
   * @deprecated Use error() instead. This method is a simple wrapper that adds "Failed to" prefix.
   */
  cleanError(operation: string, error: unknown): void {
    this.error(`Failed to ${operation}`, error);
  }
}
