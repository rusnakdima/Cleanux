/* sys lib */
import {
  ChangeDetectionStrategy,
  Component,
  signal,
  computed,
  inject,
  OnInit,
  OnDestroy,
} from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';

/* router */
import { RouterLink } from '@angular/router';

/* services */
import { FileService, ScanSummary } from '@services/file.service';
import { ApiService } from '@services/api.service';
import { HealthHistoryService } from '@services/health-history.service';
import { ThemeService } from '@services/theme.service';
import { MonitorStore } from '@store/monitor.store';
import { formatSize } from '@shared/utils/format.util';
import {
  CRITICAL_JUNK_SIZE,
  WARNING_JUNK_SIZE,
  HEALTHY_SCORE,
  WARNING_SCORE,
  CRITICAL_SCORE,
} from '@shared/constants/size.constants';
import { SCAN_COMPLETE_TIMEOUT_MS } from '@shared/constants/timeout.constants';

/* types */
interface QuickAction {
  id: string;
  label: string;
  icon: string;
  action: string;
}

interface ActivityItem {
  id: string;
  type: string;
  description: string;
  timestamp: string;
  size?: number;
}

interface ScanProgress {
  phase: string;
  progress: number;
  files_scanned: number;
  current_path: string;
}

@Component({
  selector: 'app-dashboard',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, RouterLink],
  templateUrl: './dashboard.view.html',
})
export class DashboardView implements OnInit, OnDestroy {
  private fileService = inject(FileService);
  private api = inject(ApiService);
  private healthHistoryService = inject(HealthHistoryService);
  private themeService = inject(ThemeService);
  protected monitorStore = inject(MonitorStore);

  private unlisten: (() => void) | null = null;

  formatSize = formatSize;

  isScanning = signal(false);
  isCleaning = signal(false);
  scanProgress = signal(0);

  cacheSize = signal(0);
  trashSize = signal(0);
  logSize = signal(0);
  largeFileSize = signal(0);
  largeFilesCount = signal(0);
  cacheCount = signal(0);
  trashCount = signal(0);
  logCount = signal(0);

  healthHistoryDays = signal<7 | 30>(7);
  healthHistory = signal<Array<{ timestamp: string; health_score: number }>>([]);
  healthTrend = signal<{ trend: string; change_percent: number }>({
    trend: 'insufficient_data',
    change_percent: 0,
  });

  recentActivities = signal<ActivityItem[]>([]);

  totalJunkSize = computed(
    () => this.cacheSize() + this.trashSize() + this.logSize() + this.largeFileSize()
  );

  systemStatus = computed(() => {
    const size = this.totalJunkSize();
    if (size > CRITICAL_JUNK_SIZE) return 'Critical';
    if (size > WARNING_JUNK_SIZE) return 'Needs Attention';
    return 'Healthy';
  });

  healthScore = computed(() => {
    const size = this.totalJunkSize();
    if (size > CRITICAL_JUNK_SIZE) return CRITICAL_SCORE;
    if (size > WARNING_JUNK_SIZE) return WARNING_SCORE;
    return HEALTHY_SCORE;
  });

  ngOnInit() {
    this.calculateJunkSize();
    this.loadHealthHistory();
    this.loadRecentActivities();
    this.setupScanProgressListener();
  }

  async setupScanProgressListener() {
    this.unlisten = await this.api.listen('scan-progress', (event) => {
      const payload = event.payload as ScanProgress;
      this.scanProgress.set(payload.progress * 100);
    });
  }

  ngOnDestroy() {
    if (this.unlisten) {
      this.unlisten();
      this.unlisten = null;
    }
  }

  loadRecentActivities() {
    try {
      const stored = localStorage.getItem('cleanux_recent_activities');
      if (stored) {
        const activities = JSON.parse(stored) as ActivityItem[];
        this.recentActivities.set(activities.slice(0, 10));
      }
    } catch (e) {
      this.monitorStore.error.set('Failed to load recent activities');
    }
  }

  addRecentActivity(type: ActivityItem['type'], description: string, size?: number) {
    const newActivity: ActivityItem = {
      id: Date.now().toString(),
      type,
      description,
      timestamp: new Date().toLocaleString(),
      size,
    };
    const updated = [newActivity, ...this.recentActivities()].slice(0, 10);
    this.recentActivities.set(updated);
    try {
      localStorage.setItem('cleanux_recent_activities', JSON.stringify(updated));
    } catch (e) {
      this.monitorStore.error.set('Failed to save recent activities');
    }
  }

  async handleQuickAction(action: string) {
    switch (action) {
      case 'scan':
        this.startScan();
        break;
      case 'clean-cache':
        await this.fileService.clearCache();
        this.addRecentActivity('cache', 'Cache cleared', this.cacheSize());
        this.calculateJunkSize();
        break;
      case 'clean-trash':
        await this.fileService.clearTrash();
        this.addRecentActivity('trash', 'Trash emptied', this.trashSize());
        this.calculateJunkSize();
        break;
      case 'clean-logs':
        await this.fileService.clearAllLogs();
        this.addRecentActivity('logs', 'Logs cleared', this.logSize());
        this.calculateJunkSize();
        break;
    }
  }

  async loadHealthHistory() {
    try {
      const [history, trend] = await Promise.all([
        this.healthHistoryService.getHealthHistory(this.healthHistoryDays()),
        this.healthHistoryService.getHealthTrends(this.healthHistoryDays()),
      ]);
      this.healthHistory.set(
        history.map((h) => ({ timestamp: h.timestamp, health_score: h.health_score }))
      );
      this.healthTrend.set({ trend: trend.trend, change_percent: trend.change_percent });
    } catch (e) {
      this.monitorStore.error.set('Failed to load health history');
    }
  }

  async saveHealthSnapshot() {
    try {
      await this.healthHistoryService.saveHealthSnapshot(
        this.healthScore(),
        this.cacheSize(),
        this.trashSize(),
        this.logSize(),
        this.largeFilesCount()
      );
    } catch (e) {
      this.monitorStore.error.set('Failed to save health snapshot');
    }
  }

  async calculateJunkSize() {
    try {
      const [cacheRes, trashRes, logRes, largeFilesRes] = await Promise.all([
        this.fileService.getCacheSummary(),
        this.fileService.getTrashSummary(),
        this.fileService.getLogSummary(),
        this.fileService.getLargeFilesSummary(),
      ]);
      this.cacheSize.set(cacheRes.totalSize);
      this.cacheCount.set(cacheRes.fileCount);
      this.trashSize.set(trashRes.totalSize);
      this.trashCount.set(trashRes.fileCount);
      this.logSize.set(logRes.totalSize);
      this.logCount.set(logRes.fileCount);
      this.largeFileSize.set(largeFilesRes.totalSize);
      this.largeFilesCount.set(largeFilesRes.fileCount);
    } catch (error) {
      this.monitorStore.error.set('Failed to calculate junk size');
    }
  }

  async startScan() {
    if (this.isScanning()) return;

    this.isScanning.set(true);
    this.scanProgress.set(0);
    this.addRecentActivity('scan', 'System scan started');

    const interval = setInterval(() => {
      this.scanProgress.update((p) => {
        if (p >= 90) {
          return p;
        }
        return p + Math.random() * 10;
      });
    }, 100);

    await this.calculateJunkSize();

    clearInterval(interval);
    this.scanProgress.set(100);
    await this.saveHealthSnapshot();
    this.loadHealthHistory();
    this.addRecentActivity('scan', 'System scan completed', this.totalJunkSize());
    setTimeout(() => this.isScanning.set(false), SCAN_COMPLETE_TIMEOUT_MS);
  }

  async cleanAll() {
    if (this.isCleaning() || this.totalJunkSize() === 0) return;

    this.isCleaning.set(true);
    const cleanedSize = this.totalJunkSize();
    try {
      await Promise.all([
        this.fileService.clearCache(),
        this.fileService.clearTrash(),
        this.fileService.clearAllLogs(),
        this.fileService.clearAllLargeFiles(),
      ]);

      this.cacheSize.set(0);
      this.trashSize.set(0);
      this.logSize.set(0);
      this.largeFileSize.set(0);

      await this.calculateJunkSize();
      await this.saveHealthSnapshot();
      this.loadHealthHistory();
      this.addRecentActivity('clean', `Cleaned ${this.formatSize(cleanedSize)}`, cleanedSize);
    } catch (error) {
      this.monitorStore.error.set('Failed to clean system');
    } finally {
      this.isCleaning.set(false);
    }
  }

  getChartPath(): string {
    const history = this.healthHistory();
    if (history.length < 2) return '';

    const width = 300;
    const height = 100;
    const padding = 10;
    const chartWidth = width - 2 * padding;
    const chartHeight = height - 2 * padding;

    const scores = history.map((h) => h.health_score);
    const min = Math.min(...scores, 0);
    const max = Math.max(...scores, 100);
    const range = max - min || 1;

    const points = history.map((h, i) => {
      const x = padding + (i / (history.length - 1)) * chartWidth;
      const y = padding + chartHeight - ((h.health_score - min) / range) * chartHeight;
      return `${x},${y}`;
    });

    return `M ${points.join(' L ')}`;
  }

  setHealthHistoryDays(days: 7 | 30) {
    this.healthHistoryDays.set(days);
    this.loadHealthHistory();
  }

  getHealthMessage(): string {
    const score = this.healthScore();
    if (score >= 80) return 'Your system is in great condition!';
    if (score >= 50) return 'Consider running a cleanup to improve performance.';
    return 'System needs attention. Run a deep clean recommended.';
  }

  getAccentGradient(): string {
    return this.themeService.getAccentGradient();
  }
}
