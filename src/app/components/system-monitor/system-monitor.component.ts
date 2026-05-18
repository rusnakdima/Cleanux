/* sys lib */
import { Component, OnInit, OnDestroy, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

export interface SystemStats {
  cpu_usage: number;
  memory_used: number;
  memory_total: number;
  memory_usage_percent: number;
  disk_used: number;
  disk_total: number;
  disk_usage_percent: number;
}

@Component({
  selector: 'app-system-monitor',
  standalone: true,
  imports: [CommonModule, MatIconModule],
  templateUrl: './system-monitor.component.html',
  styleUrls: ['./system-monitor.component.css'],
})
export class SystemMonitorComponent implements OnInit, OnDestroy {
  private unlistenStats: UnlistenFn | null = null;
  private pollInterval: any = null;

  cpuUsage = signal(0);
  memoryUsage = signal(0);
  diskUsage = signal(0);

  memoryUsed = signal(0);
  memoryTotal = signal(0);
  diskUsed = signal(0);
  diskTotal = signal(0);

  cpuHistory: number[] = Array(20).fill(0);
  memoryHistory: number[] = Array(20).fill(0);
  diskHistory: number[] = Array(20).fill(0);

  isMonitoring = signal(false);

  ngOnInit() {
    this.startMonitoring();
  }

  ngOnDestroy() {
    this.stopMonitoring();
  }

  async startMonitoring() {
    if (this.isMonitoring()) return;

    this.isMonitoring.set(true);
    await this.fetchStats();

    this.pollInterval = setInterval(() => {
      this.fetchStats();
    }, 2000);

    this.unlistenStats = await listen<SystemStats>('system-stats-update', (event) => {
      this.updateStats(event.payload);
    });
  }

  stopMonitoring() {
    if (this.pollInterval) {
      clearInterval(this.pollInterval);
      this.pollInterval = null;
    }
    if (this.unlistenStats) {
      this.unlistenStats();
      this.unlistenStats = null;
    }
    this.isMonitoring.set(false);
  }

  async fetchStats() {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const response = await invoke<any>('get_system_stats');
      if (response.status === 'success') {
        this.updateStats(response.data);
      }
    } catch (error) {
      console.error('Failed to fetch system stats:', error);
    }
  }

  updateStats(stats: SystemStats) {
    this.cpuUsage.set(stats.cpu_usage);
    this.memoryUsage.set(stats.memory_usage_percent);
    this.diskUsage.set(stats.disk_usage_percent);

    this.memoryUsed.set(stats.memory_used);
    this.memoryTotal.set(stats.memory_total);
    this.diskUsed.set(stats.disk_used);
    this.diskTotal.set(stats.disk_total);

    this.cpuHistory.push(stats.cpu_usage);
    this.cpuHistory.shift();

    this.memoryHistory.push(stats.memory_usage_percent);
    this.memoryHistory.shift();

    this.diskHistory.push(stats.disk_usage_percent);
    this.diskHistory.shift();
  }

  formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }

  getUsageColor(percent: number): string {
    if (percent >= 90) return '#ef4444';
    if (percent >= 70) return '#f59e0b';
    return '#22c55e';
  }

  getCpuColor(percent: number): string {
    if (percent >= 90) return '#ef4444';
    if (percent >= 70) return '#f59e0b';
    return '#3b82f6';
  }
}
