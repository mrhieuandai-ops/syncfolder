use serde::{Deserialize, Serialize};

/// Sync profile - defines a sync relationship between two folders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncProfile {
    pub id: String,
    pub name: String,
    pub source_path: String,
    pub destination_path: String,
    pub schedule_interval: Option<i32>, // minutes: 30, 60, 90, 1440
    pub auto_sync_enabled: bool,
    pub last_run_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl SyncProfile {
    pub fn new(name: String, source_path: String, destination_path: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            source_path,
            destination_path,
            schedule_interval: None,
            auto_sync_enabled: false,
            last_run_at: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}
