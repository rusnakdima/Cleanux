/* sys lib */
import { ChangeDetectionStrategy, Component, OnInit, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { MatExpansionModule } from '@angular/material/expansion';
import { MatCardModule } from '@angular/material/card';

/* services */
import { JunkCleanerService } from '@features/advanced-cleaner/services/junk-cleaner.service';

/* mixins */
import { LoadingErrorMixin } from '@pages/mixins/loading-error.mixin';

/* components */
import { LoadingSpinnerComponent } from '@components/loading-spinner/loading-spinner.component';

/* models */
import {
  JunkCategorySummary,
  JunkItem,
  JUNK_CATEGORIES,
  JunkCategoryKey,
} from '@entities/junk-cleaner.model';
import { formatSize } from '@shared/utils/format.util';
import { getErrorMessage } from '@shared/utils/error.util';

@Component({
  selector: 'app-advanced-cleaner-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    MatExpansionModule,
    MatCardModule,
    LoadingSpinnerComponent,
  ],
  templateUrl: './advanced-cleaner.view.html',
})
export class AdvancedCleanerView extends LoadingErrorMixin implements OnInit {
  private junkCleanerService = inject(JunkCleanerService);

  formatSize = formatSize;

  junkSummary = signal<JunkCategorySummary[]>([]);
  scanningCategory = signal<string | null>(null);
  lastCleaned = signal<Record<string, string>>({});

  junkCategories = JUNK_CATEGORIES;

  async ngOnInit(): Promise<void> {
    await this.loadJunkSummary();
    this.loadLastCleaned();
  }

  async loadJunkSummary(): Promise<void> {
    this.loading.set(true);
    try {
      const summary = await this.junkCleanerService.getJunkSummary();
      this.junkSummary.set(summary);
    } catch (error) {
      this.notification.error('Failed to load junk summary', error);
    } finally {
      this.loading.set(false);
    }
  }

  loadLastCleaned(): void {
    const stored = localStorage.getItem('junk_last_cleaned');
    if (stored) {
      this.lastCleaned.set(JSON.parse(stored));
    }
  }

  saveLastCleaned(category: string): void {
    const updated = { ...this.lastCleaned(), [category]: new Date().toISOString() };
    this.lastCleaned.set(updated);
    localStorage.setItem('junk_last_cleaned', JSON.stringify(updated));
  }

  async scanCategory(category: JunkCategoryKey): Promise<void> {
    this.scanningCategory.set(category);
    try {
      switch (category) {
        case 'browser':
          await this.junkCleanerService.scanBrowserCaches();
          break;
        case 'thumbnails':
          await this.junkCleanerService.scanThumbnailCaches();
          break;
        case 'applications':
          await this.junkCleanerService.scanApplicationCaches();
          break;
        case 'system':
          await this.junkCleanerService.scanSystemTemp();
          break;
        case 'logs':
          await this.junkCleanerService.scanLogRotations();
          break;
      }
      const summary = await this.junkCleanerService.getJunkSummary();
      this.junkSummary.set(summary);
    } catch (error) {
      this.notification.error(`Failed to scan ${category}`, error);
    } finally {
      this.scanningCategory.set(null);
    }
  }

  async cleanCategory(category: JunkCategoryKey): Promise<void> {
    const categoryLabel = this.junkCategories.find((c) => c.key === category)?.label || category;
    const confirmed = await this.confirmDialogService.confirm({
      title: 'Clean Category',
      message: `Are you sure you want to clean all ${categoryLabel} junk files?\n\nThis action cannot be undone.`,
      dangerous: true,
    });
    if (!confirmed) return;

    this.loading.set(true);
    try {
      await this.junkCleanerService.cleanJunkCategory(category);
      this.saveLastCleaned(category);
      await this.loadJunkSummary();
    } catch (error) {
      this.notification.error(`Failed to clean ${categoryLabel}`, getErrorMessage(error));
    } finally {
      this.loading.set(false);
    }
  }

  getCategorySummary(key: JunkCategoryKey): JunkCategorySummary | undefined {
    return this.junkSummary().find((s) => s.category.toLowerCase() === key);
  }

  formatLastCleaned(key: string): string {
    const last = this.lastCleaned()[key];
    if (!last) return 'Never';
    const date = new Date(last);
    return date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
  }

  getCategoryIcon(key: JunkCategoryKey): string {
    const cat = this.junkCategories.find((c) => c.key === key);
    return cat?.icon || 'folder';
  }
}
