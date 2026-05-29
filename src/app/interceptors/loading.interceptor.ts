import { Injectable, signal, computed } from '@angular/core';

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

  startRequest(command: string): string {
    const id = `${command}_${++this.requestCounter}`;
    this.pendingRequests.update((requests) => {
      const newRequests = new Map(requests);
      newRequests.set(id, { id, command, startedAt: new Date() });
      return newRequests;
    });
    return id;
  }

  endRequest(id: string): void {
    this.pendingRequests.update((requests) => {
      const newRequests = new Map(requests);
      newRequests.delete(id);
      return newRequests;
    });
  }

  clear(): void {
    this.pendingRequests.set(new Map());
  }
}
