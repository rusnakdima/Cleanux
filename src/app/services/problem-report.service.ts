import { Injectable, inject, signal } from '@angular/core';
import { LogStorageService } from './log-storage.service';
import { LogExportService, ProblemReport } from '@features/log-manager/services/log-export.service';
import { LogEntry, LogFilter } from '@entities/log-manager.model';
import { environment } from '@env/environment';
import { TOAST_DURATION_MS } from '@shared/utils/constants';

const ERROR_THRESHOLD = 5;
const AUTO_REPORT_KEY = 'cleanux_auto_report_enabled';

@Injectable({
  providedIn: 'root',
})
export class ProblemReportService {
  private storage = inject(LogStorageService);
  private exporter = inject(LogExportService);

  autoReportEnabled = signal<boolean>(localStorage.getItem(AUTO_REPORT_KEY) !== 'false');
  lastReport = signal<ProblemReport | null>(null);
  recentErrors = signal<LogEntry[]>([]);

  private errorCount = 0;
  private lastErrorTime = 0;
  private errorDebounceMs = TOAST_DURATION_MS;

  enable(): void {
    this.autoReportEnabled.set(true);
    localStorage.setItem(AUTO_REPORT_KEY, 'true');
  }

  disable(): void {
    this.autoReportEnabled.set(false);
    localStorage.setItem(AUTO_REPORT_KEY, 'false');
  }

  async onError(error: Error, context?: Record<string, unknown>): Promise<void> {
    if (!this.autoReportEnabled()) return;

    const now = Date.now();
    if (now - this.lastErrorTime < this.errorDebounceMs) {
      this.errorCount++;
    } else {
      this.errorCount = 1;
    }
    this.lastErrorTime = now;

    if (this.errorCount >= ERROR_THRESHOLD) {
      await this.generateAndStoreReport();
      this.errorCount = 0;
    }
  }

  async generateAndStoreReport(): Promise<ProblemReport> {
    const logs = await this.storage.getLogs({ level: 'error' } as LogFilter, 500);
    const report = await this.exporter.generateProblemReport(
      logs as LogEntry[],
      environment.version
    );

    this.lastReport.set(report);
    localStorage.setItem('cleanux_last_report', JSON.stringify(report));

    return report;
  }

  async getLastReport(): Promise<ProblemReport | null> {
    if (this.lastReport()) return this.lastReport();

    const stored = localStorage.getItem('cleanux_last_report');
    if (stored) {
      try {
        const report = JSON.parse(stored) as ProblemReport;
        this.lastReport.set(report);
        return report;
      } catch {
        return null;
      }
    }
    return null;
  }

  async getReportIfAvailable(): Promise<ProblemReport | null> {
    const errors = await this.storage.getErrors(100);
    if (errors.length < ERROR_THRESHOLD) return null;

    return this.generateAndStoreReport();
  }

  async exportCurrentReport(format: 'json' | 'csv' | 'html' = 'json'): Promise<string> {
    const report = await this.getLastReport();
    if (!report) {
      throw new Error('No report available. Generate one first.');
    }

    switch (format) {
      case 'json':
        return JSON.stringify(report, null, 2);
      case 'csv':
        return this.exporter.exportToCsv({ level: 'error' } as LogFilter);
      case 'html':
        return this.exporter.exportToHtml({ level: 'error' } as LogFilter);
    }
  }

  generateShareableReportLink(): string {
    const report = this.lastReport();
    if (!report) {
      throw new Error('No report available. Generate one first.');
    }

    const logs = report.recentActions;
    return this.exporter.generateShareableLink(logs);
  }

  clearReport(): void {
    this.lastReport.set(null);
    localStorage.removeItem('cleanux_last_report');
  }

  async getErrorSummary(): Promise<{
    count: number;
    lastError: string | null;
    sources: Record<string, number>;
  }> {
    const errors = await this.storage.getErrors(1000);
    const sources: Record<string, number> = {};

    for (const error of errors) {
      const source = error.source || 'unknown';
      sources[source] = (sources[source] || 0) + 1;
    }

    return {
      count: errors.length,
      lastError: errors.length > 0 ? errors[errors.length - 1].message : null,
      sources,
    };
  }
}
