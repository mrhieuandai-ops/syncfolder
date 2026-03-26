# Story 4.1: Phát hiện sớm thư mục không khả dụng trước khi thực thi job

Status: done

## Story

As a người dùng sử dụng ổ ngoài hoặc thư mục có thể gián đoạn,
I want hệ thống kiểm tra tính sẵn sàng của thư mục trước khi đồng bộ,
So that ứng dụng không thực hiện các thao tác rủi ro khi nguồn hoặc đích không tồn tại.

## Acceptance Criteria

**Given** một job sắp chạy cho hồ sơ đã lưu
**When** hệ thống kiểm tra source và destination path trước khi sync
**Then** job bị dừng hoặc skipped nếu một trong hai thư mục không khả dụng
**And** trạng thái lỗi nêu rõ thư mục nào gặp vấn đề

**Given** ổ đĩa ngoài bị tháo hoặc đổi trạng thái giữa các lần chạy
**When** đến chu kỳ đồng bộ tiếp theo
**Then** ứng dụng không được cố đọc hoặc ghi dữ liệu mơ hồ
**And** log ghi lại loại lỗi availability tương ứng

## Tasks / Subtasks

- [ ] Task 1: Implement path availability check (AC: #1, #2)
  - [ ] Subtask 1.1: Tạo path validation trong scanner phase
  - [ ] Subtask 1.2: Check directory exists, is accessible, readable
  - [ ] Subtask 1.3: If unavailable, mark job as `skipped` với reason
  - [ ] Subtask 1.4: Emit `sync:failed` event với availability error
- [ ] Task 2: Safe failure handling (AC: #2)
  - [ ] Subtask 2.1: Never attempt file operations on unavailable paths
  - [ ] Subtask 2.2: Log clear error: "Source directory unavailable"
  - [ ] Subtask 2.3: Profile và job state preserved after failure

## Dev Notes

### Architecture Patterns
- **Path Guard:** Check availability before any file operations
- **Error Classification:** `availability` error group for path issues
- **FR21, FR22:** Early detection, safe skip instead of risky operations
- **NFR5:** No data corruption on failure

### Source Tree Components to Touch
```
src-tauri/
├── src/services/path_guard/
│   └── allowlist.rs
├── src/services/sync_engine/
│   └── scanner.rs
```

### Testing Standards
- Test job skipped when source unavailable
- Test job skipped when destination unavailable
- Test clear error message indicates which path
- Test profile state preserved after skip

### References
- [Source: architecture.md#Path Guard]
- [Source: prd.md#FR21, FR22]
- [Source: epics.md#Story 4.1]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Implemented path availability check in PathGuard::check_all_available()
- PathGuard validates source/destination before any file operations
- Job marked as "skipped" with clear availability error when path unavailable
- SyncError::SourceUnavailable and SyncError::DestinationUnavailable for clear error classification
- Events emitted for Queued, Started, Skipped, Failed, Completed states
- Profile config preserved after skip (NFR6 compliance)

### File List
- src-tauri/src/services/path_guard/allowlist.rs (created)
- src-tauri/src/commands/sync.rs (created)
- src-tauri/src/errors/app_error.rs (modified - added SyncError variants)
- src-tauri/src/errors/error_codes.rs (created)
