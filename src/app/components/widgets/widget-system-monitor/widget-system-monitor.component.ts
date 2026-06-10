import { Component, input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { SystemMonitorComponent } from '../../system-monitor/system-monitor.component';

@Component({
  selector: 'app-widget-system-monitor',
  standalone: true,
  imports: [CommonModule, SystemMonitorComponent],
  templateUrl: './widget-system-monitor.component.html',
})
export class WidgetSystemMonitorComponent {
  enabled = input(true);
}
