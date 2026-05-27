export interface SystemServiceItem {
  name: string;
  description?: string;
  load?: string;
  active: string;
  status: string;
  isRunning: boolean;
}

export interface CacheFileItem {
  path: string;
  size: number;
  modified: string;
}

export interface TrashFileItem {
  name: string;
  path: string;
  size: number;
  deletedDate: string;
}

export interface LogFileItem {
  path: string;
  size: number;
  modified: string;
}

export interface LargeFileItem {
  name: string;
  path: string;
  size: number;
  modified: string;
}

export interface ScanSummary {
  totalSize: number;
  fileCount: number;
}

export interface ProcessItem {
  pid: number;
  name: string;
  cpu_usage: number;
  memory_usage: number;
}

export interface PaginatedData<T> {
  data: T[];
  has_more: boolean;
  total: number;
}
