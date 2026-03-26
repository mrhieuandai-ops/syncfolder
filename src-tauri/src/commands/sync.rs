//! Sync command implementation
//! 
//! Implements:
//! - Story 2-1: Manual sync trigger
//! - Story 2-2: Scheduled sync with anti-overlap
//! - Story 2-3: Sync new files/folders
//! - Story 2-4: Sync changed files (two-way)
//! - Story 2-5: Deletion policy

use crate::errors::{AppError, SyncError};
use syncfolder_lib::events::sync_events;
use crate::models::{SyncJob, SyncProfile, JobStatus, JobSource, SyncEvent, SyncEventType};
use crate::repositories::{ProfilesRepository, JobsRepository, EventsRepository};
use crate::services::path_guard::PathGuard;
use crate::services::sync_engine::{
    Scanner, Comparer, Planner, Executor, 
    DeletionPolicy, DeletionPolicyEvaluator,
    SyncPlan, OperationType,
};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, State, Manager};

/// Application state for sync operations
pub struct SyncState {
    pub profiles_repo: Arc<ProfilesRepository>,
    pub jobs_repo: Arc<JobsRepository>,
    pub events_repo: Arc<EventsRepository>,
}

impl SyncState {
    pub fn new(
        profiles_repo: Arc<ProfilesRepository>,
        jobs_repo: Arc<JobsRepository>,
        events_repo: Arc<EventsRepository>,
    ) -> Self {
        Self {
            profiles_repo,
            jobs_repo,
            events_repo,
        }
    }
}

/// Response for sync operation
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncResponse {
    pub job_id: String,
    pub status: String,
    pub message: String,
    pub retryable: bool,
    pub items_added: i32,
    pub items_updated: i32,
    pub items_deleted: i32,
}

/// Run sync immediately for a profile
/// 
/// Implements Story 2-1: Manual sync trigger
#[tauri::command]
pub async fn run_sync_now(
    profile_id: String,
    state: State<'_, SyncState>,
    app: AppHandle,
) -> Result<SyncResponse, AppError> {
    // Step 1: Get profile
    let profile = state.profiles_repo.get_by_id(&profile_id)
        .map_err(|e| AppError::validation_error(&format!("Không tìm thấy profile: {}", e)))?
        .ok_or_else(|| AppError::validation_error("Profile không tồn tại"))?;

    // Step 2: Create new job
    let job = SyncJob::new(profile_id.clone(), JobSource::Manual);
    let job_id = job.id.clone();
    
    state.jobs_repo.create(&job)
        .map_err(|e| AppError::validation_error(&format!("Lỗi tạo job: {}", e)))?;

    // Step 3: Emit queued event (Story 2-1 AC1)
    sync_events::emit_sync_queued(&app, &profile_id, &job_id);

    // Step 4: Check if job already running for this profile (anti-overlap - Story 2-2)
    // Use jobs_repo.get_running_for_profile() consistent with scheduler
    if let Ok(Some(running_job)) = state.jobs_repo.get_running_for_profile(&profile_id) {
        let job_id = running_job.id.clone();
        let message = format!("Job đang chạy cho profile này: {}", running_job.id);
        return Ok(SyncResponse {
            job_id,
            status: "running".to_string(),
            message,
            retryable: false,
            items_added: 0,
            items_updated: 0,
            items_deleted: 0,
        });
    }

    // Step 5: Mark job as running
    let mut running_job = job;
    running_job.mark_running();
    let _ = state.jobs_repo.update(&running_job);

    // Step 5b: Emit started event (Job is now actually running)
    sync_events::emit_sync_started(&app, &profile_id, &job_id);

    // Step 6: Execute sync
    let result = execute_sync(
        &job_id,
        &profile_id,
        &profile,
        &state,
        &app,
    );

    // Step 7: Handle result and update profile last_run_at
    match result {
        Ok((items_added, items_updated, items_deleted)) => {
            let mut succeeded_job = running_job;
            succeeded_job.items_added = items_added;
            succeeded_job.items_updated = items_updated;
            succeeded_job.items_deleted = items_deleted;
            succeeded_job.mark_succeeded();
            let _ = state.jobs_repo.update(&succeeded_job);

            // Update profile last_run_at (Story 2-1 AC2)
            let _ = state.profiles_repo.update_last_run(
                &profile_id,
                &chrono::Utc::now().to_rfc3339(),
            );

            // Emit completed event
            sync_events::emit_sync_completed(
                &app, &profile_id, &job_id, items_added, items_updated, items_deleted, None, None,
            );

            Ok(SyncResponse {
                job_id,
                status: "succeeded".to_string(),
                message: "Đồng bộ hoàn tất thành công".to_string(),
                retryable: false,
                items_added,
                items_updated,
                items_deleted,
            })
        }
        Err(app_error) => {
            // Check if this is an availability error (paths unavailable) - should be "skipped" not "failed"
            let is_availability_error = app_error.error_type.as_deref() == Some("availability");
            
            if is_availability_error {
                // Mark job as skipped for availability issues (Story 4-1 AC)
                let mut skipped_job = running_job;
                let reason = app_error.message.clone();
                skipped_job.mark_skipped(reason.clone());
                let _ = state.jobs_repo.update(&skipped_job);

                // Emit skipped event
                sync_events::emit_sync_skipped(&app, &profile_id, &job_id, &reason);

                Ok(SyncResponse {
                    job_id,
                    status: "skipped".to_string(),
                    message: reason,
                    retryable: true,
                    items_added: 0,
                    items_updated: 0,
                    items_deleted: 0,
                })
            } else {
                // Mark job as failed for other errors
                let mut failed_job = running_job;
                failed_job.mark_failed(app_error.code.clone(), app_error.message.clone());
                let _ = state.jobs_repo.update(&failed_job);

                // Emit failed event
                sync_events::emit_sync_failed(&app, &profile_id, &job_id, &app_error.code, &app_error.message);

                Ok(SyncResponse {
                    job_id,
                    status: "failed".to_string(),
                    message: app_error.message,
                    retryable: app_error.retryable,
                    items_added: 0,
                    items_updated: 0,
                    items_deleted: 0,
                })
            }
        }
    }
}

/// Execute the full sync operation
fn execute_sync(
    job_id: &str,
    profile_id: &str,
    profile: &SyncProfile,
    state: &SyncState,
    app: &AppHandle,
) -> Result<(i32, i32, i32), AppError> {
    let source_path = PathBuf::from(&profile.source_path);
    let dest_path = PathBuf::from(&profile.destination_path);

    // Use PathGuard to check path availability (Story 4-1)
    // This validates source and destination are accessible before any operations
    let path_guard = PathGuard::new(source_path.clone(), dest_path.clone());
    if let Err(app_error) = path_guard.check_all_available() {
        return Err(app_error);
    }

    // Additional PathGuard validation before scanning (Story 4-4)
    // Ensures no file operations occur outside allowlist
    path_guard.validate_path(&source_path, "kiểm tra nguồn")?;
    path_guard.validate_path(&dest_path, "kiểm tra đích")?;

    // Emit progress
    sync_events::emit_sync_progress(
        app, profile_id, job_id, 0, None, Some("Đang quét thư mục...".to_string()),
    );

    // Story 2-3: Scan directories to detect new files/folders
    let scan_result = Scanner::scan(&source_path, &dest_path)
        .map_err(|e| AppError::validation_error(&format!("Lỗi quét thư mục: {}", e)))?;

    // Emit progress
    sync_events::emit_sync_progress(
        app, profile_id, job_id, 0, None, Some("Đang so sánh file...".to_string()),
    );

    // Story 2-4: Compare to detect changed files
    let compare_result = Comparer::compare(&scan_result);

    // Story 2-5: Deletion policy
    let deletion_policy = DeletionPolicyEvaluator::new(DeletionPolicy::TwoWaySync);
    let safety = deletion_policy.assess_safety(&scan_result);
    let should_skip_deletions = deletion_policy.should_skip_deletions(&safety);

    // Generate sync plan
    let mut plan = Planner::plan(&scan_result, &compare_result, false);

    // Add deletion operations if safe
    if !should_skip_deletions {
        let deletion_decisions = deletion_policy.evaluate(&scan_result);
        for decision in deletion_decisions {
            // Use correct deletion direction based on where the file was deleted
            let direction = if decision.deleted_from_source {
                crate::services::sync_engine::SyncDirection::DestToSource
            } else {
                crate::services::sync_engine::SyncDirection::SourceToDest
            };
            
            plan.operations.push(crate::services::sync_engine::SyncOperation {
                relative_path: decision.relative_path.clone(),
                operation_type: OperationType::Delete,
                direction,
                size: 0,
            });
            
            log::info!("Deletion: {} (deleted_from_source={}, reason={})", 
                decision.relative_path, decision.deleted_from_source, decision.reason);
        }
    }

    // Emit progress
    sync_events::emit_sync_progress(
        app, profile_id, job_id, 0, Some(plan.total_operations() as i32), Some("Đang đồng bộ...".to_string()),
    );

    // Execute the plan
    let exec_result = Executor::execute_with_rollback(&plan, &source_path, &dest_path);

    if !exec_result.success {
        return Err(AppError::validation_error(&exec_result.error.unwrap_or_else(|| "Lỗi đồng bộ không xác định".to_string())));
    }

    // Count operations by type (only files, not directories)
    let items_added = plan.operations.iter()
        .filter(|op| op.operation_type == OperationType::Create && op.size > 0)
        .count() as i32;
    let items_updated = plan.operations.iter()
        .filter(|op| op.operation_type == OperationType::Update && op.size > 0)
        .count() as i32;
    
    // Count items deleted from execution result (MEDIUM 8)
    // Count files (not directories) from source_only and dest_only
    let items_deleted = scan_result.source_only.iter()
        .filter(|path| {
            scan_result.source_manifest.get_file(path)
                .map(|meta| !meta.is_dir)
                .unwrap_or(false)
        })
        .count() as i32 +
        scan_result.dest_only.iter()
        .filter(|path| {
            scan_result.dest_manifest.get_file(path)
                .map(|meta| !meta.is_dir)
                .unwrap_or(false)
        })
        .count() as i32;

    Ok((items_added, items_updated, items_deleted))
}

/// Get job status
#[tauri::command]
pub async fn get_job_status(
    job_id: String,
    state: State<'_, SyncState>,
) -> Result<Option<SyncJob>, AppError> {
    state.jobs_repo.get_by_id(&job_id)
        .map_err(|e| AppError::validation_error(&format!("Lỗi truy vấn job: {}", e)))
}

/// Get last job for profile
#[tauri::command]
pub async fn get_last_job_for_profile(
    profile_id: String,
    state: State<'_, SyncState>,
) -> Result<Option<SyncJob>, AppError> {
    state.jobs_repo.get_last_for_profile(&profile_id)
        .map_err(|e| AppError::validation_error(&format!("Lỗi truy vấn job: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_response_serialization() {
        let response = SyncResponse {
            job_id: "test-job-1".to_string(),
            status: "succeeded".to_string(),
            message: "Thành công".to_string(),
            retryable: false,
            items_added: 5,
            items_updated: 3,
            items_deleted: 1,
        };
        
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"jobId\":\"test-job-1\""));
        assert!(json.contains("\"status\":\"succeeded\""));
        assert!(json.contains("\"itemsAdded\":5"));
    }
}
