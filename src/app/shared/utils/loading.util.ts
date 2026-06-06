import { WritableSignal, inject } from '@angular/core';
import { NotificationService } from '@services/notification.service';

export interface LoadingOptions {
  errorMessage?: string;
  notificationKey?: string;
  notificationMessage?: string;
}

export async function withLoading<T>(
  loading: WritableSignal<boolean>,
  fn: () => Promise<T>,
  options?: LoadingOptions
): Promise<T | undefined> {
  const notification = inject(NotificationService);
  loading.set(true);
  try {
    return await fn();
  } catch (error) {
    console.error(options?.errorMessage ?? 'Operation failed:', error);
    if (options?.notificationKey) {
      notification.cleanError(options.notificationKey, error);
    } else if (options?.notificationMessage) {
      notification.error(options.notificationMessage, error);
    }
    return undefined;
  } finally {
    loading.set(false);
  }
}

export async function withNotification<T>(
  fn: (notification: NotificationService) => Promise<T>,
  options?: { notificationKey?: string; notificationMessage?: string }
): Promise<T | undefined> {
  const notification = inject(NotificationService);
  try {
    return await fn(notification);
  } catch (error) {
    console.error('Operation failed:', error);
    if (options?.notificationKey) {
      notification.cleanError(options.notificationKey, error);
    } else if (options?.notificationMessage) {
      notification.error(options.notificationMessage, error);
    }
    return undefined;
  }
}