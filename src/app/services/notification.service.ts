import { Injectable, inject } from '@angular/core';
import { getErrorMessage } from '@shared/utils/error.util';
import { LoggingService, getLoggingService } from '@tauri-apps/logger';

@Injectable({
  providedIn: 'root',
})
export class NotificationService {
  private loggingService = getLoggingService();

  constructor() {
    this.loggingService.info('NotificationService initialized');
  }

  alert(message: string): void {
    this.loggingService.debug('Showing alert', { message });
    window.alert(message);
  }

  confirm(message: string): boolean {
    this.loggingService.debug('Showing confirm dialog', { message });
    return window.confirm(message);
  }

  error(title: string, error: unknown): void {
    const message = getErrorMessage(error);
    this.loggingService.warn(`${title}: ${message}`);
    this.alert(`${title}: ${message}`);
  }

  success(message: string): void {
    this.loggingService.debug('Showing success message', { message });
    this.alert(message);
  }

  /**
   * @deprecated Use error() instead. This method is a simple wrapper that adds "Failed to" prefix.
   */
  cleanError(operation: string, error: unknown): void {
    this.loggingService.error('Operation failed', error as Error, { operation });
    this.error(`Failed to ${operation}`, error);
  }
}
