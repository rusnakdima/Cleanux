/* sys lib */
import { Injectable, inject } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';

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

  constructor() {}

  async getMediaCacheSummary(): Promise<MediaCacheSummary> {
    try {
      const result = await this.api.invoke<MediaCacheSummary>('get_media_cache_summary');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanSteamShaderCache(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clean_steam_shader_cache');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanSteamDownloadCache(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clean_steam_download_cache');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanSpotifyCache(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clean_spotify_cache');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanVlcCache(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clean_vlc_cache');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanThumbnailCache(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clean_thumbnail_cache');
      return result;
    } catch (error) {
      throw error;
    }
  }

  async cleanIconCache(): Promise<string> {
    try {
      const result = await this.api.invoke<string>('clean_icon_cache');
      return result;
    } catch (error) {
      throw error;
    }
  }
}
