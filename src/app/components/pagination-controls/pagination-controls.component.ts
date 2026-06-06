/* sys lib */
import { ChangeDetectionStrategy, Component, input, output, computed, signal, effect } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';

@Component({
  selector: 'app-pagination-controls',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, MatButtonModule],
  templateUrl: './pagination-controls.component.html',
  styleUrl: './pagination-controls.component.css',
})
export class PaginationControlsComponent {
  currentPage = input<number>(1);
  pageSize = input<number>(15);
  totalItems = input<number>(0);
  pageSizeOptions = input<number[]>([10, 15, 25, 50, 100]);

  pageChange = output<number>();
  pageSizeChange = output<number>();

  p = signal(1);

  constructor() {
    this.p.set(this.currentPage());
    effect(() => {
      this.p.set(this.currentPage());
    });
  }

  totalPages = computed(() => Math.ceil(this.totalItems() / this.pageSize()));

  canGoPrevious = computed(() => this.p() > 1);
  canGoNext = computed(() => this.p() < this.totalPages());

  goToPrevious(): void {
    if (this.canGoPrevious()) {
      const newPage = this.p() - 1;
      this.p.set(newPage);
      this.pageChange.emit(newPage);
    }
  }

  goToNext(): void {
    if (this.canGoNext()) {
      const newPage = this.p() + 1;
      this.p.set(newPage);
      this.pageChange.emit(newPage);
    }
  }

  onPageSizeChange(size: number): void {
    this.pageSizeChange.emit(size);
  }
}
