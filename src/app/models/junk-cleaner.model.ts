export interface JunkItem {
  path: string;
  size: number;
  category: string;
  description: string;
  fileCount: number;
}

export interface JunkCategorySummary {
  category: string;
  totalSize: number;
  fileCount: number;
  description: string;
  items: JunkItem[];
}

export type JunkCategoryKey = 'browser' | 'thumbnails' | 'applications' | 'system' | 'logs';

export const JUNK_CATEGORIES: { key: JunkCategoryKey; label: string; icon: string }[] = [
  { key: 'browser', label: 'Browser', icon: 'globe' },
  { key: 'thumbnails', label: 'Thumbnails', icon: 'image' },
  { key: 'applications', label: 'Applications', icon: 'apps' },
  { key: 'system', label: 'System', icon: 'computer' },
  { key: 'logs', label: 'Logs', icon: 'description' },
];
