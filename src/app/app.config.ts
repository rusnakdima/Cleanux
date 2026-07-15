/* angular */
import { ApplicationConfig, provideAppInitializer, inject } from '@angular/core';
import { provideUnifiedApp } from '@tauri-front/shared';

/* library */
import { StyleThemeService } from '@tauri-front/shared';

/* stores */
import { CleanerStore } from '@store/cleaner.store';
import { SystemStore } from '@store/system.store';
import { MonitorStore } from '@store/monitor.store';
import { AutomationStore } from '@store/automation.store';

export const appConfig: ApplicationConfig = {
  providers: [
    ...provideUnifiedApp({
      enableAnimations: true,
      enableHttpClient: true,
      enableBrowserErrorListeners: true,
      enableZoneChangeDetection: true,
    }),
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
