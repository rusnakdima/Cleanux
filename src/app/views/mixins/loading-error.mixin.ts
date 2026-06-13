import { signal, inject } from '@angular/core';
import { NotificationService } from '@services/notification.service';
import { ConfirmDialogService } from '@shared/confirm-dialog';
import { LoggerService } from '@services/logger.service';

export interface RunWithLoadingOptions {
  errorMessage?: string;
  notificationKey?: string;
  notificationMessage?: string;
}

export abstract class LoadingErrorMixin {
  protected notification = inject(NotificationService);
  protected confirmDialogService = inject(ConfirmDialogService);
  protected logger = inject(LoggerService);

  loading = signal(false);

  protected async runWithLoading<T>(
    fn: () => Promise<T>,
    options?: RunWithLoadingOptions
  ): Promise<T | undefined> {
    this.loading.set(true);
    try {
      return await fn();
    } catch (error) {
      this.logger.logError(
        'view',
        'LoadingErrorMixin',
        'runWithLoading',
        options?.errorMessage ?? 'Operation failed',
        error as Error
      );
      if (options?.notificationKey) {
        this.notification.error('Failed to ' + options.notificationKey, error);
      } else if (options?.notificationMessage) {
        this.notification.error(options.notificationMessage, error);
      }
      return undefined;
    } finally {
      this.loading.set(false);
    }
  }
}
