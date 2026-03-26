# Story 2.3: Đồng bộ file và thư mục con mới giữa hai phía

Status: done

## Story

As a người dùng làm việc giữa hai vị trí lưu trữ,
I want file mới và thư mục con mới xuất hiện ở một bên được sao chép sang bên còn lại,
So that hai thư mục luôn có cùng cấu trúc và dữ liệu mới.

## Acceptance Criteria

**Given** một file mới hoặc thư mục con mới xuất hiện ở một phía của hồ sơ
**When** job đồng bộ chạy
**Then** hệ thống phát hiện phần tử mới và sao chép sang phía còn lại
**And** cấu trúc thư mục được tạo đúng trước khi sao chép file con

**Given** quá trình sao chép thành công
**When** job hoàn tất
**Then** cả hai phía đều chứa cùng file hoặc thư mục mới
**And** log lưu số lượng item đã được thêm mới

## Tasks / Subtasks

- [x] Task 1: Implement scanner to detect new files/folders (AC: #1)
  - [x] Subtask 1.1: Tạo scanner module `src-tauri/src/services/sync_engine/scanner.rs`
  - [x] Subtask 1.2: Walk directory tree cho cả source và destination
  - [x] Subtask 1.3: Build file manifest với path, size, mtime
  - [x] Subtask 1.4: Identify "new" items - exist in one side only
- [x] Task 2: Implement executor for new items (AC: #1, #2)
  - [x] Subtask 2.1: Create missing directories first (parent before children)
  - [x] Subtask 2.2: Copy new files to destination
  - [x] Subtask 2.3: Preserve file metadata (mtime, permissions)
  - [x] Subtask 2.4: Log count of items added

## Dev Notes

### Architecture Patterns
- **Sync Engine Components:** scanner → comparer → planner → executor
- **Scanner:** Directory tree traversal, file metadata collection
- **Executor:** File operations (copy, create directory)
- **FR10, FR11, FR12, FR14:** Two-way sync for new files and directories

### Source Tree Components to Touch
```
src-tauri/
├── src/services/sync_engine/
│   ├── mod.rs
│   ├── scanner.rs
│   ├── comparer.rs
│   ├── planner.rs
│   └── executor.rs
```

### Testing Standards
- Unit test: Scanner correctly identifies new files
- Integration test: New file in source appears in destination after sync
- Test directory structure creation order (parents before children)

### References
- [Source: architecture.md#Sync Engine Components]
- [Source: architecture.md#scanner.rs, executor.rs]
- [Source: prd.md#FR10, FR11, FR12, FR14]
- [Source: epics.md#Story 2.3]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Implemented Scanner in `src-tauri/src/services/sync_engine/scanner.rs`
- Scanner uses walkdir for directory traversal
- Creates DirectoryManifest with HashMap of FileMetadata
- Comparer identifies source_only and dest_only files
- Executor creates directories before files (parent before children)
- File metadata (mtime, permissions) preserved during copy
- Items added counted and returned in SyncResponse

### File List
- Created: src-tauri/src/services/sync_engine/scanner.rs
- Created: src-tauri/src/services/sync_engine/comparer.rs
- Created: src-tauri/src/services/sync_engine/planner.rs
- Created: src-tauri/src/services/sync_engine/executor.rs
- Created: src-tauri/src/services/sync_engine/mod.rs
- Modified: src-tauri/src/commands/sync.rs
