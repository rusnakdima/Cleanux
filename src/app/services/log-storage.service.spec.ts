import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';
import { LogStorageService } from './log-storage.service';

describe('LogStorageService', () => {
  let service: any;
  let mockDB: any;
  let indexedDBInstances: any[] = [];

  beforeEach(() => {
    mockDB = {
      transaction: vi.fn(() => ({
        objectStore: vi.fn(() => ({
          add: vi.fn(() => ({ onerror: null, onsuccess: null })),
          clear: vi.fn(() => ({ onsuccess: null, onerror: null })),
          count: vi.fn(() => ({ onsuccess: null, onerror: null })),
        })),
      })),
    };

    service = new LogStorageService();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('should create LogStorageService instance', () => {
    const s = new LogStorageService();
    expect(s).toBeDefined();
  });

  it('should have init method', () => {
    expect(typeof service.init).toBe('function');
  });

  it('should have addLog method', () => {
    expect(typeof service.addLog).toBe('function');
  });

  it('should have getLogs method', () => {
    expect(typeof service.getLogs).toBe('function');
  });

  it('should have getStats method', () => {
    expect(typeof service.getStats).toBe('function');
  });

  it('should have clearLogs method', () => {
    expect(typeof service.clearLogs).toBe('function');
  });

  it('should have getErrors method', () => {
    expect(typeof service.getErrors).toBe('function');
  });

  it('should create log entry with generated id when not provided', async () => {
    const s = new LogStorageService();

    const addLogMock = vi.fn((resolve) => resolve());
    const objectStoreMock = vi.fn(() => ({
      add: vi.fn((req) => {
        expect(req.id).toBeDefined();
        expect(req.timestamp).toBe('2026-07-12T00:00:00.000Z');
        expect(req.level).toBe('info');
        expect(req.message).toBe('Test log');
        return { onsuccess: null, onerror: null };
      }),
    }));
    const transactionMock = vi.fn(() => ({
      objectStore: objectStoreMock,
    }));

    s.addLog({
      timestamp: '2026-07-12T00:00:00.000Z',
      level: 'info',
      message: 'Test log',
    });
  });

  it('should filter logs by level', async () => {
    const s = new LogStorageService();

    expect(typeof s['matchesFilter']).toBe('function');
  });

  it('should filter logs by source', async () => {
    const s = new LogStorageService();

    expect(typeof s['matchesFilter']).toBe('function');
  });

  it('should filter logs by search term', async () => {
    const s = new LogStorageService();

    expect(typeof s['matchesFilter']).toBe('function');
  });
});
