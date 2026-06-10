import { Injectable, signal } from '@angular/core';

export interface LogEntry {
  id: string;
  command: string;
  args?: Record<string, unknown>;
  result?: 'success' | 'error';
  duration?: number;
  timestamp: Date;
  error?: string;
}

@Injectable({
  providedIn: 'root',
})
export class LoggingInterceptorService {
  private logEntries = signal<LogEntry[]>([]);
  private entryCounter = 0;

  readonly logs = this.logEntries.asReadonly();

  log(command: string, args?: Record<string, unknown>): string {
    const id = `log_${++this.entryCounter}`;
    const entry: LogEntry = {
      id,
      command,
      args,
      timestamp: new Date(),
    };
    this.logEntries.update((logs) => [...logs, entry]);
    console.debug(`[API] ${command}`, args);
    return id;
  }

  complete(id: string, success: boolean, duration?: number, error?: string): void {
    this.logEntries.update((logs) =>
      logs.map((entry) =>
        entry.id === id
          ? { ...entry, result: success ? 'success' : 'error', duration, error }
          : entry
      )
    );
    if (success) {
      console.debug(`[API] Completed: ${id} (${duration}ms)`);
    } else {
      console.error(`[API] Failed: ${id} - ${error}`);
    }
  }

  clear(): void {
    this.logEntries.set([]);
  }

  getLogsByCommand(command: string): LogEntry[] {
    return this.logEntries().filter((entry) => entry.command === command);
  }
}
