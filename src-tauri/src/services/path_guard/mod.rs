//! Path guard service for file access boundary enforcement
//!
//! Provides security boundary enforcement ensuring all file operations
//! stay within configured source/destination paths.
//!
//! # Security Model
//! - NFR7: Local config data readable only by creating Windows account
//! - NFR8: No file operations outside configured paths
//!
//! # Error Classification
//! All path violations are classified as `validation` errors

pub mod allowlist;

pub use allowlist::PathGuard;
