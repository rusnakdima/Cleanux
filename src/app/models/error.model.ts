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
