import { Component, signal, computed, ChangeDetectionStrategy, inject } from '@angular/core';
import { MatIconModule } from '@angular/material/icon';
import { ToastService } from '@tauri-front/shared';

@Component({
  selector: 'app-toast',
  standalone: true,
  imports: [MatIconModule],
  templateUrl: './toast.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ToastComponent {
  toastService = inject(ToastService);

  getIcon(type: string): string {
    switch (type) {
      case 'success':
        return 'check_circle';
      case 'error':
        return 'error';
      case 'warning':
        return 'warning';
      case 'info':
      default:
        return 'info';
    }
  }

  dismiss(id: string): void {
    this.toastService.dismiss(id);
  }
}
