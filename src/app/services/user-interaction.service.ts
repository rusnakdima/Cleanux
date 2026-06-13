import { Injectable, inject, signal } from '@angular/core';
import { LoggerService } from './logger.service';

export interface UserAction {
  type:
    | 'click'
    | 'selection'
    | 'navigation'
    | 'form_submit'
    | 'copy'
    | 'scroll'
    | 'focus'
    | 'context_menu';
  view: string;
  element?: string;
  action: string;
  metadata?: Record<string, unknown>;
  timestamp: Date;
}

@Injectable({
  providedIn: 'root',
})
export class UserInteractionService {
  private logger = inject(LoggerService);
  private currentView = signal<string>('unknown');
  private actionHistory: UserAction[] = [];
  private maxHistory = 100;

  trackClick(view: string, element: string, metadata?: Record<string, unknown>): void {
    const action: UserAction = {
      type: 'click',
      view,
      element,
      action: 'click:' + element,
      metadata,
      timestamp: new Date(),
    };
    this.logAction(action);
  }

  trackSelection(
    view: string,
    type: string,
    selected: string[],
    metadata?: Record<string, unknown>
  ): void {
    const action: UserAction = {
      type: 'selection',
      view,
      action: 'select:' + type,
      metadata: { ...metadata, selected, count: selected.length },
      timestamp: new Date(),
    };
    this.logAction(action);
  }

  trackNavigation(view: string, route: string, params?: Record<string, unknown>): void {
    const action: UserAction = {
      type: 'navigation',
      view,
      action: 'navigate:' + route,
      metadata: params,
      timestamp: new Date(),
    };
    this.logAction(action);
  }

  trackFormSubmit(view: string, formId: string, data?: Record<string, unknown>): void {
    const action: UserAction = {
      type: 'form_submit',
      view,
      element: formId,
      action: 'submit:' + formId,
      metadata: data,
      timestamp: new Date(),
    };
    this.logAction(action);
  }

  trackCopy(text: string, source: string, view: string): void {
    const action: UserAction = {
      type: 'copy',
      view,
      action: 'copy:' + source,
      metadata: { textLength: text.length, truncated: text.length > 100 },
      timestamp: new Date(),
    };
    this.logAction(action);
  }

  trackScroll(view: string, direction: 'up' | 'down', element?: string): void {
    const action: UserAction = {
      type: 'scroll',
      view,
      element,
      action: 'scroll:' + direction,
      timestamp: new Date(),
    };
    this.logAction(action);
  }

  trackFocus(view: string, element: string, focused: boolean): void {
    const action: UserAction = {
      type: 'focus',
      view,
      element,
      action: focused ? 'focus:' + element : 'blur:' + element,
      timestamp: new Date(),
    };
    this.logAction(action);
  }

  trackContextMenu(view: string, element: string, metadata?: Record<string, unknown>): void {
    const action: UserAction = {
      type: 'context_menu',
      view,
      element,
      action: 'context_menu:' + element,
      metadata,
      timestamp: new Date(),
    };
    this.logAction(action);
  }

  setCurrentView(view: string): void {
    if (this.currentView() !== view) {
      this.logger.logInfo('user', view, 'viewChange', 'View changed to: ' + view);
      this.currentView.set(view);
    }
  }

  getCurrentView(): string {
    return this.currentView();
  }

  getActionHistory(): UserAction[] {
    return [...this.actionHistory];
  }

  getRecentActions(count: number = 20): UserAction[] {
    return this.actionHistory.slice(-count);
  }

  private logAction(action: UserAction): void {
    this.actionHistory.push(action);
    if (this.actionHistory.length > this.maxHistory) {
      this.actionHistory = this.actionHistory.slice(-this.maxHistory);
    }

    this.logger.logInfo(
      'user',
      action.view,
      action.action,
      'User ' + action.type + ': ' + action.action,
      action.metadata
    );
  }
}
