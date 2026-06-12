import { Injectable, inject, signal } from '@angular/core';
import { LoggerService } from '@services/logger.service';
import { ApiException } from '@models/error.model';
import { getErrorMessage } from '@shared/utils/error.util';

export interface ErrorContext {
  command: string;
  args?: Record<string, unknown>;
  timestamp: Date;
}

export type ErrorHandler = (error: ApiException, context: ErrorContext) => void;

@Injectable({
  providedIn: 'root',
})
export class ErrorInterceptorService {
  private handlers: ErrorHandler[] = [];
  private errorLog = signal<Array<{ error: ApiException; context: ErrorContext }>>([]);

  readonly errors = this.errorLog.asReadonly();

  constructor(private logger: LoggerService) {}

  registerHandler(handler: ErrorHandler): void {
    this.handlers.push(handler);
  }

  unregisterHandler(handler: ErrorHandler): void {
    this.handlers = this.handlers.filter((h) => h !== handler);
  }

  handleError(error: ApiException, context: ErrorContext): void {
    this.errorLog.update((log) => [...log, { error, context }]);
    this.handlers.forEach((handler) => handler(error, context));
  }

  clearErrors(): void {
    this.errorLog.set([]);
  }

  handle(error: unknown, command: string, args?: Record<string, unknown>): void {
    const apiError =
      error instanceof ApiException
        ? error
        : new ApiException(getErrorMessage(error), command, error);
    this.handleError(apiError, { command, args, timestamp: new Date() });
    this.logger.log('error', 'api', 'ErrorInterceptor', 'handle', apiError.message, {
      command,
      args,
      error: apiError,
    });
  }
}
