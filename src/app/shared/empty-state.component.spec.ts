import { describe, it, expect, beforeEach } from 'vitest';
import { EmptyStateComponent, EmptyTableComponent } from './empty-state.component';

describe('EmptyStateComponent', () => {
  let component: EmptyStateComponent;

  beforeEach(() => {
    component = new EmptyStateComponent();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should have default icon', () => {
    expect(component.icon()).toBe('inbox');
  });

  it('should have default title', () => {
    expect(component.title()).toBe('No data');
  });

  it('should have null description by default', () => {
    expect(component.description()).toBeNull();
  });

  it('should have null actionLabel by default', () => {
    expect(component.actionLabel()).toBeNull();
  });

  it('should set custom icon', () => {
    component.icon.set('folder_open');
    expect(component.icon()).toBe('folder_open');
  });

  it('should set custom title', () => {
    component.title.set('No files found');
    expect(component.title()).toBe('No files found');
  });

  it('should set custom description', () => {
    component.description.set('Try adjusting your search filters');
    expect(component.description()).toBe('Try adjusting your search filters');
  });

  it('should set custom actionLabel', () => {
    component.actionLabel.set('Add Files');
    expect(component.actionLabel()).toBe('Add Files');
  });

  it('should call registered action handler', () => {
    const handler = () => {};
    component.registerAction(handler);
    expect(component.actionHandler).toBe(handler);
  });

  it('should call actionHandler on onAction when set', () => {
    let called = false;
    component.registerAction(() => {
      called = true;
    });
    component.onAction();
    expect(called).toBe(true);
  });

  it('should not throw on onAction when no handler', () => {
    component.actionHandler = null;
    expect(() => component.onAction()).not.toThrow();
  });
});

describe('EmptyTableComponent', () => {
  let component: EmptyTableComponent;

  beforeEach(() => {
    component = new EmptyTableComponent();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should have default icon', () => {
    expect(component.icon()).toBe('search_off');
  });

  it('should have default message', () => {
    expect(component.message()).toBe('No matching records found');
  });

  it('should set custom icon', () => {
    component.icon.set('folder_open');
    expect(component.icon()).toBe('folder_open');
  });

  it('should set custom message', () => {
    component.message.set('No results');
    expect(component.message()).toBe('No results');
  });
});
