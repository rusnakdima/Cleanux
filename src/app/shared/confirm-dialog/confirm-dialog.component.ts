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
import { CommonModule, NgClass } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ConfirmDialogConfig, DEFAULT_DIALOG_CONFIG } from './confirm-dialog.config';

@Component({
  selector: 'app-confirm-dialog',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, NgClass, FormsModule],
  templateUrl: './confirm-dialog.component.html',
  styleUrl: './confirm-dialog.component.css',
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

  getCancelBtnClass(): string {
    return 'btn-cancel bg-[var(--bg-surface)] text-[var(--text-secondary)] border-[var(--border-color)] hover:bg-[var(--bg-elevated)] hover:text-[var(--text-primary)]';
  }

  getConfirmBtnClass(): string {
    const base = 'text-white border-transparent';
    if (this.config.confirmColor === 'accent') {
      return `btn-accent bg-[var(--accent-secondary)] ${base}`;
    }
    if (this.config.confirmColor === 'warn') {
      return `btn-warn bg-[var(--color-error)] ${base}`;
    }
    return `btn-primary bg-[var(--accent-primary)] ${base}`;
  }
}
