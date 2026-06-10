import {
  Component,
  ChangeDetectionStrategy,
  signal,
  computed,
  HostListener,
  Input,
  Output,
  EventEmitter,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ConfirmDialogConfig, DEFAULT_DIALOG_CONFIG } from './confirm-dialog.config';

@Component({
  selector: 'app-confirm-dialog',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, FormsModule],
  templateUrl: './confirm-dialog.component.html',
})
export class ConfirmDialogComponent {
  @Input() config: ConfirmDialogConfig = {
    ...DEFAULT_DIALOG_CONFIG,
    title: 'Confirm',
    message: 'Are you sure?',
  } as ConfirmDialogConfig;
  @Output() confirm = new EventEmitter<void>();
  @Output() cancel = new EventEmitter<void>();

  yesInput = signal('');
  checkboxChecked = signal(false);

  canConfirm = computed(() => {
    if (this.config.requireYesToConfirm) {
      return this.yesInput().trim().toUpperCase() === 'YES';
    }
    if (this.config.showCheckbox) {
      return this.checkboxChecked();
    }
    return true;
  });

  shouldCloseOnEscape = computed(() => {
    if (this.config.dangerous && this.config.requireYesToConfirm) {
      return false;
    }
    return this.config.closeOnEscape !== false;
  });

  @HostListener('document:keydown.escape', ['$event'])
  onEscapeKey(event: Event): void {
    if (this.shouldCloseOnEscape()) {
      event.preventDefault();
      this.onCancel();
    }
  }

  onConfirm(): void {
    if (this.canConfirm()) {
      this.confirm.emit();
    }
  }

  onCancel(): void {
    this.cancel.emit();
  }

  onBackdropClick(event: MouseEvent): void {
    if (this.config.closeOnClickOutside !== false && !this.config.dangerous) {
      this.onCancel();
    }
  }
}
