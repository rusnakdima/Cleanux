import { Response } from '@entities/response.model';

export function createSuccessResponse<T>(data: T, message = 'Success'): Response<T> {
  return {
    status: 'success',
    message,
    data,
  };
}

export function createErrorResponse(message = 'Error'): Response<null> {
  return {
    status: 'error',
    message,
    data: null,
  };
}

export function createMockCacheFiles(count = 5) {
  return Array.from({ length: count }, (_, i) => ({
    path: `/cache/file${i}.tmp`,
    size: (i + 1) * 1024 * 1024,
    modified: new Date(Date.now() - i * 86400000).toISOString(),
  }));
}

export function createMockTrashFiles(count = 3) {
  return Array.from({ length: count }, (_, i) => ({
    name: `trash${i}.bin`,
    path: `/home/user/.local/share/Trash/files/trash${i}.bin`,
    size: (i + 1) * 2 * 1024 * 1024,
    deletedDate: new Date(Date.now() - i * 86400000).toISOString(),
  }));
}

export function createMockLogFiles(count = 4) {
  return Array.from({ length: count }, (_, i) => ({
    path: `/var/log/syslog.${i}`,
    size: (i + 1) * 512 * 1024,
    modified: new Date(Date.now() - i * 86400000).toISOString(),
  }));
}

export function createMockLargeFiles(count = 3) {
  return Array.from({ length: count }, (_, i) => ({
    name: `large_video_${i}.mp4`,
    path: `/home/user/Videos/large_video_${i}.mp4`,
    size: (i + 1) * 1024 * 1024 * 1024,
    modified: new Date(Date.now() - i * 86400000).toISOString(),
  }));
}

export function createMockProfiles(count = 3) {
  return Array.from({ length: count }, (_, i) => ({
    name: `Profile ${i + 1}`,
    description: `Test profile ${i + 1}`,
    created_at: new Date(Date.now() - i * 86400000).toISOString(),
    paths: [`/path/to/clean${i}`],
    exclude_patterns: ['*.log', '*.tmp'],
    clean_cache: i % 2 === 0,
    clean_trash: true,
    clean_logs: i % 3 === 0,
    min_large_file_size: (i + 1) * 100 * 1024 * 1024,
  }));
}

export function createMockServices(count = 5) {
  return Array.from({ length: count }, (_, i) => ({
    name: `service${i}.service`,
    description: `Test service ${i}`,
    load: 'loaded',
    active: 'active',
    status: 'running',
    isRunning: true,
  }));
}

export function createMockProcesses(count = 4) {
  return Array.from({ length: count }, (_, i) => ({
    pid: 1000 + i * 100,
    name: `process_${i}`,
    cpu_usage: Math.random() * 100,
    memory_usage: Math.random() * 50,
  }));
}

export function createMockBackupItems(count = 2) {
  return Array.from({ length: count }, (_, i) => ({
    name: `backup${i}.tar.gz`,
    path: `/home/user/.config/cleanux/backups/backup${i}.tar.gz`,
    size: (i + 1) * 50 * 1024 * 1024,
    modified: new Date(Date.now() - i * 86400000).toISOString(),
  }));
}

export function createMockQuickActions() {
  return [
    {
      id: 'qa1',
      name: 'Quick Clean',
      description: 'Clean cache and temp files',
      icon: 'broom',
      actions: [{ CleanCategory: { category: 'cache' } }],
    },
    {
      id: 'qa2',
      name: 'Deep Scan',
      description: 'Full system scan',
      icon: 'search',
      actions: [{ Wait: { seconds: 5 } }],
    },
  ];
}

export function createMockRecipes() {
  return [
    {
      id: 'rec1',
      name: 'Morning Cleanup',
      steps: [
        { CleanCategory: { category: 'cache' } },
        { Wait: { seconds: 2 } },
        { CleanCategory: { category: 'trash' } },
      ],
      enabled: true,
      trigger: 'Scheduled' as const,
    },
  ];
}

export function createMockExecutionHistory() {
  return [
    {
      id: 'exec1',
      name: 'Morning Cleanup',
      status: 'completed',
      startedAt: new Date(Date.now() - 3600000).toISOString(),
      completedAt: new Date(Date.now() - 3500000).toISOString(),
      stepsExecuted: 3,
      totalSteps: 3,
    },
  ];
}

export function createMockSystemStats() {
  return {
    cpu_usage: 45.5,
    memory_used: 8 * 1024 * 1024 * 1024,
    memory_total: 16 * 1024 * 1024 * 1024,
    memory_usage_percent: 50.0,
    disk_used: 500 * 1024 * 1024 * 1024,
    disk_total: 1000 * 1024 * 1024 * 1024,
    disk_usage_percent: 50.0,
  };
}

export function createMockPaginatedData<T>(items: T[], total: number, hasMore = false) {
  return {
    data: items,
    has_more: hasMore,
    total,
  };
}
