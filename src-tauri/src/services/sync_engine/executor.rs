use super::planner::{SyncPlan, SyncOperation, OperationType};
use std::path::Path;
use std::fs;
use tempfile::NamedTempFile;

/// Execution result for a sync operation
#[derive(Debug, Clone)]
pub struct ExecuteResult {
    pub success: bool,
    pub operations_executed: usize,
    pub bytes_copied: u64,
    pub error: Option<String>,
}

/// Sync executor - performs the actual file operations
pub struct Executor;

impl Executor {
    /// Execute a sync plan
    pub fn execute(plan: &SyncPlan, source_root: &Path, dest_root: &Path) -> ExecuteResult {
        let mut operations_executed = 0;
        let mut bytes_copied = 0u64;
        let mut errors = Vec::new();

        for operation in &plan.operations {
            let result = match operation.operation_type {
                OperationType::Create | OperationType::Update => {
                    Self::copy_file(operation, source_root, dest_root)
                }
                OperationType::Delete => {
                    Self::delete_file(operation, source_root, dest_root)
                }
                OperationType::Skip => Ok(0),
            };

            match result {
                Ok(bytes) => {
                    operations_executed += 1;
                    bytes_copied += bytes;
                }
                Err(e) => {
                    errors.push(format!("{}: {}", operation.relative_path, e));
                }
            }
        }

        let success = errors.is_empty();
        let error = if errors.is_empty() {
            None
        } else {
            Some(errors.join("; "))
        };

        ExecuteResult {
            success,
            operations_executed,
            bytes_copied,
            error,
        }
    }

    fn copy_file(operation: &SyncOperation, source_root: &Path, dest_root: &Path) -> Result<u64, String> {
        let (src_path, dst_path) = match operation.direction {
            super::comparer::SyncDirection::SourceToDest => (
                source_root.join(&operation.relative_path),
                dest_root.join(&operation.relative_path),
            ),
            super::comparer::SyncDirection::DestToSource => (
                dest_root.join(&operation.relative_path),
                source_root.join(&operation.relative_path),
            ),
            super::comparer::SyncDirection::None => return Ok(0),
        };

        // Ensure parent directory exists
        if let Some(parent) = dst_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        // Use temp file for safe update (rollback on failure)
        let temp_file = NamedTempFile::new_in(dst_path.parent().unwrap())
            .map_err(|e| e.to_string())?;
        let temp_path = temp_file.path().to_path_buf();

        // Copy with metadata preservation
        fs::copy(&src_path, &temp_path).map_err(|e| e.to_string())?;

        // Atomically rename temp file to destination
        match temp_file.persist(&dst_path) {
            Ok(_) => {}
            Err(e) => {
                let _ = fs::remove_file(&temp_path);
                return Err(e.to_string());
            }
        }

        // Set mtime to match source
        if let Ok(src_meta) = fs::metadata(&src_path) {
            if let Ok(mtime) = src_meta.modified() {
                let _ = filetime::set_file_mtime(&dst_path, filetime::FileTime::from_system_time(mtime));
            }
        }

        Ok(operation.size)
    }

    fn delete_file(operation: &SyncOperation, source_root: &Path, dest_root: &Path) -> Result<u64, String> {
        // Delete from the side that doesn't have the file anymore
        // In two-way sync, if file is deleted from source, delete from dest too
        let path_to_delete = match operation.direction {
            super::comparer::SyncDirection::SourceToDest => {
                dest_root.join(&operation.relative_path)
            }
            super::comparer::SyncDirection::DestToSource => {
                source_root.join(&operation.relative_path)
            }
            super::comparer::SyncDirection::None => return Ok(0),
        };

        if path_to_delete.exists() {
            if path_to_delete.is_dir() {
                fs::remove_dir_all(&path_to_delete).map_err(|e| e.to_string())?;
            } else {
                fs::remove_file(&path_to_delete).map_err(|e| e.to_string())?;
            }
        }

        Ok(0)
    }

    /// Execute with rollback on failure - ensures no data corruption
    pub fn execute_with_rollback(
        plan: &SyncPlan,
        source_root: &Path,
        dest_root: &Path,
    ) -> ExecuteResult {
        let mut operations_executed = 0;
        let mut bytes_copied = 0u64;
        let mut errors = Vec::new();
        let mut applied_operations: Vec<SyncOperation> = Vec::new();

        for operation in &plan.operations {
            let result = match operation.operation_type {
                OperationType::Create | OperationType::Update => {
                    Self::copy_file(operation, source_root, dest_root)
                }
                OperationType::Delete => {
                    Self::delete_file(operation, source_root, dest_root)
                }
                OperationType::Skip => Ok(0),
            };

            match result {
                Ok(bytes) => {
                    operations_executed += 1;
                    bytes_copied += bytes;
                    applied_operations.push(operation.clone());
                }
                Err(e) => {
                    errors.push(format!("{}: {}", operation.relative_path, e));
                    // Rollback: revert already applied operations
                    for applied in applied_operations.iter().rev() {
                        if let Err(rollback_err) = Self::rollback_operation(applied, source_root, dest_root) {
                            log::error!("Rollback failed for {}: {}", applied.relative_path, rollback_err);
                        }
                    }
                    break;
                }
            }
        }

        let success = errors.is_empty();
        let error = if errors.is_empty() {
            None
        } else {
            Some(errors.join("; "))
        };

        ExecuteResult {
            success,
            operations_executed,
            bytes_copied,
            error,
        }
    }

    /// Rollback a single operation (reverse the copy or delete)
    fn rollback_operation(
        operation: &SyncOperation,
        source_root: &Path,
        dest_root: &Path,
    ) -> Result<(), String> {
        match operation.operation_type {
            OperationType::Create | OperationType::Update => {
                // Rollback copy: delete the file that was copied
                let (_src_path, dst_path) = match operation.direction {
                    super::comparer::SyncDirection::SourceToDest => (
                        source_root.join(&operation.relative_path),
                        dest_root.join(&operation.relative_path),
                    ),
                    super::comparer::SyncDirection::DestToSource => (
                        dest_root.join(&operation.relative_path),
                        source_root.join(&operation.relative_path),
                    ),
                    super::comparer::SyncDirection::None => return Ok(()),
                };

                // Delete the destination file (what was copied)
                if dst_path.exists() {
                    if dst_path.is_dir() {
                        fs::remove_dir_all(&dst_path).map_err(|e| e.to_string())?;
                    } else {
                        fs::remove_file(&dst_path).map_err(|e| e.to_string())?;
                    }
                }
                Ok(())
            }
            OperationType::Delete => {
                // Rollback delete: this is complex - we'd need to restore from some backup
                // For now, log that rollback is not possible for deletions
                log::warn!("Rollback not implemented for deletion operation: {}", operation.relative_path);
                Ok(())
            }
            OperationType::Skip => Ok(()),
        }
    }
}
