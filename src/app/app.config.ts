/* sys lib */
import {
  ApplicationConfig,
  provideBrowserGlobalErrorListeners,
  APP_INITIALIZER,
} from '@angular/core';
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';
import { provideHttpClient } from '@angular/common/http';

/* services */
import { StyleThemeService } from '@tauri-front/shared';

/* stores */
import { CleanerStore } from '@store/cleaner.store';
import { SystemStore } from '@store/system.store';
import { MonitorStore } from '@store/monitor.store';
import { AutomationStore } from '@store/automation.store';

// ThemeService alias for API compatibility: StyleThemeService from shared lib
// is used directly (it has init() as a no-op, constructor handles init).
class ThemeService extends StyleThemeService {}

function initializeTheme(themeService: ThemeService) {
  return () => {
    themeService.init();
  };
}

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideAnimationsAsync(),
    provideHttpClient(),
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
