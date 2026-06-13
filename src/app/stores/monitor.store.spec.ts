import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';
import { Injector, runInInjectionContext, NgZone } from '@angular/core';
import { formatSize } from '@shared/utils/format.util';

vi.mock('@api/tauri-api.service');

describe('MonitorStore', () => {
  let injector: Injector;
  let mockApi: { invoke: ReturnType<typeof vi.fn>; listen: ReturnType<typeof vi.fn> };

  const createMockSystemStats = () => ({
    cpu_usage: 45.5,
    memory_used: 8 * 1024 * 1024 * 1024,
    memory_total: 16 * 1024 * 1024 * 1024,
    memory_usage_percent: 50.0,
    disk_used: 500 * 1024 * 1024 * 1024,
    disk_total: 1000 * 1024 * 1024 * 1024,
    disk_usage_percent: 50.0,
  });

  beforeEach(async () => {
    const { TauriApiService } = await import('@api/tauri-api.service');
    mockApi = { invoke: vi.fn(), listen: vi.fn().mockResolvedValue(() => {}) };
    TauriApiService.prototype.api = mockApi;

    const mockNgZone = {
      run: (fn: () => void) => fn(),
      runOutsideAngular: (fn: () => void) => fn(),
    };

    injector = Injector.create({
      providers: [
        { provide: TauriApiService, useValue: mockApi },
        { provide: NgZone, useValue: mockNgZone },
      ],
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('should have correct initial values', async () => {
    const { MonitorStore } = await import('./monitor.store');
    const store = runInInjectionContext(injector, () => new MonitorStore());

    expect(store.systemStats().cpuUsage).toBe(0);
    expect(store.systemStats().memoryUsagePercent).toBe(0);
    expect(store.systemStats().diskUsage).toBe(0);
    expect(store.cpuHistory()).toHaveLength(20);
    expect(store.memoryHistory()).toHaveLength(20);
    expect(store.diskHistory()).toHaveLength(20);
    expect(store.isMonitoring()).toBe(false);
    expect(store.loading()).toBe(false);

    store.ngOnDestroy();
  });

  it('should format bytes correctly', async () => {
    expect(formatSize(0)).toBe('0 B');
    expect(formatSize(1024)).toBe('1 KB');
    expect(formatSize(1024 * 1024)).toBe('1 MB');
    expect(formatSize(1024 * 1024 * 1024)).toBe('1 GB');
    expect(formatSize(1.5 * 1024 * 1024 * 1024)).toBe('1.5 GB');
  });

  it('should return success for low usage', async () => {
    const { MonitorStore } = await import('./monitor.store');
    const store = runInInjectionContext(injector, () => new MonitorStore());

    expect(store.getUsageColor(50)).toBe('var(--color-success)');
    expect(store.getUsageColor(69)).toBe('var(--color-success)');

    store.ngOnDestroy();
  });

  it('should return warning for medium usage', async () => {
    const { MonitorStore } = await import('./monitor.store');
    const store = runInInjectionContext(injector, () => new MonitorStore());

    expect(store.getUsageColor(70)).toBe('var(--color-warning)');
    expect(store.getUsageColor(89)).toBe('var(--color-warning)');

    store.ngOnDestroy();
  });

  it('should return error for high usage', async () => {
    const { MonitorStore } = await import('./monitor.store');
    const store = runInInjectionContext(injector, () => new MonitorStore());

    expect(store.getUsageColor(90)).toBe('var(--color-error)');
    expect(store.getUsageColor(100)).toBe('var(--color-error)');

    store.ngOnDestroy();
  });

  it('should update systemStats and history', async () => {
    const { MonitorStore } = await import('./monitor.store');
    const store = runInInjectionContext(injector, () => new MonitorStore());
    const mockStats = createMockSystemStats();
    mockApi.invoke.mockResolvedValue(mockStats);

    await store.fetchSystemStats();

    expect(store.systemStats().cpuUsage).toBe(45.5);
    expect(store.systemStats().memoryUsagePercent).toBe(50.0);
    expect(store.systemStats().diskUsage).toBe(50.0);

    store.ngOnDestroy();
  });

  it('should handle errors gracefully', async () => {
    const { MonitorStore } = await import('./monitor.store');
    const store = runInInjectionContext(injector, () => new MonitorStore());
    mockApi.invoke.mockRejectedValue(new Error('Failed'));

    await store.fetchSystemStats();

    expect(store.systemStats().cpuUsage).toBe(0);

    store.ngOnDestroy();
  });

  it('should format memory correctly', async () => {
    const { MonitorStore } = await import('./monitor.store');
    const store = runInInjectionContext(injector, () => new MonitorStore());

    store.systemStats.set({
      cpuUsage: 0,
      memoryUsagePercent: 50,
      diskUsage: 50,
      memoryUsed: 8 * 1024 * 1024 * 1024,
      memoryTotal: 16 * 1024 * 1024 * 1024,
      diskUsed: 0,
      diskTotal: 0,
    });

    expect(store.memoryUsedFormatted()).toBe('8 GB');
    expect(store.memoryTotalFormatted()).toBe('16 GB');

    store.ngOnDestroy();
  });

  it('should format disk correctly', async () => {
    const { MonitorStore } = await import('./monitor.store');
    const store = runInInjectionContext(injector, () => new MonitorStore());

    store.systemStats.set({
      cpuUsage: 0,
      memoryUsagePercent: 50,
      diskUsage: 50,
      memoryUsed: 0,
      memoryTotal: 0,
      diskUsed: 500 * 1024 * 1024 * 1024,
      diskTotal: 1024 * 1024 * 1024 * 1024,
    });

    expect(store.diskUsedFormatted()).toBe('500 GB');
    expect(store.diskTotalFormatted()).toBe('1 TB');

    store.ngOnDestroy();
  });

  it('should call get_health_history', async () => {
    const { MonitorStore } = await import('./monitor.store');
    const store = runInInjectionContext(injector, () => new MonitorStore());
    const mockHistory = [
      { timestamp: '2024-01-01T00:00:00Z', health_score: 85 },
      { timestamp: '2024-01-02T00:00:00Z', health_score: 87 },
    ];
    mockApi.invoke.mockResolvedValue(mockHistory);

    await store.loadHealthHistory(7);

    expect(mockApi.invoke).toHaveBeenCalledWith('get_health_history', { days: 7 });
    expect(store.healthHistory()).toHaveLength(2);
    expect(store.healthHistory()[0].healthScore).toBe(85);

    store.ngOnDestroy();
  });

  it('should call get_health_trends', async () => {
    const { MonitorStore } = await import('./monitor.store');
    const store = runInInjectionContext(injector, () => new MonitorStore());
    const mockTrend = { trend: 'improving', change_percent: 5.2 };
    mockApi.invoke.mockResolvedValue(mockTrend);

    await store.loadHealthTrends(30);

    expect(mockApi.invoke).toHaveBeenCalledWith('get_health_trends', { days: 30 });
    expect(store.healthTrend().trend).toBe('improving');
    expect(store.healthTrend().changePercent).toBe(5.2);

    store.ngOnDestroy();
  });

  it('should call save_health_snapshot', async () => {
    const { MonitorStore } = await import('./monitor.store');
    const store = runInInjectionContext(injector, () => new MonitorStore());
    mockApi.invoke.mockResolvedValue({ id: 1 });

    await store.saveHealthSnapshot(85, 1024, 512, 256, 10);

    expect(mockApi.invoke).toHaveBeenCalledWith('save_health_snapshot', {
      health_score: 85,
      cache_size: 1024,
      trash_size: 512,
      log_size: 256,
      large_files_count: 10,
    });

    store.ngOnDestroy();
  });
});
