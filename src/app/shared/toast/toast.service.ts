import { Injectable, signal } from '@angular/core';

export interface ToastMessage {
  id: string;
  message: string;
  type: 'success' | 'error' | 'warning' | 'info';
}

@Injectable({
  providedIn: 'root',
})
export class ToastService {
  toasts = signal<ToastMessage[]>([]);

  private idCounter = 0;

  show(message: string, type: 'success' | 'error' | 'warning' | 'info' = 'info'): string {
    const id = `toast-${++this.idCounter}`;
    const toast: ToastMessage = { id, message, type };

    this.toasts.update((current) => [...current, toast]);

    setTimeout(() => {
      this.dismiss(id);
    }, 5000);

    return id;
  }

  dismiss(id: string): void {
    this.toasts.update((current) => current.filter((t) => t.id !== id));
  }

  success(message: string): string {
    return this.show(message, 'success');
  }

  error(message: string): string {
    return this.show(message, 'error');
  }

  warning(message: string): string {
    return this.show(message, 'warning');
  }

  info(message: string): string {
    return this.show(message, 'info');
  }
}
