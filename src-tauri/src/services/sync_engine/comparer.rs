use super::scanner::{FileMetadata, ScanResult};
use serde::{Deserialize, Serialize};

/// File change detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangedFile {
    pub relative_path: String,
    pub source_meta: FileMetadata,
    pub dest_meta: FileMetadata,
    pub change_type: ChangeType,
    pub direction: SyncDirection,
}

/// Type of change detected
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChangeType {
    Modified,
    Unchanged,
}

/// Direction to sync the change
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SyncDirection {
    SourceToDest,
    DestToSource,
    None,
}

impl ChangedFile {
    pub fn new(
        relative_path: String,
        source_meta: FileMetadata,
        dest_meta: FileMetadata,
        change_type: ChangeType,
        direction: SyncDirection,
    ) -> Self {
        Self {
            relative_path,
            source_meta,
            dest_meta,
            change_type,
            direction,
        }
    }
}

/// Comparison result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompareResult {
    pub changed_files: Vec<ChangedFile>,
    pub unchanged_count: usize,
}

impl CompareResult {
    pub fn new(changed_files: Vec<ChangedFile>, unchanged_count: usize) -> Self {
        Self {
            changed_files,
            unchanged_count,
        }
    }
}

/// Comparer service for detecting file changes
pub struct Comparer;

impl Comparer {
    /// Compare two manifests and detect which files have changed
    pub fn compare(scan_result: &ScanResult) -> CompareResult {
        let mut changed_files = Vec::new();
        let mut unchanged_count = 0;

        for path in &scan_result.common {
            let source_meta = scan_result.source_manifest.get_file(path).unwrap();
            let dest_meta = scan_result.dest_manifest.get_file(path).unwrap();

            // Skip directories
            if source_meta.is_dir || dest_meta.is_dir {
                continue;
            }

            // Compare by mtime and size
            let source_newer = source_meta.mtime > dest_meta.mtime;
            let dest_newer = dest_meta.mtime > source_meta.mtime;
            let same_content = source_meta.size == dest_meta.size && !source_newer && !dest_newer;

            let (change_type, direction) = if same_content {
                (ChangeType::Unchanged, SyncDirection::None)
            } else if source_meta.size != dest_meta.size || source_newer {
                // Source has newer content
                if source_meta.mtime != dest_meta.mtime {
                    (ChangeType::Modified, SyncDirection::SourceToDest)
                } else {
                    // Same mtime but different size - still sync
                    (ChangeType::Modified, SyncDirection::SourceToDest)
                }
            } else {
                // Dest has newer content
                (ChangeType::Modified, SyncDirection::DestToSource)
            };

            if change_type == ChangeType::Modified && direction != SyncDirection::None {
                changed_files.push(ChangedFile::new(
                    path.clone(),
                    source_meta.clone(),
                    dest_meta.clone(),
                    change_type,
                    direction,
                ));
            } else {
                unchanged_count += 1;
            }
        }

        CompareResult::new(changed_files, unchanged_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_comparer_detects_modified_file() {
        // Create mock scan result with a modified file
        let source_manifest = DirectoryManifest {
            root: PathBuf::from("/source"),
            files: std::collections::HashMap::new(),
            total_size: 100,
            file_count: 1,
        };
        let dest_manifest = DirectoryManifest {
            root: PathBuf::from("/dest"),
            files: std::collections::HashMap::new(),
            total_size: 100,
            file_count: 1,
        };
        
        // Create a scan result manually for testing
        let mut source_files = std::collections::HashMap::new();
        source_files.insert("test.txt".to_string(), FileMetadata {
            path: PathBuf::from("/source/test.txt"),
            relative_path: "test.txt".to_string(),
            size: 100,
            mtime: 2000,
            is_dir: false,
        });
        
        let mut dest_files = std::collections::HashMap::new();
        dest_files.insert("test.txt".to_string(), FileMetadata {
            path: PathBuf::from("/dest/test.txt"),
            relative_path: "test.txt".to_string(),
            size: 100,
            mtime: 1000, // older
            is_dir: false,
        });
        
        let source_manifest = DirectoryManifest {
            root: PathBuf::from("/source"),
            files: source_files,
            total_size: 100,
            file_count: 1,
        };
        let dest_manifest = DirectoryManifest {
            root: PathBuf::from("/dest"),
            files: dest_files,
            total_size: 100,
            file_count: 1,
        };
        
        let scan_result = ScanResult::new(source_manifest, dest_manifest);
        let compare_result = Comparer::compare(&scan_result);
        
        // File was modified (source newer)
        assert_eq!(compare_result.changed_files.len(), 1);
        assert_eq!(compare_result.changed_files[0].relative_path, "test.txt");
        assert_eq!(compare_result.changed_files[0].direction, SyncDirection::SourceToDest);
    }

    #[test]
    fn test_comparer_detects_unchanged_file() {
        let mut source_files = std::collections::HashMap::new();
        source_files.insert("same.txt".to_string(), FileMetadata {
            path: PathBuf::from("/source/same.txt"),
            relative_path: "same.txt".to_string(),
            size: 100,
            mtime: 1000,
            is_dir: false,
        });
        
        let mut dest_files = std::collections::HashMap::new();
        dest_files.insert("same.txt".to_string(), FileMetadata {
            path: PathBuf::from("/dest/same.txt"),
            relative_path: "same.txt".to_string(),
            size: 100,
            mtime: 1000, // same timestamp
            is_dir: false,
        });
        
        let scan_result = ScanResult::new(
            DirectoryManifest {
                root: PathBuf::from("/source"),
                files: source_files,
                total_size: 100,
                file_count: 1,
            },
            DirectoryManifest {
                root: PathBuf::from("/dest"),
                files: dest_files,
                total_size: 100,
                file_count: 1,
            },
        );
        
        let compare_result = Comparer::compare(&scan_result);
        
        // File was unchanged
        assert!(compare_result.changed_files.is_empty());
        assert_eq!(compare_result.unchanged_count, 1);
    }

    #[test]
    fn test_comparer_detects_dest_newer() {
        let mut source_files = std::collections::HashMap::new();
        source_files.insert("updated.txt".to_string(), FileMetadata {
            path: PathBuf::from("/source/updated.txt"),
            relative_path: "updated.txt".to_string(),
            size: 100,
            mtime: 1000, // older
            is_dir: false,
        });
        
        let mut dest_files = std::collections::HashMap::new();
        dest_files.insert("updated.txt".to_string(), FileMetadata {
            path: PathBuf::from("/dest/updated.txt"),
            relative_path: "updated.txt".to_string(),
            size: 100,
            mtime: 2000, // newer
            is_dir: false,
        });
        
        let scan_result = ScanResult::new(
            DirectoryManifest {
                root: PathBuf::from("/source"),
                files: source_files,
                total_size: 100,
                file_count: 1,
            },
            DirectoryManifest {
                root: PathBuf::from("/dest"),
                files: dest_files,
                total_size: 100,
                file_count: 1,
            },
        );
        
        let compare_result = Comparer::compare(&scan_result);
        
        // Dest has newer content, should sync dest -> source
        assert_eq!(compare_result.changed_files.len(), 1);
        assert_eq!(compare_result.changed_files[0].direction, SyncDirection::DestToSource);
    }
}
