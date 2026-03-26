//! Structured logging for sync jobs (Story 3.3)
//! Logs are stored in SQLite sync_events table and can be retrieved by job_id

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Structured log entry for a sync job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobLogEntry {
    pub id: String,
    pub job_id: String,
    pub profile_id: String,
    pub timestamp: String,
    pub event_type: JobEventType,
    pub message: String,
    pub details: Option<String>,
    pub error_code: Option<String>,
}

/// Type of job event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobEventType {
    JobStarted,
    JobProgress,
    JobCompleted,
    JobFailed,
    Error,
}

/// Summary of a job for history list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobLogSummary {
    pub job_id: String,
    pub profile_id: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub status: String,
    pub items_processed: Option<u32>,
    pub items_failed: Option<u32>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

/// Job logger - creates structured log entries
pub struct JobLogger;

impl JobLogger {
    /// Create a new job log entry
    pub fn new_log_entry(
        job_id: &str,
        profile_id: &str,
        event_type: JobEventType,
        message: &str,
    ) -> JobLogEntry {
        JobLogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            job_id: job_id.to_string(),
            profile_id: profile_id.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            event_type,
            message: message.to_string(),
            details: None,
            error_code: None,
        }
    }

    /// Create a job started entry
    pub fn job_started(job_id: &str, profile_id: &str) -> JobLogEntry {
        Self::new_log_entry(
            job_id,
            profile_id,
            JobEventType::JobStarted,
            "Sync job started",
        )
    }

    /// Create a job progress entry
    pub fn job_progress(
        job_id: &str,
        profile_id: &str,
        processed_items: u32,
        current_path: Option<&str>,
    ) -> JobLogEntry {
        let mut entry = Self::new_log_entry(
            job_id,
            profile_id,
            JobEventType::JobProgress,
            &format!("Processed {} items", processed_items),
        );
        
        if let Some(path) = current_path {
            entry.details = Some(format!("Current: {}", path));
        }
        
        entry
    }

    /// Create a job completed entry
    pub fn job_completed(
        job_id: &str,
        profile_id: &str,
        items_processed: u32,
        items_failed: u32,
    ) -> JobLogEntry {
        let message = if items_failed > 0 {
            format!(
                "Sync completed with {} items processed and {} items failed",
                items_processed, items_failed
            )
        } else {
            format!("Sync completed successfully with {} items processed", items_processed)
        };
        
        let mut entry = Self::new_log_entry(
            job_id,
            profile_id,
            JobEventType::JobCompleted,
            &message,
        );
        
        entry.details = Some(format!("processed:{},failed:{}", items_processed, items_failed));
        
        entry
    }

    /// Create a job failed entry
    pub fn job_failed(
        job_id: &str,
        profile_id: &str,
        error_code: &str,
        error_message: &str,
    ) -> JobLogEntry {
        let mut entry = Self::new_log_entry(
            job_id,
            profile_id,
            JobEventType::JobFailed,
            &format!("Sync failed: {}", error_message),
        );
        
        entry.error_code = Some(error_code.to_string());
        entry.details = Some(error_message.to_string());
        
        entry
    }

    /// Create an error entry
    pub fn error(
        job_id: &str,
        profile_id: &str,
        error_code: &str,
        error_message: &str,
        context: Option<&str>,
    ) -> JobLogEntry {
        let mut entry = Self::new_log_entry(
            job_id,
            profile_id,
            JobEventType::Error,
            &format!("Error: {}", error_message),
        );
        
        entry.error_code = Some(error_code.to_string());
        
        if let Some(ctx) = context {
            entry.details = Some(ctx.to_string());
        }
        
        entry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_started_entry() {
        let entry = JobLogger::job_started("job-123", "profile-456");
        
        assert_eq!(entry.job_id, "job-123");
        assert_eq!(entry.profile_id, "profile-456");
        assert!(matches!(entry.event_type, JobEventType::JobStarted));
    }

    #[test]
    fn test_job_failed_entry() {
        let entry = JobLogger::job_failed(
            "job-123",
            "profile-456",
            "SOURCE_NOT_FOUND",
            "Source directory does not exist",
        );
        
        assert_eq!(entry.error_code, Some("SOURCE_NOT_FOUND".to_string()));
        assert!(entry.details.is_some());
    }
}
