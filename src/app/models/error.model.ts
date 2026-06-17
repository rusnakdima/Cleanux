export interface ApiError {
  message: string;
  code?: string;
  details?: unknown;
}

export class ApiException extends Error {
  constructor(
    message: string,
    public code?: string,
    public details?: unknown
  ) {
    super(message);
    this.name = 'ApiException';
  }
}

export enum TauriApiErrorCode {
  Timeout = 'TIMEOUT',
  ConnectionFailed = 'CONNECTION_FAILED',
  PermissionDenied = 'PERMISSION_DENIED',
  NotFound = 'NOT_FOUND',
  Unknown = 'UNKNOWN',
}

export interface TauriApiError {
  code: TauriApiErrorCode;
  message: string;
  context?: string;
  timestamp: number;
  retryable: boolean;
}

export interface AppError {
  code: string;
  message: string;
  context?: string;
  timestamp: number;
  retryable: boolean;
}

export function isTauriApiError(error: unknown): error is TauriApiError {
  return (
    typeof error === 'object' &&
    error !== null &&
    'code' in error &&
    'message' in error
  );
}