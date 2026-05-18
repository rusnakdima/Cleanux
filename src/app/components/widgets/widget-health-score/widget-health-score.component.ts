import { Component, input, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';

@Component({
  selector: 'app-widget-health-score',
  standalone: true,
  imports: [CommonModule, MatIconModule],
  template: `
    <div class="health-score-widget">
      <div class="score-display">
        <svg class="score-ring" viewBox="0 0 100 100">
          <circle
            cx="50"
            cy="50"
            r="45"
            stroke="currentColor"
            stroke-width="8"
            fill="none"
            class="ring-bg"
          />
          <circle
            cx="50"
            cy="50"
            r="45"
            stroke="currentColor"
            stroke-width="8"
            fill="none"
            [attr.stroke-dasharray]="283"
            [attr.stroke-dashoffset]="283 - (283 * score()) / 100"
            stroke-linecap="round"
            class="score-ring-progress"
          />
        </svg>
        <div class="score-value">
          <span class="score-number">{{ score() }}</span>
          <span class="score-label">%</span>
        </div>
      </div>
      <div class="trend-display">
        @if (trend() === 'improving') {
          <mat-icon class="trend-up">trending_up</mat-icon>
          <span class="trend-text positive">+{{ changePercent() | number: '1.1-1' }}%</span>
        } @else if (trend() === 'declining') {
          <mat-icon class="trend-down">trending_down</mat-icon>
          <span class="trend-text negative">{{ changePercent() | number: '1.1-1' }}%</span>
        } @else {
          <mat-icon class="trend-flat">trending_flat</mat-icon>
          <span class="trend-text neutral">{{ changePercent() | number: '1.1-1' }}%</span>
        }
      </div>
    </div>
  `,
  styles: [
    `
      .health-score-widget {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
        gap: 1rem;
      }
      .score-display {
        position: relative;
        width: 120px;
        height: 120px;
      }
      .score-ring {
        width: 100%;
        height: 100%;
        transform: rotate(-90deg);
      }
      .ring-bg {
        color: var(--ring-bg, #e5e7eb);
      }
      .score-ring-progress {
        color: var(--score-color, #22c55e);
        transition: stroke-dashoffset 1s ease-out;
      }
      .score-value {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        text-align: center;
      }
      .score-number {
        font-size: 2rem;
        font-weight: 700;
        color: var(--text-primary, #374151);
      }
      .score-label {
        font-size: 1rem;
        color: var(--text-muted, #9ca3af);
      }
      .trend-display {
        display: flex;
        align-items: center;
        gap: 0.5rem;
      }
      .trend-up {
        color: #22c55e;
      }
      .trend-down {
        color: #ef4444;
      }
      .trend-flat {
        color: #f59e0b;
      }
      .trend-text {
        font-weight: 600;
        font-size: 0.875rem;
      }
      .positive {
        color: #22c55e;
      }
      .negative {
        color: #ef4444;
      }
      .neutral {
        color: #f59e0b;
      }
    `,
  ],
})
export class WidgetHealthScoreComponent {
  score = input(0);
  trend = input<'improving' | 'declining' | 'stable' | 'insufficient_data'>('insufficient_data');
  changePercent = input(0);
}
