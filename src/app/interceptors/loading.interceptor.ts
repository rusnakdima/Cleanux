import { Injectable, inject, signal, computed } from '@angular/core';
import { LoggerService } from '@services/logger.service';

export interface PendingRequest {
  id: string;
  command: string;
  startedAt: Date;
}

@Injectable({
  providedIn: 'root',
})
export class LoadingInterceptorService {
  private requestCounter = 0;
  private pendingRequests = signal<Map<string, PendingRequest>>(new Map());

  readonly pending = computed(() => this.pendingRequests().size);
  readonly isLoading = computed(() => this.pendingRequests().size > 0);

  constructor(private logger: LoggerService) {
    this.logger.logInfo('api', 'LoadingInterceptor', 'init', 'LoadingInterceptor initialized');
  }

  startRequest(command: string): string {
    const id = `${command}_${++this.requestCounter}`;
    this.pendingRequests.update((requests) => {
      const newRequests = new Map(requests);
      newRequests.set(id, { id, command, startedAt: new Date() });
      return newRequests;
    });
    this.logger.logInfo('api', 'LoadingInterceptor', 'requestStart', 'Request started', {
      command,
      id,
    });
    return id;
  }

  endRequest(id: string): void {
    const request = this.pendingRequests().get(id);
    const duration = request ? Date.now() - request.startedAt.getTime() : 0;
    this.pendingRequests.update((requests) => {
      const newRequests = new Map(requests);
      newRequests.delete(id);
      return newRequests;
    });
    this.logger.logInfo('api', 'LoadingInterceptor', 'requestEnd', 'Request ended', {
      id,
      duration,
    });
  }

  clear(): void {
    this.pendingRequests.set(new Map());
  }
}
