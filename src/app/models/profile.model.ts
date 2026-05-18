export interface CleaningProfile {
  name: string;
  description: string;
  created_at: string;
  paths: string[];
  exclude_patterns: string[];
  clean_cache: boolean;
  clean_trash: boolean;
  clean_logs: boolean;
  min_large_file_size: number;
}

export function createEmptyProfile(): CleaningProfile {
  return {
    name: '',
    description: '',
    created_at: new Date().toISOString(),
    paths: [],
    exclude_patterns: [],
    clean_cache: true,
    clean_trash: true,
    clean_logs: false,
    min_large_file_size: 100 * 1024 * 1024,
  };
}
