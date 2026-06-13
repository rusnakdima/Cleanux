import { Injectable, inject } from '@angular/core';
import { getErrorMessage } from '@shared/utils/error.util';
import { LoggerService } from '@services/logger.service';

@Injectable({
  providedIn: 'root',
})
export class NotificationService {
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo(
      'service',
      'NotificationService',
      'init',
      'NotificationService initialized'
    );
  }

  alert(message: string): void {
    this.logger.logDebug('service', 'NotificationService', 'alert', 'Showing alert', { message });
    window.alert(message);
  }

  confirm(message: string): boolean {
    this.logger.logDebug('service', 'NotificationService', 'confirm', 'Showing confirm dialog', {
      message,
    });
    return window.confirm(message);
  }

  error(title: string, error: unknown): void {
    const message = getErrorMessage(error);
    this.logger.logWarn('service', 'NotificationService', 'error', `${title}: ${message}`);
    this.alert(`${title}: ${message}`);
  }

  success(message: string): void {
    this.logger.logDebug('service', 'NotificationService', 'success', 'Showing success message', {
      message,
    });
    this.alert(message);
  }

  /**
   * @deprecated Use error() instead. This method is a simple wrapper that adds "Failed to" prefix.
   */
  cleanError(operation: string, error: unknown): void {
    this.logger.logError(
      'service',
      'NotificationService',
      'cleanError',
      'Operation failed',
      error as Error,
      { operation }
    );
    this.error(`Failed to ${operation}`, error);
  }
}
