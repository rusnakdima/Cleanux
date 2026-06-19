import {
  ChangeDetectionStrategy,
  Component,
  signal,
  inject,
  OnInit,
  computed,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { KernelCleanerService } from '@features/kernel-cleaner/services/kernel-cleaner.service';
import { KernelInfo, InitramfsInfo, BootSpaceInfo } from '@entities/kernel.model';
import { ConfirmDialogService } from '@shared/confirm-dialog';
import { formatSize } from '@shared/utils/format.util';
import { getErrorMessage } from '@shared/utils/error.util';
import { DataListComponent } from '@components/data-list/data-list.component';
import { ListColumn, ListOptions } from '@entities/data-list.model';
import { LoadingSpinnerComponent } from '@components/loading-spinner/loading-spinner.component';

@Component({
  selector: 'app-kernel-cleaner-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    MatProgressBarModule,
    DataListComponent,
    LoadingSpinnerComponent,
  ],
  templateUrl: './kernel-cleaner.view.html',
})
export class KernelCleanerView implements OnInit {
  private kernelService = inject(KernelCleanerService);
  private confirmDialogService = inject(ConfirmDialogService);

  formatSize = formatSize;

  loading = signal(false);
  currentKernel = signal<string>('');
  allKernels = signal<KernelInfo[]>([]);
  oldKernels = signal<KernelInfo[]>([]);
  selectedKernels = signal<Set<string>>(new Set());
  oldInitramfs = signal<InitramfsInfo[]>([]);
  selectedInitramfs = signal<Set<string>>(new Set());
  bootSpaceInfo = signal<BootSpaceInfo | null>(null);
  showGrubUpdateNotice = signal(false);
  actionResult = signal<string | null>(null);

  kernelsPage = signal(1);
  kernelsPageSize = signal(15);
  initramfsPage = signal(1);
  initramfsPageSize = signal(15);

  kernelColumns: ListColumn[] = [
    {
      key: 'version',
      primary: true,
      icon: 'memory',
      badge: 'version',
      badgeClass: 'badge-primary',
      secondaryKey: 'path',
      sortable: true,
    },
    {
      key: 'size',
      format: 'size',
      align: 'right',
      sortable: true,
    },
  ];

  kernelOptions: ListOptions = {
    showCheckbox: false,
    hoverable: true,
    showReloadButton: true,
    searchTogglable: true,
    showSearch: true,
    rowClass: (item: unknown) => {
      const kernel = item as KernelInfo;
      return kernel.is_current
        ? 'bg-success/10 border-success/30'
        : 'border-zinc-600 dark:border-zinc-600';
    },
  };

  initramfsColumns: ListColumn[] = [
    {
      key: 'version',
      primary: true,
      icon: 'description',
      sortable: true,
    },
    {
      key: 'size',
      format: 'size',
      align: 'right',
      sortable: true,
    },
  ];

  initramfsOptions: ListOptions = {
    showCheckbox: true,
    checkboxKey: 'version',
    hoverable: true,
    showReloadButton: false,
    showSearch: true,
    showSelectAll: true,
  };

  onKernelsPageChange(page: number) {
    this.kernelsPage.set(page);
  }

  onKernelsPageSizeChange(size: number) {
    this.kernelsPageSize.set(size);
    this.kernelsPage.set(1);
  }

  onInitramfsPageChange(page: number) {
    this.initramfsPage.set(page);
  }

  onInitramfsPageSizeChange(size: number) {
    this.initramfsPageSize.set(size);
    this.initramfsPage.set(1);
  }

  async ngOnInit() {
    await this.loadData();
  }

  async loadData() {
    this.loading.set(true);
    try {
      const [current, all, old, initramfs, bootInfo] = await Promise.all([
        this.kernelService.getCurrentKernel(),
        this.kernelService.getInstalledKernels(),
        this.kernelService.getOldKernels(),
        this.kernelService.getOldInitramfs(),
        this.kernelService.getBootSpaceInfo(),
      ]);

      this.currentKernel.set(current);
      this.allKernels.set(all);
      this.oldKernels.set(old);
      this.oldInitramfs.set(initramfs);
      this.bootSpaceInfo.set(bootInfo);
      this.showGrubUpdateNotice.set(false);
    } catch (error) {
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  toggleKernelSelection(version: string) {
    const current = this.selectedKernels();
    const newSet = new Set(current);
    if (newSet.has(version)) {
      newSet.delete(version);
    } else {
      newSet.add(version);
    }
    this.selectedKernels.set(newSet);
  }

  toggleAllOldKernels() {
    const old = this.oldKernels();
    if (this.selectedKernels().size === old.length) {
      this.selectedKernels.set(new Set());
    } else {
      this.selectedKernels.set(new Set(old.map((k) => k.version)));
    }
  }

  async removeSelectedKernels() {
    const selected = Array.from(this.selectedKernels());
    if (selected.length === 0) return;

    const confirmed = await this.confirmDialogService.confirm({
      title: 'Remove Kernels',
      message:
        `Remove ${selected.length} kernel(s)? This cannot be undone.\n\n` +
        `Selected versions:\n${selected.join('\n')}\n\n` +
        `WARNING: Make sure you have a working kernel to boot into!`,
      dangerous: true,
    });
    if (!confirmed) return;

    this.loading.set(true);
    this.actionResult.set(null);

    try {
      for (const version of selected) {
        await this.kernelService.removeKernel(version);
      }
      this.showGrubUpdateNotice.set(true);
      this.actionResult.set(
        `Successfully removed ${selected.length} kernel(s). Please update GRUB.`
      );
      this.selectedKernels.set(new Set());
      await this.loadData();
    } catch (error) {
      this.actionResult.set('Failed to remove some kernels: ' + getErrorMessage(error));
    } finally {
      this.loading.set(false);
    }
  }

  async updateGrub() {
    this.loading.set(true);
    this.actionResult.set(null);

    try {
      await this.kernelService.updateGrub();
      this.actionResult.set('GRUB configuration updated successfully.');
      this.showGrubUpdateNotice.set(false);
    } catch (error) {
      this.actionResult.set('Failed to update GRUB: ' + getErrorMessage(error));
    } finally {
      this.loading.set(false);
    }
  }

  toggleInitramfsSelection(version: string) {
    const current = this.selectedInitramfs();
    const newSet = new Set(current);
    if (newSet.has(version)) {
      newSet.delete(version);
    } else {
      newSet.add(version);
    }
    this.selectedInitramfs.set(newSet);
  }

  async cleanSelectedInitramfs() {
    const selected = Array.from(this.selectedInitramfs());
    if (selected.length === 0) return;

    const confirmed = await this.confirmDialogService.confirm({
      title: 'Clean Initramfs',
      message: `Remove ${selected.length} old initramfs file(s)?`,
    });
    if (!confirmed) return;

    this.loading.set(true);
    this.actionResult.set(null);

    try {
      for (const version of selected) {
        await this.kernelService.removeInitramfs(version);
      }
      this.actionResult.set(`Removed ${selected.length} initramfs file(s).`);
      this.selectedInitramfs.set(new Set());
      await this.loadData();
    } catch (error) {
      this.actionResult.set('Failed to remove some initramfs: ' + getErrorMessage(error));
    } finally {
      this.loading.set(false);
    }
  }

  getOldKernelsSize(): number {
    return this.oldKernels().reduce((sum, k) => sum + k.size, 0);
  }

  getSelectedSize(): number {
    const selected = this.selectedKernels();
    return this.oldKernels()
      .filter((k) => selected.has(k.version))
      .reduce((sum, k) => sum + k.size, 0);
  }

  getBootUsagePercent(): number {
    const info = this.bootSpaceInfo();
    return info ? Math.round(info.usage_percent) : 0;
  }

  onKernelSelectionChange(keys: Set<string>): void {
    this.selectedKernels.set(keys);
  }

  onInitramfsSelectionChange(keys: Set<string>): void {
    this.selectedInitramfs.set(keys);
  }

  isCurrentKernel(version: string): boolean {
    return version === this.currentKernel();
  }
}
