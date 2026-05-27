/* sys lib */
import { ApplicationConfig, provideBrowserGlobalErrorListeners } from '@angular/core';
import { provideRouter } from '@angular/router';
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';
import { provideHttpClient } from '@angular/common/http';

/* services */
import { I18nService } from './services/i18n.service';

/* interceptors */
import { ErrorInterceptorService } from '@interceptors/error.interceptor';
import { LoadingInterceptorService } from '@interceptors/loading.interceptor';
import { LoggingInterceptorService } from '@interceptors/logging.interceptor';

/* api */
import { TauriApiService } from '@api/tauri-api.service';

/* stores */
import { CleanerStore } from '@stores/cleaner.store';
import { SystemStore } from '@stores/system.store';
import { MonitorStore } from '@stores/monitor.store';
import { AutomationStore } from '@stores/automation.store';

/* routes */
import { routes } from './app.routes';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter(routes),
    provideAnimationsAsync(),
    provideHttpClient(),
    I18nService,
    TauriApiService,
    CleanerStore,
    SystemStore,
    MonitorStore,
    AutomationStore,
    ErrorInterceptorService,
    LoadingInterceptorService,
    LoggingInterceptorService,
  ],
};
