import { Injectable, inject } from '@angular/core';
import { TauriApiService } from '@app/api/tauri-api.service';

export type LogLevel = 'debug' | 'warn' | 'error' | 'info';

export interface LogEntry {
  id?: string;
  level: string;
  component?: string;
  source?: string;
  message: string;
  timestamp: string;
  context?: string;
  data?: unknown;
  error?: { message: string; stack?: string };
}

export interface LogFilter {
  level?: string;
  source?: string;
  search?: string;
  since?: Date;
  until?: Date;
}

export interface LogStats {
  total: number;
  byLevel: Record<string, number>;
  bySource: Record<string, number>;
  oldest: string | null;
  newest: string | null;
}

@Injectable({ providedIn: 'root' })
export class LoggerService {
  private api = inject(TauriApiService);
  private logs: LogEntry[] = [];
  private maxLogs = 1000;

  consoleOutput = true;
  memoryOutput = true;
  fileOutput = false;

  debug(message: string, data?: Record<string, unknown>): void {
    console.debug(message, data || '');
    this.api.logMessage('debug', 'app', message);
  }

  info(message: string, data?: Record<string, unknown>): void {
    console.info(message, data || '');
    this.api.logMessage('info', 'app', message);
  }

  warn(message: string, data?: Record<string, unknown>): void {
    console.warn(message, data || '');
    this.api.logMessage('warn', 'app', message);
  }

  error(message: string, error?: unknown, data?: Record<string, unknown>): void {
    console.error(message, error, data || '');
    this.api.logMessage('error', 'app', message);
  }

  logDebug(source: string, context: any, method: string, message: string): void {
    console.debug(`[${source}]`, message);
    this.api.logMessage('debug', source, message);
  }

  logInfo(source: string, message: string): void {
    console.info(`[${source}]`, message);
    this.api.logMessage('info', source, message);
  }

  logWarn(source: string, message: string): void {
    console.warn(`[${source}]`, message);
    this.api.logMessage('warn', source, message);
  }

  logError(
    source: string,
    context: any,
    method: string,
    message: string,
    error?: Error,
    extra?: Record<string, unknown>
  ): void {
    console.error(`[${source}]`, message, error, extra);
    this.api.logMessage('error', source, message);
  }

  getLogs(filter?: LogFilter): LogEntry[] {
    let result = [...this.logs];
    if (filter?.level) {
      result = result.filter((log) => log.level === filter.level);
    }
    if (filter?.source) {
      result = result.filter((log) => log.source === filter.source);
    }
    if (filter?.search) {
      const search = filter.search.toLowerCase();
      result = result.filter((log) => log.message.toLowerCase().includes(search));
    }
    return result;
  }

  getLogStats(): LogStats {
    const byLevel: Record<string, number> = {};
    const bySource: Record<string, number> = {};
    let oldest: string | null = null;
    let newest: string | null = null;

    for (const log of this.logs) {
      byLevel[log.level] = (byLevel[log.level] || 0) + 1;
      const source = log.source || 'unknown';
      bySource[source] = (bySource[source] || 0) + 1;
    }

    if (this.logs.length > 0) {
      oldest = this.logs[0]?.timestamp || null;
      newest = this.logs[this.logs.length - 1]?.timestamp || null;
    }

    return {
      total: this.logs.length,
      byLevel,
      bySource,
      oldest,
      newest,
    };
  }

  clearLogs(): void {
    this.logs = [];
  }
}

const api = new TauriApiService();

export const logger = {
  debug(message: string, component: string = 'app'): void {
    console.debug(`[${component}]`, message);
    api.logMessage('debug', component, message);
  },
  warn(message: string, component: string = 'app'): void {
    console.warn(`[${component}]`, message);
    api.logMessage('warn', component, message);
  },
  error(message: string, component: string = 'app'): void {
    console.error(`[${component}]`, message);
    api.logMessage('error', component, message);
  },
  info(message: string, component: string = 'app'): void {
    console.info(`[${component}]`, message);
    api.logMessage('info', component, message);
  },
};
