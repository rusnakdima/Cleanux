/* sys lib */
import {
  ChangeDetectionStrategy,
  Component,
  inject,
  OnInit,
  input,
  output,
  computed,
} from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';
import { MatTooltipModule } from '@angular/material/tooltip';

/* router */
import { RouterLink, RouterLinkActive } from '@angular/router';

/* services */
import { ThemeService } from '@services/theme.service';
import { MonitorStore } from '@stores/monitor.store';

interface NavItem {
  label: string;
  icon: string;
  route: string;
  badge?: string;
  badgeClass?: string;
}

@Component({
  selector: 'app-sidebar',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, MatTooltipModule, RouterLink, RouterLinkActive],
  templateUrl: './sidebar.component.html',
})
export class SidebarComponent implements OnInit {
  private themeService = inject(ThemeService);
  protected monitorStore = inject(MonitorStore);

  isOpen = input(false);
  isCollapsed = input(false);
  close = output<void>();

  isDarkMode = computed(() => this.themeService.getEffectiveMode());
  healthScore = computed(() => this.monitorStore.healthScore());
  totalJunk = computed(() => this.monitorStore.totalJunkSize());

  diskUsagePercent = computed(() => {
    const stats = this.monitorStore.systemStats();
    const used = stats.diskUsed || 0;
    const total = stats.diskTotal || 512;
    return Math.min((used / total) * 100, 100);
  });

  diskUsageDisplay = computed(() => {
    const stats = this.monitorStore.systemStats();
    const used = stats.diskUsed || 0;
    const total = stats.diskTotal || 512;
    return `${(used / (1024 * 1024 * 1024)).toFixed(1)}G / ${total}G`;
  });

  ngOnInit() {
    this.themeService.applyTheme(this.themeService.currentTheme());
  }

  onToggleDarkMode() {
    const current = this.themeService.currentTheme();
    const newMode = current.mode === 'dark' ? 'light' : 'dark';
    this.themeService.setMode(newMode);
  }

  onClose() {
    this.close.emit();
  }
}
