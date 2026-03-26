-- Migration: 0001_init.sql
-- Initialize database schema for syncfolder

-- Schema version tracking
CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL,
    description TEXT
);

-- App settings table
CREATE TABLE IF NOT EXISTS app_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Sync profiles table
CREATE TABLE IF NOT EXISTS sync_profiles (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    source_path TEXT NOT NULL,
    destination_path TEXT NOT NULL,
    schedule_interval INTEGER, -- minutes: 30, 60, 90, 1440
    auto_sync_enabled INTEGER DEFAULT 0,
    last_run_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_profiles_source ON sync_profiles(source_path);
CREATE INDEX idx_profiles_dest ON sync_profiles(destination_path);

-- Record migration version
INSERT INTO schema_version (version, applied_at, description) VALUES (1, datetime('now'), 'Initial schema');
