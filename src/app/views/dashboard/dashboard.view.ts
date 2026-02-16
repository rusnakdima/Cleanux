/* sys lib */
import { Component, signal, computed, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';

/* services */
import { SystemService, ScanSummary } from '@services/system.service';

@Component({
  selector: 'app-dashboard',
  standalone: true,
  imports: [CommonModule, MatIconModule],
  templateUrl: './dashboard.view.html',
})
export class DashboardView implements OnInit {
  private systemService = inject(SystemService);

  isScanning = signal(false);
  isCleaning = signal(false);
  scanProgress = signal(0);

  cacheSize = signal(0);
  trashSize = signal(0);
  logSize = signal(0);
  largeFileSize = signal(0);

  totalJunkSize = computed(() => this.cacheSize() + this.trashSize() + this.logSize() + this.largeFileSize());

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
    this.calculateJunkSize();
  }

  async calculateJunkSize() {
    try {
      const p1 = this.systemService.getCacheSummary().then(res => this.cacheSize.set(res.totalSize)).catch(e => console.error(e));
      const p2 = this.systemService.getTrashSummary().then(res => this.trashSize.set(res.totalSize)).catch(e => console.error(e));
      const p3 = this.systemService.getLogSummary().then(res => this.logSize.set(res.totalSize)).catch(e => console.error(e));
      const p4 = this.systemService.getLargeFilesSummary().then(res => this.largeFileSize.set(res.totalSize)).catch(e => console.error(e));

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

    // Simulate scan progress while fetching summaries for a "fresh" scan
    const interval = setInterval(() => {
      this.scanProgress.update((p) => {
        if (p >= 90) {
          return p; // Wait for the actual calculation to finish
        }
        return p + Math.random() * 10;
      });
    }, 100);

    await this.calculateJunkSize();

    clearInterval(interval);
    this.scanProgress.set(100);
    setTimeout(() => this.isScanning.set(false), 500);
  }

  async cleanAll() {
    if (this.isCleaning() || this.totalJunkSize() === 0) return;

    this.isCleaning.set(true);
    try {
      await Promise.all([
        this.systemService.clearCache(),
        this.systemService.clearTrash(),
        this.systemService.clearAllLogs(),
        this.systemService.clearAllLargeFiles()
      ]);

      // Refresh after cleaning
      this.cacheSize.set(0);
      this.trashSize.set(0);
      this.logSize.set(0);
      this.largeFileSize.set(0);

      await this.calculateJunkSize();
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
}
