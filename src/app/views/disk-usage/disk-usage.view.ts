import { Component, OnInit, signal, inject, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { ApiService } from '@services/api.service';
import { formatSize } from '@shared/utils/format.util';

export interface DirectoryNode {
  name: string;
  path: string;
  size: number;
  children: DirectoryNode[];
}

interface ScanResult {
  tree: DirectoryNode;
  totalSize: number;
}

@Component({
  selector: 'app-disk-usage-view',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    MatInputModule,
    MatFormFieldModule,
  ],
  templateUrl: './disk-usage.view.html',
})
export class DiskUsageView implements OnInit {
  private apiService = inject(ApiService);

  rootPath = signal('');
  currentPath = signal('');
  directoryTree = signal<DirectoryNode | null>(null);
  currentNode = signal<DirectoryNode | null>(null);
  loading = signal(false);
  breadcrumbs = signal<{ name: string; path: string }[]>([]);
  treemapData = signal<TreemapItem[]>([]);

  private originalTree = signal<DirectoryNode | null>(null);

  ngOnInit() {}

  async scanDirectory() {
    if (!this.rootPath()) return;

    this.loading.set(true);
    try {
      const result = await this.apiService.invoke<ScanResult>('scan_directory', {
        path: this.rootPath(),
        maxDepth: 3,
      });

      this.originalTree.set(result.tree);
      this.directoryTree.set(result.tree);
      this.currentNode.set(result.tree);
      this.currentPath.set(result.tree.path);
      this.breadcrumbs.set([{ name: result.tree.name, path: result.tree.path }]);
      this.calculateTreemap(result.tree);
    } catch (error) {
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  navigateToNode(node: DirectoryNode) {
    this.currentNode.set(node);
    this.currentPath.set(node.path);
    this.updateBreadcrumbs(node);
    this.calculateTreemap(node);
  }

  navigateToBreadcrumb(index: number) {
    const crumbs = this.breadcrumbs();
    if (index === 0) {
      const tree = this.originalTree();
      if (tree) {
        this.currentNode.set(tree);
        this.currentPath.set(tree.path);
        this.calculateTreemap(tree);
      }
    } else {
      const crumb = crumbs[index];
      const node = this.findNodeByPath(this.originalTree(), crumb.path);
      if (node) {
        this.currentNode.set(node);
        this.currentPath.set(node.path);
        this.calculateTreemap(node);
      }
    }
    this.breadcrumbs.set(crumbs.slice(0, index + 1));
  }

  private updateBreadcrumbs(node: DirectoryNode) {
    const crumbs: { name: string; path: string }[] = [];
    let current: DirectoryNode | null = node;

    while (current) {
      crumbs.unshift({ name: current.name, path: current.path });
      current = this.findParent(this.originalTree(), current.path);
    }

    this.breadcrumbs.set(crumbs);
  }

  private findParent(root: DirectoryNode | null, path: string): DirectoryNode | null {
    if (!root) return null;

    for (const child of root.children) {
      if (child.path === path) {
        return root;
      }
      const found = this.findParent(child, path);
      if (found) return found;
    }
    return null;
  }

  private findNodeByPath(root: DirectoryNode | null, path: string): DirectoryNode | null {
    if (!root) return null;
    if (root.path === path) return root;

    for (const child of root.children) {
      const found = this.findNodeByPath(child, path);
      if (found) return found;
    }
    return null;
  }

  private calculateTreemap(node: DirectoryNode) {
    const items = this.buildTreemapItems(node, 0);
    this.treemapData.set(items);
  }

  private buildTreemapItems(node: DirectoryNode, depth: number): TreemapItem[] {
    const items: TreemapItem[] = [];

    for (const child of node.children) {
      if (child.children.length > 0) {
        items.push({
          node: child,
          depth,
          percentage: node.size > 0 ? (child.size / node.size) * 100 : 0,
        });
        items.push(...this.buildTreemapItems(child, depth + 1));
      } else {
        items.push({
          node: child,
          depth,
          percentage: node.size > 0 ? (child.size / node.size) * 100 : 0,
        });
      }
    }

    return items.sort((a, b) => b.node.size - a.node.size);
  }

  getDepthColor(depth: number): string {
    const colors = [
      'from-blue-400 to-blue-600',
      'from-green-400 to-green-600',
      'from-yellow-400 to-yellow-600',
      'from-orange-400 to-orange-600',
      'from-red-400 to-red-600',
      'from-purple-400 to-purple-600',
      'from-pink-400 to-pink-600',
    ];
    return colors[depth % colors.length];
  }

  formatSize = formatSize;
}

export interface TreemapItem {
  node: DirectoryNode;
  depth: number;
  percentage: number;
}
