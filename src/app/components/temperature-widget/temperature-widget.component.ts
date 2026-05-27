/* sys lib */
import { Component, OnInit, ChangeDetectionStrategy, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';

import { MonitorStore, TemperatureInfo } from '@stores/monitor.store';

@Component({
  selector: 'app-temperature-widget',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule],
  templateUrl: './temperature-widget.component.html',
  styleUrls: ['./temperature-widget.component.css'],
})
export class TemperatureWidgetComponent implements OnInit {
  protected store = inject(MonitorStore);

  ngOnInit() {
    if (!this.store.isMonitoring()) {
      this.store.startMonitoring();
    }
  }

  getTemperature(temp: TemperatureInfo | null): number {
    return temp?.temperature_celsius ?? 0;
  }

  getTemperatureColor(temp: number): string {
    if (temp >= 80) return 'var(--error)';
    if (temp >= 60) return 'var(--warning)';
    return 'var(--success)';
  }

  getProgressWidth(temp: number): number {
    return Math.min((temp / 100) * 100, 100);
  }
}