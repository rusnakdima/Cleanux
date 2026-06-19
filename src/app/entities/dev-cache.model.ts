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
