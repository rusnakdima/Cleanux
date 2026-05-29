/* sys lib */
import { ChangeDetectionStrategy, Component, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';

/* router */
import { RouterLink } from '@angular/router';

/* services */
import { MonitorStore } from '@stores/monitor.store';

@Component({
  selector: 'app-power-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, RouterLink],
  templateUrl: './power.view.html',
})
export class PowerView {
  protected monitorStore = inject(MonitorStore);

  powerTools = [
    {
      id: 'memory-optimizer',
      label: 'Memory',
      icon: 'memory',
      route: '/memory-optimizer',
      desc: 'Optimize RAM usage',
    },
    {
      id: 'processes',
      label: 'Processes',
      icon: 'list',
      route: '/processes',
      desc: 'Manage running processes',
    },
    {
      id: 'startup',
      label: 'Startup',
      icon: 'rocket_launch',
      route: '/startup',
      desc: 'Control startup programs',
    },
    {
      id: 'automation',
      label: 'Automation',
      icon: 'schedule',
      route: '/automation',
      desc: 'Set up automated tasks',
    },
  ];
}
