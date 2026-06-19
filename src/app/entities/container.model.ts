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
