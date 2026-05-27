/* sys lib */
import { Component, signal, inject, OnInit, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router, RouterOutlet, NavigationEnd } from '@angular/router';
import { filter } from 'rxjs/operators';

/* components */
import { SidebarComponent } from '@components/sidebar/sidebar.component';
import { TopHeaderComponent } from '@components/top-header/top-header.component';

@Component({
  selector: 'app-root',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, RouterOutlet, SidebarComponent, TopHeaderComponent],
  templateUrl: './app.html',
  styleUrls: ['./app.css'],
})
export class App implements OnInit {
  isMobileMenuOpen = signal(false);
  isCollapsed = signal(false);
  pageTitle = signal('Dashboard');
  pageBreadcrumb = signal('Home');

  toggleMobileMenu() {
    this.isMobileMenuOpen.update((v) => !v);
  }

  closeMobileMenu() {
    this.isMobileMenuOpen.set(false);
  }

  toggleSidebar() {
    this.isCollapsed.update((v) => !v);
  }

  private router = inject(Router);

  private routeTitles: Record<string, { title: string; breadcrumb: string }> = {
    'dashboard': { title: 'Dashboard', breadcrumb: 'Home' },
    'cleaner': { title: 'Cleaner', breadcrumb: 'System Tools' },
    'system': { title: 'System Information', breadcrumb: 'Hardware' },
    'large-files': { title: 'Large Files', breadcrumb: 'System Tools' },
    'duplicate-finder': { title: 'Duplicate Finder', breadcrumb: 'System Tools' },
    'settings': { title: 'Settings', breadcrumb: 'Preferences' },
    'startup': { title: 'Startup Programs', breadcrumb: 'System' },
    'backup': { title: 'Backup & Restore', breadcrumb: 'Tools' },
    'profiles': { title: 'Profiles', breadcrumb: 'Settings' },
    'system-repair': { title: 'System Repair', breadcrumb: 'System Tools' },
    'memory-optimizer': { title: 'Memory Optimizer', breadcrumb: 'Performance' },
    'automation': { title: 'Automation', breadcrumb: 'Settings' },
    'log-manager': { title: 'Log Manager', breadcrumb: 'System Tools' },
    'power': { title: 'Battery & Power', breadcrumb: 'Hardware' },
    'kernel-cleaner': { title: 'Kernel & Boot', breadcrumb: 'System Tools' },
    'reports': { title: 'Reports', breadcrumb: 'Analytics' },
    'processes': { title: 'Processes', breadcrumb: 'System' },
    'media-cleaner': { title: 'Media Cleaner', breadcrumb: 'System Tools' },
    'container-cleaner': { title: 'Container Cleaner', breadcrumb: 'System Tools' },
    'dev-cleaner': { title: 'Dev Cleaner', breadcrumb: 'System Tools' },
    'package-deep-clean': { title: 'Package Deep Clean', breadcrumb: 'System Tools' },
    'app-residue-cleaner': { title: 'App Residue Cleaner', breadcrumb: 'System Tools' },
    'advanced-cleaner': { title: 'Advanced Cleaner', breadcrumb: 'System Tools' },
  };

  ngOnInit() {
    this.updateRouteInfo(this.router.url);

    this.router.events
      .pipe(filter((event) => event instanceof NavigationEnd))
      .subscribe((event: NavigationEnd) => {
        this.updateRouteInfo(event.urlAfterRedirects);
      });
  }

  private updateRouteInfo(url: string) {
    const path = url.split('/').pop()?.split('?')[0] || 'dashboard';
    const info = this.routeTitles[path] || { title: this.formatTitle(path), breadcrumb: 'Page' };
    this.pageTitle.set(info.title);
    this.pageBreadcrumb.set(info.breadcrumb);
  }

  private formatTitle(path: string): string {
    return path
      .split('-')
      .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
      .join(' ');
  }
}