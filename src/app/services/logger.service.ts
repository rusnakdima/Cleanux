import { Injectable } from '@angular/core';
import { getLoggingService, LogEntry } from '@tauri-apps/logger';

export type { LogEntry };

export type LogFilter = {
  level?: 'warn' | 'error';
  source?: string;
  search?: string;
  since?: Date;
  until?: Date;
  operationId?: string;
};

export type LogLevel = 'warn' | 'error';

export interface LogStats {
  total: number;
  byLevel: Record<LogLevel, number>;
  bySource: Record<string, number>;
}

@Injectable({
  providedIn: 'root',
})
export class LoggerService {
  private logger = getLoggingService();

  get consoleOutput(): boolean {
    return this.logger.consoleOutput;
  }

  set consoleOutput(value: boolean) {
    this.logger.setConsoleOutput(value);
  }

  get memoryOutput(): boolean {
    return this.logger.memoryOutput;
  }

  set memoryOutput(value: boolean) {
    this.logger.setMemoryOutput(value);
  }

  get fileOutput(): boolean {
    return this.logger.fileOutput;
  }

  set fileOutput(value: boolean) {
    this.logger.setFileOutput(value);
  }

  logInfo(source: string, context?: string, operation?: string, message?: string): void {
    const msg = message || context || '';
    const ctx = operation ? { operation } : undefined;
    this.logger.info(source, msg, ctx);
  }

  logError(
    source: string,
    context: string | undefined,
    operation: string | undefined,
    message: string,
    error?: unknown,
    extraContext?: Record<string, unknown>
  ): void {
    this.logger.error(source, message, error, extraContext);
  }

  logDebug(
    source: string,
    context: string | undefined,
    operation: string | undefined,
    message: string
  ): void {
    this.logger.debug(source, message, { source, context, operation });
  }

  info(message: string, context?: Record<string, unknown>, data?: unknown): void {
    this.logger.info('', message, context, data);
  }

  debug(message: string, context?: Record<string, unknown>, data?: unknown): void {
    this.logger.debug('', message, context, data);
  }

  warn(message: string, context?: Record<string, unknown>, data?: unknown): void {
    this.logger.warn('', message, context, data);
  }

  error(message: string, error?: unknown, context?: Record<string, unknown>): void {
    this.logger.error('', message, error, context);
  }

  startOperation(name: string, context?: Record<string, unknown>): string {
    return this.logger.startOperation(name, context);
  }

  completeOperation(name: string, operationId: string, success?: boolean, data?: unknown): void {
    this.logger.completeOperation(name, operationId, success, data);
  }

  getLogs(filter?: LogFilter): LogEntry[] {
    return this.logger.getLogs(filter);
  }

  getLogStats(): LogStats {
    return this.logger.getLogStats() as LogStats;
  }

  clearLogs(): void {
    this.logger.clearLogs();
  }
}
