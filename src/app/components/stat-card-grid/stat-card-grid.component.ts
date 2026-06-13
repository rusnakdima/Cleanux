/* sys lib */
import { ChangeDetectionStrategy, Component, input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';

export interface StatCard {
  icon: string;
  label: string;
  value: string | number;
  color?: string;
}

@Component({
  selector: 'app-stat-card-grid',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule],
  templateUrl: './stat-card-grid.component.html',
  styleUrl: './stat-card-grid.component.css',
})
export class StatCardGridComponent {
  cards = input<StatCard[]>([]);

  private colorMap: Record<string, string> = {
    info: 'bg-info/10 text-info',
    success: 'bg-success/10 text-success',
    error: 'bg-error/10 text-error',
    warning: 'bg-warning/10 text-warning',
  };

  getAccentGradient(color?: string): string {
    if (!color) {
      return 'linear-gradient(135deg, var(--accent) 0%, var(--accent-secondary) 100%)';
    }
    return this.colorMap[color] || this.colorMap['info'];
  }
}
