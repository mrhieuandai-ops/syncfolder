//! SyncFolder Library
//! 
//! Desktop file synchronization app for Windows 11
//! 
//! Architecture:
//! - Tauri 2.x for desktop shell
//! - React + TypeScript + Vite for frontend
//! - Rust native core with SQLite persistence

pub mod models;
pub mod errors;
pub mod repositories;
pub mod services;
pub mod commands;

// Re-export commonly used types
pub use models::{SyncProfile, SyncJob, JobStatus, JobSource, SyncEvent, SyncEventType};
pub use errors::{AppError, AppResult, ErrorCode, ErrorType, SyncError};
pub use repositories::{ProfilesRepository, JobsRepository, EventsRepository};

use rusqlite::Connection;
use std::sync::Arc;

/// Application state shared across commands
pub struct AppState {
    pub db: Arc<Connection>,
}

impl AppState {
    pub fn new(db: Connection) -> Self {
        Self { db: Arc::new(db) }
    }
}
