import { ChangeDetectionStrategy, Component, OnInit, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { DevCacheService } from '@services/dev-cache.service';
import { DevCacheItem, DevCacheSummary } from '@models/dev-cache.model';
import { formatSize } from '@shared/utils/format.util';
import { LoadingErrorMixin } from '@views/mixins/loading-error.mixin';
import { LoadingSpinnerComponent } from '@components/loading-spinner/loading-spinner.component';

interface DevToolCard {
  name: string;
  displayName: string;
  icon: string;
  item: DevCacheItem | null;
  lastCleaned: string | null;
}

@Component({
  selector: 'app-dev-cleaner-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    LoadingSpinnerComponent,
  ],
  templateUrl: './dev-cleaner.view.html',
})
export class DevCleanerView extends LoadingErrorMixin implements OnInit {
  private devCacheService = inject(DevCacheService);

  formatSize = formatSize;

  cleaning = signal(false);
  summary = signal<DevCacheSummary | null>(null);

  devTools = signal<DevToolCard[]>([
    { name: 'npm', displayName: 'NPM', icon: 'box', item: null, lastCleaned: null },
    { name: 'pip', displayName: 'Pip', icon: 'package', item: null, lastCleaned: null },
    { name: 'cargo', displayName: 'Cargo', icon: 'settings', item: null, lastCleaned: null },
    { name: 'go', displayName: 'Go', icon: 'terminal', item: null, lastCleaned: null },
    { name: 'maven', displayName: 'Maven', icon: 'layers', item: null, lastCleaned: null },
    { name: 'gradle', displayName: 'Gradle', icon: 'build', item: null, lastCleaned: null },
  ]);

  async ngOnInit() {
    await this.loadSummary();
  }

  async loadSummary(): Promise<void> {
    this.loading.set(true);
    try {
      const summary = await this.devCacheService.getDevCacheSummary();
      this.summary.set(summary);
      this.updateDevTools(summary);
    } catch (error) {
      this.notification.error('Failed to load dev cache summary', error);
    } finally {
      this.loading.set(false);
    }
  }

  private updateDevTools(summary: DevCacheSummary): void {
    const tools = this.devTools();
    const cacheMap = Object.fromEntries(
      Object.keys(summary).map((key) => [key, summary[key as keyof DevCacheSummary]])
    );

    this.devTools.set(
      tools.map((tool) => ({
        ...tool,
        item: cacheMap[tool.name] || null,
      }))
    );
  }

  async cleanTool(name: string): Promise<void> {
    const confirmed = await this.confirmDialogService.confirm({
      title: 'Clean Cache',
      message: `Clean ${name} cache?`,
    });
    if (!confirmed) return;

    this.cleaning.set(true);
    try {
      let result: string;
      switch (name) {
        case 'npm':
          result = await this.devCacheService.cleanNpmCache();
          break;
        case 'pip':
          result = await this.devCacheService.cleanPipCache();
          break;
        case 'cargo':
          result = await this.devCacheService.cleanCargoCache();
          break;
        case 'go':
          result = await this.devCacheService.cleanGoCache();
          break;
        case 'maven':
          result = await this.devCacheService.cleanMavenCache();
          break;
        case 'gradle':
          result = await this.devCacheService.cleanGradleCache();
          break;
        default:
          return;
      }
      this.notification.success(result);
      await this.loadSummary();
    } catch (error) {
      this.notification.error('Failed to clean cache', error);
    } finally {
      this.cleaning.set(false);
    }
  }

  async cleanAll(): Promise<void> {
    const confirmed = await this.confirmDialogService.confirm({
      title: 'Clean All Caches',
      message: 'Clean all dev caches?',
    });
    if (!confirmed) return;

    this.cleaning.set(true);
    try {
      const result = await this.devCacheService.cleanAllDevCaches();
      this.notification.success(result);
      await this.loadSummary();
    } catch (error) {
      this.notification.error('Failed to clean caches', error);
    } finally {
      this.cleaning.set(false);
    }
  }

  getTotalSize(): string {
    const summary = this.summary();
    if (!summary) return '0 B';

    const total = [
      summary.npm.size,
      summary.pip.size,
      summary.cargo.size,
      summary.go.size,
      summary.maven.size,
      summary.gradle.size,
    ].reduce((a, b) => a + b, 0);

    return formatSize(total);
  }

  getToolSize(item: DevCacheItem | null): string {
    return item ? formatSize(item.size) : '0 B';
  }

  trackByName(_: number, tool: DevToolCard): string {
    return tool.name;
  }
}
