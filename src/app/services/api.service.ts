/* sys lib */
import { Injectable, inject } from '@angular/core';

/* api */
import { TauriApiService, ApiException } from '@api/tauri-api.service';

/* models */
import { Response, getData } from '@models/response.model';

export { ApiException } from '@models/error.model';

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
    return this.tauriApi.invoke<T>(command, args, options);
  }

  async listen<T>(event: string, handler: (event: { payload: T }) => void): Promise<() => void> {
    return this.tauriApi.listen<T>(event, handler);
  }
}
