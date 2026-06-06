/* sys lib */
import { Injectable } from '@angular/core';

/* services */
import { BaseApiService } from '@services/base-api.service';
import { MediaCacheSummary } from '@models/media-cache.model';

@Injectable({
  providedIn: 'root',
})
export class MediaCacheService extends BaseApiService {
  async getMediaCacheSummary(): Promise<MediaCacheSummary> {
    return this.call<MediaCacheSummary>('get_media_cache_summary');
  }

  async cleanSteamShaderCache(): Promise<string> {
    return this.call<string>('clean_steam_shader_cache');
  }

  async cleanSteamDownloadCache(): Promise<string> {
    return this.call<string>('clean_steam_download_cache');
  }

  async cleanSpotifyCache(): Promise<string> {
    return this.call<string>('clean_spotify_cache');
  }

  async cleanVlcCache(): Promise<string> {
    return this.call<string>('clean_vlc_cache');
  }

  async cleanThumbnailCache(): Promise<string> {
    return this.call<string>('clean_thumbnail_cache');
  }

  async cleanIconCache(): Promise<string> {
    return this.call<string>('clean_icon_cache');
  }
}
