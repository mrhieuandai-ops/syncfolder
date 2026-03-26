-- Migration: 0002_sync_jobs.sql
-- Sync jobs table for tracking sync execution

CREATE TABLE IF NOT EXISTS sync_jobs (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('scheduled', 'queued', 'running', 'succeeded', 'failed', 'skipped')),
    source TEXT NOT NULL CHECK(source IN ('manual', 'scheduled')),
    error_code TEXT,
    error_message TEXT,
    started_at TEXT,
    completed_at TEXT,
    items_processed INTEGER DEFAULT 0,
    items_added INTEGER DEFAULT 0,
    items_updated INTEGER DEFAULT 0,
    items_deleted INTEGER DEFAULT 0,
    skipped_reason TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (profile_id) REFERENCES sync_profiles(id)
);

CREATE INDEX idx_jobs_profile ON sync_jobs(profile_id);
CREATE INDEX idx_jobs_status ON sync_jobs(status);
CREATE INDEX idx_jobs_created ON sync_jobs(created_at);
