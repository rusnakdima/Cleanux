import { Injectable, inject } from '@angular/core';
import { ApiService } from '@services/api.service';

@Injectable({
  providedIn: 'root',
})
export class BaseApiService {
  protected api = inject(ApiService);

  protected call<R>(command: string, args?: Record<string, unknown>): Promise<R> {
    return this.api.invoke<R>(command, args);
  }
}