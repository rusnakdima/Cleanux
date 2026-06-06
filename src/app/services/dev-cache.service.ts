import { Injectable } from '@angular/core';
import { BaseApiService } from '@services/base-api.service';
import { DevCacheItem, DevCacheSummary } from '@models/dev-cache.model';

@Injectable({
  providedIn: 'root',
})
export class DevCacheService extends BaseApiService {
  async getDevCacheSummary(): Promise<DevCacheSummary> {
    return this.call<DevCacheSummary>('get_dev_cache_summary');
  }

  async cleanNpmCache(): Promise<string> {
    return this.call<string>('clean_npm_cache');
  }

  async cleanPipCache(): Promise<string> {
    return this.call<string>('clean_pip_cache');
  }

  async cleanCargoCache(): Promise<string> {
    return this.call<string>('clean_cargo_cache');
  }

  async cleanGoCache(): Promise<string> {
    return this.call<string>('clean_go_cache');
  }

  async cleanMavenCache(): Promise<string> {
    return this.call<string>('clean_maven_cache');
  }

  async cleanGradleCache(): Promise<string> {
    return this.call<string>('clean_gradle_cache');
  }

  async cleanAllDevCaches(): Promise<string> {
    return this.call<string>('clean_all_dev_caches');
  }
}
