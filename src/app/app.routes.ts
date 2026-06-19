/* sys lib */
import { Routes } from '@angular/router';

export const routes: Routes = [
  { path: '', redirectTo: 'dashboard', pathMatch: 'full' },

  {
    path: 'dashboard',
    loadComponent: () => import('@pages/dashboard/dashboard.view').then((m) => m.DashboardView),
  },

  {
    path: 'files',
    loadComponent: () => import('@pages/files/files.view').then((m) => m.FilesView),
  },
  {
    path: 'clean',
    loadComponent: () => import('@pages/clean/clean.view').then((m) => m.CleanView),
  },
  {
    path: 'power',
    loadComponent: () => import('@pages/power/power.view').then((m) => m.PowerView),
  },
  {
    path: 'settings',
    loadComponent: () => import('@pages/settings/settings.view').then((m) => m.SettingsView),
  },

  {
    path: 'cleaner',
    loadComponent: () => import('@pages/cleaner/cleaner.view').then((m) => m.CleanerView),
  },
  {
    path: 'system',
    loadComponent: () => import('@pages/system/system.view').then((m) => m.SystemView),
  },
  {
    path: 'large-files',
    loadComponent: () =>
      import('@pages/large-files/large-files.view').then((m) => m.LargeFilesView),
  },
  {
    path: 'duplicate-finder',
    loadComponent: () =>
      import('@pages/duplicate-finder/duplicate-finder.view').then((m) => m.DuplicateFinderView),
  },
  {
    path: 'startup',
    loadComponent: () => import('@pages/startup/startup.view').then((m) => m.StartupView),
  },
  {
    path: 'backup',
    loadComponent: () => import('@pages/backup/backup.view').then((m) => m.BackupView),
  },
  {
    path: 'profiles',
    loadComponent: () => import('@pages/profiles/profiles.view').then((m) => m.ProfilesView),
  },
  {
    path: 'system-repair',
    loadComponent: () =>
      import('@pages/system-repair/system-repair.view').then((m) => m.SystemRepairView),
  },
  {
    path: 'memory-optimizer',
    loadComponent: () =>
      import('@pages/memory-optimizer/memory-optimizer.view').then((m) => m.MemoryOptimizerView),
  },
  {
    path: 'automation',
    loadComponent: () => import('@pages/automation/automation.view').then((m) => m.AutomationView),
  },
  {
    path: 'log-manager',
    loadComponent: () =>
      import('@pages/log-manager/log-manager.view').then((m) => m.LogManagerView),
  },

  {
    path: 'kernel-cleaner',
    loadComponent: () =>
      import('@pages/kernel-cleaner/kernel-cleaner.view').then((m) => m.KernelCleanerView),
  },
  {
    path: 'reports',
    loadComponent: () => import('@pages/reports/reports.view').then((m) => m.ReportsView),
  },
  {
    path: 'processes',
    loadComponent: () => import('@pages/processes/processes.view').then((m) => m.ProcessesView),
  },
  {
    path: 'media-cleaner',
    loadComponent: () =>
      import('@pages/media-cleaner/media-cleaner.view').then((m) => m.MediaCleanerView),
  },
  {
    path: 'container-cleaner',
    loadComponent: () =>
      import('@pages/container-cleaner/container-cleaner.view').then((m) => m.ContainerCleanerView),
  },
  {
    path: 'dev-cleaner',
    loadComponent: () =>
      import('@pages/dev-cleaner/dev-cleaner.view').then((m) => m.DevCleanerView),
  },
  {
    path: 'package-deep-clean',
    loadComponent: () =>
      import('@pages/package-deep-clean/package-deep-clean.view').then(
        (m) => m.PackageDeepCleanView
      ),
  },
  {
    path: 'app-residue-cleaner',
    loadComponent: () =>
      import('@pages/app-residue-cleaner/app-residue-cleaner.view').then(
        (m) => m.AppResidueCleanerView
      ),
  },
  {
    path: 'advanced-cleaner',
    loadComponent: () =>
      import('@pages/advanced-cleaner/advanced-cleaner.view').then((m) => m.AdvancedCleanerView),
  },
  {
    path: 'disk-usage',
    loadComponent: () => import('@pages/disk-usage/disk-usage.view').then((m) => m.DiskUsageView),
  },

  {
    path: 'downloads',
    loadComponent: () => import('@pages/downloads/downloads.view').then((m) => m.DownloadsView),
  },
  {
    path: 'recent',
    loadComponent: () => import('@pages/recent/recent.view').then((m) => m.RecentView),
  },
  {
    path: 'clipboard',
    loadComponent: () => import('@pages/clipboard/clipboard.view').then((m) => m.ClipboardView),
  },

  { path: '**', redirectTo: 'dashboard' },
];
