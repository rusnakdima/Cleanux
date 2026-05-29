/* sys lib */
import { ChangeDetectionStrategy, Component, signal, computed } from '@angular/core';
import { CommonModule } from '@angular/common';

/* materials */
import { MatIconModule } from '@angular/material/icon';
import { PaginationComponent } from '@components/pagination/pagination.component';

interface RecentItem {
  id: string;
  name: string;
  app: string;
  path: string;
  timestamp: string;
}

@Component({
  selector: 'app-recent-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, MatIconModule, PaginationComponent],
  templateUrl: './recent.view.html',
})
export class RecentView {
  currentPage = signal(1);
  pageSize = signal(15);

  items = signal<RecentItem[]>([
    {
      id: '1',
      name: 'document.pdf',
      app: 'Firefox',
      path: '~/Documents/',
      timestamp: '2 hours ago',
    },
    {
      id: '2',
      name: 'spreadsheet.xlsx',
      app: 'LibreOffice',
      path: '~/Documents/work/',
      timestamp: '5 hours ago',
    },
    { id: '3', name: 'image.png', app: 'GIMP', path: '~/Pictures/', timestamp: '1 day ago' },
    { id: '4', name: 'notes.txt', app: 'VS Code', path: '~/Projects/', timestamp: '2 days ago' },
  ]);

  paginatedItems = computed(() => {
    const start = (this.currentPage() - 1) * this.pageSize();
    return this.items().slice(start, start + this.pageSize());
  });

  apps = [
    { id: 'firefox', name: 'Firefox', icon: 'public' },
    { id: 'libreoffice', name: 'LibreOffice', icon: 'article' },
    { id: 'gimp', name: 'GIMP', icon: 'brush' },
    { id: 'vscode', name: 'VS Code', icon: 'code' },
  ];

  onPageChange(page: number) {
    this.currentPage.set(page);
  }

  onPageSizeChange(size: number) {
    this.pageSize.set(size);
    this.currentPage.set(1);
  }

  removeItem(id: string) {
    this.items.update((items) => items.filter((i) => i.id !== id));
  }

  clearApp(appId: string) {
    const app = this.apps.find((a) => a.id === appId);
    if (app && confirm(`Clear recent history for ${app.name}?`)) {
      this.items.update((items) =>
        items.filter((i) => i.app.toLowerCase() !== appId.toLowerCase())
      );
    }
  }

  clearAll() {
    if (confirm('Clear all recent file history? This cannot be undone.')) {
      this.items.set([]);
    }
  }
}
