import { Component, input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { SystemMonitorComponent } from '../../system-monitor/system-monitor.component';

@Component({
  selector: 'app-widget-system-monitor',
  standalone: true,
  imports: [CommonModule, SystemMonitorComponent],
  template: `<app-system-monitor></app-system-monitor>`,
})
export class WidgetSystemMonitorComponent {
  enabled = input(true);
}
