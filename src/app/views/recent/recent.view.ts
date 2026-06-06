/* sys lib */
import { ChangeDetectionStrategy, Component, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';

import { MatIconModule } from '@angular/material/icon';
import { ConfirmDialogService } from '@shared/confirm-dialog';

import { DataListComponent } from '@components/data-list/data-list.component';
import { ListColumn, ListOptions } from '@models/data-list.model';

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
  imports: [CommonModule, MatIconModule, DataListComponent],
  templateUrl: './recent.view.html',
})
export class RecentView {
  private confirmDialogService = inject(ConfirmDialogService);

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

  columns: ListColumn[] = [
    {
      key: 'name',
      primary: true,
      icon: 'description',
      badge: 'app',
      secondary: 'path',
      timestamp: 'timestamp',
      actions: [
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
    emptyMessage: 'No recent files tracked',
  };

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

  async clearApp(appId: string) {
    const app = this.apps.find((a) => a.id === appId);
    if (app && await this.confirmDialogService.confirm({
      title: 'Clear History',
      message: `Clear recent history for ${app.name}?`,
    })) {
      this.items.update((items) =>
        items.filter((i) => i.app.toLowerCase() !== appId.toLowerCase())
      );
    }
  }

  async clearAll() {
    if (await this.confirmDialogService.confirm({
      title: 'Clear All History',
      message: 'Clear all recent file history? This cannot be undone.',
      dangerous: true,
    })) {
      this.items.set([]);
    }
  }

  onRowAction(event: { action: string; item: RecentItem }) {
    if (event.action === 'remove') {
      this.removeItem(event.item.id);
    }
  }
}
