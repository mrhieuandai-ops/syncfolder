//! Logs Tauri commands (Story 3.3, 3.4)
//!
//! Commands:
//! - get_job_logs: Get logs for a specific job
//! - list_jobs: Get job history for a profile
//! - get_last_job_status: Get last job status for dashboard

use crate::errors::AppError;
use crate::models::{SyncEvent, SyncJob, JobStatus, SyncEventType};
use crate::repositories::{JobsRepository, EventsRepository};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

/// Log entry response for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntryResponse {
    pub id: String,
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub details: Option<String>,
    pub error_code: Option<String>,
}

/// Job logs response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobLogsResponse {
    pub job_id: String,
    pub logs: Vec<LogEntryResponse>,
}

/// Job summary for history list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobSummaryResponse {
    pub job_id: String,
    pub profile_id: String,
    pub status: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub items_processed: Option<i32>,
    pub items_failed: Option<i32>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

/// Jobs list response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobsListResponse {
    pub jobs: Vec<JobSummaryResponse>,
}

/// Application state for log operations
pub struct LogState {
    pub jobs_repo: Arc<JobsRepository>,
    pub events_repo: Arc<EventsRepository>,
}

impl LogState {
    pub fn new(
        jobs_repo: Arc<JobsRepository>,
        events_repo: Arc<EventsRepository>,
    ) -> Self {
        Self {
            jobs_repo,
            events_repo,
        }
    }
}

/// Get logs for a specific job
///
/// AC: Given a job ID, when user requests logs
/// Then return structured log entries with human-readable messages
#[tauri::command]
pub async fn get_job_logs(
    job_id: String,
    state: State<'_, LogState>,
) -> Result<JobLogsResponse, AppError> {
    // Get job to verify it exists
    let _job = state.jobs_repo.get_by_id(&job_id)
        .map_err(|e| AppError::validation_error(&format!("Lỗi truy vấn job: {}", e)))?
        .ok_or_else(|| AppError::validation_error("Job không tồn tại"))?;

    // Get events for this job
    let events = state.events_repo.get_by_job_id(&job_id)
        .map_err(|e| AppError::validation_error(&format!("Lỗi truy vấn log: {}", e)))?;

    // Convert to log entries with human-readable messages (Story 3.4 AC#2)
    let logs: Vec<LogEntryResponse> = events.iter().map(|event| {
        let level = match event.event_type {
            SyncEventType::Failed => "error",
            SyncEventType::Completed => "success",
            SyncEventType::Progress => "info",
            SyncEventType::Started => "info",
            SyncEventType::Queued => "info",
            SyncEventType::Skipped => "warning",
        };

        // Apply user-friendly message mapping
        let message = if let Some(ref err_code) = event.error_code {
            map_error_to_user_message(err_code)
        } else {
            event.message.clone().unwrap_or_else(|| get_default_message(&event.event_type))
        };

        LogEntryResponse {
            id: event.id.clone(),
            timestamp: event.timestamp.clone(),
            level: level.to_string(),
            message,
            details: event.details.clone(),
            error_code: event.error_code.clone(),
        }
    }).collect();

    Ok(JobLogsResponse {
        job_id,
        logs,
    })
}

/// Get list of jobs for a profile (Story 3.4 AC#1)
#[tauri::command]
pub async fn list_jobs(
    profile_id: String,
    limit: Option<i32>,
    state: State<'_, LogState>,
) -> Result<JobsListResponse, AppError> {
    let limit = limit.unwrap_or(50);

    // Get all jobs and filter if profile_id is provided
    let all_jobs = state.jobs_repo.get_all()
        .map_err(|e| AppError::validation_error(&format!("Lỗi truy vấn jobs: {}", e)))?;

    let mut filtered_jobs: Vec<SyncJob> = if profile_id.is_empty() {
        all_jobs
    } else {
        all_jobs.into_iter()
            .filter(|j| j.profile_id == profile_id)
            .collect()
    };

    // Sort by created_at descending
    filtered_jobs.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    // Take limit
    filtered_jobs.truncate(limit as usize);

    // Convert to response
    let jobs: Vec<JobSummaryResponse> = filtered_jobs.into_iter().map(|job| {
        JobSummaryResponse {
            job_id: job.id,
            profile_id: job.profile_id,
            status: job.status.to_string(),
            started_at: job.started_at.unwrap_or_default(),
            completed_at: job.completed_at,
            items_processed: job.items_processed,
            items_failed: job.items_failed,
            error_code: job.error_code,
            error_message: job.error_message,
        }
    }).collect();

    Ok(JobsListResponse { jobs })
}

/// Get last job status for a profile (for dashboard)
#[tauri::command]
pub async fn get_last_job_status(
    profile_id: String,
    state: State<'_, LogState>,
) -> Result<Option<JobSummaryResponse>, AppError> {
    let job = state.jobs_repo.get_last_for_profile(&profile_id)
        .map_err(|e| AppError::validation_error(&format!("Lỗi truy vấn: {}", e)))?;

    Ok(job.map(|j| JobSummaryResponse {
        job_id: j.id,
        profile_id: j.profile_id,
        status: j.status.to_string(),
        started_at: j.started_at.unwrap_or_default(),
        completed_at: j.completed_at,
        items_processed: j.items_processed,
        items_failed: j.items_failed,
        error_code: j.error_code,
        error_message: j.error_message,
    }))
}

/// Map error code to Vietnamese user-friendly message (Story 3.4 AC#2, NFR12)
fn map_error_to_user_message(error_code: &str) -> String {
    match error_code {
        // Availability errors
        "SOURCE_NOT_FOUND" => "Thư mục nguồn không tồn tại".to_string(),
        "DESTINATION_NOT_FOUND" => "Thư mục đích không tồn tại".to_string(),
        "SOURCE_UNAVAILABLE" => "Thư mục nguồn hiện không khả dụng".to_string(),
        "DESTINATION_UNAVAILABLE" => "Thư mục đích hiện không khả dụng".to_string(),
        "ENOENT" => "Thư mục hoặc file không tồn tại".to_string(),

        // Access errors
        "ACCESS_DENIED_SOURCE" | "EACCES" => "Không có quyền truy cập thư mục nguồn".to_string(),
        "ACCESS_DENIED_DESTINATION" => "Không có quyền truy cập thư mục đích".to_string(),

        // Lock errors
        "FILE_LOCKED" | "EBUSY" => "File đang bị khóa bởi ứng dụng khác".to_string(),

        // Validation errors
        "INVALID_CONFIG" | "EINVAL" => "Cấu hình hồ sơ không hợp lệ".to_string(),
        "PROFILE_NOT_FOUND" => "Không tìm thấy hồ sơ đồng bộ".to_string(),
        "JOB_RUNNING" => "Đang có job đồng bộ khác đang chạy".to_string(),

        _ => format!("Lỗi: {}", error_code),
    }
}

/// Get default message for event type
fn get_default_message(event_type: &SyncEventType) -> String {
    match event_type {
        SyncEventType::Started => "Đã bắt đầu đồng bộ".to_string(),
        SyncEventType::Completed => "Đồng bộ hoàn tất".to_string(),
        SyncEventType::Failed => "Đồng bộ thất bại".to_string(),
        SyncEventType::Progress => "Đang xử lý...".to_string(),
        SyncEventType::Queued => "Đã đưa vào hàng đợi".to_string(),
        SyncEventType::Skipped => "Job đã bị bỏ qua".to_string(),
    }
}
