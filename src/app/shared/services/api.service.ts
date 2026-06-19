/* sys lib */
import { Injectable, inject } from '@angular/core';

/* api */
import { TauriApiService, ApiException } from '@api/tauri-api.service';

/* models */
import { Response, getData } from '@entities/response.model';

export { ApiException } from '@entities/error.model';

@Injectable({
  providedIn: 'root',
})
export class ApiService {
  private tauriApi = inject(TauriApiService);

  async invoke<T>(
    command: string,
    args?: Record<string, unknown>,
    options: { suppressError?: boolean } = {}
  ): Promise<T> {
    try {
      const result = await this.tauriApi.invoke<T>(command, args, options);
      return result;
    } catch (error) {
      throw error;
    }
  }

  async listen<T>(event: string, handler: (event: { payload: T }) => void): Promise<() => void> {
    return this.tauriApi.listen<T>(event, handler);
  }
}
