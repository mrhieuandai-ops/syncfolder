# Story 2.4: Đồng bộ thay đổi nội dung file hai chiều

Status: done

## Story

As a người dùng thường xuyên chỉnh sửa tài liệu,
I want các file đã thay đổi ở một bên được cập nhật sang bên còn lại,
So that dữ liệu luôn phản ánh phiên bản mới nhất theo chính sách đồng bộ đã định.

## Acceptance Criteria

**Given** một file đã tồn tại ở cả hai bên nhưng chỉ một phía thay đổi hợp lệ
**When** job đồng bộ chạy
**Then** hệ thống phát hiện file thay đổi và cập nhật sang phía còn lại
**And** metadata của job ghi nhận file được cập nhật thay vì tạo mới

**Given** việc cập nhật gặp lỗi giữa chừng
**When** job kết thúc thất bại
**Then** hệ thống không để lại trạng thái ghi đè ngoài chính sách đồng bộ đã định
**And** lỗi được phân loại để hỗ trợ chẩn đoán

## Tasks / Subtasks

- [x] Task 1: Implement comparer to detect changed files (AC: #1)
  - [x] Subtask 1.1: Compare file mtime và size giữa source và destination
  - [x] Subtask 1.2: Identify "changed" files - same path, different content
  - [x] Subtask 1.3: Determine which direction to sync (newer → older)
- [x] Task 2: Implement safe update with rollback (AC: #2 - NFR5)
  - [x] Subtask 2.1: Copy updated file to destination
  - [x] Subtask 2.2: If copy fails mid-way, preserve original destination file
  - [x] Subtask 2.3: Log update operations separately from creates
  - [x] Subtask 2.4: Mark job as failed if data integrity issue detected

## Dev Notes

### Architecture Patterns
- **Comparer:** Compare file metadata (mtime, size) to detect changes
- **Direction Logic:** Newer file wins - copy newer → older
- **NFR5:** Không được ghi đè dữ liệu ngoài chính sách đồng bộ ngay cả khi job thất bại giữa chừng
- **Rollback Safety:** Temp file pattern for safe updates

### Source Tree Components to Touch
```
src-tauri/
├── src/services/sync_engine/
│   ├── comparer.rs
│   └── executor.rs
```

### Testing Standards
- Test comparer detects changed files correctly
- Test update direction (newer wins)
- Test failed update leaves original intact
- NFR5 verification test

### References
- [Source: architecture.md#Comparer]
- [Source: architecture.md#Executor]
- [Source: prd.md#FR13, NFR5]
- [Source: epics.md#Story 2.4]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Implemented Comparer in `src-tauri/src/services/sync_engine/comparer.rs`
- Comparer detects changes via mtime and size comparison
- SyncDirection: SourceToDest if source newer, DestToSource if dest newer
- Executor uses temp file pattern (NamedTempFile) for safe updates
- On failed copy, original file preserved (temp file dropped)
- ChangedFile.operation_type = Update vs Create distinguished
- items_updated counter in SyncResponse

### File List
- Modified: src-tauri/src/services/sync_engine/comparer.rs
- Modified: src-tauri/src/services/sync_engine/executor.rs
- Modified: src-tauri/src/commands/sync.rs
