//! ProfileService — manages cleaning profiles via global_state JsonDocService.

use crate::domain::entities::cleaning_profile::CleaningProfile;
use crate::error::AppError;
use crate::global_state::cleaning_profile_service;
use uuid::Uuid;

pub type Result<T, E = AppError> = std::result::Result<T, E>;

/// ProfileService for managing cleaning profiles.
pub struct ProfileService;

impl ProfileService {
  /// Get all cleaning profiles.
  /// Note: JsonDocService get returns Option<T>, iterate through known IDs.
  pub async fn get_profiles() -> Result<Vec<CleaningProfile>> {
    // JsonDocService::get returns Option<T>, scan for profiles
    // Return empty vec as placeholder since service needs ID-based access
    Ok(Vec::new())
  }

  /// Create a new cleaning profile.
  pub async fn create_profile(profile: CleaningProfile) -> Result<CleaningProfile> {
    let svc = cleaning_profile_service();
    let id = profile
      .id
      .clone()
      .unwrap_or_else(|| Uuid::new_v4().to_string());
    let profile_with_id = CleaningProfile {
      id: Some(id.clone()),
      ..profile
    };
    svc
      .save(&id, profile_with_id.clone())
      .await
      .map_err(|e| AppError::Internal(e.to_string().into()))?;
    Ok(profile_with_id)
  }

  /// Update an existing cleaning profile.
  pub async fn update_profile(id: &str, profile: CleaningProfile) -> Result<CleaningProfile> {
    let svc = cleaning_profile_service();
    svc
      .save(id, profile.clone())
      .await
      .map_err(|e| AppError::Internal(e.to_string().into()))?;
    Ok(profile)
  }

  /// Delete a cleaning profile by ID.
  pub async fn delete_profile(id: &str) -> Result<bool> {
    let svc = cleaning_profile_service();
    svc
      .delete(id)
      .await
      .map_err(|e| AppError::Internal(e.to_string().into()))?;
    Ok(true)
  }

  /// Switch to a different profile by ID.
  pub async fn switch_profile(id: &str) -> Result<CleaningProfile> {
    let svc = cleaning_profile_service();
    let profile = svc
      .get(id)
      .await
      .map_err(|e| AppError::Internal(e.to_string().into()))?;
    match profile {
      Some(p) => Ok(p),
      None => Err(AppError::Internal("Profile not found".into())),
    }
  }
}
