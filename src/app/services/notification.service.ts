import { Injectable, inject } from '@angular/core';
import { getErrorMessage } from '@shared/utils/error.util';

@Injectable({
  providedIn: 'root',
})
export class NotificationService {
  alert(message: string): void {
    window.alert(message);
  }

  confirm(message: string): boolean {
    return window.confirm(message);
  }

  error(title: string, error: unknown): void {
    const message = getErrorMessage(error);
    this.alert(`${title}: ${message}`);
  }

  success(message: string): void {
    this.alert(message);
  }

  /**
   * @deprecated Use error() instead. This method is a simple wrapper that adds "Failed to" prefix.
   */
  cleanError(operation: string, error: unknown): void {
    this.error(`Failed to ${operation}`, error);
  }
}
