import { Component, input, output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';

export interface QuickAction {
  id: string;
  label: string;
  icon: string;
  action: string;
}

@Component({
  selector: 'app-widget-quick-actions',
  standalone: true,
  imports: [CommonModule, MatIconModule],
  template: `
    <div class="quick-actions-widget">
      @for (action of actions(); track action.id) {
        <button class="action-btn" (click)="onAction(action.action)">
          <mat-icon class="action-icon">{{ action.icon }}</mat-icon>
          <span class="action-label">{{ action.label }}</span>
        </button>
      }
    </div>
  `,
  styles: [
    `
      .quick-actions-widget {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 0.75rem;
      }
      .action-btn {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.5rem;
        padding: 1rem;
        background: var(--btn-bg, #f3f4f6);
        border: none;
        border-radius: 8px;
        cursor: pointer;
        transition: all 0.2s ease;
      }
      .action-btn:hover {
        background: var(--btn-hover, #e5e7eb);
        transform: translateY(-1px);
      }
      .action-icon {
        font-size: 1.5rem;
        width: 1.5rem;
        height: 1.5rem;
        color: var(--primary-color, #6366f1);
      }
      .action-label {
        font-size: 0.75rem;
        font-weight: 500;
        color: var(--text-primary, #374151);
      }
    `,
  ],
})
export class WidgetQuickActionsComponent {
  actions = input<QuickAction[]>([]);
  actionTriggered = output<string>();

  onAction(action: string) {
    this.actionTriggered.emit(action);
  }
}
