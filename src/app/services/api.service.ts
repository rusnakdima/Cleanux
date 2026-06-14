/* sys lib */
import { Injectable, inject } from '@angular/core';

/* api */
import { TauriApiService, ApiException } from '@api/tauri-api.service';
import { LoggingService, getLoggingService } from '@tauri-apps/logger';

/* models */
import { Response, getData } from '@models/response.model';

export { ApiException } from '@models/error.model';

@Injectable({
  providedIn: 'root',
})
export class ApiService {
  private tauriApi = inject(TauriApiService);
  private loggingService = getLoggingService();

  constructor() {
    this.loggingService.info('ApiService initialized');
  }

  async invoke<T>(
    command: string,
    args?: Record<string, unknown>,
    options: { suppressError?: boolean } = {}
  ): Promise<T> {
    this.loggingService.debug('Invoking command', { command, hasArgs: !!args });
    try {
      const result = await this.tauriApi.invoke<T>(command, args, options);
      this.loggingService.debug('Command completed', { command });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { command, args });
      throw error;
    }
  }

  async listen<T>(event: string, handler: (event: { payload: T }) => void): Promise<() => void> {
    this.loggingService.debug('Listening to event', { event });
    return this.tauriApi.listen<T>(event, handler);
  }
}
