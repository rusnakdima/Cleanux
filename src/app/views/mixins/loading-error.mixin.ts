import { signal, inject } from '@angular/core';
import { NotificationService } from '@services/notification.service';
import { ConfirmDialogService } from '@shared/confirm-dialog';

export interface RunWithLoadingOptions {
  errorMessage?: string;
  notificationKey?: string;
  notificationMessage?: string;
}

export abstract class LoadingErrorMixin {
  protected notification = inject(NotificationService);
  protected confirmDialogService = inject(ConfirmDialogService);

  loading = signal(false);

  protected async runWithLoading<T>(fn: () => Promise<T>, options?: RunWithLoadingOptions): Promise<T | undefined> {
    this.loading.set(true);
    try {
      return await fn();
    } catch (error) {
      console.error(options?.errorMessage ?? 'Operation failed:', error);
      if (options?.notificationKey) {
        this.notification.cleanError(options.notificationKey, error);
      } else if (options?.notificationMessage) {
        this.notification.error(options.notificationMessage, error);
      }
      return undefined;
    } finally {
      this.loading.set(false);
    }
  }
}