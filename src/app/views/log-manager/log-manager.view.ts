import {
  ChangeDetectionStrategy,
  Component,
  OnInit,
  signal,
  inject,
  computed,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTabsModule } from '@angular/material/tabs';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatSelectModule } from '@angular/material/select';
import { HeaderComponent } from '@components/header/header.component';
import {
  LogManagerService,
  JournalInfo,
  LogManagerSummary,
  RotatedLogInfo,
  LogrotateAnalysis,
  LogrotateConfig,
  LogFileInfo,
  VarLogUsage,
} from '@services/log-manager.service';
import { formatSize } from '@shared/utils/format.util';

type TabType = 'journal' | 'rotated' | 'logrotate';

@Component({
  selector: 'app-log-manager-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    FormsModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTabsModule,
    MatInputModule,
    MatFormFieldModule,
    MatSelectModule,
    HeaderComponent,
  ],
  templateUrl: './log-manager.view.html',
})
export class LogManagerView implements OnInit {
  private logManagerService = inject(LogManagerService);

  loading = signal(false);
  activeTab = signal<TabType>('journal');

  summary = signal<LogManagerSummary | null>(null);
  journalInfo = signal<JournalInfo | null>(null);
  rotatedLogs = signal<RotatedLogInfo[]>([]);
  rotatedLogsSize = signal(0);
  logrotateAnalysis = signal<LogrotateAnalysis | null>(null);
  varLogUsage = signal<VarLogUsage | null>(null);
  largestLogs = signal<LogFileInfo[]>([]);

  vacuumSizeMb = signal(100);
  vacuumDays = signal(30);
  cleanRotatedDays = signal(30);

  formatSize = formatSize;

  ngOnInit() {
    this.loadData();
  }

  async loadData() {
    this.loading.set(true);
    try {
      const [summary, journalInfo, rotatedSize, rotatedLogs, analysis, varLogUsage, largestLogs] =
        await Promise.all([
          this.logManagerService.getLogManagerSummary(),
          this.logManagerService.getJournalUsage(),
          this.logManagerService.getRotatedLogsSize(),
          this.logManagerService.getRotatedLogs(),
          this.logManagerService.analyzeLogrotate(),
          this.logManagerService.getVarLogUsage(),
          this.logManagerService.getLargestLogFiles(20),
        ]);

      this.summary.set(summary);
      this.journalInfo.set(journalInfo);
      this.rotatedLogsSize.set(rotatedSize);
      this.rotatedLogs.set(rotatedLogs);
      this.logrotateAnalysis.set(analysis);
      this.varLogUsage.set(varLogUsage);
      this.largestLogs.set(largestLogs);
    } catch (error) {
      console.error('Failed to load log manager data:', error);
    } finally {
      this.loading.set(false);
    }
  }

  async vacuumJournal() {
    if (!confirm(`Vacuum journal to retain ${this.vacuumSizeMb()}MB?`)) return;
    this.loading.set(true);
    try {
      await this.logManagerService.vacuumJournal(this.vacuumSizeMb());
      await this.loadData();
    } catch (error) {
      console.error('Failed to vacuum journal:', error);
      alert('Failed to vacuum journal');
    } finally {
      this.loading.set(false);
    }
  }

  async vacuumJournalByDays() {
    if (!confirm(`Vacuum journal to retain last ${this.vacuumDays()} days?`)) return;
    this.loading.set(true);
    try {
      await this.logManagerService.vacuumJournalByDays(this.vacuumDays());
      await this.loadData();
    } catch (error) {
      console.error('Failed to vacuum journal:', error);
      alert('Failed to vacuum journal');
    } finally {
      this.loading.set(false);
    }
  }

  async cleanRotatedLogs() {
    if (!confirm(`Clean rotated logs older than ${this.cleanRotatedDays()} days?`)) return;
    this.loading.set(true);
    try {
      await this.logManagerService.cleanRotatedLogs(this.cleanRotatedDays());
      await this.loadData();
    } catch (error) {
      console.error('Failed to clean rotated logs:', error);
      alert('Failed to clean rotated logs');
    } finally {
      this.loading.set(false);
    }
  }

  getLogrotateStatusColor(enabled: boolean): string {
    return enabled ? 'text-green-500' : 'text-slate-400';
  }

  getLogrotateBgColor(enabled: boolean): string {
    return enabled ? 'bg-green-100 dark:bg-green-900/30' : 'bg-slate-100 dark:bg-slate-800/50';
  }
}
