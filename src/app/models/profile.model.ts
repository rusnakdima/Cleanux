export interface CleaningProfile {
  name: string;
  description: string;
  createdAt: string;
  paths: string[];
  excludePatterns: string[];
  cleanCache: boolean;
  cleanTrash: boolean;
  cleanLogs: boolean;
  minLargeFileSize: number;
}

export function createEmptyProfile(): CleaningProfile {
  return {
    name: '',
    description: '',
    createdAt: new Date().toISOString(),
    paths: [],
    excludePatterns: [],
    cleanCache: true,
    cleanTrash: true,
    cleanLogs: false,
    minLargeFileSize: 100 * 1024 * 1024,
  };
}
