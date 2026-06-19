import { Injectable, inject } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { Response, getData } from '@entities/response.model';
import { ApiException } from '@entities/error.model';
import { getErrorMessage } from '@shared/utils/error.util';
import { DEFAULT_TIMEOUT_MS } from '@shared/utils/constants';

export interface InvokeOptions {
  suppressError?: boolean;
  timeoutMs?: number;
}

@Injectable({
  providedIn: 'root',
})
export class TauriApiService {
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

  logMessage(level: string, component: string, message: string): void {
    invoke('log_message', { level, component, message }).catch(() => {});
  }
}

export { ApiException } from '@entities/error.model';
export type { Response } from '@entities/response.model';
export { getData } from '@entities/response.model';
