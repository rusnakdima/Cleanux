import { ChangeDetectionStrategy, Component, signal, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { MatCheckboxModule } from '@angular/material/checkbox';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { HeaderComponent } from '@components/header/header.component';
import {
  KernelCleanerService,
  KernelInfo,
  InitramfsInfo,
  BootSpaceInfo,
} from '@services/kernel-cleaner.service';
import { formatSize as formatBytesUtil } from '@shared/utils/format.util';

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
    MatCheckboxModule,
    MatProgressBarModule,
    HeaderComponent,
  ],
  templateUrl: './kernel-cleaner.view.html',
})
export class KernelCleanerView implements OnInit {
  private kernelService = inject(KernelCleanerService);

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
      console.error('Failed to load kernel data:', error);
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

    const confirmed = confirm(
      `Remove ${selected.length} kernel(s)? This cannot be undone.\n\n` +
        `Selected versions:\n${selected.join('\n')}\n\n` +
        `WARNING: Make sure you have a working kernel to boot into!`
    );
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
      this.actionResult.set(
        'Failed to remove some kernels: ' + (error instanceof Error ? error.message : String(error))
      );
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
      this.actionResult.set(
        'Failed to update GRUB: ' + (error instanceof Error ? error.message : String(error))
      );
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

    const confirmed = confirm(`Remove ${selected.length} old initramfs file(s)?`);
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
      this.actionResult.set(
        'Failed to remove some initramfs: ' +
          (error instanceof Error ? error.message : String(error))
      );
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

  isCurrentKernel(version: string): boolean {
    return version === this.currentKernel();
  }

  formatSize(bytes: number): string {
    return formatBytesUtil(bytes);
  }

  getBootUsagePercent(): number {
    const info = this.bootSpaceInfo();
    return info ? Math.round(info.usage_percent) : 0;
  }
}
