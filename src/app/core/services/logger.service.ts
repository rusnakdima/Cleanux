import { Injectable, inject } from '@angular/core';
import { TauriApiService } from '@app/api/tauri-api.service';

export enum LogLevel {
  Debug = 0,
  Info = 1,
  Warn = 2,
  Error = 3,
}

export interface LogEntry {
  timestamp: string;
  level: LogLevel;
  message: string;
  context?: string;
  data?: any;
}

@Injectable({ providedIn: 'root' })
export class CoreLoggerService {
  private api = inject(TauriApiService);
  private logs: LogEntry[] = [];
  private maxLogs = 1000;
  private level = LogLevel.Info;

  private logToBackend(level: string, message: string, context?: string): void {
    this.api.logMessage(level, context || 'app', message);
  }

  private format(level: LogLevel, message: string, context?: string, data?: any): LogEntry {
    return {
      timestamp: new Date().toISOString(),
      level,
      message,
      context,
      data,
    };
  }

  debug(message: string, context?: string, data?: any): void {
    if (this.level <= LogLevel.Debug) {
      this.log(this.format(LogLevel.Debug, message, context, data), context);
    }
  }

  info(message: string, context?: string, data?: any): void {
    if (this.level <= LogLevel.Info) {
      this.log(this.format(LogLevel.Info, message, context, data), context);
    }
  }

  warn(message: string, context?: string, data?: any): void {
    if (this.level <= LogLevel.Warn) {
      this.log(this.format(LogLevel.Warn, message, context, data), context);
    }
  }

  error(message: string, context?: string, data?: any): void {
    if (this.level <= LogLevel.Error) {
      this.log(this.format(LogLevel.Error, message, context, data), context);
    }
  }

  private log(entry: LogEntry, context?: string): void {
    console.log(`[${entry.level}] ${entry.message}`, entry.data || '');
    this.logs.push(entry);
    if (this.logs.length > this.maxLogs) {
      this.logs.shift();
    }
    const levelStr = ['debug', 'info', 'warn', 'error'][entry.level];
    this.logToBackend(levelStr, entry.message, context);
  }

  getLogs(): LogEntry[] {
    return [...this.logs];
  }

  clearLogs(): void {
    this.logs = [];
  }

  setLevel(level: LogLevel): void {
    this.level = level;
  }
}
