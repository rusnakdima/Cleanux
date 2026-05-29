export interface QuickAction {
  id: string;
  name: string;
  description: string;
  icon: string;
  actions: ActionStep[];
}

export interface ActionStep {
  CleanCategory?: { category: string };
  RunProfile?: { profileName: string };
  ExecuteCommand?: { command: string };
  Wait?: { seconds: number };
}

export interface AutomationRecipe {
  id: string;
  name: string;
  steps: ActionStep[];
  enabled: boolean;
  trigger: 'Manual' | 'Scheduled' | 'Event';
}

export interface ExecutionHistoryEntry {
  id: string;
  name: string;
  status: string;
  startedAt: string;
  completedAt?: string;
  stepsExecuted: number;
  totalSteps: number;
}
