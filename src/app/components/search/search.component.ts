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
  @Input() data: any[] = [];
  @Input() searchFields: string[] = [];
  @Output() filteredData = new EventEmitter<any[]>();

  searchQuery = signal('');
  isVisible = signal(false);
  private _originalData: any[] = [];

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
      if (this.searchFields.length > 0) {
        return this.searchFields.some(field => {
          const value = this.getValue(item, field);
          return String(value).toLowerCase().includes(query);
        });
      }
      return Object.values(item).some(value => {
        if (typeof value === 'object') return false;
        return String(value).toLowerCase().includes(query);
      });
    });

    this.filteredData.emit(filtered);
  }

  private getValue(obj: any, path: string): any {
    return path.split('.').reduce((acc, part) => acc?.[part], obj);
  }
}
