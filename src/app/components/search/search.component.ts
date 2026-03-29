/* sys lib */
import { Component, EventEmitter, Input, Output, signal, OnChanges, SimpleChanges } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

/* materials */
import { MatIconModule } from '@angular/material/icon';

@Component({
  selector: 'app-search',
  standalone: true,
  imports: [CommonModule, FormsModule, MatIconModule],
  templateUrl: './search.component.html',
})
export class SearchComponent implements OnChanges {
  @Input() data: object[] = [];
  @Input() searchFields: string[] = [];
  @Output() filteredData = new EventEmitter<object[]>();

  searchQuery = signal('');
  isVisible = signal(false);
  private _originalData: object[] = [];

  private rowRecord(item: object): Record<string, unknown> {
    return item as Record<string, unknown>;
  }

  ngOnChanges(changes: SimpleChanges): void {
    if (changes['data'] && this.data) {
      this._originalData = [...this.data];
      const query = this.searchQuery().toLowerCase().trim();
      if (!query) {
        this.filteredData.emit(this._originalData);
      } else {
        this.onSearch();
      }
    }
  }

  show(): void {
    this.isVisible.set(true);
    setTimeout(() => {
      const input = document.querySelector('input[placeholder="Search..."]') as HTMLInputElement;
      if (input) input.focus();
    }, 100);
  }

  hide(): void {
    this.isVisible.set(false);
    this.searchQuery.set('');
    this.filteredData.emit(this._originalData);
  }

  clearSearch(): void {
    this.searchQuery.set('');
    this.filteredData.emit(this._originalData);
  }

  onSearch(): void {
    const query = this.searchQuery().toLowerCase().trim();

    if (!query) {
      this.filteredData.emit(this._originalData);
      return;
    }

    const filtered = this._originalData.filter(item => {
      const rec = this.rowRecord(item);
      if (this.searchFields.length > 0) {
        return this.searchFields.some(field => {
          const value = this.getValue(rec, field);
          return String(value).toLowerCase().includes(query);
        });
      }
      return Object.values(rec).some(value => {
        if (typeof value === 'object' && value !== null) return false;
        return String(value).toLowerCase().includes(query);
      });
    });

    this.filteredData.emit(filtered);
  }

  private getValue(obj: Record<string, unknown>, path: string): unknown {
    return path.split('.').reduce<unknown>((acc, part) => {
      if (acc !== null && typeof acc === 'object' && part in (acc as Record<string, unknown>)) {
        return (acc as Record<string, unknown>)[part];
      }
      return undefined;
    }, obj);
  }
}
