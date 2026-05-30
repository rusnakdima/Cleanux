import { describe, it, expect, beforeEach, vi } from 'vitest';
import { SearchComponent } from './search.component';

describe('SearchComponent', () => {
  let component: SearchComponent;

  beforeEach(() => {
    vi.useFakeTimers();
    component = new SearchComponent();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should have empty search query by default', () => {
    expect(component.searchQuery()).toBe('');
  });

  it('should not be visible by default', () => {
    expect(component.isVisible()).toBe(false);
  });

  it('should show search', () => {
    component.show();
    expect(component.isVisible()).toBe(true);
  });

  it('should hide search', () => {
    component.show();
    component.hide();
    expect(component.isVisible()).toBe(false);
    expect(component.searchQuery()).toBe('');
  });

  it('should clear search query', () => {
    component.searchControl.setValue('test query');
    component.searchQuery.set('test query');

    component.clearSearch();

    expect(component.searchQuery()).toBe('');
  });

  it('should filter data by search fields', () => {
    const testData = [
      { name: 'John', age: 30 },
      { name: 'Jane', age: 25 },
      { name: 'Bob', age: 35 },
    ];

    component.searchFields = ['name'];
    component.data = testData;

    component.ngOnChanges({
      data: {
        currentValue: testData,
        previousValue: [],
        firstChange: false,
      } as any,
    });

    const emittedValues: object[][] = [];
    component.filteredData.subscribe((data) => emittedValues.push([...data]));

    component.searchControl.setValue('j');
    vi.advanceTimersByTime(350);

    expect(emittedValues[emittedValues.length - 1]).toHaveLength(2);
  });

  it('should filter across all fields when no searchFields specified', () => {
    const testData = [
      { name: 'John', city: 'NYC' },
      { name: 'Jane', city: 'LA' },
    ];

    component.data = testData;

    component.ngOnChanges({
      data: {
        currentValue: testData,
        previousValue: [],
        firstChange: false,
      } as any,
    });

    const emittedValues: object[][] = [];
    component.filteredData.subscribe((data) => emittedValues.push([...data]));

    component.searchControl.setValue('la');
    vi.advanceTimersByTime(350);

    expect(emittedValues[emittedValues.length - 1]).toHaveLength(1);
    expect((emittedValues[emittedValues.length - 1] as any)[0].name).toBe('Jane');
  });

  it('should return all data when search query is empty', () => {
    const testData = [{ name: 'John' }, { name: 'Jane' }];

    component.data = testData;

    const emittedValues: object[][] = [];
    component.filteredData.subscribe((data) => emittedValues.push([...data]));

    component.searchControl.setValue('');
    vi.advanceTimersByTime(350);

    expect(emittedValues[emittedValues.length - 1]).toHaveLength(2);
  });

  it('should handle nested fields', () => {
    const testData = [
      { name: 'John', address: { city: 'NYC' } },
      { name: 'Jane', address: { city: 'LA' } },
    ];

    component.searchFields = ['address.city'];
    component.data = testData;

    component.ngOnChanges({
      data: {
        currentValue: testData,
        previousValue: [],
        firstChange: false,
      } as any,
    });

    const emittedValues: object[][] = [];
    component.filteredData.subscribe((data) => emittedValues.push([...data]));

    component.searchControl.setValue('la');
    vi.advanceTimersByTime(350);

    expect(emittedValues[emittedValues.length - 1]).toHaveLength(1);
    expect((emittedValues[emittedValues.length - 1] as any)[0].name).toBe('Jane');
  });

  it('should be case insensitive', () => {
    const testData = [{ name: 'JOHN' }, { name: 'Jane' }];

    component.data = testData;

    const emittedValues: object[][] = [];
    component.filteredData.subscribe((data) => emittedValues.push([...data]));

    component.searchControl.setValue('jane');
    vi.advanceTimersByTime(350);

    expect(emittedValues[emittedValues.length - 1]).toHaveLength(1);
  });

  it('should exclude object values from search', () => {
    const testData = [
      { name: 'John', metadata: { key: 'value' } },
      { name: 'Jane', metadata: { key: 'other' } },
    ];

    component.data = testData;

    const emittedValues: object[][] = [];
    component.filteredData.subscribe((data) => emittedValues.push([...data]));

    component.searchControl.setValue('value');
    vi.advanceTimersByTime(350);

    expect(emittedValues[emittedValues.length - 1]).toHaveLength(2);
  });
});
