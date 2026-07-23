import { ApplicationConfig, provideAppInitializer, inject } from '@angular/core';
import { provideZoneChangeDetection } from '@angular/core';
import { provideRouter } from '@angular/router';
import { StyleThemeService } from '@tauri-front/shared';
import { routes } from './app.routes';
import { CleanerStore } from '@store/cleaner.store';
import { SystemStore } from '@store/system.store';
import { MonitorStore } from '@store/monitor.store';
import { AutomationStore } from '@store/automation.store';

export const appConfig: ApplicationConfig = {
  providers: [
    provideZoneChangeDetection({ eventCoalescing: true }),
    provideRouter(routes),
    CleanerStore,
    SystemStore,
    MonitorStore,
    AutomationStore,
    provideAppInitializer(() => {
      const themeService = inject(StyleThemeService);
      themeService.init();
    }),
  ],
};
