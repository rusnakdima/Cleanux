import { Injectable, inject, signal, computed } from '@angular/core';
import { StorageCacheService } from '@app/core/services/storage-cache.service';

export interface AppState {
  initialized: boolean;
  loading: boolean;
  error: string | null;
}

@Injectable({
  providedIn: 'root',
})
export class MainService {
  private cache = inject(StorageCacheService);

  private state = signal<AppState>({
    initialized: false,
    loading: false,
    error: null,
  });

  readonly initialized = computed(() => this.state().initialized);
  readonly loading = computed(() => this.state().loading);
  readonly error = computed(() => this.state().error);

  async initialize(): Promise<void> {
    if (this.state().initialized) return;

    this.state.update((s) => ({ ...s, loading: true, error: null }));

    try {
      this.cache.set('app_initialized', true);
      this.state.update((s) => ({ ...s, initialized: true, loading: false }));
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Initialization failed';
      this.state.update((s) => ({ ...s, loading: false, error: message }));
      throw error;
    }
  }

  reset(): void {
    this.state.set({
      initialized: false,
      loading: false,
      error: null,
    });
    this.cache.clear();
  }
}
