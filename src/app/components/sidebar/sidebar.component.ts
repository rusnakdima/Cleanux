/* sys lib */
import { ChangeDetectionStrategy, Component, signal, output, inject } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';
import { MatTooltipModule } from '@angular/material/tooltip';

/* router */
import { RouterLink, RouterLinkActive } from '@angular/router';

/* services */
import { ThemeService } from '@services/theme.service';

@Component({
  selector: 'app-sidebar',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, MatTooltipModule, RouterLink, RouterLinkActive],
  templateUrl: './sidebar.component.html',
})
export class SidebarComponent {
  private themeService = inject(ThemeService);

  isCollapsed = signal(false);
  isDarkMode = signal(true);
  darkToggle = output<void>();

  navItems = [
    { label: 'Dashboard', icon: 'dashboard', route: '/dashboard' },
    { label: 'System', icon: 'memory', route: '/system' },
    { label: 'Battery & Power', icon: 'battery_charging_full', route: '/power' },
    { label: 'Cleaner', icon: 'cleaning_services', route: '/cleaner' },
    { label: 'Kernel & Boot', icon: 'settings_input_component', route: '/kernel-cleaner' },
    { label: 'Large Files', icon: 'file_copy', route: '/large-files' },
    { label: 'Reports', icon: 'analytics', route: '/reports' },
    { label: 'Settings', icon: 'settings', route: '/settings' },
  ];

  constructor() {
    this.themeService.applyTheme(this.themeService.currentTheme());
    this.isDarkMode.set(this.themeService.getEffectiveMode());
  }

  toggleSidebar() {
    this.isCollapsed.update((v) => !v);
  }

  onToggleDarkMode() {
    const current = this.themeService.currentTheme();
    const newMode = current.mode === 'dark' ? 'light' : 'dark';
    this.themeService.setMode(newMode);
    this.isDarkMode.set(this.themeService.getEffectiveMode());
  }
}
