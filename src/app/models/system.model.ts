import type {
  CacheFile,
  TrashFile,
  LogFile,
  LargeFile,
  ScanSummary,
  SystemService,
  PaginatedData,
  CacheFileItem,
  TrashFileItem,
  LogFileItem,
  LargeFileItem,
  SystemServiceItem,
} from './types-generated';

export type {
  CacheFile,
  TrashFile,
  LogFile,
  LargeFile,
  ScanSummary,
  SystemService,
  PaginatedData,
  CacheFileItem,
  TrashFileItem,
  LogFileItem,
  LargeFileItem,
  SystemServiceItem,
};

export interface ProcessItem {
  pid: number;
  name: string;
  cpu_usage: number;
  memory_usage: number;
}