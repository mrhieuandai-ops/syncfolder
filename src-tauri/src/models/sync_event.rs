use serde::{Deserialize, Serialize};

/// Sync event types
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

/// Sync event - detailed log entry for a sync job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEvent {
    pub id: String,
    pub job_id: String,
    pub event_type: SyncEventType,
    pub profile_id: String,
    pub timestamp: String,
    pub message: Option<String>,
    pub details: Option<String>,
    pub file_path: Option<String>,
    pub action: Option<String>,
    pub result: Option<String>,
}

impl SyncEvent {
    pub fn new(
        job_id: String,
        profile_id: String,
        event_type: SyncEventType,
        message: Option<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            job_id,
            event_type,
            profile_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
            message,
            details: None,
            file_path: None,
            action: None,
            result: None,
        }
    }

    pub fn with_file_info(mut self, file_path: String, action: String, result: String) -> Self {
        self.file_path = Some(file_path);
        self.action = Some(action);
        self.result = Some(result);
        self
    }
}
