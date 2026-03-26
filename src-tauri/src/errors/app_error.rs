//! Application error types with standardized format
//! Format: {code, message, details?, retryable}

use serde::{Deserialize, Serialize};
use super::error_codes::ErrorCode;

/// Standardized error response format for all Tauri commands
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    /// Machine-readable error code
    pub code: String,
    /// User-friendly error message
    pub message: String,
    /// Optional technical details for debugging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    /// Whether this operation can be retried
    pub retryable: bool,
    /// Error classification type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
}

impl AppError {
    /// Create a new AppError from an ErrorCode
    pub fn from_error_code(code: ErrorCode, message: String) -> Self {
        Self {
            code: code.code.to_string(),
            message,
            details: None,
            retryable: code.retryable,
            error_type: Some(format!("{:?}", code.error_type).to_lowercase()),
        }
    }

    /// Create a new AppError with custom message
    pub fn new(code: ErrorCode, message: &str) -> Self {
        Self {
            code: code.code.to_string(),
            message: message.to_string(),
            details: None,
            retryable: code.retryable,
            error_type: Some(format!("{:?}", code.error_type).to_lowercase()),
        }
    }

    /// Create an access error
    pub fn access_error(path: &str, operation: &str) -> Self {
        Self::from_error_code(
            ErrorCode::EACCES,
            format!("{}: {} - {}", ErrorCode::EACCES.error_type.vietnamese_message(), operation, path),
        )
    }

    /// Create an availability error for path
    pub fn availability_error(path: &str, reason: &str) -> Self {
        Self::from_error_code(
            ErrorCode::ENOENT,
            format!("{}: {} ({})", ErrorCode::ENOENT.error_type.vietnamese_message(), path, reason),
        )
    }

    /// Create a lock error for file
    pub fn lock_error(path: &str) -> Self {
        Self::from_error_code(
            ErrorCode::EBUSY,
            format!("{}: {}", ErrorCode::EBUSY.error_type.vietnamese_message(), path),
        )
    }

    /// Create a validation error
    pub fn validation_error(message: &str) -> Self {
        Self::from_error_code(
            ErrorCode::EINVAL,
            format!("{}: {}", ErrorCode::EINVAL.error_type.vietnamese_message(), message),
        )
    }

    /// Create a path outside allowlist error
    pub fn path_outside_allowlist(path: &str, allowed_paths: &[String]) -> Self {
        Self {
            code: "PATH_OUTSIDE_ALLOWLIST".to_string(),
            message: format!(
                "Truy cập bị từ chối: đường dẫn '{}' nằm ngoài thư mục được phép. Chỉ các thư mục {} mới được phép truy cập.",
                path,
                allowed_paths.join(", ")
            ),
            details: Some(serde_json::json!({
                "path": path,
                "allowed_paths": allowed_paths,
                "security": true
            })),
            retryable: false,
            error_type: Some("validation".to_string()),
        }
    }

    /// Create error from std::io::Error
    pub fn from_io_error(path: &str, operation: &str, io_error: std::io::Error) -> Self {
        let error_code = super::error_codes::classify_os_error(Some(io_error.raw_os_error().unwrap_or(0)));
        Self {
            code: error_code.code.to_string(),
            message: format!(
                "{}: {} - {} ({})",
                error_code.error_type.vietnamese_message(),
                operation,
                path,
                io_error
            ),
            details: Some(serde_json::json!({
                "path": path,
                "operation": operation,
                "os_error": io_error.raw_os_error(),
                "io_error": io_error.to_string()
            })),
            retryable: error_code.retryable,
            error_type: Some(format!("{:?}", error_code.error_type).to_lowercase()),
        }
    }

    /// Add technical details to error
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {} (retryable: {})", self.code, self.message, self.retryable)
    }
}

impl std::error::Error for AppError {}

/// Result type alias for Tauri commands
pub type AppResult<T> = Result<T, AppError>;

/// Sync-specific error types
#[derive(Debug, Clone)]
pub enum SyncError {
    /// Source path unavailable
    SourceUnavailable { path: String, reason: String },
    /// Destination path unavailable
    DestinationUnavailable { path: String, reason: String },
    /// Source path doesn't exist
    SourceNotFound { path: String },
    /// Destination path doesn't exist
    DestinationNotFound { path: String },
    /// Path outside allowlist
    PathOutsideAllowlist { path: String, allowed_paths: Vec<String> },
    /// File locked during operation
    FileLocked { path: String },
    /// Access denied
    AccessDenied { path: String, operation: String },
    /// Invalid configuration
    InvalidConfig { message: String },
    /// Job already running
    JobAlreadyRunning { profile_id: i64 },
}

impl SyncError {
    /// Convert to AppError
    pub fn into_app_error(self) -> AppError {
        match self {
            SyncError::SourceUnavailable { path, reason } => {
                AppError::availability_error(&path, &format!("nguồn không khả dụng: {}", reason))
            }
            SyncError::DestinationUnavailable { path, reason } => {
                AppError::availability_error(&path, &format!("đích không khả dụng: {}", reason))
            }
            SyncError::SourceNotFound { path } => {
                AppError::availability_error(&path, "thư mục nguồn không tồn tại")
            }
            SyncError::DestinationNotFound { path } => {
                AppError::availability_error(&path, "thư mục đích không tồn tại")
            }
            SyncError::PathOutsideAllowlist { path, allowed_paths } => {
                AppError::path_outside_allowlist(&path, &allowed_paths)
            }
            SyncError::FileLocked { path } => {
                AppError::lock_error(&path)
            }
            SyncError::AccessDenied { path, operation } => {
                AppError::access_error(&path, &operation)
            }
            SyncError::InvalidConfig { message } => {
                AppError::validation_error(&message)
            }
            SyncError::JobAlreadyRunning { profile_id } => {
                AppError::validation_error(&format!("Job đang chạy cho profile {}", profile_id))
            }
        }
    }
}
