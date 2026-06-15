/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';
import { LoggerService } from '@services/logger.service';

export interface MediaCacheSummary {
  steamShaderSize: number;
  steamDownloadSize: number;
  steamGameCount: number;
  spotifyCacheSize: number;
  vlcCacheSize: number;
  thumbnailCacheSize: number;
  iconCacheSize: number;
}

@Injectable({
  providedIn: 'root',
})
export class MediaCacheService {
  private api = inject(ApiService);
  private loggingService = new LoggerService();

  constructor() {
    this.loggingService.info('MediaCacheService initialized');
  }

  async getMediaCacheSummary(): Promise<MediaCacheSummary> {
    this.loggingService.info('Getting media cache summary');
    try {
      const result = await this.api.invoke<MediaCacheSummary>('get_media_cache_summary');
      this.loggingService.info('Media cache summary retrieved');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanSteamShaderCache(): Promise<string> {
    this.loggingService.info('Cleaning Steam shader cache');
    try {
      const result = await this.api.invoke<string>('clean_steam_shader_cache');
      this.loggingService.info('Steam shader cache cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanSteamDownloadCache(): Promise<string> {
    this.loggingService.info('Cleaning Steam download cache');
    try {
      const result = await this.api.invoke<string>('clean_steam_download_cache');
      this.loggingService.info('Steam download cache cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanSpotifyCache(): Promise<string> {
    this.loggingService.info('Cleaning Spotify cache');
    try {
      const result = await this.api.invoke<string>('clean_spotify_cache');
      this.loggingService.info('Spotify cache cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanVlcCache(): Promise<string> {
    this.loggingService.info('Cleaning VLC cache');
    try {
      const result = await this.api.invoke<string>('clean_vlc_cache');
      this.loggingService.info('VLC cache cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanThumbnailCache(): Promise<string> {
    this.loggingService.info('Cleaning thumbnail cache');
    try {
      const result = await this.api.invoke<string>('clean_thumbnail_cache');
      this.loggingService.info('Thumbnail cache cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }

  async cleanIconCache(): Promise<string> {
    this.loggingService.info('Cleaning icon cache');
    try {
      const result = await this.api.invoke<string>('clean_icon_cache');
      this.loggingService.info('Icon cache cleaned');
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error);
      throw error;
    }
  }
}
