//! Profile management Tauri commands
//! 
//! Commands:
//! - create_profile: Create a new sync profile
//! - get_profile: Get a profile by ID
//! - list_profiles: List all profiles
//! - update_profile: Update an existing profile
//! - delete_profile: Delete a profile

use crate::AppState;
use crate::errors::{AppError, AppResult};
use crate::models::{SyncProfile, SyncJob, JobSource};
use crate::repositories::ProfilesRepository;
use crate::services::scheduler::job_scheduler::SCHEDULE_INTERVALS;
use serde::{Deserialize, Serialize};

/// Request payload for creating a profile
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProfileRequest {
    pub name: String,
    pub source_path: String,
    pub destination_path: String,
    pub schedule_minutes: Option<i32>,
    pub is_enabled: bool,
}

/// Request payload for updating a profile
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileRequest {
    pub id: String,
    pub name: Option<String>,
    pub source_path: Option<String>,
    pub destination_path: Option<String>,
    pub schedule_minutes: Option<i32>,
    pub is_enabled: Option<bool>,
}

/// Response format for profile operations
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    pub id: String,
    pub name: String,
    pub source_path: String,
    pub destination_path: String,
    pub schedule_minutes: Option<i32>,
    pub is_enabled: bool,
    pub last_run_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<SyncProfile> for ProfileResponse {
    fn from(p: SyncProfile) -> Self {
        Self {
            id: p.id,
            name: p.name,
            source_path: p.source_path,
            destination_path: p.destination_path,
            schedule_minutes: p.schedule_interval,
            is_enabled: p.auto_sync_enabled,
            last_run_at: p.last_run_at,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}

/// Validate schedule minutes value
fn validate_schedule(minutes: Option<i32>) -> AppResult<Option<i32>> {
    match minutes {
        Some(m) if SCHEDULE_INTERVALS.contains(&m) => Ok(Some(m)),
        Some(m) => Err(AppError::validation_error(&format!(
            "Invalid schedule interval: {}. Must be one of: {:?}",
            m, SCHEDULE_INTERVALS
        ))),
        None => Ok(None),
    }
}

/// Validate that a path exists and is accessible
fn validate_path_exists(path: &str, path_type: &str) -> AppResult<()> {
    let p = std::path::Path::new(path);
    if !p.exists() {
        return Err(AppError::validation_error(&format!(
            "{} path does not exist: {}",
            path_type, path
        )));
    }
    if !p.is_dir() {
        return Err(AppError::validation_error(&format!(
            "{} path is not a directory: {}",
            path_type, path
        )));
    }
    Ok(())
}

/// Validate source and destination are different
fn validate_source_not_destination(source: &str, destination: &str) -> AppResult<()> {
    let source_path = std::path::Path::new(source);
    let dest_path = std::path::Path::new(destination);
    
    // Check if paths are the same (after canonicalization)
    if let (Ok(source_canon), Ok(dest_canon)) = (source_path.canonicalize(), dest_path.canonicalize()) {
        if source_canon == dest_canon {
            return Err(AppError::validation_error(
                "Source and destination paths cannot be the same"
            ));
        }
    }
    Ok(())
}

/// Create a new sync profile
#[tauri::command]
pub fn create_profile(
    state: tauri::State<AppState>,
    request: CreateProfileRequest,
) -> AppResult<ProfileResponse> {
    log::info!("Creating profile: {:?}", request.name);

    // Validate required fields
    if request.source_path.trim().is_empty() {
        return Err(AppError::validation_error("Source path is required"));
    }
    if request.destination_path.trim().is_empty() {
        return Err(AppError::validation_error("Destination path is required"));
    }
    if request.name.trim().is_empty() {
        return Err(AppError::validation_error("Profile name is required"));
    }

    // Validate path existence
    validate_path_exists(&request.source_path, "Source")?;
    validate_path_exists(&request.destination_path, "Destination")?;

    // Validate source != destination
    validate_source_not_destination(&request.source_path, &request.destination_path)?;

    // Validate schedule
    validate_schedule(request.schedule_minutes)?;

    // Validate auto_sync=true requires schedule_interval
    if request.is_enabled && request.schedule_minutes.is_none() {
        return Err(AppError::validation_error(
            "Auto-sync cannot be enabled without a schedule interval"
        ));
    }

    // Create profile model
    let profile = SyncProfile::new(
        request.name,
        request.source_path,
        request.destination_path,
    );

    // Override schedule and auto_sync if provided
    let mut profile = profile;
    if let Some(minutes) = request.schedule_minutes {
        profile.schedule_interval = Some(minutes);
    }
    profile.auto_sync_enabled = request.is_enabled;

    // Save to database
    let repo = ProfilesRepository::new(state.db.clone());
    repo.create(&profile).map_err(|e| {
        log::error!("Failed to create profile: {}", e);
        AppError::validation_error(&format!("Failed to save profile: {}", e))
    })?;

    log::info!("Profile created successfully: {}", profile.id);
    Ok(profile.into())
}

/// Get a profile by ID
#[tauri::command]
pub fn get_profile(
    state: tauri::State<AppState>,
    id: String,
) -> AppResult<Option<ProfileResponse>> {
    log::info!("Getting profile: {}", id);
    
    let repo = ProfilesRepository::new(state.db.clone());
    let profile = repo.get_by_id(&id).map_err(|e| {
        log::error!("Failed to get profile: {}", e);
        AppError::validation_error(&format!("Failed to retrieve profile: {}", e))
    })?;
    
    Ok(profile.map(|p| p.into()))
}

/// List all profiles
#[tauri::command]
pub fn list_profiles(
    state: tauri::State<AppState>,
) -> AppResult<Vec<ProfileResponse>> {
    log::info!("Listing all profiles");
    
    let repo = ProfilesRepository::new(state.db.clone());
    let profiles = repo.get_all().map_err(|e| {
        log::error!("Failed to list profiles: {}", e);
        AppError::validation_error(&format!("Failed to retrieve profiles: {}", e))
    })?;
    
    Ok(profiles.into_iter().map(|p| p.into()).collect())
}

/// Update an existing profile
#[tauri::command]
pub fn update_profile(
    state: tauri::State<AppState>,
    request: UpdateProfileRequest,
) -> AppResult<ProfileResponse> {
    log::info!("Updating profile: {}", request.id);

    // Validate schedule if provided
    validate_schedule(request.schedule_minutes)?;

    // Get existing profile
    let repo = ProfilesRepository::new(state.db.clone());
    let mut profile = repo.get_by_id(&request.id).map_err(|e| {
        log::error!("Failed to get profile for update: {}", e);
        AppError::validation_error(&format!("Failed to retrieve profile: {}", e))
    })?
    .ok_or_else(|| AppError::validation_error("Profile not found"))?;

    // Determine new paths (use existing if not provided)
    let new_source = request.source_path.as_ref()
        .filter(|p| !p.trim().is_empty())
        .map(|p| p.as_str())
        .unwrap_or(&profile.source_path);
    let new_dest = request.destination_path.as_ref()
        .filter(|p| !p.trim().is_empty())
        .map(|p| p.as_str())
        .unwrap_or(&profile.destination_path);

    // Validate source != destination for new paths
    validate_source_not_destination(new_source, new_dest)?;

    // Validate path existence for new paths
    if request.source_path.is_some() && !request.source_path.as_ref().unwrap().trim().is_empty() {
        validate_path_exists(&request.source_path.as_ref().unwrap(), "Source")?;
    }
    if request.destination_path.is_some() && !request.destination_path.as_ref().unwrap().trim().is_empty() {
        validate_path_exists(&request.destination_path.as_ref().unwrap(), "Destination")?;
    }

    // Update fields if provided
    if let Some(name) = request.name {
        profile.name = name;
    }
    if let Some(source_path) = request.source_path {
        if !source_path.trim().is_empty() {
            profile.source_path = source_path;
        }
    }
    if let Some(destination_path) = request.destination_path {
        if !destination_path.trim().is_empty() {
            profile.destination_path = destination_path;
        }
    }
    if let Some(schedule_minutes) = request.schedule_minutes {
        profile.schedule_interval = Some(schedule_minutes);
    }
    if let Some(is_enabled) = request.is_enabled {
        profile.auto_sync_enabled = is_enabled;
    }

    // Validate auto_sync=true requires schedule_interval
    if profile.auto_sync_enabled && profile.schedule_interval.is_none() {
        return Err(AppError::validation_error(
            "Auto-sync cannot be enabled without a schedule interval"
        ));
    }
    
    // Update timestamp
    profile.updated_at = chrono::Utc::now().to_rfc3339();

    // Save to database using repository
    repo.update(&profile).map_err(|e| {
        log::error!("Failed to update profile: {}", e);
        AppError::validation_error(&format!("Failed to update profile: {}", e))
    })?;

    log::info!("Profile updated successfully: {}", profile.id);
    Ok(profile.into())
}

/// Delete a profile
#[tauri::command]
pub fn delete_profile(
    state: tauri::State<AppState>,
    id: String,
) -> AppResult<()> {
    log::info!("Deleting profile: {}", id);

    let repo = ProfilesRepository::new(state.db.clone());
    repo.delete(&id).map_err(|e| {
        log::error!("Failed to delete profile: {}", e);
        AppError::validation_error(&format!("Failed to delete profile: {}", e))
    })?;

    log::info!("Profile deleted successfully: {}", id);
    Ok(())
}
