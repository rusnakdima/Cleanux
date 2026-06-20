use crate::models::Response;
use crate::services::app_residue_service::{AppResidueService, AppResidueSummary};
use crate::services::automation_service::{AutomationRecipe, AutomationService};
use crate::services::container_service::ContainerService;
use crate::services::repair_service::RepairService;
use crate::services::startup_service::StartupService;
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn find_broken_symlinks() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  RepairService::find_broken_symlinks()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn find_orphaned_packages() -> Result<Response<serde_json::Value>, Response<serde_json::Value>>
{
  RepairService::find_orphaned_packages()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn clean_font_cache() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  RepairService::clean_font_cache()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn clean_repair_icon_cache() -> Result<Response<serde_json::Value>, Response<serde_json::Value>>
{
  RepairService::clean_icon_cache()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn repair_permissions() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  RepairService::repair_permissions()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn remove_broken_symlink(
  path: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  RepairService::remove_broken_symlink(&path)
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn clean_repair_orphaned_pkg(
  path: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  RepairService::remove_orphaned_package(&path)
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_startup_items() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  StartupService::get_startup_items()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn disable_startup_item(
  path: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  StartupService::disable_startup_item(&path)
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn enable_startup_item(
  path: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  StartupService::enable_startup_item(&path)
}
#[tauri::command(rename_all = "camelCase")]
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
#[tauri::command(rename_all = "camelCase")]
pub fn docker_system_prune(
  all: bool,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  ContainerService
    .docker_system_prune(all)
    .map_err(|e| e.into_response())
}
#[tauri::command(rename_all = "camelCase")]
pub fn docker_image_prune(
  all: bool,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  ContainerService
    .docker_image_prune(all)
    .map_err(|e| e.into_response())
}
#[tauri::command(rename_all = "camelCase")]
pub fn docker_container_prune() -> Result<Response<serde_json::Value>, Response<serde_json::Value>>
{
  ContainerService
    .docker_container_prune()
    .map_err(|e| e.into_response())
}
#[tauri::command(rename_all = "camelCase")]
pub fn docker_volume_prune() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  ContainerService
    .docker_volume_prune()
    .map_err(|e| e.into_response())
}
#[tauri::command(rename_all = "camelCase")]
pub fn docker_preview_prune(
  all: bool,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  ContainerService
    .docker_preview_prune(all)
    .map_err(|e| e.into_response())
}
#[tauri::command(rename_all = "camelCase")]
pub fn podman_system_prune(
  all: bool,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  ContainerService
    .podman_system_prune(all)
    .map_err(|e| e.into_response())
}
#[tauri::command(rename_all = "camelCase")]
pub fn podman_image_prune(
  all: bool,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  ContainerService
    .podman_image_prune(all)
    .map_err(|e| e.into_response())
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_quick_actions() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  AutomationService::get_quick_actions()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn execute_action(
  action_id: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  AutomationService::execute_action(action_id)
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_recipes() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  AutomationService::get_recipes()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn save_recipe(
  recipe: AutomationRecipe,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  AutomationService::save_recipe(recipe)
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn delete_recipe(
  recipe_id: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  AutomationService::delete_recipe(recipe_id)
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn execute_recipe(
  recipe_id: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  AutomationService::execute_recipe(recipe_id)
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_execution_history() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  AutomationService::get_execution_history_list()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_residue_summary() -> Result<AppResidueSummary, Response<serde_json::Value>> {
  Ok(AppResidueService.get_residue_summary())
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn scan_user_configs() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  AppResidueService.scan_user_configs_response()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn scan_user_data() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  AppResidueService.scan_user_data_response()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn scan_user_caches() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  AppResidueService.scan_user_caches_response()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn scan_home_residues() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  AppResidueService.scan_home_residues_response()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn get_orphaned_configs() -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  AppResidueService.get_orphaned_configs_response()
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn clean_app_residue(
  path: String,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  AppResidueService.clean_residue(&path)
}
#[tauri::command(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub fn clean_multiple_app_residues(
  paths: Vec<String>,
) -> Result<Response<serde_json::Value>, Response<serde_json::Value>> {
  AppResidueService.clean_multiple(paths)
}
