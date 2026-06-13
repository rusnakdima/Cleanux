/* sys lib */
import { ChangeDetectionStrategy, Component, OnInit, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { MatCardModule } from '@angular/material/card';
import { MatTabsModule } from '@angular/material/tabs';

/* services */
import { MediaCacheService } from '@services/media-cache.service';
import { MediaCacheSummary } from '@models/media-cache.model';

/* models */
import { formatSize } from '@shared/utils/format.util';
import { LoadingErrorMixin } from '@views/mixins/loading-error.mixin';

@Component({
  selector: 'app-media-cleaner-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    MatCardModule,
    MatTabsModule,
  ],
  templateUrl: './media-cleaner.view.html',
})
export class MediaCleanerView extends LoadingErrorMixin implements OnInit {
  private mediaCacheService = inject(MediaCacheService);

  formatSize = formatSize;

  summary = signal<MediaCacheSummary | null>(null);
  cleaning = signal<string | null>(null);

  tabs = [
    { key: 'steam', label: 'Steam', icon: '🎮' },
    { key: 'spotify', label: 'Spotify', icon: '🎵' },
    { key: 'vlc', label: 'VLC', icon: '📺' },
    { key: 'thumbnails', label: 'Thumbnails', icon: '🖼️' },
    { key: 'icons', label: 'Icons', icon: '🔲' },
  ];

  async ngOnInit(): Promise<void> {
    await this.loadSummary();
  }

  async loadSummary(): Promise<void> {
    await this.runWithLoading(
      async () => {
        const data = await this.mediaCacheService.getMediaCacheSummary();
        this.summary.set(data);
        return data;
      },
      { errorMessage: 'Failed to load media cache summary' }
    );
  }

  async cleanSteamShaderCache(): Promise<void> {
    if (!(await this.confirmClean('Steam shader cache'))) return;
    this.cleaning.set('steam-shader');
    try {
      await this.mediaCacheService.cleanSteamShaderCache();
      await this.loadSummary();
    } catch (error) {
      this.notification.error('Failed to clean Steam shader cache', error);
    } finally {
      this.cleaning.set(null);
    }
  }

  async cleanSteamDownloadCache(): Promise<void> {
    if (!(await this.confirmClean('Steam download cache'))) return;
    this.cleaning.set('steam-download');
    try {
      await this.mediaCacheService.cleanSteamDownloadCache();
      await this.loadSummary();
    } catch (error) {
      this.notification.error('Failed to clean Steam download cache', error);
    } finally {
      this.cleaning.set(null);
    }
  }

  async cleanSpotifyCache(): Promise<void> {
    if (!(await this.confirmClean('Spotify cache'))) return;
    this.cleaning.set('spotify');
    try {
      await this.mediaCacheService.cleanSpotifyCache();
      await this.loadSummary();
    } catch (error) {
      this.notification.error('Failed to clean Spotify cache', error);
    } finally {
      this.cleaning.set(null);
    }
  }

  async cleanVlcCache(): Promise<void> {
    if (!(await this.confirmClean('VLC cache'))) return;
    this.cleaning.set('vlc');
    try {
      await this.mediaCacheService.cleanVlcCache();
      await this.loadSummary();
    } catch (error) {
      this.notification.error('Failed to clean VLC cache', error);
    } finally {
      this.cleaning.set(null);
    }
  }

  async cleanThumbnailCache(): Promise<void> {
    if (!(await this.confirmClean('thumbnail cache'))) return;
    this.cleaning.set('thumbnails');
    try {
      await this.mediaCacheService.cleanThumbnailCache();
      await this.loadSummary();
    } catch (error) {
      this.notification.error('Failed to clean thumbnail cache', error);
    } finally {
      this.cleaning.set(null);
    }
  }

  async cleanIconCache(): Promise<void> {
    if (!(await this.confirmClean('icon cache'))) return;
    this.cleaning.set('icons');
    try {
      await this.mediaCacheService.cleanIconCache();
      await this.loadSummary();
    } catch (error) {
      this.notification.error('Failed to clean icon cache', error);
    } finally {
      this.cleaning.set(null);
    }
  }

  private async confirmClean(type: string): Promise<boolean> {
    return this.confirmDialogService.confirm({
      title: 'Confirm Clean',
      message: `Are you sure you want to clean the ${type}?\n\nThis action cannot be undone.`,
      dangerous: true,
    });
  }

  isCleaning(key: string): boolean {
    return this.cleaning() === key;
  }
}
