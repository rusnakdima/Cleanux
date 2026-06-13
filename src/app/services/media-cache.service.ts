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
  private logger = inject(LoggerService);

  constructor() {
    this.logger.logInfo('service', 'MediaCacheService', 'init', 'MediaCacheService initialized');
  }

  async getMediaCacheSummary(): Promise<MediaCacheSummary> {
    this.logger.logInfo(
      'service',
      'MediaCacheService',
      'getMediaCacheSummary',
      'Getting media cache summary'
    );
    try {
      const result = await this.api.invoke<MediaCacheSummary>('get_media_cache_summary');
      this.logger.logInfo(
        'service',
        'MediaCacheService',
        'getMediaCacheSummary',
        'Media cache summary retrieved'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'MediaCacheService',
        'getMediaCacheSummary',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanSteamShaderCache(): Promise<string> {
    this.logger.logInfo(
      'service',
      'MediaCacheService',
      'cleanSteamShaderCache',
      'Cleaning Steam shader cache'
    );
    try {
      const result = await this.api.invoke<string>('clean_steam_shader_cache');
      this.logger.logInfo(
        'service',
        'MediaCacheService',
        'cleanSteamShaderCache',
        'Steam shader cache cleaned'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'MediaCacheService',
        'cleanSteamShaderCache',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanSteamDownloadCache(): Promise<string> {
    this.logger.logInfo(
      'service',
      'MediaCacheService',
      'cleanSteamDownloadCache',
      'Cleaning Steam download cache'
    );
    try {
      const result = await this.api.invoke<string>('clean_steam_download_cache');
      this.logger.logInfo(
        'service',
        'MediaCacheService',
        'cleanSteamDownloadCache',
        'Steam download cache cleaned'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'MediaCacheService',
        'cleanSteamDownloadCache',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanSpotifyCache(): Promise<string> {
    this.logger.logInfo(
      'service',
      'MediaCacheService',
      'cleanSpotifyCache',
      'Cleaning Spotify cache'
    );
    try {
      const result = await this.api.invoke<string>('clean_spotify_cache');
      this.logger.logInfo(
        'service',
        'MediaCacheService',
        'cleanSpotifyCache',
        'Spotify cache cleaned'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'MediaCacheService',
        'cleanSpotifyCache',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanVlcCache(): Promise<string> {
    this.logger.logInfo('service', 'MediaCacheService', 'cleanVlcCache', 'Cleaning VLC cache');
    try {
      const result = await this.api.invoke<string>('clean_vlc_cache');
      this.logger.logInfo('service', 'MediaCacheService', 'cleanVlcCache', 'VLC cache cleaned');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'MediaCacheService',
        'cleanVlcCache',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanThumbnailCache(): Promise<string> {
    this.logger.logInfo(
      'service',
      'MediaCacheService',
      'cleanThumbnailCache',
      'Cleaning thumbnail cache'
    );
    try {
      const result = await this.api.invoke<string>('clean_thumbnail_cache');
      this.logger.logInfo(
        'service',
        'MediaCacheService',
        'cleanThumbnailCache',
        'Thumbnail cache cleaned'
      );
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'MediaCacheService',
        'cleanThumbnailCache',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }

  async cleanIconCache(): Promise<string> {
    this.logger.logInfo('service', 'MediaCacheService', 'cleanIconCache', 'Cleaning icon cache');
    try {
      const result = await this.api.invoke<string>('clean_icon_cache');
      this.logger.logInfo('service', 'MediaCacheService', 'cleanIconCache', 'Icon cache cleaned');
      return result;
    } catch (error) {
      this.logger.logError(
        'service',
        'MediaCacheService',
        'cleanIconCache',
        'Operation failed',
        error as Error
      );
      throw error;
    }
  }
}
