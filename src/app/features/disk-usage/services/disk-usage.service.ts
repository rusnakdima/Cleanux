/* sys lib */
import { Injectable, inject, signal } from '@angular/core';

/* services */
import { ApiService } from '@services/api.service';
import { LoggerService } from '@services/logger.service';

/* models */
import { DirectoryNode, TreemapItem } from '@views/disk-usage/disk-usage.view';

interface ScanResult {
  tree: DirectoryNode;
  totalSize: number;
}

@Injectable({
  providedIn: 'root',
})
export class DiskUsageService {
  private api = inject(ApiService);
  private loggingService = new LoggerService();

  private _originalTree = signal<DirectoryNode | null>(null);
  private _currentNode = signal<DirectoryNode | null>(null);
  private _treemapData = signal<TreemapItem[]>([]);
  private _breadcrumbs = signal<{ name: string; path: string }[]>([]);

  readonly originalTree = this._originalTree.asReadonly();
  readonly currentNode = this._currentNode.asReadonly();
  readonly treemapData = this._treemapData.asReadonly();
  readonly breadcrumbs = this._breadcrumbs.asReadonly();

  constructor() {
    this.loggingService.info('DiskUsageService initialized');
  }

  async scanDirectory(path: string): Promise<ScanResult> {
    this.loggingService.info('Scanning directory', { path });
    try {
      const result = await this.api.invoke<ScanResult>('scan_directory', {
        path,
        maxDepth: 3,
      });

      this._originalTree.set(result.tree);
      this._currentNode.set(result.tree);
      this._breadcrumbs.set([{ name: result.tree.name, path: result.tree.path }]);
      this.calculateTreemap(result.tree);

      this.loggingService.info('Directory scanned successfully', {
        path,
        totalSize: result.totalSize,
      });
      return result;
    } catch (error) {
      this.loggingService.error('Operation failed', error as Error, { path });
      throw error;
    }
  }

  navigateToNode(node: DirectoryNode): void {
    this._currentNode.set(node);
    this.updateBreadcrumbs(node);
    this.calculateTreemap(node);
  }

  navigateToBreadcrumb(index: number): void {
    const crumbs = this._breadcrumbs();
    if (index === 0) {
      const tree = this._originalTree();
      if (tree) {
        this._currentNode.set(tree);
        this.calculateTreemap(tree);
      }
    } else {
      const crumb = crumbs[index];
      const node = this.findNodeByPath(this._originalTree(), crumb.path);
      if (node) {
        this._currentNode.set(node);
        this.calculateTreemap(node);
      }
    }
    this._breadcrumbs.set(crumbs.slice(0, index + 1));
  }

  private updateBreadcrumbs(node: DirectoryNode): void {
    const crumbs: { name: string; path: string }[] = [];
    let current: DirectoryNode | null = node;

    while (current) {
      crumbs.unshift({ name: current.name, path: current.path });
      current = this.findParent(this._originalTree(), current.path);
    }

    this._breadcrumbs.set(crumbs);
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

  private calculateTreemap(node: DirectoryNode): void {
    const items = this.buildTreemapItems(node, 0);
    this._treemapData.set(items);
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
}
