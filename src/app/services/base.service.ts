import { inject } from '@angular/core';
import { Signal, signal } from '@angular/core';
import { ApiService } from '@services/api.service';
import { NotificationService } from '@services/notification.service';
import { LoggerService } from '@services/logger.service';

export interface LoadingState {
  loading: Signal<boolean>;
  error: Signal<string | null>;
}

export abstract class BaseService {
  protected api = inject(ApiService);
  protected notification = inject(NotificationService);
  protected logger = inject(LoggerService);

  protected loading = signal(false);
  protected error = signal<string | null>(null);

  get loadingState(): LoadingState {
    return {
      loading: this.loading,
      error: this.error,
    };
  }

  protected async invoke<T>(command: string, args?: Record<string, unknown>): Promise<T | null> {
    this.loading.set(true);
    this.error.set(null);

    try {
      const result = await this.api.invoke<T>(command, args);
      return result;
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      this.error.set(message);
      this.logger.logError(
        'service',
        this.constructor.name,
        'invoke',
        'Operation failed',
        err as Error,
        { command, args }
      );
      return null;
    } finally {
      this.loading.set(false);
    }
  }

  protected async invokeWithNotification<T>(
    command: string,
    args: Record<string, unknown> | undefined,
    operation: string
  ): Promise<T | null> {
    this.loading.set(true);
    this.error.set(null);

    try {
      const result = await this.api.invoke<T>(command, args);
      return result;
    } catch (err) {
      this.notification.cleanError(operation, err);
      this.logger.logError(
        'service',
        this.constructor.name,
        'invokeWithNotification',
        'Operation failed',
        err as Error,
        { command, operation }
      );
      return null;
    } finally {
      this.loading.set(false);
    }
  }
}
