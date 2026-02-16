/* sys lib */
import { Component, Output, EventEmitter, OnChanges, SimpleChanges, computed, signal, input, effect } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatTooltipModule } from '@angular/material/tooltip';

@Component({
  selector: 'app-pagination',
  standalone: true,
  imports: [CommonModule, MatIconModule, MatButtonModule, MatTooltipModule],
  templateUrl: './pagination.component.html',
})
export class PaginationComponent {
  currentPage = input<number>(1);
  pageSize = input<number>(15);
  totalItems = input<number>(0);
  pageSizeOptions = input<number[]>([10, 15, 25, 50, 100]);
  showPageSizeSelector = input<boolean>(true);

  @Output() pageChange = new EventEmitter<number>();
  @Output() pageSizeChange = new EventEmitter<number>();

  p = signal(1);

  constructor() {
    effect(() => {
      this.p.set(this.currentPage());
    });
  }

  totalPages = computed(() => Math.ceil(this.totalItems() / this.pageSize()));

  pages = computed(() => {
    const total = this.totalPages();
    const current = this.p();
    const delta = 2;
    const range: number[] = [];
    const rangeWithDots: (number | string)[] = [];
    let l: number | undefined;

    for (let i = 1; i <= total; i++) {
      if (i === 1 || i === total || (i >= current - delta && i <= current + delta)) {
        range.push(i);
      }
    }

    for (const i of range) {
      if (l) {
        if (i - l === 2) {
          rangeWithDots.push(l + 1);
        } else if (i - l !== 1) {
          rangeWithDots.push('...');
        }
      }
      rangeWithDots.push(i);
      l = i;
    }
    return rangeWithDots;
  });

  goToPage(page: number | string): void {
    if (typeof page === 'number') {
      this.p.set(page);
      this.pageChange.emit(page);
    }
  }

  nextPage(): void {
    if (this.p() < this.totalPages()) {
      this.p.update(v => v + 1);
      this.pageChange.emit(this.p());
    }
  }

  previousPage(): void {
    if (this.p() > 1) {
      this.p.update(v => v - 1);
      this.pageChange.emit(this.p());
    }
  }

  onPageSizeChange(size: number): void {
    this.pageSizeChange.emit(size);
  }
}
