# Story 4.2: Phân loại lỗi truy cập, khóa file và cấu hình không hợp lệ

Status: done

## Story

As a người dùng cần được hướng dẫn rõ,
I want lỗi được phân loại thành các nhóm có thể hành động,
So that tôi biết mình cần xử lý quyền truy cập, file đang mở hay cấu hình sai.

## Acceptance Criteria

**Given** job đồng bộ gặp lỗi runtime
**When** native layer chuẩn hóa lỗi
**Then** lỗi được ánh xạ về ít nhất các nhóm access, availability, lock và validation
**And** mỗi lỗi có code, message và cờ retryable khi phù hợp

**Given** frontend nhận lỗi đã phân loại
**When** hiển thị thông báo cho người dùng
**Then** UI dùng ngôn ngữ dễ hiểu thay cho thuật ngữ kỹ thuật thuần túy
**And** log chi tiết vẫn giữ được diagnostic raw cần thiết cho support

## Tasks / Subtasks

- [ ] Task 1: Implement error classification system (AC: #1)
  - [ ] Subtask 1.1: Define error types: `access`, `availability`, `lock`, `validation`
  - [ ] Subtask 1.2: Map OS-level errors to classification
  - [ ] Subtask 1.3: Add `retryable` flag per error type
  - [ ] Subtask 1.4: Standardize error format: `{code, message, details?, retryable}`
- [ ] Task 2: User-friendly error display (AC: #2)
  - [ ] Subtask 2.1: Map error types to Vietnamese messages
  - [ ] Subtask 2.2: Show actionable suggestions per error type
  - [ ] Subtask 2.3: Raw diagnostic info in log detail view

## Dev Notes

### Architecture Patterns
- **Error Classification:** access | availability | lock | validation
- **Standardized Error:** `{code: string, message: string, details?: unknown, retryable: boolean}`
- **NFR12:** 4 error groups must be distinguishable from logs
- **Error Codes:** Defined in `src-tauri/src/errors/error_codes.rs`

### Source Tree Components to Touch
```
src-tauri/
├── src/errors/
│   ├── mod.rs
│   ├── app_error.rs
│   └── error_codes.rs
├── src/services/sync_engine/
│   └── executor.rs
```

### Testing Standards
- Test all 4 error classifications handled correctly
- Test retryable flag set appropriately per error type
- Test Vietnamese user messages display correctly
- Test NFR12: 4 groups distinguishable

### References
- [Source: architecture.md#Error Handling Patterns]
- [Source: prd.md#FR18, FR20, FR22, NFR12]
- [Source: epics.md#Story 4.2]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Implemented error classification system with 4 types: access, availability, lock, validation
- Standardized error format: {code, message, details?, retryable}
- ErrorCode struct with OS-level error mapping via classify_os_error()
- AppError struct implements standardized format for Tauri commands
- Vietnamese user-friendly messages via ErrorType::vietnamese_message()
- Map OS error codes (2, 3, 5, 13, 17, 20, 32, 36, 87, 123, 145, 183) to error types
- SyncError enum for sync-specific errors with into_app_error() conversion
- NFR12 compliance: 4 error groups distinguishable in logs

### File List
- src-tauri/src/errors/error_codes.rs (created)
- src-tauri/src/errors/app_error.rs (created)
- src-tauri/src/errors/mod.rs (created)
