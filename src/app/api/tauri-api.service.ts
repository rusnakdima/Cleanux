import { Injectable, inject } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { Response, getData } from '@models/response.model';
import { ApiException } from '@models/error.model';
import { getErrorMessage } from '@shared/utils/error.util';
import { LoggerService } from '@services/logger.service';
import { DEFAULT_TIMEOUT_MS } from '@shared/utils/constants';

export interface InvokeOptions {
  suppressError?: boolean;
  timeoutMs?: number;
}

@Injectable({
  providedIn: 'root',
})
export class TauriApiService {
  private logger = inject(LoggerService);

  async invoke<T>(
    command: string,
    args?: Record<string, unknown>,
    options: InvokeOptions = {}
  ): Promise<T> {
    const timeoutMs = options.timeoutMs ?? DEFAULT_TIMEOUT_MS;

    try {
      const response = await Promise.race([
        invoke<Response>(command, args),
        new Promise<never>((_, reject) =>
          setTimeout(
            () => reject(new Error(`Command "${command}" timed out after ${timeoutMs}ms`)),
            timeoutMs
          )
        ),
      ]);

      if (response.status === 'success') {
        return getData<T>(response) as T;
      } else {
        throw new ApiException(response.message || `Operation failed: ${command}`, command);
      }
    } catch (error: unknown) {
      if (!options.suppressError) {
        const err = error instanceof Error ? error : new Error(String(error));
        this.logger.logError(
          'api',
          undefined,
          command,
          `Error invoking command "${command}"`,
          err,
          { args }
        );
      }
      if (error instanceof ApiException) {
        throw error;
      }
      const message = getErrorMessage(error);
      throw new ApiException(message, command, error);
    }
  }

  async listen<T>(event: string, handler: (event: { payload: T }) => void): Promise<UnlistenFn> {
    return await listen<T>(event, handler);
  }
}

export { ApiException } from '@models/error.model';
export type { Response } from '@models/response.model';
export { getData } from '@models/response.model';
