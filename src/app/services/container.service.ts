import { Injectable, inject } from '@angular/core';
import { ApiService } from './api.service';
import { formatSize } from '@shared/utils/format.util';

export interface ContainerSummary {
  docker_installed: boolean;
  docker_images_size: number;
  docker_containers_count: number;
  docker_volumes_size: number;
  docker_version: string | null;
  podman_installed: boolean;
  podman_images_size: number;
  podman_containers_count: number;
  podman_version: string | null;
}

export interface PruneResult {
  status: string;
  message: string;
  data: string;
}

@Injectable({
  providedIn: 'root',
})
export class ContainerService {
  private api = inject(ApiService);

  async getContainerSummary(): Promise<ContainerSummary> {
    return this.api.invoke<ContainerSummary>('get_container_summary');
  }

  async dockerSystemPrune(all: boolean): Promise<string> {
    return this.api.invoke<string>('docker_system_prune', { all });
  }

  async dockerImagePrune(all: boolean): Promise<string> {
    return this.api.invoke<string>('docker_image_prune', { all });
  }

  async dockerContainerPrune(): Promise<string> {
    return this.api.invoke<string>('docker_container_prune');
  }

  async dockerVolumePrune(): Promise<string> {
    return this.api.invoke<string>('docker_volume_prune');
  }

  async dockerPreviewPrune(all: boolean): Promise<string> {
    return this.api.invoke<string>('docker_preview_prune', { all });
  }

  async podmanSystemPrune(all: boolean): Promise<string> {
    return this.api.invoke<string>('podman_system_prune', { all });
  }

  async podmanImagePrune(all: boolean): Promise<string> {
    return this.api.invoke<string>('podman_image_prune', { all });
  }

  formatSize = formatSize;
}
