/* sys lib */
import { ChangeDetectionStrategy, Component, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

import { MatIconModule } from '@angular/material/icon';

import { DataListComponent } from '@components/data-list/data-list.component';
import { ListColumn, ListOptions } from '@models/data-list.model';

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
  imports: [CommonModule, FormsModule, MatIconModule, DataListComponent],
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

  columns: ListColumn[] = [
    {
      key: 'content',
      primary: true,
      iconKey: 'type',
      secondary: 'timestamp',
      actions: [
        {
          id: 'pin',
          icon: 'push_pin',
          tooltip: 'Toggle pin',
          toggle: true,
          toggleState: (item: unknown) => (item as ClipboardItem).pinned,
        },
        {
          id: 'remove',
          icon: 'close',
          tooltip: 'Remove',
        },
      ],
    },
  ];

  options: ListOptions = {
    showSearch: false,
    showCheckbox: false,
    showActions: true,
    actionsPosition: 'right',
    emptyMessage: 'Clipboard is empty',
  };

  currentPage = signal(1);
  pageSize = signal(15);

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

  onRowAction(event: { action: string; item: ClipboardItem }) {
    if (event.action === 'pin') {
      this.togglePin(event.item.id);
    } else if (event.action === 'remove') {
      this.removeItem(event.item.id);
    }
  }
}
