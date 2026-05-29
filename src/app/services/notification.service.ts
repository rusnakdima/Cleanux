import { Injectable } from '@angular/core';

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
    const message = error instanceof Error ? error.message : String(error);
    this.alert(`${title}: ${message}`);
  }

  success(message: string): void {
    this.alert(message);
  }

  cleanError(operation: string, error: unknown): void {
    this.error(`Failed to ${operation}`, error);
  }
}
