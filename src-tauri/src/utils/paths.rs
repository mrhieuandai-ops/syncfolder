//! Path utilities for SyncFolder

use std::path::{Path, PathBuf};

/// Normalize a path for comparison
pub fn normalize_path(path: &str) -> PathBuf {
    Path::new(path).to_path_buf()
}

/// Check if a path is valid (exists and is a directory)
pub fn is_valid_directory(path: &str) -> bool {
    Path::new(path).is_dir()
}

/// Join a path with a filename
pub fn join_path(base: &str, filename: &str) -> PathBuf {
    Path::new(base).join(filename)
}

/// Get the relative path from base to target
pub fn relative_path(base: &Path, target: &Path) -> Option<PathBuf> {
    target.strip_prefix(base).ok().map(|p| p.to_path_buf())
}

/// Check if path is within base directory (for security)
pub fn is_within_base(base: &Path, target: &Path) -> bool {
    target.starts_with(base)
}
