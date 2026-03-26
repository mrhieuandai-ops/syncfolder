use super::scanner::ScanResult;
use super::comparer::{ChangedFile, CompareResult, SyncDirection};
use serde::{Deserialize, Serialize};

/// Planned sync operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncOperation {
    pub relative_path: String,
    pub operation_type: OperationType,
    pub direction: SyncDirection,
    pub size: u64,
}

/// Type of sync operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    Create,
    Update,
    Delete,
    Skip,
}

/// Sync plan - the complete set of operations to perform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPlan {
    pub operations: Vec<SyncOperation>,
    pub total_size: u64,
    pub files_to_add: usize,
    pub files_to_update: usize,
    pub files_to_delete: usize,
    pub files_to_skip: usize,
}

impl SyncPlan {
    pub fn new(operations: Vec<SyncOperation>) -> Self {
        let total_size = operations.iter().map(|op| op.size).sum();
        let files_to_add = operations.iter().filter(|op| op.operation_type == OperationType::Create).count();
        let files_to_update = operations.iter().filter(|op| op.operation_type == OperationType::Update).count();
        let files_to_delete = operations.iter().filter(|op| op.operation_type == OperationType::Delete).count();
        let files_to_skip = operations.iter().filter(|op| op.operation_type == OperationType::Skip).count();

        Self {
            operations,
            total_size,
            files_to_add,
            files_to_update,
            files_to_delete,
            files_to_skip,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    pub fn total_operations(&self) -> usize {
        self.operations.len()
    }
}

/// Planner service for generating sync operations
pub struct Planner;

impl Planner {
    /// Generate a sync plan from scan and compare results
    pub fn plan(
        scan_result: &ScanResult,
        compare_result: &CompareResult,
        source_only_newer: bool,
    ) -> SyncPlan {
        let mut operations = Vec::new();

        // Add operations for new files in source (copy to dest)
        for path in &scan_result.source_only {
            if let Some(meta) = scan_result.source_manifest.get_file(path) {
                if !meta.is_dir {
                    operations.push(SyncOperation {
                        relative_path: path.clone(),
                        operation_type: OperationType::Create,
                        direction: SyncDirection::SourceToDest,
                        size: meta.size,
                    });
                } else {
                    // Create directory
                    operations.push(SyncOperation {
                        relative_path: path.clone(),
                        operation_type: OperationType::Create,
                        direction: SyncDirection::SourceToDest,
                        size: 0,
                    });
                }
            }
        }

        // Add operations for new files in dest (copy to source)
        for path in &scan_result.dest_only {
            if let Some(meta) = scan_result.dest_manifest.get_file(path) {
                if !meta.is_dir {
                    operations.push(SyncOperation {
                        relative_path: path.clone(),
                        operation_type: OperationType::Create,
                        direction: SyncDirection::DestToSource,
                        size: meta.size,
                    });
                } else {
                    // Create directory
                    operations.push(SyncOperation {
                        relative_path: path.clone(),
                        operation_type: OperationType::Create,
                        direction: SyncDirection::DestToSource,
                        size: 0,
                    });
                }
            }
        }

        // Add operations for changed files
        for changed in &compare_result.changed_files {
            if changed.direction != SyncDirection::None {
                operations.push(SyncOperation {
                    relative_path: changed.relative_path.clone(),
                    operation_type: OperationType::Update,
                    direction: changed.direction.clone(),
                    size: changed.source_meta.size.max(changed.dest_meta.size),
                });
            }
        }

        // Sort operations so parents are created before children
        Self::sort_operations(&mut operations);

        SyncPlan::new(operations)
    }

    /// Sort operations to ensure parents are processed before children
    /// This is necessary because creating a directory requires its parent to exist
    pub fn sort_operations(operations: &mut Vec<SyncOperation>) {
        operations.sort_by(|a, b| {
            // Count path depth (number of separators)
            let depth_a = a.relative_path.matches(std::path::MAIN_SEPARATOR.as_str()).count();
            let depth_b = b.relative_path.matches(std::path::MAIN_SEPARATOR.as_str()).count();
            
            // Parents first (shallower paths)
            depth_a.cmp(&depth_b)
        });
    }
}
