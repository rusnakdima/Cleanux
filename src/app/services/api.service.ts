/* sys lib */
import { Injectable, inject } from '@angular/core';

/* api */
import { TauriApiService, ApiException } from '@api/tauri-api.service';
import { LoggerService } from '@services/logger.service';

/* models */
import { Response, getData } from '@models/response.model';

export { ApiException } from '@models/error.model';

@Injectable({
  providedIn: 'root',
})
export class ApiService {
  private tauriApi = inject(TauriApiService);
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'ApiService', 'init', 'ApiService initialized');
  }

  async invoke<T>(
    command: string,
    args?: Record<string, unknown>,
    options: { suppressError?: boolean } = {}
  ): Promise<T> {
    this.logger.logDebug('service', 'ApiService', 'invoke', 'Invoking command', {
      command,
      hasArgs: !!args,
    });
    try {
      const result = await this.tauriApi.invoke<T>(command, args, options);
      this.logger.logDebug('service', 'ApiService', 'invoke', 'Command completed', { command });
      return result;
    } catch (error) {
      this.logger.logError('service', 'ApiService', 'invoke', 'Operation failed', error as Error, {
        command,
        args,
      });
      throw error;
    }
  }

  async listen<T>(event: string, handler: (event: { payload: T }) => void): Promise<() => void> {
    this.logger.logDebug('service', 'ApiService', 'listen', 'Listening to event', { event });
    return this.tauriApi.listen<T>(event, handler);
  }
}
