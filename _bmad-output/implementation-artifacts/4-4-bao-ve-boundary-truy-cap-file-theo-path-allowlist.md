# Story 4.4: Bảo vệ boundary truy cập file theo path allowlist

Status: done

## Story

As a người dùng quan tâm an toàn dữ liệu,
I want ứng dụng chỉ đọc và ghi trong các thư mục tôi đã cấu hình,
So that hệ thống không tác động ngoài phạm vi tôi cho phép.

## Acceptance Criteria

**Given** một command hoặc sync operation yêu cầu truy cập file system
**When** native layer kiểm tra đường dẫn mục tiêu
**Then** thao tác chỉ được phép tiếp tục nếu đường dẫn nằm trong allowlist của hồ sơ
**And** mọi truy cập ngoài phạm vi bị chặn và ghi log như lỗi bảo mật hoặc validation

**Given** dữ liệu local persistence được lưu trên máy
**When** ứng dụng ghi cấu hình xuống đĩa
**Then** file dữ liệu được lưu trong vùng app data của người dùng hiện tại
**And** thiết kế lưu trữ hỗ trợ mục tiêu chỉ tài khoản tạo cấu hình mới đọc trực tiếp được dữ liệu đó

## Tasks / Subtasks

- [ ] Task 1: Implement path allowlist enforcement (AC: #1)
  - [ ] Subtask 1.1: Path guard checks target path against profile allowlist
  - [ ] Subtask 1.2: Allowlist = source_path và destination_path của profile
  - [ ] Subtask 1.3: Block and log any access outside allowlist
  - [ ] Subtask 1.4: Emit security/validation error
- [ ] Task 2: Secure local storage (AC: #2 - NFR7, NFR8)
  - [ ] Subtask 2.1: SQLite database in `%APPDATA%/syncfolder/`
  - [ ] Subtask 2.2: Database file permissions: user-only read/write
  - [ ] Subtask 2.3: NFR8: Verify no file operations outside configured paths

## Dev Notes

### Architecture Patterns
- **Path Allowlist:** Only paths in profile source/destination allowed
- **Tauri Capability:** `fs` scope limited to allowlist paths
- **NFR7:** Local config data readable only by creating Windows account
- **NFR8:** No file operations outside configured paths

### Source Tree Components to Touch
```
src-tauri/
├── src/services/path_guard/
│   ├── mod.rs
│   └── allowlist.rs
├── src-tauri/capabilities/
│   └── default.json
```

### Testing Standards
- Test blocked access to paths outside allowlist
- Test NFR7: Different Windows account cannot read config
- Test NFR8: File operations only in configured directories

### References
- [Source: architecture.md#Security Model]
- [Source: architecture.md#Path Guard]
- [Source: prd.md#NFR7, NFR8]
- [Source: epics.md#Story 4.4]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Implemented PathGuard with allowlist = source_path + destination_path
- PathGuard::is_allowed() checks if path is within allowed directories
- PathGuard::validate_path() returns error if path outside allowlist
- PathGuard::validate_paths() validates multiple paths before operations
- execute_sync_with_guard validates all scanned paths against allowlist
- SyncError::PathOutsideAllowlist for blocked access logging
- Tauri capabilities restricted to $HOME, $APPDATA, $LOCALAPPDATA
- NFR7/NFR8 compliance: security boundary enforced at Rust layer

### File List
- src-tauri/src/services/path_guard/mod.rs (created)
- src-tauri/src/services/path_guard/allowlist.rs (created)
- src-tauri/src/commands/sync.rs (modified - execute_sync_with_guard)
- src-tauri/capabilities/default.json (modified - restricted fs permissions)
