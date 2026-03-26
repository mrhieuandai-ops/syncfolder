use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// File metadata for comparison
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub relative_path: String,
    pub size: u64,
    pub mtime: i64, // Unix timestamp
    pub is_dir: bool,
}

impl FileMetadata {
    pub fn new(path: PathBuf, relative_path: String, size: u64, mtime: i64, is_dir: bool) -> Self {
        Self {
            path,
            relative_path,
            size,
            mtime,
            is_dir,
        }
    }
}

/// Directory manifest - all files in a directory tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryManifest {
    pub root: PathBuf,
    pub files: HashMap<String, FileMetadata>, // relative_path -> metadata
    pub total_size: u64,
    pub file_count: usize,
}

impl DirectoryManifest {
    pub fn scan(root: &Path) -> Result<Self, String> {
        let root = root.to_path_buf();
        let mut files = HashMap::new();
        let mut total_size = 0u64;

        if !root.exists() {
            return Err(format!("Path does not exist: {}", root.display()));
        }

        for entry in WalkDir::new(&root)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path().to_path_buf();
            let relative_path = path
                .strip_prefix(&root)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();

            if relative_path.is_empty() {
                continue;
            }

            let metadata = entry.metadata().map_err(|e| e.to_string())?;
            let is_dir = metadata.is_dir();

            if !is_dir {
                let size = metadata.len();
                let mtime = metadata
                    .modified()
                    .map_err(|e| e.to_string())?
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|e| e.to_string())?
                    .as_secs() as i64;

                total_size += size;
                files.insert(
                    relative_path.clone(),
                    FileMetadata::new(path.clone(), relative_path.clone(), size, mtime, is_dir),
                );
            } else {
                // Add directory entry
                files.insert(
                    relative_path.clone(),
                    FileMetadata::new(path.clone(), relative_path.clone(), 0, 0, true),
                );
            }
        }

        let file_count = files.values().filter(|f| !f.is_dir).count();

        Ok(Self {
            root,
            files,
            total_size,
            file_count,
        })
    }

    pub fn get_file(&self, relative_path: &str) -> Option<&FileMetadata> {
        self.files.get(relative_path)
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}

/// Scanner result for sync engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub source_manifest: DirectoryManifest,
    pub dest_manifest: DirectoryManifest,
    pub source_only: Vec<String>, // relative paths only in source
    pub dest_only: Vec<String>,   // relative paths only in dest
    pub common: Vec<String>,      // relative paths in both
}

impl ScanResult {
    pub fn new(
        source_manifest: DirectoryManifest,
        dest_manifest: DirectoryManifest,
    ) -> Self {
        let mut source_only = Vec::new();
        let mut dest_only = Vec::new();
        let mut common = Vec::new();

        // Find files only in source
        for path in source_manifest.files.keys() {
            if !dest_manifest.files.contains_key(path) {
                source_only.push(path.clone());
            }
        }

        // Find files only in dest
        for path in dest_manifest.files.keys() {
            if !source_manifest.files.contains_key(path) {
                dest_only.push(path.clone());
            }
        }

        // Find common files
        for path in source_manifest.files.keys() {
            if dest_manifest.files.contains_key(path) {
                common.push(path.clone());
            }
        }

        Self {
            source_manifest,
            dest_manifest,
            source_only,
            dest_only,
            common,
        }
    }
}

/// Scanner service for directory comparison
pub struct Scanner;

impl Scanner {
    pub fn scan(source: &Path, destination: &Path) -> Result<ScanResult, String> {
        let source_manifest = DirectoryManifest::scan(source)?;
        let dest_manifest = DirectoryManifest::scan(destination)?;

        Ok(ScanResult::new(source_manifest, dest_manifest))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;

    #[test]
    fn test_scanner_detects_new_file() {
        let source_dir = tempdir().unwrap();
        let dest_dir = tempdir().unwrap();

        // Create a file only in source
        File::create(source_dir.path().join("new_file.txt"))
            .unwrap()
            .write_all(b"content")
            .unwrap();

        let result = Scanner::scan(source_dir.path(), dest_dir.path()).unwrap();

        assert!(result.source_only.contains(&"new_file.txt".to_string()));
        assert!(result.dest_only.is_empty());
    }
}
