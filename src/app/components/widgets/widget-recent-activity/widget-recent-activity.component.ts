import { Component, input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';

export interface ActivityItem {
  id: string;
  type: 'scan' | 'clean' | 'cache' | 'trash' | 'logs' | 'large_files';
  description: string;
  timestamp: string;
  size?: number;
}

@Component({
  selector: 'app-widget-recent-activity',
  standalone: true,
  imports: [CommonModule, MatIconModule],
  template: `
    <div class="activity-widget">
      @for (item of activities(); track item.id) {
        <div class="activity-item">
          <div class="activity-icon" [class]="item.type">
            <mat-icon>{{ getIcon(item.type) }}</mat-icon>
          </div>
          <div class="activity-details">
            <span class="activity-desc">{{ item.description }}</span>
            <span class="activity-time">{{ item.timestamp }}</span>
          </div>
          @if (item.size) {
            <span class="activity-size">{{ formatSize(item.size) }}</span>
          }
        </div>
      }
      @if (activities().length === 0) {
        <div class="empty-state">
          <mat-icon>history</mat-icon>
          <span>No recent activity</span>
        </div>
      }
    </div>
  `,
  styles: [
    `
      .activity-widget {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        max-height: 200px;
        overflow-y: auto;
      }
      .activity-item {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.5rem;
        border-radius: 6px;
        background: var(--bg-secondary);
      }
      .activity-icon {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 32px;
        height: 32px;
        border-radius: 6px;
      }
      .activity-icon.scan {
        background: var(--info-bg);
        color: var(--info);
      }
      .activity-icon.clean {
        background: var(--success-bg);
        color: var(--success);
      }
      .activity-icon.cache {
        background: var(--accent-light);
        color: var(--accent-color);
      }
      .activity-icon.trash {
        background: var(--warning-bg);
        color: var(--warning);
      }
      .activity-icon.logs {
        background: rgba(236, 72, 153, 0.15);
        color: var(--cyber-rose);
      }
      .activity-icon.large_files {
        background: var(--error-bg);
        color: var(--error);
      }
      .activity-icon mat-icon {
        font-size: 1rem;
        width: 1rem;
        height: 1rem;
      }
      .activity-details {
        flex: 1;
        display: flex;
        flex-direction: column;
        min-width: 0;
      }
      .activity-desc {
        font-size: 0.75rem;
        font-weight: 500;
        color: var(--text-primary);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
      }
      .activity-time {
        font-size: 0.625rem;
        color: var(--text-muted);
      }
      .activity-size {
        font-size: 0.75rem;
        font-weight: 600;
        color: var(--text-muted);
      }
      .empty-state {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 2rem;
        color: var(--text-muted);
        gap: 0.5rem;
      }
      .empty-state mat-icon {
        font-size: 2rem;
        width: 2rem;
        height: 2rem;
      }
    `,
  ],
})
export class WidgetRecentActivityComponent {
  activities = input<ActivityItem[]>([]);

  getIcon(type: string): string {
    const icons: Record<string, string> = {
      scan: 'search',
      clean: 'cleaning_services',
      cache: 'auto_delete',
      trash: 'delete_outline',
      logs: 'description',
      large_files: 'file_copy',
    };
    return icons[type] || 'circle';
  }

  formatSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }
}