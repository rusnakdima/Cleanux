import { Injectable, signal, inject, OnDestroy, computed, NgZone } from '@angular/core';
import { formatSize } from '@shared/utils/format.util';
import { ApiService } from '@services/api.service';

export interface SystemStats {
  cpuUsage: number;
  memoryUsagePercent: number;
  diskUsage: number;
  memoryUsed?: number;
  memoryTotal?: number;
  diskUsed?: number;
  diskTotal?: number;
  temperature?: number;
}

export interface HealthHistoryEntry {
  timestamp: string;
  healthScore: number;
}

export interface HealthTrend {
  trend: 'improving' | 'declining' | 'stable' | 'insufficient_data';
  changePercent: number;
}

export interface TemperatureInfo {
  sensor_type: 'cpu' | 'gpu' | 'other';
  name: string;
  temperature_celsius: number;
}

const MAX_HISTORY_LENGTH = 20;

@Injectable({
  providedIn: 'root',
})
export class MonitorStore implements OnDestroy {
  private api = inject(ApiService);
  private ngZone = inject(NgZone);
  private unlisten: (() => void) | null = null;
  private unlistenTemps: (() => void) | null = null;
  private statsInterval: number | null = null;
  private tempInterval: number | null = null;
  private readonly pollIntervalMs = 2000;
  private visibilityHandler: ((this: Document, ev: Event) => void) | null = null;

  readonly systemStats = signal<SystemStats>({
    cpuUsage: 0,
    memoryUsagePercent: 0,
    diskUsage: 0,
    memoryUsed: 0,
    memoryTotal: 0,
    diskUsed: 0,
    diskTotal: 0,
  });

  readonly cpuHistory = signal<number[]>(Array(MAX_HISTORY_LENGTH).fill(0));
  readonly memoryHistory = signal<number[]>(Array(MAX_HISTORY_LENGTH).fill(0));
  readonly diskHistory = signal<number[]>(Array(MAX_HISTORY_LENGTH).fill(0));

  readonly cpuTemp = signal<TemperatureInfo | null>(null);
  readonly gpuTemp = signal<TemperatureInfo | null>(null);
  readonly temperaturesLoading = signal(false);

  readonly healthHistory = signal<HealthHistoryEntry[]>([]);
  readonly healthTrend = signal<HealthTrend>({
    trend: 'insufficient_data',
    changePercent: 0,
  });

  readonly loading = signal(false);
  readonly isMonitoring = signal(false);
  readonly error = signal<string | null>(null);

  readonly memoryUsedFormatted = computed(() => formatSize(this.systemStats().memoryUsed ?? 0));
  readonly memoryTotalFormatted = computed(() => formatSize(this.systemStats().memoryTotal ?? 0));
  readonly diskUsedFormatted = computed(() => formatSize(this.systemStats().diskUsed ?? 0));
  readonly diskTotalFormatted = computed(() => formatSize(this.systemStats().diskTotal ?? 0));

  constructor() {
    this.visibilityHandler = () => {
      if (document.hidden) {
        this.stopMonitoring();
      } else {
        this.startMonitoring();
      }
    };
  }

  async startMonitoring(): Promise<void> {
    if (this.isMonitoring()) return;

    this.isMonitoring.set(true);

    // Fire initial fetches without awaiting - let them run in background
    this.fetchSystemStats().catch((e) => this.error.set('Initial stats fetch failed'));
    this.fetchTemperatures().catch((e) => this.error.set('Initial temp fetch failed'));

    this.ngZone.runOutsideAngular(() => {
      this.statsInterval = setInterval(() => {
        this.ngZone.run(() => this.fetchSystemStats());
      }, this.pollIntervalMs);

      this.tempInterval = setInterval(() => {
        this.ngZone.run(() => this.fetchTemperatures());
      }, 3000);
    });

    if (this.visibilityHandler) {
      document.addEventListener('visibilitychange', this.visibilityHandler);
    }
  }

  stopMonitoring(): void {
    if (this.statsInterval) {
      clearInterval(this.statsInterval);
      this.statsInterval = null;
    }
    if (this.tempInterval) {
      clearInterval(this.tempInterval);
      this.tempInterval = null;
    }
    this.isMonitoring.set(false);
  }

  private async fetchSystemStats(): Promise<void> {
    try {
      const response = await this.api.invoke<{
        cpu_usage: number;
        memory_used: number;
        memory_total: number;
        memory_usage_percent: number;
        disk_used: number;
        disk_total: number;
        disk_usage_percent: number;
      }>('get_system_stats', {}, { suppressError: true });

      this.systemStats.set({
        cpuUsage: response.cpu_usage,
        memoryUsagePercent: response.memory_usage_percent,
        diskUsage: response.disk_usage_percent,
        memoryUsed: response.memory_used,
        memoryTotal: response.memory_total,
        diskUsed: response.disk_used,
        diskTotal: response.disk_total,
      });

      this.cpuHistory.update((history) => {
        const newHistory = [...history, response.cpu_usage];
        return newHistory.slice(-MAX_HISTORY_LENGTH);
      });

      this.memoryHistory.update((history) => {
        const newHistory = [...history, response.memory_usage_percent];
        return newHistory.slice(-MAX_HISTORY_LENGTH);
      });

      this.diskHistory.update((history) => {
        const newHistory = [...history, response.disk_usage_percent];
        return newHistory.slice(-MAX_HISTORY_LENGTH);
      });
    } catch (error) {
      this.error.set('Failed to fetch system stats');
    }
  }

  private async fetchTemperatures(): Promise<void> {
    try {
      this.temperaturesLoading.set(true);
      const response = await this.api.invoke<{ cpu_temp?: number; gpu_temp?: number }>(
        'get_temperatures',
        {},
        { suppressError: true }
      );

      if (response && typeof response === 'object' && !Array.isArray(response)) {
        const data = response as { cpu_temp?: number; gpu_temp?: number };
        if (data.cpu_temp !== undefined) {
          this.cpuTemp.set({
            sensor_type: 'cpu',
            name: 'CPU',
            temperature_celsius: data.cpu_temp,
          });
        }
        if (data.gpu_temp !== undefined) {
          this.gpuTemp.set({
            sensor_type: 'gpu',
            name: 'GPU',
            temperature_celsius: data.gpu_temp,
          });
        }
      }
    } catch (error) {
      this.error.set('Failed to fetch temperatures');
    } finally {
      this.temperaturesLoading.set(false);
    }
  }

  async setupScanProgressListener(handler: (progress: number) => void): Promise<void> {
    this.unlisten = await this.api.listen<{ progress: number }>('scan-progress', (event) => {
      handler(event.payload.progress * 100);
    });
  }

  async loadHealthHistory(days: number): Promise<void> {
    try {
      const history = await this.api.invoke<Array<{ timestamp: string; health_score: number }>>(
        'get_health_history',
        { days }
      );
      this.healthHistory.set(
        history.map((h) => ({ timestamp: h.timestamp, healthScore: h.health_score }))
      );
    } catch (error) {
      this.error.set('Failed to load health history');
    }
  }

  async loadHealthTrends(days: number): Promise<void> {
    try {
      const trend = await this.api.invoke<{ trend: string; change_percent: number }>(
        'get_health_trends',
        { days }
      );
      this.healthTrend.set({
        trend: trend.trend as HealthTrend['trend'],
        changePercent: trend.change_percent,
      });
    } catch (error) {
      this.error.set('Failed to load health trends');
    }
  }

  async saveHealthSnapshot(
    healthScore: number,
    cacheSize: number,
    trashSize: number,
    logSize: number,
    largeFilesCount: number
  ): Promise<void> {
    try {
      await this.api.invoke<{ id: number }>('save_health_snapshot', {
        health_score: healthScore,
        cache_size: cacheSize,
        trash_size: trashSize,
        log_size: logSize,
        large_files_count: largeFilesCount,
      });
    } catch (error) {
      this.error.set('Failed to save health snapshot');
    }
  }

  getUsageColor(percent: number): string {
    if (percent >= 90) return 'var(--color-error)';
    if (percent >= 70) return 'var(--color-warning)';
    return 'var(--color-success)';
  }

  ngOnDestroy(): void {
    this.stopMonitoring();
    if (this.unlisten) {
      this.unlisten();
      this.unlisten = null;
    }
    if (this.unlistenTemps) {
      this.unlistenTemps();
      this.unlistenTemps = null;
    }
    if (this.visibilityHandler) {
      document.removeEventListener('visibilitychange', this.visibilityHandler);
      this.visibilityHandler = null;
    }
  }
}
