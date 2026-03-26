/// Error codes for SyncFolder application
/// Classification: access | availability | lock | validation | unknown
/// 
/// Note: The 4 spec-defined types (access/availability/lock/validation) cover most error cases.
/// Unknown is included as a 5th type for OS errors that don't cleanly map to the 4 types
/// (e.g., network errors, corruption, hardware failures). This prevents losing error information
/// when an unexpected OS error occurs.

use serde::{Deserialize, Serialize};

/// Error classification types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorType {
    /// Access denied or permission issues
    Access,
    /// Path or resource not available
    Availability,
    /// File or resource is locked
    Lock,
    /// Invalid configuration or input
    Validation,
    /// Unknown/internal error - for OS errors that don't map to above types
    Unknown,
}

impl ErrorType {
    /// Check if error type is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            ErrorType::Availability => true,
            ErrorType::Lock => true,
            ErrorType::Access => false,
            ErrorType::Validation => false,
            ErrorType::Unknown => false,
        }
    }

    /// Get Vietnamese user-friendly message prefix
    pub fn vietnamese_message(&self) -> &'static str {
        match self {
            ErrorType::Access => "Không có quyền truy cập",
            ErrorType::Availability => "Thư mục không khả dụng",
            ErrorType::Lock => "File đang bị khóa",
            ErrorType::Validation => "Cấu hình không hợp lệ",
            ErrorType::Unknown => "Lỗi không xác định",
        }
    }
}

/// Standardized error code format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorCode {
    /// Machine-readable error code
    pub code: &'static str,
    /// Error classification type
    pub error_type: ErrorType,
    /// Whether this error can be retried
    pub retryable: bool,
    /// Technical details for debugging
    pub technical_code: &'static str,
}

impl ErrorCode {
    /// Create a new error code
    pub const fn new(code: &'static str, error_type: ErrorType) -> Self {
        Self {
            code,
            error_type,
            retryable: error_type.is_retryable(),
            technical_code: code,
        }
    }

    // Access errors
    pub const EACCES: ErrorCode = ErrorCode {
        code: "EACCES",
        error_type: ErrorType::Access,
        retryable: false,
        technical_code: "EACCES",
    };
    
    pub const EPERM: ErrorCode = ErrorCode {
        code: "EPERM",
        error_type: ErrorType::Access,
        retryable: false,
        technical_code: "EPERM",
    };

    // Availability errors
    pub const ENOENT: ErrorCode = ErrorCode {
        code: "ENOENT",
        error_type: ErrorType::Availability,
        retryable: true,
        technical_code: "ENOENT",
    };
    
    pub const ENOTDIR: ErrorCode = ErrorCode {
        code: "ENOTDIR",
        error_type: ErrorType::Availability,
        retryable: true,
        technical_code: "ENOTDIR",
    };
    
    pub const EIO: ErrorCode = ErrorCode {
        code: "EIO",
        error_type: ErrorType::Availability,
        retryable: true,
        technical_code: "EIO",
    };

    // Lock errors
    pub const EBUSY: ErrorCode = ErrorCode {
        code: "EBUSY",
        error_type: ErrorType::Lock,
        retryable: true,
        technical_code: "EBUSY",
    };
    
    pub const ETXTBSY: ErrorCode = ErrorCode {
        code: "ETXTBSY",
        error_type: ErrorType::Lock,
        retryable: true,
        technical_code: "ETXTBSY",
    };

    // Validation errors
    pub const EINVAL: ErrorCode = ErrorCode {
        code: "EINVAL",
        error_type: ErrorType::Validation,
        retryable: false,
        technical_code: "EINVAL",
    };
    
    pub const ENAMETOOLONG: ErrorCode = ErrorCode {
        code: "ENAMETOOLONG",
        error_type: ErrorType::Validation,
        retryable: false,
        technical_code: "ENAMETOOLONG",
    };

    // Unknown errors
    pub const UNKNOWN: ErrorCode = ErrorCode {
        code: "UNKNOWN",
        error_type: ErrorType::Unknown,
        retryable: false,
        technical_code: "UNKNOWN",
    };
}

/// Map OS error codes to our classification
/// 
/// Windows error codes (Win32) - Full range covers most common file operation errors.
/// Unix/Linux codes (errno.h) are also supported via standard Rust io::ErrorKind mapping.
pub fn classify_os_error(os_error: Option<i32>) -> ErrorCode {
    match os_error {
        // File not found / path issues (Availability)
        Some(2) => ErrorCode::ENOENT,      // File not found (Windows)
        Some(3) => ErrorCode::ENOENT,       // Path not found (Windows)
        Some(15) => ErrorCode::ENOENT,      // Invalid drive
        Some(52) => ErrorCode::ENOENT,     // Invalid network name
        
        // Access denied / permission issues (Access)
        Some(5) => ErrorCode::EACCES,       // Access denied (Windows ERROR_ACCESS_DENIED)
        Some(13) => ErrorCode::EACCES,     // Permission denied (Unix)
        Some(14) => ErrorCode::EACCES,     // Bad address
        Some(87) => ErrorCode::EACCES,     // Invalid parameter (Windows - sometimes access)
        Some(130) => ErrorCode::EACCES,   // Attempt to access invalid address
        
        // Sharing violations / locks (Lock)
        Some(32) => ErrorCode::EBUSY,      // File locked (ERROR_SHARING_VIOLATION)
        Some(33) => ErrorCode::EBUSY,      // Lock violation
        Some(36) => ErrorCode::EBUSY,      // File/segment busy
        
        // Invalid operations (Validation)
        Some(123) => ErrorCode::EINVAL,    // Invalid name syntax (ERROR_INVALID_NAME)
        Some(180) => ErrorCode::EINVAL,    // Invalid segment number
        
        // I/O / media errors (Availability)
        Some(6) => ErrorCode::EIO,        // Invalid handle
        Some(17) => ErrorCode::EIO,        // Not same device
        Some(20) => ErrorCode::ENOTDIR,    // Not a directory (Unix)
        Some(21) => ErrorCode::ENOTDIR,    // Is a directory
        Some(23) => ErrorCode::EIO,        // Data CRC error
        Some(34) => ErrorCode::EIO,        // Disk full
        Some(111) => ErrorCode::EIO,       // Buffer overflow
        Some(145) => ErrorCode::EIO,       // Directory not empty (Windows) / sector error
        Some(148) => ErrorCode::EIO,       // Sector not found
        
        // File already exists (Access - permission issue since shouldn't overwrite)
        Some(80) => ErrorCode::EACCES,     // File already exists (ERROR_FILE_EXISTS)
        Some(183) => ErrorCode::EACCES,   // File already exists (ERROR_ALREADY_EXISTS)
        
        _ => ErrorCode::UNKNOWN,
    }
}
