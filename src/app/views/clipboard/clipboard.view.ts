/* sys lib */
import { ChangeDetectionStrategy, Component, signal, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

/* materials */
import { MatIconModule } from '@angular/material/icon';
import { PaginationComponent } from '@components/pagination/pagination.component';

interface ClipboardItem {
  id: string;
  content: string;
  type: 'text' | 'image' | 'link';
  timestamp: string;
  pinned: boolean;
}

@Component({
  selector: 'app-clipboard-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, FormsModule, MatIconModule, PaginationComponent],
  templateUrl: './clipboard.view.html',
})
export class ClipboardView {
  historyEnabled = true;

  items = signal<ClipboardItem[]>([
    {
      id: '1',
      content: 'https://github.com/user/repo',
      type: 'link',
      timestamp: '2 minutes ago',
      pinned: false,
    },
    {
      id: '2',
      content: 'Remember to update the documentation after the meeting tomorrow at 3pm.',
      type: 'text',
      timestamp: '15 minutes ago',
      pinned: true,
    },
    {
      id: '3',
      content: 'sudo apt update && sudo apt upgrade',
      type: 'text',
      timestamp: '1 hour ago',
      pinned: false,
    },
  ]);

  currentPage = signal(1);
  pageSize = signal(15);

  paginatedItems = computed(() => {
    const start = (this.currentPage() - 1) * this.pageSize();
    return this.items().slice(start, start + this.pageSize());
  });

  onPageChange(page: number) {
    this.currentPage.set(page);
  }

  onPageSizeChange(size: number) {
    this.pageSize.set(size);
    this.currentPage.set(1);
  }

  togglePin(id: string) {
    this.items.update((items) => items.map((i) => (i.id === id ? { ...i, pinned: !i.pinned } : i)));
  }

  removeItem(id: string) {
    this.items.update((items) => items.filter((i) => i.id !== id));
  }

  clearUnpinned() {
    this.items.update((items) => items.filter((i) => i.pinned));
  }
}
