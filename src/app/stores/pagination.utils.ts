import { signal, computed, WritableSignal } from '@angular/core';

export interface PaginatedState<T> {
  data: WritableSignal<T[]>;
  hasMore: WritableSignal<boolean>;
  offset: WritableSignal<number>;
  total: WritableSignal<number>;
  loading: WritableSignal<boolean>;
}

export function createPaginationState<T>(initialData: T[] = []): PaginatedState<T> {
  return {
    data: signal<T[]>(initialData),
    hasMore: signal(false),
    offset: signal(0),
    total: signal(0),
    loading: signal(false),
  };
}

export function createPaginatedLoader<T>(
  state: PaginatedState<T>,
  fetcher: (limit: number, offset: number) => Promise<{ data: T[]; has_more: boolean; total: number }>
) {
  return async (limit = 50, offset = 0, reset = false) => {
    if (state.loading()) return { data: [], has_more: false, total: state.total() };
    if (reset) {
      state.data.set([]);
      state.offset.set(0);
      state.total.set(0);
    }
    state.loading.set(true);
    try {
      const result = await fetcher(limit, offset);
      state.data.update(current => reset ? result.data : [...current, ...result.data]);
      state.hasMore.set(result.has_more);
      state.offset.set(offset + result.data.length);
      state.total.set(result.total);
      return result;
    } finally {
      state.loading.set(false);
    }
  };
}