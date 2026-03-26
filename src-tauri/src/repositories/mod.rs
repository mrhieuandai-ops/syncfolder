use crate::models::{SyncProfile, SyncJob, JobStatus, JobSource, SyncEvent, SyncEventType};
use crate::services::persistence::Database;
use rusqlite::params;
use std::sync::Arc;

pub struct ProfilesRepository {
    db: Arc<Database>,
}

impl ProfilesRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn create(&self, profile: &SyncProfile) -> Result<(), String> {
        let conn = self.db.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO sync_profiles (id, name, source_path, destination_path, schedule_interval, auto_sync_enabled, last_run_at, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                profile.id,
                profile.name,
                profile.source_path,
                profile.destination_path,
                profile.schedule_interval,
                profile.auto_sync_enabled as i32,
                profile.last_run_at,
                profile.created_at,
                profile.updated_at,
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_by_id(&self, id: &str) -> Result<Option<SyncProfile>, String> {
        let conn = self.db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, name, source_path, destination_path, schedule_interval, auto_sync_enabled, last_run_at, created_at, updated_at FROM sync_profiles WHERE id = ?1")
            .map_err(|e| e.to_string())?;
        
        let result = stmt.query_row(params![id], |row| {
            Ok(SyncProfile {
                id: row.get(0)?,
                name: row.get(1)?,
                source_path: row.get(2)?,
                destination_path: row.get(3)?,
                schedule_interval: row.get(4)?,
                auto_sync_enabled: row.get::<_, i32>(5)? != 0,
                last_run_at: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        });

        match result {
            Ok(profile) => Ok(Some(profile)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn get_all(&self) -> Result<Vec<SyncProfile>, String> {
        let conn = self.db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, name, source_path, destination_path, schedule_interval, auto_sync_enabled, last_run_at, created_at, updated_at FROM sync_profiles")
            .map_err(|e| e.to_string())?;
        
        let rows = stmt.query_map([], |row| {
            Ok(SyncProfile {
                id: row.get(0)?,
                name: row.get(1)?,
                source_path: row.get(2)?,
                destination_path: row.get(3)?,
                schedule_interval: row.get(4)?,
                auto_sync_enabled: row.get::<_, i32>(5)? != 0,
                last_run_at: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?;

        let mut profiles = Vec::new();
        for row in rows {
            profiles.push(row.map_err(|e| e.to_string())?);
        }
        Ok(profiles)
    }

    pub fn update_last_run(&self, profile_id: &str, last_run_at: &str) -> Result<(), String> {
        let conn = self.db.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "UPDATE sync_profiles SET last_run_at = ?1, updated_at = ?2 WHERE id = ?3",
            params![last_run_at, chrono::Utc::now().to_rfc3339(), profile_id],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn update(&self, profile: &SyncProfile) -> Result<(), String> {
        let conn = self.db.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "UPDATE sync_profiles SET name = ?1, source_path = ?2, destination_path = ?3, schedule_interval = ?4, auto_sync_enabled = ?5, last_run_at = ?6, updated_at = ?7 WHERE id = ?8",
            params![
                profile.name,
                profile.source_path,
                profile.destination_path,
                profile.schedule_interval,
                profile.auto_sync_enabled as i32,
                profile.last_run_at,
                profile.updated_at,
                profile.id,
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<(), String> {
        let conn = self.db.conn.lock().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM sync_profiles WHERE id = ?1", rusqlite::params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_auto_sync_enabled(&self) -> Result<Vec<SyncProfile>, String> {
        let conn = self.db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, name, source_path, destination_path, schedule_interval, auto_sync_enabled, last_run_at, created_at, updated_at FROM sync_profiles WHERE auto_sync_enabled = 1 AND schedule_interval IS NOT NULL")
            .map_err(|e| e.to_string())?;
        
        let rows = stmt.query_map([], |row| {
            Ok(SyncProfile {
                id: row.get(0)?,
                name: row.get(1)?,
                source_path: row.get(2)?,
                destination_path: row.get(3)?,
                schedule_interval: row.get(4)?,
                auto_sync_enabled: row.get::<_, i32>(5)? != 0,
                last_run_at: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?;

        let mut profiles = Vec::new();
        for row in rows {
            profiles.push(row.map_err(|e| e.to_string())?);
        }
        Ok(profiles)
    }
}

pub struct JobsRepository {
    db: Arc<Database>,
}

impl JobsRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn create(&self, job: &SyncJob) -> Result<(), String> {
        let conn = self.db.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO sync_jobs (id, profile_id, status, source, error_code, error_message, started_at, completed_at, items_processed, items_added, items_updated, items_deleted, skipped_reason, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                job.id,
                job.profile_id,
                job.status.to_string(),
                job.source.to_string(),
                job.error_code,
                job.error_message,
                job.started_at,
                job.completed_at,
                job.items_processed,
                job.items_added,
                job.items_updated,
                job.items_deleted,
                job.skipped_reason,
                job.created_at,
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn update(&self, job: &SyncJob) -> Result<(), String> {
        let conn = self.db.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "UPDATE sync_jobs SET status = ?1, error_code = ?2, error_message = ?3, started_at = ?4, completed_at = ?5, items_processed = ?6, items_added = ?7, items_updated = ?8, items_deleted = ?9, skipped_reason = ?10 WHERE id = ?11",
            params![
                job.status.to_string(),
                job.error_code,
                job.error_message,
                job.started_at,
                job.completed_at,
                job.items_processed,
                job.items_added,
                job.items_updated,
                job.items_deleted,
                job.skipped_reason,
                job.id,
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_by_id(&self, id: &str) -> Result<Option<SyncJob>, String> {
        let conn = self.db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, profile_id, status, source, error_code, error_message, started_at, completed_at, items_processed, items_added, items_updated, items_deleted, skipped_reason, created_at FROM sync_jobs WHERE id = ?1")
            .map_err(|e| e.to_string())?;
        
        let result = stmt.query_row(params![id], |row| {
            let status_str: String = row.get(2)?;
            let source_str: String = row.get(3)?;
            Ok(SyncJob {
                id: row.get(0)?,
                profile_id: row.get(1)?,
                status: status_str.parse().unwrap_or(JobStatus::Queued),
                source: source_str.parse().unwrap_or(JobSource::Manual),
                error_code: row.get(4)?,
                error_message: row.get(5)?,
                started_at: row.get(6)?,
                completed_at: row.get(7)?,
                items_processed: row.get(8)?,
                items_added: row.get(9)?,
                items_updated: row.get(10)?,
                items_deleted: row.get(11)?,
                skipped_reason: row.get(12)?,
                created_at: row.get(13)?,
            })
        });

        match result {
            Ok(job) => Ok(Some(job)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn get_running_for_profile(&self, profile_id: &str) -> Result<Option<SyncJob>, String> {
        let conn = self.db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, profile_id, status, source, error_code, error_message, started_at, completed_at, items_processed, items_added, items_updated, items_deleted, skipped_reason, created_at FROM sync_jobs WHERE profile_id = ?1 AND status = 'running'")
            .map_err(|e| e.to_string())?;
        
        let result = stmt.query_row(params![profile_id], |row| {
            let status_str: String = row.get(2)?;
            let source_str: String = row.get(3)?;
            Ok(SyncJob {
                id: row.get(0)?,
                profile_id: row.get(1)?,
                status: status_str.parse().unwrap_or(JobStatus::Queued),
                source: source_str.parse().unwrap_or(JobSource::Manual),
                error_code: row.get(4)?,
                error_message: row.get(5)?,
                started_at: row.get(6)?,
                completed_at: row.get(7)?,
                items_processed: row.get(8)?,
                items_added: row.get(9)?,
                items_updated: row.get(10)?,
                items_deleted: row.get(11)?,
                skipped_reason: row.get(12)?,
                created_at: row.get(13)?,
            })
        });

        match result {
            Ok(job) => Ok(Some(job)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn get_last_for_profile(&self, profile_id: &str) -> Result<Option<SyncJob>, String> {
        let conn = self.db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, profile_id, status, source, error_code, error_message, started_at, completed_at, items_processed, items_added, items_updated, items_deleted, skipped_reason, created_at FROM sync_jobs WHERE profile_id = ?1 ORDER BY created_at DESC LIMIT 1")
            .map_err(|e| e.to_string())?;
        
        let result = stmt.query_row(params![profile_id], |row| {
            let status_str: String = row.get(2)?;
            let source_str: String = row.get(3)?;
            Ok(SyncJob {
                id: row.get(0)?,
                profile_id: row.get(1)?,
                status: status_str.parse().unwrap_or(JobStatus::Queued),
                source: source_str.parse().unwrap_or(JobSource::Manual),
                error_code: row.get(4)?,
                error_message: row.get(5)?,
                started_at: row.get(6)?,
                completed_at: row.get(7)?,
                items_processed: row.get(8)?,
                items_added: row.get(9)?,
                items_updated: row.get(10)?,
                items_deleted: row.get(11)?,
                skipped_reason: row.get(12)?,
                created_at: row.get(13)?,
            })
        });

        match result {
            Ok(job) => Ok(Some(job)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Get all jobs (for job history)
    pub fn get_all(&self) -> Result<Vec<SyncJob>, String> {
        let conn = self.db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, profile_id, status, source, error_code, error_message, started_at, completed_at, items_processed, items_added, items_updated, items_deleted, skipped_reason, created_at FROM sync_jobs ORDER BY created_at DESC")
            .map_err(|e| e.to_string())?;
        
        let rows = stmt.query_map([], |row| {
            let status_str: String = row.get(2)?;
            let source_str: String = row.get(3)?;
            Ok(SyncJob {
                id: row.get(0)?,
                profile_id: row.get(1)?,
                status: status_str.parse().unwrap_or(JobStatus::Queued),
                source: source_str.parse().unwrap_or(JobSource::Manual),
                error_code: row.get(4)?,
                error_message: row.get(5)?,
                started_at: row.get(6)?,
                completed_at: row.get(7)?,
                items_processed: row.get(8)?,
                items_added: row.get(9)?,
                items_updated: row.get(10)?,
                items_deleted: row.get(11)?,
                skipped_reason: row.get(12)?,
                created_at: row.get(13)?,
            })
        })
        .map_err(|e| e.to_string())?;

        let mut jobs = Vec::new();
        for row in rows {
            jobs.push(row.map_err(|e| e.to_string())?);
        }
        Ok(jobs)
    }
}

pub struct EventsRepository {
    db: Arc<Database>,
}

impl EventsRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn create(&self, event: &SyncEvent) -> Result<(), String> {
        let conn = self.db.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO sync_events (id, job_id, event_type, profile_id, timestamp, message, details, file_path, action, result)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                event.id,
                event.job_id,
                serde_json::to_string(&event.event_type).unwrap_or_default(),
                event.profile_id,
                event.timestamp,
                event.message,
                event.details,
                event.file_path,
                event.action,
                event.result,
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_by_job_id(&self, job_id: &str) -> Result<Vec<SyncEvent>, String> {
        let conn = self.db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, job_id, event_type, profile_id, timestamp, message, details, file_path, action, result FROM sync_events WHERE job_id = ?1 ORDER BY timestamp")
            .map_err(|e| e.to_string())?;
        
        let rows = stmt.query_map(params![job_id], |row| {
            let event_type_str: String = row.get(2)?;
            let event_type = serde_json::from_str(&event_type_str).unwrap_or(SyncEventType::Completed);
            Ok(SyncEvent {
                id: row.get(0)?,
                job_id: row.get(1)?,
                event_type,
                profile_id: row.get(3)?,
                timestamp: row.get(4)?,
                message: row.get(5)?,
                details: row.get(6)?,
                file_path: row.get(7)?,
                action: row.get(8)?,
                result: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?;

        let mut events = Vec::new();
        for row in rows {
            events.push(row.map_err(|e| e.to_string())?);
        }
        Ok(events)
    }
}
