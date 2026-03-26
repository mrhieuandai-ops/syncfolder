-- Migration: 0003_sync_events.sql
-- Sync events table for detailed job logging

CREATE TABLE IF NOT EXISTS sync_events (
    id TEXT PRIMARY KEY,
    job_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    message TEXT,
    details TEXT,
    file_path TEXT,
    action TEXT,
    result TEXT,
    FOREIGN KEY (job_id) REFERENCES sync_jobs(id),
    FOREIGN KEY (profile_id) REFERENCES sync_profiles(id)
);

CREATE INDEX idx_events_job ON sync_events(job_id);
CREATE INDEX idx_events_profile ON sync_events(profile_id);
CREATE INDEX idx_events_timestamp ON sync_events(timestamp);
