/**
 * Shared TypeScript types mirroring Rust models from src-tauri/src/models/
 * These types are manually maintained to match their Rust counterparts.
 * snake_case in Rust → camelCase in TypeScript
 */

import { Response } from '@tauri-front/shared';

export interface PaginatedData<T> {
  data: T[];
  has_more: boolean;
  total: number;
}

export {
  isSuccess,
  isError,
  getErrorMessage,
  unwrapResponse,
  mapResponse,
} from '@tauri-front/shared';

// ============================================================
// src-tauri/src/models/cleaner.model.rs
// ============================================================

export interface CacheFile {
  path: string;
  size: number;
  modified: string;
}

export interface LargeFile {
  name: string;
  path: string;
  size: number;
  modified: string;
}

export interface LogFile {
  path: string;
  size: number;
  modified: string;
}

export interface TrashFile {
  name: string;
  path: string;
  size: number;
  deletedDate: string;
}

export interface ScanSummary {
  fileCount: number;
  totalSize: number;
}

export interface CleaningProfile {
  name: string;
  description: string;
  createdAt: string;
  paths: string[];
  excludePatterns: string[];
  cleanCache: boolean;
  cleanTrash: boolean;
  cleanLogs: boolean;
  minLargeFileSize: number;
}

// ============================================================
// src-tauri/src/models/system.model.rs
// ============================================================

export interface SystemService {
  name: string;
  description: string;
  status: string;
  isRunning: boolean;
}

// ============================================================
// src-tauri/src/helpers/constants.helper.rs
// ============================================================

export const LARGE_FILE_THRESHOLD = 100 * 1024 * 1024;
export const CRITICAL_JUNK_SIZE = 2 * 1024 * 1024 * 1024;
export const WARNING_JUNK_SIZE = 500 * 1024 * 1024;
export const DEFAULT_JUNK_THRESHOLD = 256 * 1024 * 1024;

export const HEALTHY_SCORE = 95;
export const WARNING_SCORE = 65;
export const CRITICAL_SCORE = 40;

// ============================================================
// Type aliases for backward compatibility
// ============================================================

export type ResponseModel<T = unknown> = Response<T>;
export type CacheFileItem = CacheFile;
export type TrashFileItem = TrashFile;
export type LogFileItem = LogFile;
export type LargeFileItem = LargeFile;
export type SystemServiceItem = SystemService;
