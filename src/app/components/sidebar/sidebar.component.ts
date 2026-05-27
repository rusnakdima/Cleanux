/* sys lib */
import { ChangeDetectionStrategy, Component, inject, OnInit, input, output, computed } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';
import { MatTooltipModule } from '@angular/material/tooltip';

/* router */
import { RouterLink, RouterLinkActive } from '@angular/router';

/* services */
import { ThemeService } from '@services/theme.service';
import { MonitorStore } from '@stores/monitor.store';
import { formatSize } from '@shared/utils/format.util';

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
  styleUrls: ['./sidebar.component.css'],
})
export class SidebarComponent implements OnInit {
  private themeService = inject(ThemeService);
  protected monitorStore = inject(MonitorStore);

  formatSize = formatSize;

  isOpen = input(false);
  isCollapsed = input(false);
  close = output<void>();

  isDarkMode = computed(() => this.themeService.getEffectiveMode());
  healthScore = computed(() => this.monitorStore.healthScore());
  totalJunk = computed(() => this.monitorStore.totalJunkSize());

  mainNavItems: NavItem[] = [
    { label: 'Dashboard', icon: 'dashboard', route: '/dashboard' },
    { label: 'Cleaner', icon: 'cleaning_services', route: '/cleaner' },
    { label: 'System', icon: 'memory', route: '/system' },
    { label: 'Battery & Power', icon: 'battery_charging_full', route: '/power' },
  ];

  toolsNavItems: NavItem[] = [
    { label: 'Large Files', icon: 'file_copy', route: '/large-files', badge: 'New', badgeClass: 'badge-info' },
    { label: 'Duplicate Finder', icon: 'content_copy', route: '/duplicate-finder' },
    { label: 'Kernel & Boot', icon: 'settings_input_component', route: '/kernel-cleaner' },
    { label: 'Log Manager', icon: 'description', route: '/log-manager' },
    { label: 'Startup', icon: 'rocket_launch', route: '/startup' },
    { label: 'Memory', icon: 'memory', route: '/memory-optimizer' },
  ];

  footerNavItems: NavItem[] = [
    { label: 'Automation', icon: 'bolt', route: '/automation' },
    { label: 'Reports', icon: 'analytics', route: '/reports' },
    { label: 'Settings', icon: 'settings', route: '/settings' },
  ];

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