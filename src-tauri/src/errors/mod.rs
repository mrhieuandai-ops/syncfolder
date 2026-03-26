//! Error handling module for SyncFolder
//! 
//! Provides standardized error classification and formatting:
//! - Error types: access, availability, lock, validation
//! - Standardized format: {code, message, details?, retryable}
//! - User-friendly Vietnamese messages

pub mod app_error;
pub mod error_codes;

pub use app_error::{AppError, AppResult, SyncError};
pub use error_codes::{ErrorCode, ErrorType, classify_os_error};
