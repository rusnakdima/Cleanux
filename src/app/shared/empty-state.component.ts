import { Component, ChangeDetectionStrategy, input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';

@Component({
  selector: 'app-empty-state',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, MatButtonModule],
  templateUrl: './empty-state.component.html',
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
  templateUrl: './empty-table.component.html',
})
export class EmptyTableComponent {
  icon = input<string>('search_off');
  message = input<string>('No matching records found');
}
