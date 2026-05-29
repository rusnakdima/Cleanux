/* sys lib */
import { Component, signal, inject, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router, RouterOutlet, NavigationEnd } from '@angular/router';
import { filter } from 'rxjs/operators';

/* components */
import { BottomNavComponent } from '@components/bottom-nav/bottom-nav.component';
import { HeaderBarComponent } from '@components/header-bar/header-bar.component';

interface RouteInfo {
  title: string;
  breadcrumb: string;
  parent: string;
  showBack: boolean;
}

@Component({
  selector: 'app-root',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, RouterOutlet, BottomNavComponent, HeaderBarComponent],
  templateUrl: './app.html',
})
export class App {
  pageTitle = signal('Home');
  pageBreadcrumb = signal('');
  showBackButton = signal(false);

  private router = inject(Router);

  private mainTabs = ['home', 'files', 'clean', 'power', 'settings'];

  private routeInfo: Record<string, RouteInfo> = {
    home: { title: 'Home', breadcrumb: '', parent: '', showBack: false },
    files: { title: 'Files', breadcrumb: '', parent: '', showBack: false },
    clean: { title: 'Clean', breadcrumb: '', parent: '', showBack: false },
    power: { title: 'Power', breadcrumb: '', parent: '', showBack: false },
    settings: { title: 'Settings', breadcrumb: '', parent: '', showBack: false },
    dashboard: { title: 'Home', breadcrumb: '', parent: '', showBack: false },
    cleaner: { title: 'Quick Clean', breadcrumb: 'Clean', parent: 'clean', showBack: true },
    'large-files': { title: 'Large Files', breadcrumb: 'Files', parent: 'files', showBack: true },
    'duplicate-finder': {
      title: 'Duplicates',
      breadcrumb: 'Files',
      parent: 'files',
      showBack: true,
    },
    'disk-usage': { title: 'Disk Usage', breadcrumb: 'Files', parent: 'files', showBack: true },
    downloads: { title: 'Downloads', breadcrumb: 'Clean', parent: 'clean', showBack: true },
    recent: { title: 'Recent Files', breadcrumb: 'Clean', parent: 'clean', showBack: true },
    clipboard: { title: 'Clipboard', breadcrumb: 'Clean', parent: 'clean', showBack: true },
    system: { title: 'System', breadcrumb: 'Home', parent: 'home', showBack: true },
    startup: { title: 'Startup', breadcrumb: 'Power', parent: 'power', showBack: true },
    backup: { title: 'Backup', breadcrumb: 'Settings', parent: 'settings', showBack: true },
    profiles: { title: 'Profiles', breadcrumb: 'Settings', parent: 'settings', showBack: true },
    'system-repair': { title: 'Repair', breadcrumb: 'Home', parent: 'home', showBack: true },
    'memory-optimizer': { title: 'Memory', breadcrumb: 'Power', parent: 'power', showBack: true },
    automation: { title: 'Automation', breadcrumb: 'Power', parent: 'power', showBack: true },
    'log-manager': { title: 'Logs', breadcrumb: 'Clean', parent: 'clean', showBack: true },
    'kernel-cleaner': { title: 'Kernel', breadcrumb: 'Clean', parent: 'clean', showBack: true },
    reports: { title: 'Reports', breadcrumb: 'Home', parent: 'home', showBack: true },
    processes: { title: 'Processes', breadcrumb: 'Power', parent: 'power', showBack: true },
    'media-cleaner': { title: 'Media', breadcrumb: 'Clean', parent: 'clean', showBack: true },
    'container-cleaner': {
      title: 'Containers',
      breadcrumb: 'Clean',
      parent: 'clean',
      showBack: true,
    },
    'dev-cleaner': { title: 'Dev Cache', breadcrumb: 'Clean', parent: 'clean', showBack: true },
    'package-deep-clean': {
      title: 'Packages',
      breadcrumb: 'Clean',
      parent: 'clean',
      showBack: true,
    },
    'app-residue-cleaner': {
      title: 'Residue',
      breadcrumb: 'Clean',
      parent: 'clean',
      showBack: true,
    },
    'advanced-cleaner': { title: 'Advanced', breadcrumb: 'Clean', parent: 'clean', showBack: true },
  };

  constructor() {
    this.updateRouteInfo(this.router.url);

    this.router.events
      .pipe(filter((event) => event instanceof NavigationEnd))
      .subscribe((event: NavigationEnd) => {
        this.updateRouteInfo(event.urlAfterRedirects);
      });
  }

  onBack() {
    const currentPath = this.router.url.split('/').pop()?.split('?')[0] || 'home';
    const info = this.routeInfo[currentPath];
    if (info && info.parent) {
      this.router.navigate(['/' + info.parent]);
    } else {
      this.router.navigate(['/home']);
    }
  }

  private updateRouteInfo(url: string) {
    const path = url.split('/').pop()?.split('?')[0] || 'home';
    const info = this.routeInfo[path];

    if (info) {
      this.pageTitle.set(info.title);
      this.pageBreadcrumb.set(info.breadcrumb);
      this.showBackButton.set(info.showBack);
    } else {
      this.pageTitle.set(this.formatTitle(path));
      this.pageBreadcrumb.set('');
      this.showBackButton.set(false);
    }
  }

  private formatTitle(path: string): string {
    return path
      .split('-')
      .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
      .join(' ');
  }
}
