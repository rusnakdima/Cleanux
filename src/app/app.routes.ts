/* sys lib */
import { Routes } from '@angular/router';
import { authGuard } from '@guards/auth.guard';
import { dashboardDataResolver } from '@resolvers/dashboard.resolver';
import { profilesResolver } from '@resolvers/profile.resolver';
import { automationResolver } from '@resolvers/automation.resolver';

export const routes: Routes = [
  { path: '', redirectTo: 'dashboard', pathMatch: 'full' },
  {
    path: 'dashboard',
    loadComponent: () => import('@views/dashboard/dashboard.view').then((m) => m.DashboardView),
    resolve: {
      data: dashboardDataResolver,
    },
  },
  {
    path: 'cleaner',
    loadComponent: () => import('@views/cleaner/cleaner.view').then((m) => m.CleanerView),
  },
  {
    path: 'system',
    loadComponent: () => import('@views/system/system.view').then((m) => m.SystemView),
  },
  {
    path: 'large-files',
    loadComponent: () =>
      import('@views/large-files/large-files.view').then((m) => m.LargeFilesView),
  },
  {
    path: 'duplicate-finder',
    loadComponent: () =>
      import('@views/duplicate-finder/duplicate-finder.view').then((m) => m.DuplicateFinderView),
  },
  {
    path: 'settings',
    loadComponent: () => import('@views/settings/settings.view').then((m) => m.SettingsView),
  },
  {
    path: 'startup',
    loadComponent: () => import('@views/startup/startup.view').then((m) => m.StartupView),
  },
  {
    path: 'backup',
    loadComponent: () => import('@views/backup/backup.view').then((m) => m.BackupView),
  },
  {
    path: 'profiles',
    loadComponent: () => import('@views/profiles/profiles.view').then((m) => m.ProfilesView),
    resolve: {
      profiles: profilesResolver,
    },
  },
  {
    path: 'system-repair',
    loadComponent: () =>
      import('@views/system-repair/system-repair.view').then((m) => m.SystemRepairView),
  },
  {
    path: 'memory-optimizer',
    loadComponent: () =>
      import('@views/memory-optimizer/memory-optimizer.view').then((m) => m.MemoryOptimizerView),
  },
  {
    path: 'automation',
    loadComponent: () => import('@views/automation/automation.view').then((m) => m.AutomationView),
    resolve: {
      automationData: automationResolver,
    },
  },
  {
    path: 'log-manager',
    loadComponent: () =>
      import('@views/log-manager/log-manager.view').then((m) => m.LogManagerView),
  },
  {
    path: 'power',
    loadComponent: () => import('@views/power/power.view').then((m) => m.PowerView),
  },
  {
    path: 'kernel-cleaner',
    loadComponent: () =>
      import('@views/kernel-cleaner/kernel-cleaner.view').then((m) => m.KernelCleanerView),
  },
  {
    path: 'reports',
    loadComponent: () => import('@views/reports/reports.view').then((m) => m.ReportsView),
  },
  {
    path: 'processes',
    loadComponent: () => import('@views/processes/processes.view').then((m) => m.ProcessesView),
  },
  {
    path: 'media-cleaner',
    loadComponent: () =>
      import('@views/media-cleaner/media-cleaner.view').then((m) => m.MediaCleanerView),
  },
  {
    path: 'container-cleaner',
    loadComponent: () =>
      import('@views/container-cleaner/container-cleaner.view').then((m) => m.ContainerCleanerView),
  },
  {
    path: 'dev-cleaner',
    loadComponent: () => import('@views/dev-cleaner/dev-cleaner.view').then((m) => m.DevCleanerView),
  },
  {
    path: 'package-deep-clean',
    loadComponent: () =>
      import('@views/package-deep-clean/package-deep-clean.view').then((m) => m.PackageDeepCleanView),
  },
  {
    path: 'app-residue-cleaner',
    loadComponent: () =>
      import('@views/app-residue-cleaner/app-residue-cleaner.view').then(
        (m) => m.AppResidueCleanerView
      ),
  },
  {
    path: 'advanced-cleaner',
    loadComponent: () =>
      import('@views/advanced-cleaner/advanced-cleaner.view').then((m) => m.AdvancedCleanerView),
  },
  {
    path: 'disk-usage',
    loadComponent: () => import('@views/disk-usage/disk-usage.view').then((m) => m.DiskUsageView),
  },
];
