export type CleaningType = 'cache' | 'trash' | 'logs' | 'largefiles' | 'all';

export interface ScheduleConfig {
  enabled: boolean;
  interval_hours: number;
  cleaning_type: CleaningType;
  paths: string[];
  last_run: string | null;
  next_run: string | null;
}

export const defaultScheduleConfig: ScheduleConfig = {
  enabled: false,
  interval_hours: 24,
  cleaning_type: 'all',
  paths: [],
  last_run: null,
  next_run: null,
};
