import { Injectable } from '@angular/core';
import { ApiService } from '@services/api.service';

export interface DevCacheItem {
  name: string;
  cache_path: string;
  size: number;
  description: string;
}

export interface DevCacheSummary {
  npm: DevCacheItem;
  pip: DevCacheItem;
  cargo: DevCacheItem;
  go: DevCacheItem;
  maven: DevCacheItem;
  gradle: DevCacheItem;
}

@Injectable({
  providedIn: 'root',
})
export class DevCacheService {
  constructor(private api: ApiService) {}

  async getDevCacheSummary(): Promise<DevCacheSummary> {
    return this.api.invoke<DevCacheSummary>('get_dev_cache_summary');
  }

  async cleanNpmCache(): Promise<string> {
    return this.api.invoke<string>('clean_npm_cache');
  }

  async cleanPipCache(): Promise<string> {
    return this.api.invoke<string>('clean_pip_cache');
  }

  async cleanCargoCache(): Promise<string> {
    return this.api.invoke<string>('clean_cargo_cache');
  }

  async cleanGoCache(): Promise<string> {
    return this.api.invoke<string>('clean_go_cache');
  }

  async cleanMavenCache(): Promise<string> {
    return this.api.invoke<string>('clean_maven_cache');
  }

  async cleanGradleCache(): Promise<string> {
    return this.api.invoke<string>('clean_gradle_cache');
  }

  async cleanAllDevCaches(): Promise<string> {
    return this.api.invoke<string>('clean_all_dev_caches');
  }
}
