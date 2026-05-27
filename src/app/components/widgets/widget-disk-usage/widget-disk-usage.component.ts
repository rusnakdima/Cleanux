import { Component, input, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';

@Component({
  selector: 'app-widget-disk-usage',
  standalone: true,
  imports: [CommonModule, MatIconModule],
  template: `
    <div class="disk-usage-widget">
      <div class="disk-info">
        <div class="disk-icon">
          <mat-icon>storage</mat-icon>
        </div>
        <div class="disk-details">
          <span class="disk-used">{{ formatSize(used()) }}</span>
          <span class="disk-separator">/</span>
          <span class="disk-total">{{ formatSize(total()) }}</span>
        </div>
      </div>
      <div class="usage-bar">
        <div
          class="usage-fill"
          [style.width.%]="usagePercent()"
          [class.warning]="usagePercent() >= 70"
          [class.critical]="usagePercent() >= 90"
        ></div>
      </div>
      <div class="usage-label">
        <span>{{ usagePercent() | number: '1.0-0' }}% used</span>
      </div>
    </div>
  `,
  styles: [
    `
      .disk-usage-widget {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
      }
      .disk-info {
        display: flex;
        align-items: center;
        gap: 0.75rem;
      }
      .disk-icon {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 40px;
        height: 40px;
        background: var(--accent-light);
        border-radius: 8px;
      }
      .disk-icon mat-icon {
        color: var(--accent-color);
      }
      .disk-details {
        font-size: 0.875rem;
      }
      .disk-used {
        font-weight: 700;
        color: var(--text-primary);
      }
      .disk-separator {
        color: var(--text-muted);
        margin: 0 0.25rem;
      }
      .disk-total {
        color: var(--text-muted);
      }
      .usage-bar {
        height: 8px;
        background: var(--progress-track);
        border-radius: 4px;
        overflow: hidden;
      }
      .usage-fill {
        height: 100%;
        background: var(--success);
        border-radius: 4px;
        transition: width 0.5s ease;
      }
      .usage-fill.warning {
        background: var(--warning);
      }
      .usage-fill.critical {
        background: var(--error);
      }
      .usage-label {
        font-size: 0.75rem;
        color: var(--text-muted);
        text-align: center;
      }
    `,
  ],
})
export class WidgetDiskUsageComponent {
  used = input(0);
  total = input(0);

  usagePercent = computed(() => {
    const t = this.total();
    if (t === 0) return 0;
    return Math.min(100, (this.used() / t) * 100);
  });

  formatSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }
}