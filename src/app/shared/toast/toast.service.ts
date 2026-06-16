import { Injectable, signal } from '@angular/core';
import { TOAST_DURATION_MS } from '@shared/utils/constants';

export interface ToastMessage {
  id: string;
  message: string;
  type: 'success' | 'error' | 'warning' | 'info';
}

export interface ToastOptions {
  persistent?: boolean;
}

@Injectable({
  providedIn: 'root',
})
export class ToastService {
  toasts = signal<ToastMessage[]>([]);

  private idCounter = 0;

  show(
    message: string,
    type: 'success' | 'error' | 'warning' | 'info' = 'info',
    options?: ToastOptions
  ): string {
    const id = `toast-${++this.idCounter}`;
    const toast: ToastMessage = { id, message, type };

    this.toasts.update((current) => [...current, toast]);

    if (!options?.persistent) {
      setTimeout(() => {
        this.dismiss(id);
      }, TOAST_DURATION_MS);
    }

    return id;
  }

  dismiss(id: string): void {
    this.toasts.update((current) => current.filter((t) => t.id !== id));
  }

  success(message: string, options?: ToastOptions): string {
    return this.show(message, 'success', options);
  }

  error(message: string, options?: ToastOptions): string {
    return this.show(message, 'error', options);
  }

  warning(message: string, options?: ToastOptions): string {
    return this.show(message, 'warning', options);
  }

  info(message: string, options?: ToastOptions): string {
    return this.show(message, 'info', options);
  }
}
