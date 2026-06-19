export interface PackageManagerSummary {
  aptAvailable: boolean;
  aptCacheSize: number;
  aptAutoremoveSize: number;
  aptOrphanedCount: number;
  aptPartialDownloads: number;
  dnfAvailable: boolean;
  dnfCacheSize: number;
  pacmanAvailable: boolean;
  pacmanCacheSize: number;
  zypperAvailable: boolean;
  zypperCacheSize: number;
}

export interface OrphanedPackage {
  name: string;
  version: string;
  description: string;
}

export interface CleanResult {
  command: string;
  spaceFreed: number;
  message: string;
}

export interface DeepCleanResponse {
  totalSpaceFreed: number;
  results: CleanResult[];
}
