/* sys lib */
import {
  ApplicationConfig,
  provideBrowserGlobalErrorListeners,
  APP_INITIALIZER,
} from '@angular/core';
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';
import { provideHttpClient } from '@angular/common/http';

/* services */
import { ThemeService } from './services/theme.service';

/* stores */
import { CleanerStore } from '@store/cleaner.store';
import { SystemStore } from '@store/system.store';
import { MonitorStore } from '@store/monitor.store';
import { AutomationStore } from '@store/automation.store';

function initializeTheme(themeService: ThemeService) {
  return () => {
    themeService.applyTheme(themeService.currentTheme());
  };
}

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideAnimationsAsync(),
    provideHttpClient(),
    CleanerStore,
    SystemStore,
    MonitorStore,
    AutomationStore,
    {
      provide: APP_INITIALIZER,
      useFactory: initializeTheme,
      deps: [ThemeService],
      multi: true,
    },
  ],
};
