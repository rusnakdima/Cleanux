import { ChangeDetectionStrategy, Component, signal, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatTooltipModule } from '@angular/material/tooltip';
import { MatTabsModule } from '@angular/material/tabs';
import { ContainerService } from '@features/container-cleaner/services/container.service';
import { ContainerSummary } from '@entities/container.model';
import { formatSize } from '@shared/utils/format.util';
import { getErrorMessage } from '@shared/utils/error.util';
import { LoadingSpinnerComponent } from '@components/loading-spinner/loading-spinner.component';
import { ConfirmDialogService } from '@shared/confirm-dialog';

type ContainerTab = 'docker' | 'podman';

@Component({
  selector: 'app-container-cleaner-view',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatProgressSpinnerModule,
    MatTooltipModule,
    MatTabsModule,
    LoadingSpinnerComponent,
  ],
  templateUrl: './container-cleaner.view.html',
})
export class ContainerCleanerView implements OnInit {
  private containerService = inject(ContainerService);
  private confirmDialogService = inject(ConfirmDialogService);

  formatSize = formatSize;

  loading = signal(false);
  activeTab = signal<ContainerTab>('docker');
  summary = signal<ContainerSummary | null>(null);

  dockerResult = signal<string | null>(null);
  podmanResult = signal<string | null>(null);

  dockerPreview = signal<string | null>(null);
  podmanPreview = signal<string | null>(null);

  ngOnInit(): void {
    this.loadSummary();
  }

  async loadSummary() {
    this.loading.set(true);
    this.dockerResult.set(null);
    this.podmanResult.set(null);
    try {
      const data = await this.containerService.getContainerSummary();
      this.summary.set(data);
    } catch (error: unknown) {
      throw error;
    } finally {
      this.loading.set(false);
    }
  }

  async previewDockerPrune(all: boolean) {
    this.loading.set(true);
    try {
      const preview = await this.containerService.dockerPreviewPrune(all);
      this.dockerPreview.set(preview);
    } catch (error: unknown) {
      this.dockerPreview.set('Failed to preview: ' + getErrorMessage(error));
    } finally {
      this.loading.set(false);
    }
  }

  async dockerSystemPrune(all: boolean) {
    const warning = all
      ? 'Warning: Using -a flag will remove ALL unused images, not just dangling ones. This cannot be undone!'
      : 'This will remove all stopped containers, unused networks, and dangling images. Continue?';

    const confirmed = await this.confirmDialogService.confirm({
      title: 'Docker System Prune',
      message: warning,
    });
    if (!confirmed) return;

    this.loading.set(true);
    this.dockerPreview.set(null);
    try {
      const result = await this.containerService.dockerSystemPrune(all);
      this.dockerResult.set(result);
      await this.loadSummary();
    } catch (error: unknown) {
      this.dockerResult.set('Failed: ' + getErrorMessage(error));
    } finally {
      this.loading.set(false);
    }
  }

  async dockerImagePrune(all: boolean) {
    const warning = all
      ? 'Warning: Using -a flag will remove ALL unused images. This cannot be undone!'
      : 'This will remove all dangling images. Continue?';

    const confirmed = await this.confirmDialogService.confirm({
      title: 'Docker Image Prune',
      message: warning,
    });
    if (!confirmed) return;

    this.loading.set(true);
    this.dockerPreview.set(null);
    try {
      const result = await this.containerService.dockerImagePrune(all);
      this.dockerResult.set(result);
      await this.loadSummary();
    } catch (error: unknown) {
      this.dockerResult.set('Failed: ' + getErrorMessage(error));
    } finally {
      this.loading.set(false);
    }
  }

  async dockerContainerPrune() {
    const confirmed = await this.confirmDialogService.confirm({
      title: 'Docker Container Prune',
      message: 'This will remove all stopped containers. Continue?',
    });
    if (!confirmed) return;

    this.loading.set(true);
    this.dockerPreview.set(null);
    try {
      const result = await this.containerService.dockerContainerPrune();
      this.dockerResult.set(result);
      await this.loadSummary();
    } catch (error: unknown) {
      this.dockerResult.set('Failed: ' + getErrorMessage(error));
    } finally {
      this.loading.set(false);
    }
  }

  async dockerVolumePrune() {
    const confirmed = await this.confirmDialogService.confirm({
      title: 'Docker Volume Prune',
      message:
        'This will remove all unused volumes. Data in these volumes will be lost forever! Continue?',
    });
    if (!confirmed) return;

    this.loading.set(true);
    this.dockerPreview.set(null);
    try {
      const result = await this.containerService.dockerVolumePrune();
      this.dockerResult.set(result);
      await this.loadSummary();
    } catch (error: unknown) {
      this.dockerResult.set('Failed: ' + getErrorMessage(error));
    } finally {
      this.loading.set(false);
    }
  }

  async podmanSystemPrune(all: boolean) {
    const warning = all
      ? 'Warning: Using -a flag will remove ALL unused images and containers. This cannot be undone!'
      : 'This will remove all stopped containers, unused networks, and dangling images. Continue?';

    const confirmed = await this.confirmDialogService.confirm({
      title: 'Podman System Prune',
      message: warning,
    });
    if (!confirmed) return;

    this.loading.set(true);
    this.podmanPreview.set(null);
    try {
      const result = await this.containerService.podmanSystemPrune(all);
      this.podmanResult.set(result);
      await this.loadSummary();
    } catch (error: unknown) {
      this.podmanResult.set('Failed: ' + getErrorMessage(error));
    } finally {
      this.loading.set(false);
    }
  }

  async podmanImagePrune(all: boolean) {
    const warning = all
      ? 'Warning: Using -a flag will remove ALL unused images. This cannot be undone!'
      : 'This will remove all dangling images. Continue?';

    const confirmed = await this.confirmDialogService.confirm({
      title: 'Podman Image Prune',
      message: warning,
    });
    if (!confirmed) return;

    this.loading.set(true);
    this.podmanPreview.set(null);
    try {
      const result = await this.containerService.podmanImagePrune(all);
      this.podmanResult.set(result);
      await this.loadSummary();
    } catch (error: unknown) {
      this.podmanResult.set('Failed: ' + getErrorMessage(error));
    } finally {
      this.loading.set(false);
    }
  }

  async cleanAllDockerContainers() {
    const s = this.summary();
    if (!s || s.docker_containers_count === 0) return;

    const confirmed = await this.confirmDialogService.confirmDangerous(
      `You are about to remove ${s.docker_containers_count} Docker containers. This is irreversible!`,
      true
    );
    if (!confirmed) return;

    this.loading.set(true);
    this.dockerPreview.set(null);
    try {
      const result = await this.containerService.dockerContainerPrune();
      this.dockerResult.set(result);
      await this.loadSummary();
    } catch (error: unknown) {
      this.dockerResult.set('Failed: ' + getErrorMessage(error));
    } finally {
      this.loading.set(false);
    }
  }

  onTabChange(index: number) {
    const tabs: ContainerTab[] = ['docker', 'podman'];
    this.activeTab.set(tabs[index]);
  }
}
