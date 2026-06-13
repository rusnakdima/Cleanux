import { Component, OnInit, signal, inject, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { DiskUsageService } from '@services/disk-usage.service';
import { formatSize } from '@shared/utils/format.util';

export interface DirectoryNode {
  name: string;
  path: string;
  size: number;
  children: DirectoryNode[];
}

export interface TreemapItem {
  node: DirectoryNode;
  depth: number;
  percentage: number;
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
  private diskUsageService = inject(DiskUsageService);

  rootPath = signal('');
  currentPath = signal('');
  directoryTree = computed(() => this.diskUsageService.originalTree());
  currentNode = this.diskUsageService.currentNode;
  loading = signal(false);
  breadcrumbs = this.diskUsageService.breadcrumbs;
  treemapData = this.diskUsageService.treemapData;

  ngOnInit() {}

  async scanDirectory() {
    if (!this.rootPath()) return;

    this.loading.set(true);
    try {
      const result = await this.diskUsageService.scanDirectory(this.rootPath());
      this.currentPath.set(result.tree.path);
    } catch (error) {
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  navigateToNode(node: DirectoryNode) {
    this.diskUsageService.navigateToNode(node);
  }

  navigateToBreadcrumb(index: number) {
    this.diskUsageService.navigateToBreadcrumb(index);
  }

  getDepthColor(depth: number): string {
    return this.diskUsageService.getDepthColor(depth);
  }

  formatSize = formatSize;
}
