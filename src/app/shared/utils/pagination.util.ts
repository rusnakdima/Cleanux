import { WritableSignal } from '@angular/core';
import { PaginatedData } from '@models/system.model';

export interface PaginatedState<T> {
  data: WritableSignal<T[]>;
  hasMore: WritableSignal<boolean>;
  offset: WritableSignal<number>;
  total: WritableSignal<number>;
  loading: WritableSignal<boolean>;
}

export function createPaginatedLoader<T>(
  state: PaginatedState<T>,
  fetcher: (limit: number, offset: number) => Promise<{ data: T[]; has_more: boolean; total: number }>
) {
  return async (limit = 50, offset = 0, reset = false): Promise<PaginatedData<T>> => {
    if (state.loading()) {
      return { data: [], has_more: false, total: state.total() };
    }
    if (reset) {
      state.data.set([]);
      state.offset.set(0);
      state.total.set(0);
    }
    state.loading.set(true);
    try {
      const result = await fetcher(limit, offset);
      state.data.update((current) => (reset ? result.data : [...current, ...result.data]));
      state.hasMore.set(result.has_more);
      state.offset.set(offset + result.data.length);
      state.total.set(result.total);
      return result;
    } finally {
      state.loading.set(false);
    }
  };
}
