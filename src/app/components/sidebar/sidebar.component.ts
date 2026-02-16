/* sys lib */
import { Component, signal, output, input } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';
import { MatTooltipModule } from '@angular/material/tooltip';

/* router */
import { RouterLink, RouterLinkActive } from '@angular/router';

@Component({
  selector: 'app-sidebar',
  standalone: true,
  imports: [CommonModule, MatIconModule, MatTooltipModule, RouterLink, RouterLinkActive],
  templateUrl: './sidebar.component.html',
})
export class SidebarComponent {
  isCollapsed = signal(false);
  isDarkMode = input<boolean>(true);
  darkToggle = output<void>();

  navItems = [
    { label: 'Dashboard', icon: 'dashboard', route: '/dashboard' },
    { label: 'System', icon: 'memory', route: '/system' },
    { label: 'Cleaner', icon: 'cleaning_services', route: '/cleaner' },
    { label: 'Large Files', icon: 'file_copy', route: '/large-files' },
    { label: 'Settings', icon: 'settings', route: '/settings' },
  ];

  toggleSidebar() {
    this.isCollapsed.update((v) => !v);
  }

  onToggleDarkMode() {
    this.darkToggle.emit();
  }
}
