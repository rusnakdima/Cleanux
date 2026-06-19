import { Injectable } from '@angular/core';
import { LogStorageService } from '@services/log-storage.service';
import { LogEntry, LogFilter } from '@entities/log-manager.model';

export interface LogStats {
  total: number;
  byLevel: Record<string, number>;
  bySource: Record<string, number>;
  oldest: string | null;
  newest: string | null;
}

export interface ProblemReport {
  generatedAt: string;
  appVersion: string;
  summary: {
    totalLogs: number;
    errors: number;
    warnings: number;
    bySource: Record<string, number>;
  };
  problems: ProblemEntry[];
  recentActions: LogEntry[];
}

export interface ProblemEntry {
  severity: 'critical' | 'high' | 'medium' | 'low';
  operation: string;
  errorMessage: string;
  occurrences: number;
  firstOccurrence: string;
  lastOccurrence: string;
  possibleCause: string;
  suggestedAction: string;
}

@Injectable({
  providedIn: 'root',
})
export class LogExportService {
  constructor(private storage: LogStorageService) {}

  async exportToJson(filter?: LogFilter): Promise<string> {
    const logs = await this.storage.getLogs(filter, 10000);
    return JSON.stringify(logs, null, 2);
  }

  async exportToCsv(filter?: LogFilter): Promise<string> {
    const logs = await this.storage.getLogs(filter, 10000);
    const headers = ['timestamp', 'level', 'source', 'message', 'data', 'error'];
    const rows = logs.map((log) =>
      [
        log.timestamp,
        log.level,
        log.source,
        `"${log.message.replace(/"/g, '""')}"`,
        log.data ? `"${JSON.stringify(log.data).replace(/"/g, '""')}"` : '',
        log.error ? `"${log.error.message.replace(/"/g, '""')}"` : '',
      ].join(',')
    );
    return [headers.join(','), ...rows].join('\n');
  }

  async exportToHtml(filter?: LogFilter): Promise<string> {
    const logs = await this.storage.getLogs(filter, 10000);
    const stats = await this.storage.getStats();

    const rows = logs
      .map(
        (log) => `
      <tr class="${log.level === 'error' ? 'bg-red-100' : log.level === 'warn' ? 'bg-yellow-100' : ''}">
        <td>${log.timestamp}</td>
        <td><span class="px-2 py-1 rounded text-xs font-semibold ${
          log.level === 'error'
            ? 'bg-red-500 text-white'
            : log.level === 'warn'
              ? 'bg-yellow-500 text-white'
              : log.level === 'info'
                ? 'bg-blue-500 text-white'
                : 'bg-gray-500 text-white'
        }">${log.level}</span></td>
        <td>${log.source}</td>
        <td>${log.message}</td>
        <td>${log.error ? `<pre class="text-red-500">${log.error.message}</pre>` : '-'}</td>
      </tr>
    `
      )
      .join('');

    return `
<!DOCTYPE html>
<html>
<head>
  <title>Cleanux Logs - ${new Date().toISOString()}</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 20px; }
    table { width: 100%; border-collapse: collapse; }
    th, td { border: 1px solid #ddd; padding: 8px; text-align: left; font-size: 12px; }
    th { background: #333; color: white; position: sticky; top: 0; }
    tr:nth-child(even) { background: #f9f9f9; }
    .stats { display: flex; gap: 20px; margin-bottom: 20px; }
    .stat-box { background: #f0f0f0; padding: 10px 20px; border-radius: 8px; }
    pre { white-space: pre-wrap; word-break: break-all; }
  </style>
</head>
<body>
  <h1>Cleanux Logs</h1>
  <div class="stats">
    <div class="stat-box"><strong>Total:</strong> ${stats.total}</div>
    <div class="stat-box"><strong>Errors:</strong> ${stats.byLevel['error'] || 0}</div>
    <div class="stat-box"><strong>Warnings:</strong> ${stats.byLevel['warn'] || 0}</div>
    <div class="stat-box"><strong>Oldest:</strong> ${stats.oldest || 'N/A'}</div>
    <div class="stat-box"><strong>Newest:</strong> ${stats.newest || 'N/A'}</div>
  </div>
  <table>
    <thead>
      <tr>
        <th>Timestamp</th>
        <th>Level</th>
        <th>Source</th>
        <th>Message</th>
        <th>Error</th>
      </tr>
    </thead>
    <tbody>
      ${rows}
    </tbody>
  </table>
</body>
</html>
    `;
  }

  generateShareableLink(logs: LogEntry[]): string {
    const data = JSON.stringify(logs);
    const compressed = this.compress(data);
    const hash = btoa(compressed).replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');
    return `${window.location.origin}${window.location.pathname}#logs=${hash}`;
  }

  parseShareableLink(hash: string): LogEntry[] | null {
    try {
      const base64 = hash.replace(/-/g, '+').replace(/_/g, '/');
      const compressed = atob(base64);
      const data = this.decompress(compressed);
      return JSON.parse(data);
    } catch {
      return null;
    }
  }

  private compress(data: string): string {
    const encoded = encodeURIComponent(data);
    let result = '';
    let i = 0;
    while (i < encoded.length) {
      const charCode = encoded.charCodeAt(i);
      if (charCode < 128) {
        result += String.fromCharCode(charCode);
        i++;
      } else if (charCode > 127) {
        result += String.fromCharCode((charCode & 0x3f) | 0x80);
        result += String.fromCharCode(((charCode >> 6) & 0x07) | 0xc0);
        i++;
      } else {
        const next = encoded.charCodeAt(i + 1);
        if (next > 127) {
          result += String.fromCharCode((next & 0x3f) | 0x80);
          result += String.fromCharCode(((next >> 6) & 0x07) | 0xc0);
          result += String.fromCharCode(((charCode >> 12) & 0x0f) | 0xe0);
          result += String.fromCharCode(((charCode >> 6) & 0x07) | 0x80);
          i += 2;
        } else {
          result += String.fromCharCode((next & 0x3f) | 0x80);
          result += String.fromCharCode(((next >> 6) & 0x07) | 0xc0);
          result += String.fromCharCode(((charCode >> 12) & 0x0f) | 0xe0);
          i += 2;
        }
      }
    }
    return result;
  }

  private decompress(data: string): string {
    const bytes: number[] = [];
    let i = 0;
    while (i < data.length) {
      const charCode = data.charCodeAt(i);
      if (charCode < 128) {
        bytes.push(charCode);
        i++;
      } else if ((charCode & 0xe0) === 0xc0) {
        const next = data.charCodeAt(i + 1);
        bytes.push(((charCode & 0x1f) << 6) | (next & 0x3f));
        i += 2;
      } else if ((charCode & 0xf0) === 0xe0) {
        const next1 = data.charCodeAt(i + 1);
        const next2 = data.charCodeAt(i + 2);
        bytes.push(((charCode & 0x0f) << 12) | ((next1 & 0x3f) << 6) | (next2 & 0x3f));
        i += 3;
      } else {
        bytes.push(charCode);
        i++;
      }
    }
    return decodeURIComponent(String.fromCharCode(...bytes));
  }

  async generateProblemReport(logs: LogEntry[], appVersion: string): Promise<ProblemReport> {
    const errors = logs.filter((l) => l.level === 'error');
    const warnings = logs.filter((l) => l.level === 'warn');
    const bySource: Record<string, number> = {};
    for (const log of logs) {
      const source = log.source || 'unknown';
      bySource[source] = (bySource[source] || 0) + 1;
    }

    const groupedErrors = this.groupErrors(errors);
    const problems: ProblemEntry[] = groupedErrors.map((group) => this.analyzeErrorGroup(group));

    return {
      generatedAt: new Date().toISOString(),
      appVersion,
      summary: {
        totalLogs: logs.length,
        errors: errors.length,
        warnings: warnings.length,
        bySource,
      },
      problems,
      recentActions: logs.slice(-50),
    };
  }

  private groupErrors(errors: LogEntry[]): LogEntry[][] {
    const groups: Map<string, LogEntry[]> = new Map();

    for (const error of errors) {
      const key = this.getErrorKey(error);
      if (!groups.has(key)) {
        groups.set(key, []);
      }
      groups.get(key)!.push(error);
    }

    return Array.from(groups.values());
  }

  private getErrorKey(error: LogEntry): string {
    const errorObj = error.error as { message?: string } | undefined;
    const message = errorObj?.message || error.message;
    const firstLine = message.split('\n')[0].substring(0, 100);
    return `${error.source || 'unknown'}:${firstLine}`;
  }

  private analyzeErrorGroup(group: LogEntry[]): ProblemEntry {
    const first = group[0];
    const last = group[group.length - 1];
    const method = first.source || 'unknown';
    const firstError = first.error as { message?: string } | undefined;
    const message = firstError?.message || first.message;

    let severity: 'critical' | 'high' | 'medium' | 'low' = 'medium';
    if (group.length > 10) severity = 'critical';
    else if (group.length > 5) severity = 'high';
    else if (group.length > 2) severity = 'medium';
    else severity = 'low';

    const possibleCauses = this.suggestCauses(method, message);
    const suggestedActions = this.suggestActions(method, message);

    return {
      severity,
      operation: method,
      errorMessage: message.substring(0, 200),
      occurrences: group.length,
      firstOccurrence: first.timestamp,
      lastOccurrence: last.timestamp,
      possibleCause: possibleCauses[0],
      suggestedAction: suggestedActions[0],
    };
  }

  private suggestCauses(method: string, message: string): string[] {
    const causes: string[] = [];

    if (method.includes('invoke') || method.includes('ApiCall')) {
      causes.push('Backend command failed - check if Tauri backend is running');
      causes.push('Network timeout or connection issue');
    }

    if (message.includes('ENOENT') || message.includes('not found')) {
      causes.push('File or directory does not exist');
      causes.push('Path was deleted or moved');
    }

    if (message.includes('permission') || message.includes('Permission')) {
      causes.push('Insufficient permissions to perform operation');
      causes.push('File may be owned by another user');
    }

    if (message.includes('timeout')) {
      causes.push('Operation took too long and was aborted');
      causes.push('System may be under heavy load');
    }

    if (causes.length === 0) {
      causes.push('Unexpected error occurred');
      causes.push('Possible bug or compatibility issue');
    }

    return causes;
  }

  private suggestActions(method: string, message: string): string[] {
    const actions: string[] = [];

    if (method.includes('clear') || method.includes('clean')) {
      actions.push('Try running the operation again with admin privileges');
      actions.push('Check if target files are in use by another process');
    }

    if (method.includes('load') || method.includes('get')) {
      actions.push('Refresh the page and try again');
      actions.push('Check if the required data exists');
    }

    if (message.includes('permission')) {
      actions.push('Run the application with elevated permissions');
      actions.push('Check file ownership and permissions');
    }

    if (actions.length === 0) {
      actions.push('Try restarting the application');
      actions.push('Check application logs for more details');
    }

    return actions;
  }
}
