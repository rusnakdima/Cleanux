export function sortData<T>(
  data: T[],
  key: string,
  direction: 'asc' | 'desc',
  cellValue: (item: T, key: string) => unknown
): T[] {
  return [...data].sort((a, b) => {
    const aVal = cellValue(a, key);
    const bVal = cellValue(b, key);

    if (aVal === null || aVal === undefined) return direction === 'asc' ? 1 : -1;
    if (bVal === null || bVal === undefined) return direction === 'asc' ? -1 : 1;

    if (typeof aVal === 'number' && typeof bVal === 'number') {
      return direction === 'asc' ? aVal - bVal : bVal - aVal;
    }

    const aStr = String(aVal).toLowerCase();
    const bStr = String(bVal).toLowerCase();

    if (aStr < bStr) return direction === 'asc' ? -1 : 1;
    if (aStr > bStr) return direction === 'asc' ? 1 : -1;
    return 0;
  });
}

export function filterData<T>(
  data: T[],
  query: string,
  rowRecord: (item: T) => Record<string, unknown>
): T[] {
  if (!query.trim()) {
    return [...data];
  }

  const lowerQuery = query.toLowerCase().trim();
  return data.filter((item) => {
    const record = rowRecord(item);
    return Object.values(record).some((val) => {
      if (val === null || val === undefined) return false;
      return String(val).toLowerCase().includes(lowerQuery);
    });
  });
}

export function getCellValue<T extends object>(item: T, key: string): unknown {
  const keys = key.split('.');
  let value: unknown = item;
  for (const k of keys) {
    if (value && typeof value === 'object' && k in value) {
      value = (value as Record<string, unknown>)[k];
    } else {
      return undefined;
    }
  }
  return value;
}

export function rowRecord<T extends object>(item: T): Record<string, unknown> {
  return item as unknown as Record<string, unknown>;
}
