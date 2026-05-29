import { Component, ChangeDetectionStrategy, input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';

@Component({
  selector: 'app-empty-state',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, MatButtonModule],
  template: `
    <div
      class="flex flex-col items-center justify-center p-12 text-center"
      role="status"
      aria-live="polite"
    >
      <mat-icon class="text-[64px] w-16 h-16 text-[var(--text-muted)] mb-4" aria-hidden="true">{{
        icon()
      }}</mat-icon>
      <h3 class="text-xl font-medium m-0 mb-2 text-[var(--text-primary)]">{{ title() }}</h3>
      @if (description()) {
        <p class="text-[var(--text-secondary)] m-0 mb-6 max-w-[300px]">{{ description() }}</p>
      }
      @if (actionLabel()) {
        <button mat-raised-button color="primary" (click)="onAction()">
          {{ actionLabel() }}
        </button>
      }
    </div>
  `,
})
export class EmptyStateComponent {
  icon = input<string>('inbox');
  title = input<string>('No data');
  description = input<string | null>(null);
  actionLabel = input<string | null>(null);

  actionHandler: (() => void) | null = null;

  registerAction(handler: () => void): void {
    this.actionHandler = handler;
  }

  onAction(): void {
    this.actionHandler?.();
  }
}

@Component({
  selector: 'app-empty-table',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule],
  template: `
    <div class="flex flex-col items-center justify-center p-8 text-[var(--text-secondary)]">
      <mat-icon class="text-[48px] w-12 h-12 mb-2">{{ icon() }}</mat-icon>
      <span>{{ message() }}</span>
    </div>
  `,
})
export class EmptyTableComponent {
  icon = input<string>('search_off');
  message = input<string>('No matching records found');
}
