import { Injectable, inject } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { Response, getData } from '@models/response.model';
import { ApiException } from '@models/error.model';

export interface InvokeOptions {
  suppressError?: boolean;
  timeoutMs?: number;
}

const DEFAULT_TIMEOUT_MS = 10000;

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
      if (!options.suppressError) {
        console.error(`Error invoking command "${command}":`, error);
      }
      if (error instanceof ApiException) {
        throw error;
      }
      const message = error instanceof Error ? error.message : String(error);
      throw new ApiException(message, command, error);
    }
  }

  async listen<T>(event: string, handler: (event: { payload: T }) => void): Promise<UnlistenFn> {
    return await listen<T>(event, handler);
  }
}

export { ApiException } from '@models/error.model';
export type { Response } from '@models/response.model';
export { getData, isSuccess, isError } from '@models/response.model';
