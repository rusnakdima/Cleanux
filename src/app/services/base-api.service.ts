import { Injectable, inject } from '@angular/core';
import { InvokeWrapperService } from '@tauri-front/shared';

@Injectable({
  providedIn: 'root',
})
export class BaseApiService {
  protected api = inject(InvokeWrapperService);

  protected call<R>(command: string, args?: Record<string, unknown>): Promise<R> {
    return this.api.invoke<R>(command, args);
  }
}
