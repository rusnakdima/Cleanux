/* sys lib */
import {
  ChangeDetectionStrategy,
  Component,
  OnInit,
  OnDestroy,
  signal,
  inject,
} from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';

/* router */
import { Router, RouterModule, NavigationEnd } from '@angular/router';
import { filter, Subscription } from 'rxjs';

interface NavRouteConfig {
  pattern: RegExp;
  icon: string;
  label: string;
}

interface NavItem {
  label: string;
  icon: string;
  route: string;
  childRoutes?: NavRouteConfig[];
}

@Component({
  selector: 'app-bottom-nav',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, RouterModule],
  templateUrl: './bottom-nav.component.html',
})
export class BottomNavComponent implements OnInit, OnDestroy {
  private router = inject(Router);
  url = signal(this.router.url.split('?')[0]);
  private routerSub?: Subscription;

  get listNavs(): NavItem[] {
    return [
      {
        label: 'Home',
        icon: 'home',
        route: '/home',
        childRoutes: [
          { pattern: /^\/home$/, icon: 'home', label: 'Home' },
          { pattern: /^\/dashboard$/, icon: 'home', label: 'Dashboard' },
          { pattern: /^\/system$/, icon: 'computer', label: 'System' },
          { pattern: /^\/system-repair$/, icon: 'build', label: 'Repair' },
          { pattern: /^\/reports$/, icon: 'assessment', label: 'Reports' },
        ],
      },
      {
        label: 'Files',
        icon: 'folder_open',
        route: '/files',
        childRoutes: [
          { pattern: /^\/files$/, icon: 'folder_open', label: 'Files' },
          { pattern: /^\/large-files$/, icon: 'file_upload', label: 'Large Files' },
          { pattern: /^\/duplicate-finder$/, icon: 'file_copy', label: 'Duplicates' },
          { pattern: /^\/disk-usage$/, icon: 'pie_chart', label: 'Disk Usage' },
        ],
      },
      {
        label: 'Clean',
        icon: 'auto_fix_high',
        route: '/clean',
        childRoutes: [
          { pattern: /^\/clean$/, icon: 'auto_fix_high', label: 'Clean' },
          { pattern: /^\/cleaner$/, icon: 'cleaning_services', label: 'Quick Clean' },
          { pattern: /^\/log-manager$/, icon: 'description', label: 'Logs' },
          { pattern: /^\/kernel-cleaner$/, icon: 'memory', label: 'Kernels' },
          { pattern: /^\/media-cleaner$/, icon: 'photo_library', label: 'Media' },
          { pattern: /^\/container-cleaner$/, icon: 'inbox', label: 'Containers' },
          { pattern: /^\/dev-cleaner$/, icon: 'code', label: 'Dev Cache' },
          { pattern: /^\/package-deep-clean$/, icon: 'inventory_2', label: 'Packages' },
          { pattern: /^\/app-residue-cleaner$/, icon: 'app_settings_alt', label: 'Residue' },
          { pattern: /^\/advanced-cleaner$/, icon: 'settings', label: 'Advanced' },
          { pattern: /^\/downloads$/, icon: 'download', label: 'Downloads' },
          { pattern: /^\/recent$/, icon: 'schedule', label: 'Recent' },
          { pattern: /^\/clipboard$/, icon: 'content_paste', label: 'Clipboard' },
        ],
      },
      {
        label: 'Power',
        icon: 'bolt',
        route: '/power',
        childRoutes: [
          { pattern: /^\/power$/, icon: 'bolt', label: 'Power' },
          { pattern: /^\/startup$/, icon: 'rocket_launch', label: 'Startup' },
          { pattern: /^\/memory-optimizer$/, icon: 'memory', label: 'Memory' },
          { pattern: /^\/automation$/, icon: 'automation', label: 'Automation' },
          { pattern: /^\/processes$/, icon: 'memory', label: 'Processes' },
        ],
      },
      {
        label: 'Settings',
        icon: 'settings',
        route: '/settings',
        childRoutes: [
          { pattern: /^\/settings$/, icon: 'settings', label: 'Settings' },
          { pattern: /^\/backup$/, icon: 'backup', label: 'Backup' },
          { pattern: /^\/profiles$/, icon: 'account_circle', label: 'Profiles' },
        ],
      },
    ];
  }

  getIcon(nav: NavItem): string {
    if (nav.childRoutes) {
      const match = this.findRouteMatch(nav.childRoutes);
      return match?.icon ?? nav.icon;
    }
    return nav.icon;
  }

  getLabel(nav: NavItem): string {
    if (nav.childRoutes) {
      const match = this.findRouteMatch(nav.childRoutes);
      return match?.label ?? nav.label;
    }
    return nav.label;
  }

  private findRouteMatch(routes: NavRouteConfig[]): NavRouteConfig | undefined {
    return routes.find((r) => r.pattern.test(this.url()));
  }

  isActiveRoute(nav: NavItem): boolean {
    if (nav.childRoutes) {
      return this.findRouteMatch(nav.childRoutes) !== undefined;
    }
    return false;
  }

  ngOnInit(): void {
    this.routerSub = this.router.events
      .pipe(filter((event) => event instanceof NavigationEnd))
      .subscribe((event) => {
        const url = (event as NavigationEnd).urlAfterRedirects;
        const lastIndex = url.indexOf('?') > -1 ? url.indexOf('?') : url.length;
        this.url.set(url.slice(0, lastIndex));
      });
  }

  ngOnDestroy(): void {
    this.routerSub?.unsubscribe();
  }
}
