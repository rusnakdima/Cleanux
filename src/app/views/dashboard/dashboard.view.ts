/* sys lib */
import {
  ChangeDetectionStrategy,
  Component,
  signal,
  computed,
  inject,
  OnInit,
} from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';

/* services */
import { FileService, ScanSummary } from '@services/file.service';
import { SystemService } from '@services/system.service';
import { HealthHistoryService } from '@services/health-history.service';
import { SystemMonitorComponent } from '@components/system-monitor/system-monitor.component';
import {
  WidgetContainerComponent,
  WidgetConfig,
} from '@components/widget-container/widget-container.component';
import { WidgetHealthScoreComponent } from '@components/widgets/widget-health-score/widget-health-score.component';
import {
  WidgetQuickActionsComponent,
  QuickAction,
} from '@components/widgets/widget-quick-actions/widget-quick-actions.component';
import { WidgetDiskUsageComponent } from '@components/widgets/widget-disk-usage/widget-disk-usage.component';
import {
  WidgetRecentActivityComponent,
  ActivityItem,
} from '@components/widgets/widget-recent-activity/widget-recent-activity.component';

interface WidgetState {
  id: string;
  title: string;
  icon: string;
  enabled: boolean;
  order: number;
}

const WIDGET_STORAGE_KEY = 'cleanux_widget_layout';

@Component({
  selector: 'app-dashboard',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    MatIconModule,
    SystemMonitorComponent,
    WidgetContainerComponent,
    WidgetHealthScoreComponent,
    WidgetQuickActionsComponent,
    WidgetDiskUsageComponent,
    WidgetRecentActivityComponent,
  ],
  templateUrl: './dashboard.view.html',
})
export class DashboardView implements OnInit {
  private fileService = inject(FileService);
  private systemService = inject(SystemService);
  private healthHistoryService = inject(HealthHistoryService);

  isScanning = signal(false);
  isCleaning = signal(false);
  scanProgress = signal(0);

  cacheSize = signal(0);
  trashSize = signal(0);
  logSize = signal(0);
  largeFileSize = signal(0);
  largeFilesCount = signal(0);

  healthHistoryDays = signal<7 | 30>(7);
  healthHistory = signal<Array<{ timestamp: string; health_score: number }>>([]);
  healthTrend = signal<{ trend: string; change_percent: number }>({
    trend: 'insufficient_data',
    change_percent: 0,
  });

  enabledWidgets = signal<WidgetState[]>([
    { id: 'system-monitor', title: 'System Monitor', icon: 'monitor', enabled: true, order: 0 },
    { id: 'health-score', title: 'Health Score', icon: 'favorite', enabled: true, order: 1 },
    { id: 'quick-actions', title: 'Quick Actions', icon: 'flash_on', enabled: true, order: 2 },
    { id: 'disk-usage', title: 'Disk Usage', icon: 'storage', enabled: true, order: 3 },
    { id: 'recent-activity', title: 'Recent Activity', icon: 'history', enabled: true, order: 4 },
  ]);

  showWidgetSettings = signal(false);
  recentActivities = signal<ActivityItem[]>([]);

  quickActions: QuickAction[] = [
    { id: 'scan', label: 'Scan', icon: 'search', action: 'scan' },
    { id: 'clean-cache', label: 'Clear Cache', icon: 'auto_delete', action: 'clean-cache' },
    { id: 'clean-trash', label: 'Empty Trash', icon: 'delete_outline', action: 'clean-trash' },
    { id: 'clean-logs', label: 'Clear Logs', icon: 'description', action: 'clean-logs' },
  ];

  totalJunkSize = computed(
    () => this.cacheSize() + this.trashSize() + this.logSize() + this.largeFileSize()
  );

  systemStatus = computed(() => {
    const size = this.totalJunkSize();
    if (size > 1024 * 1024 * 1024 * 2) return 'Critical';
    if (size > 1024 * 1024 * 500) return 'Needs Attention';
    return 'Healthy';
  });

  healthScore = computed(() => {
    const size = this.totalJunkSize();
    if (size > 1024 * 1024 * 1024 * 2) return 40;
    if (size > 1024 * 1024 * 500) return 65;
    return 95;
  });

  ngOnInit() {
    this.loadWidgetLayout();
    this.calculateJunkSize();
    this.loadHealthHistory();
    this.loadRecentActivities();
  }

  loadWidgetLayout() {
    try {
      const stored = localStorage.getItem(WIDGET_STORAGE_KEY);
      if (stored) {
        const layout = JSON.parse(stored) as WidgetState[];
        this.enabledWidgets.set(layout);
      }
    } catch (e) {
      console.error('Failed to load widget layout:', e);
    }
  }

  saveWidgetLayout(widgets: WidgetState[]) {
    try {
      localStorage.setItem(WIDGET_STORAGE_KEY, JSON.stringify(widgets));
    } catch (e) {
      console.error('Failed to save widget layout:', e);
    }
  }

  onWidgetVisibilityChange(event: { id: string; enabled: boolean }) {
    const widgets = this.enabledWidgets().map((w) =>
      w.id === event.id ? { ...w, enabled: event.enabled } : w
    );
    this.enabledWidgets.set(widgets);
    this.saveWidgetLayout(widgets);
  }

  onWidgetOrderChange(widgets: WidgetConfig[]) {
    const updatedWidgets = widgets.map((w, index) => ({
      id: w.id,
      title: w.title,
      icon: w.icon,
      enabled: this.enabledWidgets().find((ew) => ew.id === w.id)?.enabled ?? true,
      order: index,
    }));
    this.enabledWidgets.set(updatedWidgets);
    this.saveWidgetLayout(updatedWidgets);
  }

  toggleWidgetSettings() {
    this.showWidgetSettings.update((v) => !v);
  }

  loadRecentActivities() {
    try {
      const stored = localStorage.getItem('cleanux_recent_activities');
      if (stored) {
        const activities = JSON.parse(stored) as ActivityItem[];
        this.recentActivities.set(activities.slice(0, 10));
      }
    } catch (e) {
      console.error('Failed to load recent activities:', e);
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
      console.error('Failed to save recent activities:', e);
    }
  }

  handleQuickAction(action: string) {
    switch (action) {
      case 'scan':
        this.startScan();
        break;
      case 'clean-cache':
        this.fileService.clearCache().then(() => {
          this.addRecentActivity('cache', 'Cache cleared', this.cacheSize());
          this.calculateJunkSize();
        });
        break;
      case 'clean-trash':
        this.fileService.clearTrash().then(() => {
          this.addRecentActivity('trash', 'Trash emptied', this.trashSize());
          this.calculateJunkSize();
        });
        break;
      case 'clean-logs':
        this.fileService.clearAllLogs().then(() => {
          this.addRecentActivity('logs', 'Logs cleared', this.logSize());
          this.calculateJunkSize();
        });
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
      console.error('Failed to load health history:', e);
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
      console.error('Failed to save health snapshot:', e);
    }
  }

  async calculateJunkSize() {
    try {
      const p1 = this.fileService
        .getCacheSummary()
        .then((res) => {
          this.cacheSize.set(res.totalSize);
          return res;
        })
        .catch((e) => console.error(e));
      const p2 = this.fileService
        .getTrashSummary()
        .then((res) => {
          this.trashSize.set(res.totalSize);
          return res;
        })
        .catch((e) => console.error(e));
      const p3 = this.fileService
        .getLogSummary()
        .then((res) => {
          this.logSize.set(res.totalSize);
          return res;
        })
        .catch((e) => console.error(e));
      const p4 = this.fileService
        .getLargeFilesSummary()
        .then((res) => {
          this.largeFileSize.set(res.totalSize);
          this.largeFilesCount.set(res.fileCount);
          return res;
        })
        .catch((e) => console.error(e));

      return Promise.all([p1, p2, p3, p4]);
    } catch (error) {
      console.error('Failed to calculate junk size:', error);
      return Promise.resolve();
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
    setTimeout(() => this.isScanning.set(false), 500);
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
      console.error('Failed to clean system:', error);
    } finally {
      this.isCleaning.set(false);
    }
  }

  formatSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
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
}
