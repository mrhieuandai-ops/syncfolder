# Story 3.4: Hiển thị lịch sử và chi tiết log dễ đọc để chẩn đoán

Status: done

## Story

As a người dùng gặp sự cố,
I want xem lịch sử job và log chi tiết theo ngôn ngữ dễ hiểu,
So that tôi có thể tự xác định nguyên nhân lỗi phổ biến mà không cần đoán.

## Acceptance Criteria

**Given** người dùng mở màn hình Activity Log hoặc chi tiết hồ sơ
**When** họ chọn một job đã chạy
**Then** ứng dụng hiển thị log chi tiết theo từng job với lỗi và ngữ cảnh dễ hiểu
**And** người dùng có thể phân biệt ít nhất các nhóm lỗi: thư mục không khả dụng, lỗi quyền truy cập, file bị khóa, cấu hình không hợp lệ

**Given** trạng thái job là failed
**When** giao diện hiển thị thông báo lỗi
**Then** thông báo lỗi có gợi ý hành động kế tiếp phù hợp
**And** thông tin quan trọng không được biểu diễn chỉ bằng màu sắc

## Tasks / Subtasks

- [ ] Task 1: Build Activity Log screen (AC: #1)
  - [ ] Subtask 1.1: Tạo ActivityLog component với job history list
  - [ ] Subtask 1.2: Job list hiển thị: thời gian, trạng thái (text), profile name
  - [ ] Subtask 1.3: Chi tiết job hiển thị log entries với human-readable messages
  - [ ] Subtask 1.4: Error classification: availability, access, lock, validation
- [ ] Task 2: User-friendly error messages (AC: #2)
  - [ ] Subtask 2.1: Map error codes to Vietnamese user messages
  - [ ] Subtask 2.2: Include suggested next action in error display
  - [ ] Subtask 2.3: Status info NOT color-only (text + icon)

## Dev Notes

### Architecture Patterns
- **Error Classification:** availability | access | lock | validation
- **Error Codes:** Standardized in `src-tauri/src/errors/error_codes.rs`
- **User Messages:** Vietnamese localized, actionable suggestions
- **FR18, FR20, NFR12:** Error messages with diagnostic info and action suggestions

### Source Tree Components to Touch
```
src/
├── features/sync-history/
│   ├── components/
│   │   ├── ActivityLog.tsx
│   │   └── LogDetailView.tsx
│   └── services/tauri/commands.ts
src-tauri/
├── src/errors/
│   ├── error_codes.rs
│   └── app_error.rs
```

### Testing Standards
- Test error classification display
- Test human-readable error messages
- Test actionable error suggestions
- Test NFR12: 4 error groups distinguishable

### References
- [Source: architecture.md#Error Handling Patterns]
- [Source: prd.md#FR18, FR20, NFR12]
- [Source: ux-design-specification.md#Feedback Patterns]
- [Source: epics.md#Story 3.4]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Implemented ActivityLog component with job history list
- Implemented LogDetailView component for detailed log entries
- Created errorMappings.ts for Vietnamese error message mapping
- Error classification (NFR12): availability, access, lock, validation groups
- User-friendly error messages with suggested actions
- Status display with icons (not color-only) per NFR10
- ErrorSummary component with actionable suggestions

### File List
- src/types/error.ts (created)
- src/features/sync-history/components/ActivityLog.tsx (created)
- src/features/sync-history/components/LogDetailView.tsx (created)
- src/features/sync-history/components/errorMappings.ts (created)
- src-tauri/src/errors/error_codes.rs (existing - error classification)
- src-tauri/src/errors/app_error.rs (existing - AppError struct)
- src-tauri/src/commands/logs.rs (updated - user message mapping)
