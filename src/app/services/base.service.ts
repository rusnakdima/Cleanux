import { inject } from '@angular/core';
import { Signal, signal } from '@angular/core';
import { ApiService } from '@services/api.service';
import { NotificationService } from '@services/notification.service';

export interface LoadingState {
  loading: Signal<boolean>;
  error: Signal<string | null>;
}

export abstract class BaseService {
  protected api = inject(ApiService);
  protected notification = inject(NotificationService);

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
      return null;
    } finally {
      this.loading.set(false);
    }
  }
}
