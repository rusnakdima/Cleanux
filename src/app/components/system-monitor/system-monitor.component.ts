/* sys lib */
import {
  Component,
  OnInit,
  OnDestroy,
  signal,
  inject,
  ChangeDetectionStrategy,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';

import { MonitorStore } from '@stores/monitor.store';

@Component({
  selector: 'app-system-monitor',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule],
  templateUrl: './system-monitor.component.html',
})
export class SystemMonitorComponent implements OnInit, OnDestroy {
  protected store = inject(MonitorStore);

  ngOnInit() {
    this.store.startMonitoring();
  }

  ngOnDestroy() {}

  formatBytes(bytes: number): string {
    return this.store.formatBytes(bytes);
  }

  getUsageColor(percent: number): string {
    return this.store.getUsageColor(percent);
  }

  getCpuColor(percent: number): string {
    if (percent >= 90) return 'var(--error)';
    if (percent >= 70) return 'var(--warning)';
    return 'var(--icon-cpu)';
  }
}
