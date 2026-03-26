//! Path guard module for security boundary enforcement
//! 
//! Ensures all file operations stay within the configured allowlist paths.
//! NFR8: No file operations outside configured paths

use std::path::{Path, PathBuf};
use crate::errors::{AppError, AppResult, SyncError};

/// Path guard for enforcing allowlist boundaries
#[derive(Debug, Clone)]
pub struct PathGuard {
    /// Allowed source path
    source_path: PathBuf,
    /// Allowed destination path
    destination_path: PathBuf,
}

impl PathGuard {
    /// Create a new PathGuard with source and destination paths
    pub fn new(source_path: PathBuf, destination_path: PathBuf) -> Self {
        Self {
            source_path,
            destination_path,
        }
    }

    /// Get all allowed paths
    pub fn allowed_paths(&self) -> Vec<String> {
        vec![
            self.source_path.to_string_lossy().to_string(),
            self.destination_path.to_string_lossy().to_string(),
        ]
    }

    /// Check if a path is within the allowlist
    pub fn is_allowed(&self, path: &Path) -> bool {
        let path = Self::normalize_path(path);
        
        // Check if path is within source or destination
        self.is_within_directory(&path, &self.source_path) 
            || self.is_within_directory(&path, &self.destination_path)
            || path == self.source_path
            || path == self.destination_path
    }

    /// Validate path is allowed, return error if not
    pub fn validate_path(&self, path: &Path, operation: &str) -> AppResult<()> {
        let path = Self::normalize_path(path);
        
        // SECURITY: Check for path traversal attacks (../)
        if Self::contains_path_traversal(&path) {
            return Err(SyncError::PathOutsideAllowlist {
                path: path.to_string_lossy().to_string(),
                allowed_paths: self.allowed_paths(),
            }.into_app_error());
        }
        
        if !self.is_allowed(&path) {
            return Err(SyncError::PathOutsideAllowlist {
                path: path.to_string_lossy().to_string(),
                allowed_paths: self.allowed_paths(),
            }.into_app_error());
        }
        
        Ok(())
    }
    
    /// Check if path contains path traversal components
    fn contains_path_traversal(path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        path_str.contains("..\\") || path_str.contains("../") || path_str.ends_with("..")
    }

    /// Validate multiple paths are all within allowlist
    pub fn validate_paths(&self, paths: &[PathBuf], operation: &str) -> AppResult<()> {
        for path in paths {
            self.validate_path(path, operation)?;
        }
        Ok(())
    }

    /// Check if a directory path is available (exists and accessible)
    pub fn check_directory_available(&self, path: &Path) -> Result<bool, AppError> {
        let path = Self::normalize_path(path);
        
        // Check if path exists
        if !path.exists() {
            return Ok(false);
        }
        
        // Check if it's a directory
        if !path.is_dir() {
            return Err(AppError::validation_error(&format!(
                "Đường dẫn '{}' không phải là thư mục",
                path.display()
            )));
        }
        
        // Check if readable by trying to read directory contents
        match std::fs::read_dir(&path) {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                Err(AppError::access_error(&path.to_string_lossy(), "đọc thư mục"))
            }
            Err(e) => Err(AppError::from_io_error(
                &path.to_string_lossy(),
                "kiểm tra thư mục",
                e,
            )),
        }
    }

    /// Check if both source and destination are available
    pub fn check_all_available(&self) -> Result<(), AppError> {
        // Check source
        match self.check_directory_available(&self.source_path) {
            Ok(true) => {}
            Ok(false) => {
                return Err(SyncError::SourceUnavailable {
                    path: self.source_path.to_string_lossy().to_string(),
                    reason: "thư mục không tồn tại hoặc không khả dụng".to_string(),
                }.into_app_error());
            }
            Err(e) => {
                return Err(SyncError::SourceUnavailable {
                    path: self.source_path.to_string_lossy().to_string(),
                    reason: e.message.clone(),
                }.into_app_error());
            }
        }
        
        // Check destination
        match self.check_directory_available(&self.destination_path) {
            Ok(true) => {}
            Ok(false) => {
                return Err(SyncError::DestinationUnavailable {
                    path: self.destination_path.to_string_lossy().to_string(),
                    reason: "thư mục không tồn tại hoặc không khả dụng".to_string(),
                }.into_app_error());
            }
            Err(e) => {
                return Err(SyncError::DestinationUnavailable {
                    path: self.destination_path.to_string_lossy().to_string(),
                    reason: e.message.clone(),
                }.into_app_error());
            }
        }
        
        Ok(())
    }

    /// Normalize a path for consistent comparison
    fn normalize_path(path: &Path) -> PathBuf {
        path
            .canonicalize()
            .unwrap_or_else(|_| path.to_path_buf())
    }

    /// Check if a path is within a directory
    fn is_within_directory(&self, path: &Path, directory: &Path) -> bool {
        path.starts_with(directory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_path_guard_allows_own_paths() {
        let temp_dir = env::temp_dir();
        let guard = PathGuard::new(temp_dir.clone(), temp_dir.join("dest"));
        
        // Own paths should be allowed
        assert!(guard.is_allowed(&temp_dir));
        assert!(guard.is_allowed(&temp_dir.join("some_file.txt")));
    }

    #[test]
    fn test_path_guard_blocks_outside_paths() {
        let source = PathBuf::from("C:/source");
        let dest = PathBuf::from("C:/dest");
        let guard = PathGuard::new(source.clone(), dest.clone());
        
        // Other paths should be blocked
        assert!(!guard.is_allowed(&PathBuf::from("C:/other")));
        assert!(!guard.is_allowed(&PathBuf::from("D:/something")));
    }

    #[test]
    fn test_allowed_paths_returns_both() {
        let source = PathBuf::from("C:/source");
        let dest = PathBuf::from("C:/dest");
        let guard = PathGuard::new(source.clone(), dest.clone());
        
        let allowed = guard.allowed_paths();
        assert_eq!(allowed.len(), 2);
        assert!(allowed.contains(&"C:/source".to_string()));
        assert!(allowed.contains(&"C:/dest".to_string()));
    }
}
