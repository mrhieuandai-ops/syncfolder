use serde::{Deserialize, Serialize};

/// Sync job status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Scheduled,
    Queued,
    Running,
    Succeeded,
    Failed,
    Skipped,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Scheduled => write!(f, "scheduled"),
            JobStatus::Queued => write!(f, "queued"),
            JobStatus::Running => write!(f, "running"),
            JobStatus::Succeeded => write!(f, "succeeded"),
            JobStatus::Failed => write!(f, "failed"),
            JobStatus::Skipped => write!(f, "skipped"),
        }
    }
}

impl std::str::FromStr for JobStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "scheduled" => Ok(JobStatus::Scheduled),
            "queued" => Ok(JobStatus::Queued),
            "running" => Ok(JobStatus::Running),
            "succeeded" => Ok(JobStatus::Succeeded),
            "failed" => Ok(JobStatus::Failed),
            "skipped" => Ok(JobStatus::Skipped),
            _ => Err(format!("Unknown job status: {}", s)),
        }
    }
}

/// Sync job source enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JobSource {
    Manual,
    Scheduled,
}

impl std::fmt::Display for JobSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobSource::Manual => write!(f, "manual"),
            JobSource::Scheduled => write!(f, "scheduled"),
        }
    }
}

impl std::str::FromStr for JobSource {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "manual" => Ok(JobSource::Manual),
            "scheduled" => Ok(JobSource::Scheduled),
            _ => Err(format!("Unknown job source: {}", s)),
        }
    }
}

/// Sync job - represents a single sync execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncJob {
    pub id: String,
    pub profile_id: String,
    pub status: JobStatus,
    pub source: JobSource,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub items_processed: i32,
    pub items_added: i32,
    pub items_updated: i32,
    pub items_deleted: i32,
    pub skipped_reason: Option<String>,
    pub created_at: String,
}

impl SyncJob {
    pub fn new(profile_id: String, source: JobSource) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            profile_id,
            status: JobStatus::Queued,
            source,
            error_code: None,
            error_message: None,
            started_at: None,
            completed_at: None,
            items_processed: 0,
            items_added: 0,
            items_updated: 0,
            items_deleted: 0,
            skipped_reason: None,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn mark_running(&mut self) {
        self.status = JobStatus::Running;
        self.started_at = Some(chrono::Utc::now().to_rfc3339());
    }

    pub fn mark_succeeded(&mut self) {
        self.status = JobStatus::Succeeded;
        self.completed_at = Some(chrono::Utc::now().to_rfc3339());
    }

    pub fn mark_failed(&mut self, error_code: String, error_message: String) {
        self.status = JobStatus::Failed;
        self.error_code = Some(error_code);
        self.error_message = Some(error_message);
        self.completed_at = Some(chrono::Utc::now().to_rfc3339());
    }

    pub fn mark_skipped(&mut self, reason: String) {
        self.status = JobStatus::Skipped;
        self.skipped_reason = Some(reason);
        self.completed_at = Some(chrono::Utc::now().to_rfc3339());
    }
}
