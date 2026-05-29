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
import { MediaCacheService, MediaCacheSummary } from '@services/media-cache.service';
import { NotificationService } from '@services/notification.service';

/* models */
import { formatSize } from '@shared/utils/format.util';

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
export class MediaCleanerView implements OnInit {
  private mediaCacheService = inject(MediaCacheService);
  private notification = inject(NotificationService);

  formatSize = formatSize;

  summary = signal<MediaCacheSummary | null>(null);
  loading = signal(false);
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
    this.loading.set(true);
    try {
      const data = await this.mediaCacheService.getMediaCacheSummary();
      this.summary.set(data);
    } catch (error) {
      console.error('Failed to load media cache summary:', error);
    } finally {
      this.loading.set(false);
    }
  }

  async cleanSteamShaderCache(): Promise<void> {
    if (!this.confirmClean('Steam shader cache')) return;
    this.cleaning.set('steam-shader');
    try {
      await this.mediaCacheService.cleanSteamShaderCache();
      await this.loadSummary();
    } catch (error) {
      this.notification.cleanError('clean Steam shader cache', error);
    } finally {
      this.cleaning.set(null);
    }
  }

  async cleanSteamDownloadCache(): Promise<void> {
    if (!this.confirmClean('Steam download cache')) return;
    this.cleaning.set('steam-download');
    try {
      await this.mediaCacheService.cleanSteamDownloadCache();
      await this.loadSummary();
    } catch (error) {
      this.notification.cleanError('clean Steam download cache', error);
    } finally {
      this.cleaning.set(null);
    }
  }

  async cleanSpotifyCache(): Promise<void> {
    if (!this.confirmClean('Spotify cache')) return;
    this.cleaning.set('spotify');
    try {
      await this.mediaCacheService.cleanSpotifyCache();
      await this.loadSummary();
    } catch (error) {
      this.notification.cleanError('clean Spotify cache', error);
    } finally {
      this.cleaning.set(null);
    }
  }

  async cleanVlcCache(): Promise<void> {
    if (!this.confirmClean('VLC cache')) return;
    this.cleaning.set('vlc');
    try {
      await this.mediaCacheService.cleanVlcCache();
      await this.loadSummary();
    } catch (error) {
      this.notification.cleanError('clean VLC cache', error);
    } finally {
      this.cleaning.set(null);
    }
  }

  async cleanThumbnailCache(): Promise<void> {
    if (!this.confirmClean('thumbnail cache')) return;
    this.cleaning.set('thumbnails');
    try {
      await this.mediaCacheService.cleanThumbnailCache();
      await this.loadSummary();
    } catch (error) {
      this.notification.cleanError('clean thumbnail cache', error);
    } finally {
      this.cleaning.set(null);
    }
  }

  async cleanIconCache(): Promise<void> {
    if (!this.confirmClean('icon cache')) return;
    this.cleaning.set('icons');
    try {
      await this.mediaCacheService.cleanIconCache();
      await this.loadSummary();
    } catch (error) {
      this.notification.cleanError('clean icon cache', error);
    } finally {
      this.cleaning.set(null);
    }
  }

  private confirmClean(type: string): boolean {
    return confirm(`Are you sure you want to clean the ${type}?\n\nThis action cannot be undone.`);
  }

  isCleaning(key: string): boolean {
    return this.cleaning() === key;
  }
}
