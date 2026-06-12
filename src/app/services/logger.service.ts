import { Injectable, signal } from '@angular/core';
import { environment } from '@env/environment';

export type LogLevel = 'debug' | 'info' | 'warn' | 'error';
export type LogSource = 'view' | 'service' | 'store' | 'api' | 'user' | 'router';

export interface LogEntry {
  id: string;
  timestamp: Date;
  level: LogLevel;
  source: LogSource;
  view?: string;
  method?: string;
  message: string;
  data?: unknown;
  error?: { message: string; stack?: string };
}

export interface LogFilter {
  level?: LogLevel;
  source?: LogSource;
  view?: string;
  method?: string;
  search?: string;
  since?: Date;
  until?: Date;
}

const SENSITIVE_PATTERNS = [
  { pattern: /password/gi, replacement: '[REDACTED]' },
  { pattern: /token/gi, replacement: '[REDACTED]' },
  { pattern: /api[_-]?key/gi, replacement: '[REDACTED]' },
  { pattern: /secret/gi, replacement: '[REDACTED]' },
  { pattern: /bearer/gi, replacement: '[REDACTED]' },
  { pattern: /auth/gi, replacement: '[REDACTED]' },
];

const LEVEL_PRIORITY: Record<LogLevel, number> = {
  debug: 0,
  info: 1,
  warn: 2,
  error: 3,
};

@Injectable({
  providedIn: 'root',
})
export class LoggerService {
  private logBuffer = signal<LogEntry[]>([]);
  private minLevel = signal<LogLevel>('debug');
  private entryCounter = 0;
  private config = environment.logging;
  private enabled = this.config.enabled;

  readonly logs = this.logBuffer.asReadonly();

  constructor() {
    const savedLevel = localStorage.getItem('cleanux_log_level') as LogLevel;
    if (savedLevel && ['debug', 'info', 'warn', 'error'].includes(savedLevel)) {
      this.minLevel.set(savedLevel);
    }
  }

  private sanitize(data: unknown): unknown {
    if (typeof data === 'string') {
      let result = data;
      for (const { pattern, replacement } of SENSITIVE_PATTERNS) {
        result = result.replace(pattern, replacement);
      }
      return result;
    }

    if (Array.isArray(data)) {
      return data.map((item) => this.sanitize(item));
    }

    if (data && typeof data === 'object') {
      const sanitized: Record<string, unknown> = {};
      for (const [key, value] of Object.entries(data)) {
        let sanitizedKey = key;
        for (const { pattern, replacement } of SENSITIVE_PATTERNS) {
          sanitizedKey = sanitizedKey.replace(pattern, replacement);
        }
        sanitized[sanitizedKey] = this.sanitize(value);
      }
      return sanitized;
    }

    return data;
  }

  private createEntry(
    level: LogLevel,
    source: LogSource,
    view: string | undefined,
    method: string | undefined,
    message: string,
    data?: unknown,
    error?: { message: string; stack?: string }
  ): LogEntry {
    const id = `log_${++this.entryCounter}`;
    return {
      id,
      timestamp: new Date(),
      level,
      source,
      view,
      method,
      message,
      data: data ? this.sanitize(data) : undefined,
      error: error ? { message: error.message, stack: error.stack } : undefined,
    };
  }

  private shouldLog(entry: LogEntry): boolean {
    if (!this.enabled) return false;
    if (!this.isLevelEnabled(entry.level)) return false;
    if (!this.isSourceEnabled(entry.source)) return false;
    return LEVEL_PRIORITY[entry.level] >= LEVEL_PRIORITY[this.minLevel()];
  }

  isLevelEnabled(level: LogLevel): boolean {
    return this.config.levels[level] ?? true;
  }

  isSourceEnabled(source: LogSource): boolean {
    return this.config.sources[source] ?? true;
  }

  getLevelsConfig(): Record<LogLevel, boolean> {
    return this.config.levels;
  }
  getSourcesConfig(): Record<LogSource, boolean> {
    return this.config.sources;
  }
  isGlobalEnabled(): boolean {
    return this.enabled;
  }

  private emit(entry: LogEntry): void {
    if (!this.shouldLog(entry)) return;

    const consoleMsg = `[${entry.source.toUpperCase()}] ${entry.timestamp.toISOString()} - ${entry.message}`;

    switch (entry.level) {
      case 'debug':
        console.debug(consoleMsg, entry.data || '');
        break;
      case 'info':
        console.info(consoleMsg, entry.data || '');
        break;
      case 'warn':
        console.warn(consoleMsg, entry.data || '');
        break;
      case 'error':
        console.error(consoleMsg, entry.data || '', entry.error || '');
        break;
    }

    this.logBuffer.update((logs) => [...logs.slice(-999), entry]);
  }

  log(
    level: LogLevel,
    source: LogSource,
    view: string | undefined,
    method: string | undefined,
    message: string,
    data?: unknown
  ): void {
    this.emit(this.createEntry(level, source, view, method, message, data));
  }

  logDebug(
    source: LogSource,
    view: string | undefined,
    method: string | undefined,
    message: string,
    data?: unknown
  ): void {
    this.log('debug', source, view, method, message, data);
  }

  logInfo(
    source: LogSource,
    view: string | undefined,
    method: string | undefined,
    message: string,
    data?: unknown
  ): void {
    this.log('info', source, view, method, message, data);
  }

  logWarn(
    source: LogSource,
    view: string | undefined,
    method: string | undefined,
    message: string,
    data?: unknown
  ): void {
    this.log('warn', source, view, method, message, data);
  }

  logError(
    source: LogSource,
    view: string | undefined,
    method: string | undefined,
    message: string,
    error: Error,
    data?: unknown
  ): void {
    this.emit(
      this.createEntry('error', source, view, method, message, data, {
        message: error.message,
        stack: error.stack,
      })
    );
  }

  logUserAction(view: string, action: string, data?: object): void {
    this.log('info', 'user', view, action, `User action: ${action}`, data);
  }

  logStateChange(view: string, signalName: string, oldValue: unknown, newValue: unknown): void {
    this.log('debug', 'store', view, signalName, `State change: ${signalName}`, {
      oldValue,
      newValue,
    });
  }

  logApiCall(
    command: string,
    args: Record<string, unknown>,
    response?: unknown,
    error?: Error
  ): void {
    if (error) {
      this.logError('api', undefined, command, `API call failed: ${command}`, error, { args });
    } else {
      this.log('info', 'api', undefined, command, `API call: ${command}`, {
        args,
        responseSize: response ? JSON.stringify(response).length : 0,
      });
    }
  }

  setMinLevel(level: LogLevel): void {
    this.minLevel.set(level);
    localStorage.setItem('cleanux_log_level', level);
  }

  getMinLevel(): LogLevel {
    return this.minLevel();
  }

  getLogs(filter?: LogFilter): LogEntry[] {
    let logs = this.logBuffer();

    if (filter) {
      if (filter.level) {
        logs = logs.filter((l) => l.level === filter.level);
      }
      if (filter.source) {
        logs = logs.filter((l) => l.source === filter.source);
      }
      if (filter.view) {
        logs = logs.filter((l) => l.view === filter.view);
      }
      if (filter.method) {
        logs = logs.filter((l) => l.method === filter.method);
      }
      if (filter.search) {
        const search = filter.search.toLowerCase();
        logs = logs.filter(
          (l) =>
            l.message.toLowerCase().includes(search) ||
            (l.data && JSON.stringify(l.data).toLowerCase().includes(search))
        );
      }
      if (filter.since) {
        logs = logs.filter((l) => l.timestamp >= filter.since!);
      }
      if (filter.until) {
        logs = logs.filter((l) => l.timestamp <= filter.until!);
      }
    }

    return logs;
  }

  getLogStats(): {
    total: number;
    byLevel: Record<string, number>;
    bySource: Record<string, number>;
  } {
    const logs = this.logBuffer();
    const byLevel: Record<string, number> = {};
    const bySource: Record<string, number> = {};

    for (const log of logs) {
      byLevel[log.level] = (byLevel[log.level] || 0) + 1;
      bySource[log.source] = (bySource[log.source] || 0) + 1;
    }

    return { total: logs.length, byLevel, bySource };
  }

  clearLogs(): void {
    this.logBuffer.set([]);
  }

  getRecentLogs(count: number = 100): LogEntry[] {
    return this.logBuffer().slice(-count);
  }
}
