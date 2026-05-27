/* sys lib */
import { ChangeDetectionStrategy, Component, input, output } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';
import { MatTooltipModule, TooltipPosition } from '@angular/material/tooltip';

@Component({
  selector: 'app-top-header',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, MatTooltipModule],
  template: `
    <header class="top-header">
      <div class="header-left">
        <button class="mobile-menu-btn" (click)="mobileMenuToggle.emit()">
          <mat-icon>menu</mat-icon>
        </button>
        
        <div class="breadcrumb">
          <span class="breadcrumb-item">{{ breadcrumb() }}</span>
          <mat-icon class="breadcrumb-separator">chevron_right</mat-icon>
          <span class="breadcrumb-current">{{ title() }}</span>
        </div>
      </div>
      
      <div class="header-right">
        <button 
          class="collapse-btn hidden lg:flex" 
          (click)="sidebarToggle.emit()"
          [matTooltip]="isCollapsed() ? 'Expand sidebar' : 'Collapse sidebar'"
          [matTooltipPosition]="tooltipPosition">
          <mat-icon>{{ isCollapsed() ? 'chevron_right' : 'chevron_left' }}</mat-icon>
        </button>
        
        <div class="header-actions">
          <button class="action-btn" matTooltip="Notifications" [matTooltipPosition]="tooltipPosition">
            <mat-icon>notifications</mat-icon>
          </button>
        </div>
      </div>
    </header>
  `,
  styles: [`
    :host {
      display: block;
    }

    .top-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      height: 4rem;
      padding: 0 1rem;
      background: var(--bg-elevated);
      backdrop-filter: blur(12px);
      border-bottom: 1px solid var(--border-color);
      flex-shrink: 0;
    }

    @media (min-width: 1024px) {
      .top-header {
        padding: 0 1.5rem;
      }
    }

    .header-left {
      display: flex;
      align-items: center;
      gap: 1rem;
    }

    .mobile-menu-btn {
      padding: 0.5rem;
      border-radius: 0.5rem;
      color: var(--text-secondary);
      transition: colors;
      cursor: pointer;
      background: transparent;
      border: none;
    }

    .mobile-menu-btn:hover {
      background: var(--bg-tertiary);
      color: var(--text-primary);
    }

    .breadcrumb {
      display: flex;
      align-items: center;
      font-size: 0.875rem;
      gap: 0.25rem;
    }

    .breadcrumb-item {
      color: var(--text-muted);
    }

    .breadcrumb-current {
      font-weight: 500;
      color: var(--text-primary);
    }

    .breadcrumb-separator {
      font-size: 16px;
      color: var(--text-muted);
    }

    .header-right {
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }

    .collapse-btn {
      display: none;
      padding: 0.5rem;
      border-radius: 0.5rem;
      color: var(--text-secondary);
      transition: colors;
      cursor: pointer;
      background: transparent;
      border: none;
    }

    @media (min-width: 1024px) {
      .collapse-btn {
        display: flex;
      }
    }

    .collapse-btn:hover {
      background: var(--bg-tertiary);
      color: var(--text-primary);
    }

    .header-actions {
      display: flex;
      align-items: center;
      gap: 0.25rem;
    }

    .action-btn {
      padding: 0.5rem;
      border-radius: 0.5rem;
      color: var(--text-secondary);
      transition: colors;
      cursor: pointer;
      background: transparent;
      border: none;
    }

    .action-btn:hover {
      background: var(--bg-tertiary);
      color: var(--text-primary);
    }
  `],
})
export class TopHeaderComponent {
  title = input('');
  breadcrumb = input('');
  isCollapsed = input(false);
  
  protected readonly tooltipPosition: TooltipPosition = 'below';

  mobileMenuToggle = output<void>();
  sidebarToggle = output<void>();
}