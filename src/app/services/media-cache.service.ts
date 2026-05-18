/* sys lib */
import { Injectable } from '@angular/core';

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
  constructor(private api: ApiService) {}

  async getMediaCacheSummary(): Promise<MediaCacheSummary> {
    return this.api.invoke<MediaCacheSummary>('get_media_cache_summary');
  }

  async cleanSteamShaderCache(): Promise<string> {
    return this.api.invoke<string>('clean_steam_shader_cache');
  }

  async cleanSteamDownloadCache(): Promise<string> {
    return this.api.invoke<string>('clean_steam_download_cache');
  }

  async cleanSpotifyCache(): Promise<string> {
    return this.api.invoke<string>('clean_spotify_cache');
  }

  async cleanVlcCache(): Promise<string> {
    return this.api.invoke<string>('clean_vlc_cache');
  }

  async cleanThumbnailCache(): Promise<string> {
    return this.api.invoke<string>('clean_thumbnail_cache');
  }

  async cleanIconCache(): Promise<string> {
    return this.api.invoke<string>('clean_icon_cache');
  }
}
