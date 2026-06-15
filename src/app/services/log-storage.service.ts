import { Injectable, inject } from '@angular/core';
import { LogEntry, LogFilter, LoggerService } from './logger.service';

const DB_NAME = 'cleanux_logs';
const DB_VERSION = 1;
const STORE_NAME = 'logs';
const MAX_ENTRIES = 10000;

interface StoredLog {
  id: string;
  timestamp: string;
  level: string;
  source: string;
  message: string;
  data?: unknown;
  error?: { message: string; stack?: string };
}

@Injectable({
  providedIn: 'root',
})
export class LogStorageService {
  private db: IDBDatabase | null = null;
  private initPromise: Promise<void> | null = null;
  private logger = inject(LoggerService);

  async init(): Promise<void> {
    if (this.db) return;
    if (this.initPromise) return this.initPromise;

    this.initPromise = new Promise((resolve, reject) => {
      const request = indexedDB.open(DB_NAME, DB_VERSION);

      request.onerror = () => {
        this.logger.logError(
          'service',
          undefined,
          'init',
          '[LogStorage] Failed to open IndexedDB',
          request.error as Error
        );
        reject(request.error);
      };

      request.onsuccess = () => {
        this.db = request.result;
        this.logger.logDebug('service', undefined, 'init', '[LogStorage] IndexedDB initialized');
        resolve();
      };

      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;
        if (!db.objectStoreNames.contains(STORE_NAME)) {
          const store = db.createObjectStore(STORE_NAME, { keyPath: 'id' });
          store.createIndex('timestamp', 'timestamp', { unique: false });
          store.createIndex('level', 'level', { unique: false });
          store.createIndex('source', 'source', { unique: false });
          store.createIndex('view', 'view', { unique: false });
        }
        this.logger.logDebug('service', undefined, 'init', '[LogStorage] IndexedDB schema created');
      };
    });

    return this.initPromise;
  }

  private async ensureDb(): Promise<IDBDatabase> {
    await this.init();
    if (!this.db) {
      throw new Error('IndexedDB not initialized');
    }
    return this.db;
  }

  async addLog(entry: LogEntry): Promise<void> {
    const db = await this.ensureDb();

    const stored: StoredLog = {
      id: entry.id,
      timestamp: entry.timestamp,
      level: entry.level,
      source: entry.source || '',
      message: entry.message,
      data: entry.data,
      error: entry.error as { message: string; stack?: string } | undefined,
    };

    return new Promise((resolve, reject) => {
      const tx = db.transaction(STORE_NAME, 'readwrite');
      const store = tx.objectStore(STORE_NAME);
      const request = store.add(stored);

      request.onerror = () => reject(request.error);
      request.onsuccess = () => {
        this.cleanupIfNeeded(tx);
        resolve();
      };
    });
  }

  private async cleanupIfNeeded(tx: IDBTransaction): Promise<void> {
    return new Promise((resolve, reject) => {
      const store = tx.objectStore(STORE_NAME);
      const countRequest = store.count();

      countRequest.onsuccess = () => {
        const count = countRequest.result;
        if (count > MAX_ENTRIES) {
          this.removeOldest(count - MAX_ENTRIES)
            .then(resolve)
            .catch(reject);
        } else {
          resolve();
        }
      };
      countRequest.onerror = () => reject(countRequest.error);
    });
  }

  private async removeOldest(count: number): Promise<void> {
    const db = await this.ensureDb();

    return new Promise((resolve, reject) => {
      const tx = db.transaction(STORE_NAME, 'readwrite');
      const store = tx.objectStore(STORE_NAME);
      const index = store.index('timestamp');
      const cursorRequest = index.openCursor();
      let deleted = 0;

      cursorRequest.onsuccess = (event) => {
        const cursor = (event.target as IDBRequest<IDBCursorWithValue>).result;
        if (cursor && deleted < count) {
          cursor.delete();
          deleted++;
          cursor.continue();
        } else {
          this.logger.logDebug(
            'service',
            undefined,
            'cleanup',
            `[LogStorage] Cleaned up ${deleted} old log entries`
          );
          resolve();
        }
      };
      cursorRequest.onerror = () => reject(cursorRequest.error);
    });
  }

  async getLogs(filter?: LogFilter, limit: number = 1000): Promise<StoredLog[]> {
    const db = await this.ensureDb();

    return new Promise((resolve, reject) => {
      const tx = db.transaction(STORE_NAME, 'readonly');
      const store = tx.objectStore(STORE_NAME);
      const index = store.index('timestamp');
      const logs: StoredLog[] = [];
      let cursorRequest: IDBRequest<IDBCursorWithValue | null>;

      if (filter?.since || filter?.until) {
        const range = IDBKeyRange.bound(
          filter.since?.toISOString() || '',
          filter.until?.toISOString() || '\uffff'
        );
        cursorRequest = index.openCursor(range);
      } else {
        cursorRequest = index.openCursor(null, 'prev');
      }

      cursorRequest.onsuccess = (event) => {
        const cursor = (event.target as IDBRequest<IDBCursorWithValue>).result;
        if (cursor && logs.length < limit) {
          const log = cursor.value as StoredLog;

          if (this.matchesFilter(log, filter)) {
            logs.push(log);
          }
          cursor.continue();
        } else {
          resolve(logs);
        }
      };
      cursorRequest.onerror = () => reject(cursorRequest.error);
    });
  }

  private matchesFilter(log: StoredLog, filter?: LogFilter): boolean {
    if (!filter) return true;

    if (filter.level && log.level !== filter.level) return false;
    if (filter.source && log.source !== filter.source) return false;

    if (filter.search) {
      const search = filter.search.toLowerCase();
      if (!log.message.toLowerCase().includes(search)) {
        return false;
      }
    }

    return true;
  }

  async getStats(): Promise<{
    total: number;
    byLevel: Record<string, number>;
    bySource: Record<string, number>;
    oldest: string | null;
    newest: string | null;
  }> {
    const db = await this.ensureDb();

    return new Promise((resolve, reject) => {
      const tx = db.transaction(STORE_NAME, 'readonly');
      const store = tx.objectStore(STORE_NAME);
      const countRequest = store.count();
      const index = store.index('timestamp');
      const firstRequest = index.openCursor(null, 'next');
      const lastRequest = index.openCursor(null, 'prev');

      let total = 0;
      const byLevel: Record<string, number> = {};
      const bySource: Record<string, number> = {};
      let oldest: string | null = null;
      let newest: string | null = null;

      countRequest.onsuccess = () => {
        total = countRequest.result;
      };
      countRequest.onerror = () => reject(countRequest.error);

      firstRequest.onsuccess = () => {
        const cursor = firstRequest.result;
        if (cursor) {
          oldest = cursor.value.timestamp;
        }
      };

      lastRequest.onsuccess = () => {
        const cursor = lastRequest.result;
        if (cursor) {
          newest = cursor.value.timestamp;
        }
        resolve({ total, byLevel, bySource, oldest, newest });
      };
      lastRequest.onerror = () => reject(lastRequest.error);
    });
  }

  async clearLogs(): Promise<void> {
    const db = await this.ensureDb();

    return new Promise((resolve, reject) => {
      const tx = db.transaction(STORE_NAME, 'readwrite');
      const store = tx.objectStore(STORE_NAME);
      const request = store.clear();

      request.onsuccess = () => {
        this.logger.logDebug('service', undefined, 'clearLogs', '[LogStorage] All logs cleared');
        resolve();
      };
      request.onerror = () => reject(request.error);
    });
  }

  async getErrors(limit: number = 100): Promise<StoredLog[]> {
    return this.getLogs({ level: 'error' } as LogFilter, limit);
  }
}
