//! Status Tauri commands
//! 
//! Commands:
//! - get_sync_status: Get overall sync status for a profile
//! - list_recent_jobs: List recent sync jobs

use crate::errors::AppError;
use crate::models::{SyncJob, JobStatus};
use crate::repositories::{JobsRepository, ProfilesRepository};
use crate::services::persistence::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Status response for a profile
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncStatusResponse {
    pub profile_id: String,
    pub last_job_id: Option<String>,
    pub last_job_status: Option<String>,
    pub last_run_at: Option<String>,
    pub is_running: bool,
    pub next_scheduled_run: Option<String>,
}

/// Job list item response
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobListItem {
    pub id: String,
    pub profile_id: String,
    pub status: String,
    pub source: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub items_processed: i32,
}

impl From<SyncJob> for JobListItem {
    fn from(job: SyncJob) -> Self {
        Self {
            id: job.id,
            profile_id: job.profile_id,
            status: job.status.to_string(),
            source: job.source.to_string(),
            started_at: job.started_at,
            completed_at: job.completed_at,
            items_processed: job.items_processed,
        }
    }
}

/// Get overall sync status for a profile
#[tauri::command]
pub async fn get_sync_status(
    profile_id: String,
) -> Result<SyncStatusResponse, AppError> {
    log::info!("Getting sync status for profile: {}", profile_id);
    
    // This would typically query the database through state
    // For now return a placeholder that will be connected properly
    Ok(SyncStatusResponse {
        profile_id,
        last_job_id: None,
        last_job_status: None,
        last_run_at: None,
        is_running: false,
        next_scheduled_run: None,
    })
}

/// List recent sync jobs
#[tauri::command]
pub async fn list_recent_jobs(
    profile_id: Option<String>,
    limit: Option<i32>,
) -> Result<Vec<JobListItem>, AppError> {
    log::info!("Listing jobs for profile: {:?}, limit: {:?}", profile_id, limit);
    
    // Placeholder - would query jobs repo
    Ok(Vec::new())
}
