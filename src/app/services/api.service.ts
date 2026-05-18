/* sys lib */
import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';

/* models */
import { Response, getData } from '@models/response.model';
import { ApiException } from '@models/error.model';

@Injectable({
  providedIn: 'root',
})
export class ApiService {
  constructor() {}

  async invoke<T>(
    command: string,
    args?: Record<string, unknown>,
    options: { suppressError?: boolean } = {}
  ): Promise<T> {
    try {
      const response = await invoke<Response>(command, args);
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
}
