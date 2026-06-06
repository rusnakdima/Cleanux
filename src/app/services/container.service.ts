import { Injectable } from '@angular/core';
import { BaseApiService } from '@services/base-api.service';
import { ContainerSummary, PruneResult } from '@models/container.model';

@Injectable({
  providedIn: 'root',
})
export class ContainerService extends BaseApiService {
  async getContainerSummary(): Promise<ContainerSummary> {
    return this.call<ContainerSummary>('get_container_summary');
  }

  async dockerSystemPrune(all: boolean): Promise<string> {
    return this.call<string>('docker_system_prune', { all });
  }

  async dockerImagePrune(all: boolean): Promise<string> {
    return this.call<string>('docker_image_prune', { all });
  }

  async dockerContainerPrune(): Promise<string> {
    return this.call<string>('docker_container_prune');
  }

  async dockerVolumePrune(): Promise<string> {
    return this.call<string>('docker_volume_prune');
  }

  async dockerPreviewPrune(all: boolean): Promise<string> {
    return this.call<string>('docker_preview_prune', { all });
  }

  async podmanSystemPrune(all: boolean): Promise<string> {
    return this.call<string>('podman_system_prune', { all });
  }

  async podmanImagePrune(all: boolean): Promise<string> {
    return this.call<string>('podman_image_prune', { all });
  }
}
