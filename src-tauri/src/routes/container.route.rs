/* models */
use crate::models::ResponseModel;
/* services */
use crate::services::container_service::ContainerService;

#[tauri::command]
pub fn get_container_summary() -> ContainerSummaryResponse {
  let summary = ContainerService.get_container_summary();
  ContainerSummaryResponse {
    docker_installed: summary.docker.installed,
    docker_images_size: summary.docker.images_size,
    docker_containers_count: summary.docker.containers_count,
    docker_volumes_size: summary.docker.volumes_size,
    docker_version: summary.docker.version,
    podman_installed: summary.podman.installed,
    podman_images_size: summary.podman.images_size,
    podman_containers_count: summary.podman.containers_count,
    podman_version: summary.podman.version,
  }
}

#[derive(Debug, serde::Serialize)]
pub struct ContainerSummaryResponse {
  pub docker_installed: bool,
  pub docker_images_size: u64,
  pub docker_containers_count: usize,
  pub docker_volumes_size: u64,
  pub docker_version: Option<String>,
  pub podman_installed: bool,
  pub podman_images_size: u64,
  pub podman_containers_count: usize,
  pub podman_version: Option<String>,
}

#[tauri::command]
pub fn docker_system_prune(all: bool) -> Result<ResponseModel, ResponseModel> {
  ContainerService
    .docker_system_prune(all)
    .map_err(|e| e.into_response())
}

#[tauri::command]
pub fn docker_image_prune(all: bool) -> Result<ResponseModel, ResponseModel> {
  ContainerService
    .docker_image_prune(all)
    .map_err(|e| e.into_response())
}

#[tauri::command]
pub fn docker_container_prune() -> Result<ResponseModel, ResponseModel> {
  ContainerService
    .docker_container_prune()
    .map_err(|e| e.into_response())
}

#[tauri::command]
pub fn docker_volume_prune() -> Result<ResponseModel, ResponseModel> {
  ContainerService
    .docker_volume_prune()
    .map_err(|e| e.into_response())
}

#[tauri::command]
pub fn docker_preview_prune(all: bool) -> Result<ResponseModel, ResponseModel> {
  ContainerService
    .docker_preview_prune(all)
    .map_err(|e| e.into_response())
}

#[tauri::command]
pub fn podman_system_prune(all: bool) -> Result<ResponseModel, ResponseModel> {
  ContainerService
    .podman_system_prune(all)
    .map_err(|e| e.into_response())
}

#[tauri::command]
pub fn podman_image_prune(all: bool) -> Result<ResponseModel, ResponseModel> {
  ContainerService
    .podman_image_prune(all)
    .map_err(|e| e.into_response())
}
