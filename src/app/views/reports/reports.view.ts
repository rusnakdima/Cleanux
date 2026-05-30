import {
  ChangeDetectionStrategy,
  Component,
  signal,
  computed,
  inject,
  OnInit,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';

import { ReportService } from '@services/report.service';

import { CleaningReport, SnapshotComparison } from '@models/report.model';
import { formatSize } from '@shared/utils/format.util';
import { DataListComponent } from '@components/data-list/data-list.component';
import { ListColumn, ListOptions } from '@models/data-list.model';

@Component({
  selector: 'app-reports',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    FormsModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    DataListComponent,
  ],
  templateUrl: './reports.view.html',
})
export class ReportsView implements OnInit {
  private reportService = inject(ReportService);

  formatSize = formatSize;
  Math = Math;

  reports = signal<CleaningReport[]>([]);
  loading = signal(false);
  selectedReport = signal<CleaningReport | null>(null);
  comparison = signal<SnapshotComparison | null>(null);
  showComparison = signal(false);
  comparingReportIds = signal<{ before?: number; after?: number }>({});
  exportHtml = signal<string | null>(null);

  currentPage = signal(1);
  pageSize = signal(15);

  reportsWithChart = computed(() => {
    const data = this.reports();
    if (data.length < 2) return [];
    return [...data].reverse().slice(-10);
  });

  chartPath = computed(() => {
    const data = this.reportsWithChart();
    if (data.length < 2) return '';

    const width = 400;
    const height = 150;
    const padding = 30;
    const chartWidth = width - 2 * padding;
    const chartHeight = height - 2 * padding;

    const maxSpace = Math.max(...data.map((r) => r.space_reclaimed), 1);
    const minSpace = 0;

    const points = data.map((r, i) => {
      const x = padding + (i / (data.length - 1)) * chartWidth;
      const y =
        padding +
        chartHeight -
        ((r.space_reclaimed - minSpace) / (maxSpace - minSpace || 1)) * chartHeight;
      return `${x},${y}`;
    });

    return `M ${points.join(' L ')}`;
  });

  columns: ListColumn[] = [
    {
      key: 'date',
      primary: true,
      icon: 'description',
      badge: 'space_reclaimed',
      badgeClass: 'badge-info',
      secondaryKey: 'duration',
      actions: [
        {
          id: 'export',
          icon: 'download',
          tooltip: 'Export to HTML',
        },
      ],
    },
  ];

  options: ListOptions = {
    showSearch: true,
    showCheckbox: false,
    showActions: true,
    actionsPosition: 'right',
    showReloadButton: true,
    searchPlaceholder: 'Search reports...',
    emptyMessage: 'No reports found',
  };

  onPageChange(page: number) {
    this.currentPage.set(page);
  }

  onPageSizeChange(size: number) {
    this.pageSize.set(size);
    this.currentPage.set(1);
  }

  ngOnInit() {
    this.loadReports();
  }

  async loadReports() {
    this.loading.set(true);
    try {
      const reports = await this.reportService.getCleaningHistory();
      this.reports.set(reports);
    } catch (e) {
      console.error('Failed to load reports:', e);
    } finally {
      this.loading.set(false);
    }
  }

  async generateReport() {
    try {
      await this.reportService.generateCleaningReport(
        Math.floor(Math.random() * 100) + 10,
        Math.floor(Math.random() * 5000000000) + 1000000,
        Math.random() * 60 + 5,
        Math.floor(Math.random() * 50) + 5,
        Math.floor(Math.random() * 30) + 3,
        Math.floor(Math.random() * 20) + 2,
        Math.floor(Math.random() * 10) + 1,
        Math.floor(Math.random() * 5)
      );
      await this.loadReports();
    } catch (e) {
      console.error('Failed to generate report:', e);
    }
  }

  selectReport(report: CleaningReport) {
    this.selectedReport.set(this.selectedReport()?.id === report.id ? null : report);
  }

  setBeforeReport(reportId: number) {
    this.comparingReportIds.update((c) => ({ ...c, before: reportId }));
  }

  setAfterReport(reportId: number) {
    this.comparingReportIds.update((c) => ({ ...c, after: reportId }));
  }

  async compareSnapshots() {
    const ids = this.comparingReportIds();
    if (!ids.before || !ids.after) return;

    try {
      const result = await this.reportService.compareSnapshots(ids.before, ids.after);
      this.comparison.set(result);
      this.showComparison.set(true);
    } catch (e) {
      console.error('Failed to compare snapshots:', e);
    }
  }

  async exportHtmlReport(reportId: number) {
    try {
      const result = await this.reportService.exportToHtml(reportId);
      this.exportHtml.set(result.html);
    } catch (e) {
      console.error('Failed to export HTML:', e);
    }
  }

  closeExportHtml() {
    this.exportHtml.set(null);
  }

  closeComparison() {
    this.showComparison.set(false);
    this.comparison.set(null);
  }

  formatDate(dateStr: string): string {
    const date = new Date(dateStr);
    return date.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  }

  formatReportDate(report: CleaningReport): string {
    return this.formatDate(report.date);
  }

  formatReportSpace(report: CleaningReport): string {
    return this.formatSize(report.space_reclaimed);
  }

  getChartPathForReport(reportId: number): string {
    const data = this.reportsWithChart();
    if (data.length < 2) return '';

    const width = 300;
    const height = 100;
    const padding = 10;
    const chartWidth = width - 2 * padding;
    const chartHeight = height - 2 * padding;

    const maxSpace = Math.max(...data.map((r) => r.space_reclaimed), 1);

    const idx = data.findIndex((r) => r.id === reportId);
    if (idx < 0) return '';

    const report = data[idx];
    const prevReport = idx > 0 ? data[idx - 1] : report;

    const spaceChange = report.space_reclaimed - prevReport.space_reclaimed;
    if (spaceChange <= 0) return '';

    const x = padding + (idx / (data.length - 1)) * chartWidth;
    const y = padding + chartHeight - (spaceChange / maxSpace) * chartHeight;

    return `M ${x},${chartHeight + padding} L ${x},${y}`;
  }

  onRowAction(event: { action: string; item: CleaningReport }): void {
    if (event.action === 'export') {
      this.exportHtmlReport(event.item.id!);
    }
  }
}
