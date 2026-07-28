import { afterEach } from 'vitest';

/* Mock indexedDB for LogStorageService tests in Node environment */
if (typeof indexedDB === 'undefined') {
  const mockDB: any = {
    transaction: () => ({
      objectStore: () => ({
        add: () => ({ onsuccess: null, onerror: null }),
        clear: () => ({ onsuccess: null, onerror: null }),
        count: () => ({ onsuccess: null, onerror: null, result: 0 }),
      }),
    }),
  };
  (globalThis as any).indexedDB = {
    open: () => ({ onerror: null, onsuccess: null, result: mockDB }),
  };
}
