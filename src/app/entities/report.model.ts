export interface ReportCategories {
  cache: number;
  trash: number;
  logs: number;
  large_files: number;
  duplicates: number;
}

export interface CleaningReport {
  id?: number;
  date: string;
  items_cleaned: number;
  space_reclaimed: number;
  duration: number;
  categories: ReportCategories;
}

export interface SnapshotComparison {
  before_id: number;
  after_id: number;
  space_reclaimed: number;
  items_cleaned: number;
  health_improvement: number;
  details: ComparisonDetails;
}

export interface ComparisonDetails {
  cache_change: number;
  trash_change: number;
  log_change: number;
  large_file_change: number;
}
