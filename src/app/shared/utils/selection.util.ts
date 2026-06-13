import { computed, WritableSignal } from '@angular/core';

export function createSelectionHelpers<T extends { path: string }>(
  dataSignal: WritableSignal<T[]>,
  selectedSignal: WritableSignal<Set<string>>
) {
  return {
    allSelected: computed(() => {
      const total = dataSignal().length;
      const selected = selectedSignal().size;
      return total > 0 && selected === total;
    }),
    indeterminate: computed(() => {
      const total = dataSignal().length;
      const selected = selectedSignal().size;
      return selected > 0 && selected < total;
    }),
    toggleSelection: (path: string) => {
      const current = new Set(selectedSignal());
      current.has(path) ? current.delete(path) : current.add(path);
      selectedSignal.set(current);
    },
    selectAll: (items: T[], checked = true) => {
      selectedSignal.set(checked ? new Set(items.map((i) => i.path)) : new Set());
    },
    deselectAll: () => {
      selectedSignal.set(new Set());
    },
  };
}
