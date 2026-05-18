/* sys lib */
import { Routes } from '@angular/router';

export const routes: Routes = [
  { path: '', redirectTo: 'dashboard', pathMatch: 'full' },
  {
    path: 'dashboard',
    loadComponent: () => import('@views/dashboard/dashboard.view').then((m) => m.DashboardView),
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
  },
  {
    path: 'log-manager',
    loadComponent: () =>
      import('@views/log-manager/log-manager.view').then((m) => m.LogManagerView),
  },
];
