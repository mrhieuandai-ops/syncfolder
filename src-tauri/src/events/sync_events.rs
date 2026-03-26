//! Sync event types and helpers
//! 
//! Event payload structure for all sync-related events

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

/// Sync event types following architecture naming convention
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncEventType {
    Queued,
    Started,
    Progress,
    Completed,
    Failed,
    Skipped,
}

impl std::fmt::Display for SyncEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncEventType::Queued => write!(f, "sync:queued"),
            SyncEventType::Started => write!(f, "sync:started"),
            SyncEventType::Progress => write!(f, "sync:progress"),
            SyncEventType::Completed => write!(f, "sync:completed"),
            SyncEventType::Failed => write!(f, "sync:failed"),
            SyncEventType::Skipped => write!(f, "sync:skipped"),
        }
    }
}

/// Event payload for sync queued
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncQueuedPayload {
    pub profile_id: String,
    pub job_id: String,
    pub timestamp: String,
}

/// Event payload for sync started
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncStartedPayload {
    pub profile_id: String,
    pub job_id: String,
    pub timestamp: String,
}

/// Event payload for sync progress
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncProgressPayload {
    pub profile_id: String,
    pub job_id: String,
    pub timestamp: String,
    pub phase: String,
    pub processed_items: i32,
    pub total_items: Option<i32>,
    pub current_path: Option<String>,
}

/// Event payload for sync completed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncCompletedPayload {
    pub profile_id: String,
    pub job_id: String,
    pub timestamp: String,
    pub status: String,
    pub items_processed: i32,
    pub items_failed: i32,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

/// Event payload for sync failed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncFailedPayload {
    pub profile_id: String,
    pub job_id: String,
    pub timestamp: String,
    pub error_code: String,
    pub error_message: String,
}

/// Event payload for sync skipped
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncSkippedPayload {
    pub profile_id: String,
    pub job_id: String,
    pub reason: String,
    pub skipped_at: String,
}

/// Event names as per architecture
pub const EVENT_SYNC_QUEUED: &str = "sync:queued";
pub const EVENT_SYNC_STARTED: &str = "sync:started";
pub const EVENT_SYNC_PROGRESS: &str = "sync:progress";
pub const EVENT_SYNC_COMPLETED: &str = "sync:completed";
pub const EVENT_SYNC_FAILED: &str = "sync:failed";
pub const EVENT_SYNC_SKIPPED: &str = "sync:skipped";

/// Emit sync:started event
pub fn emit_sync_started(app: &AppHandle, profile_id: &str, job_id: &str) {
    let payload = SyncStartedPayload {
        profile_id: profile_id.to_string(),
        job_id: job_id.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    if let Err(e) = app.emit(EVENT_SYNC_STARTED, payload) {
        log::error!("Failed to emit sync:started event: {}", e);
    }
}

/// Emit sync:queued event
pub fn emit_sync_queued(app: &AppHandle, profile_id: &str, job_id: &str) {
    let payload = SyncQueuedPayload {
        profile_id: profile_id.to_string(),
        job_id: job_id.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    if let Err(e) = app.emit(EVENT_SYNC_QUEUED, payload) {
        log::error!("Failed to emit sync:queued event: {}", e);
    }
}

/// Emit sync:progress event
pub fn emit_sync_progress(
    app: &AppHandle,
    profile_id: &str,
    job_id: &str,
    processed: i32,
    total: Option<i32>,
    current: Option<String>,
) {
    let payload = SyncProgressPayload {
        profile_id: profile_id.to_string(),
        job_id: job_id.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        phase: "running".to_string(),
        processed_items: processed,
        total_items: total,
        current_path: current,
    };
    if let Err(e) = app.emit(EVENT_SYNC_PROGRESS, payload) {
        log::error!("Failed to emit sync:progress event: {}", e);
    }
}

/// Emit sync:completed event
pub fn emit_sync_completed(
    app: &AppHandle,
    profile_id: &str,
    job_id: &str,
    items_added: i32,
    items_updated: i32,
    items_deleted: i32,
    error_code: Option<String>,
    error_message: Option<String>,
) {
    let items_processed = items_added + items_updated + items_deleted;
    let status = if error_code.is_some() {
        "failed"
    } else {
        "succeeded"
    };
    let payload = SyncCompletedPayload {
        profile_id: profile_id.to_string(),
        job_id: job_id.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        status: status.to_string(),
        items_processed,
        items_failed: 0, // TODO: track actual failures if available
        error_code,
        error_message,
    };
    if let Err(e) = app.emit(EVENT_SYNC_COMPLETED, payload) {
        log::error!("Failed to emit sync:completed event: {}", e);
    }
}

/// Emit sync:failed event
pub fn emit_sync_failed(
    app: &AppHandle,
    profile_id: &str,
    job_id: &str,
    error_code: &str,
    error_message: &str,
) {
    let payload = SyncFailedPayload {
        profile_id: profile_id.to_string(),
        job_id: job_id.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        error_code: error_code.to_string(),
        error_message: error_message.to_string(),
    };
    if let Err(e) = app.emit(EVENT_SYNC_FAILED, payload) {
        log::error!("Failed to emit sync:failed event: {}", e);
    }
}

/// Emit sync:skipped event
pub fn emit_sync_skipped(app: &AppHandle, profile_id: &str, job_id: &str, reason: &str) {
    let payload = SyncSkippedPayload {
        profile_id: profile_id.to_string(),
        job_id: job_id.to_string(),
        reason: reason.to_string(),
        skipped_at: chrono::Utc::now().to_rfc3339(),
    };
    if let Err(e) = app.emit(EVENT_SYNC_SKIPPED, payload) {
        log::error!("Failed to emit sync:skipped event: {}", e);
    }
}
