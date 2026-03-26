# Story 1.2: Tạo hồ sơ đồng bộ mới bằng chọn thư mục nguồn và đích

Status: ready-for-dev

## Story

As a người dùng Windows 11,
I want chọn thư mục nguồn và thư mục đích bằng folder picker native,
So that tôi có thể thiết lập hồ sơ đồng bộ đầu tiên một cách nhanh và rõ ràng.

## Acceptance Criteria

**Given** người dùng đang ở màn hình tạo hồ sơ mới
**When** họ bấm chọn thư mục nguồn hoặc thư mục đích
**Then** ứng dụng mở folder picker native của Windows
**And** đường dẫn đã chọn được hiển thị rõ trên form

**Given** người dùng chưa chọn đủ cả hai thư mục
**When** họ cố lưu hồ sơ
**Then** hệ thống chặn thao tác lưu
**And** hiển thị lỗi dễ hiểu chỉ ra trường còn thiếu hoặc không hợp lệ

## Tasks / Subtasks

- [ ] Task 1: Xây dựng form tạo hồ sơ đồng bộ (AC: #1, #2)
  - [ ] Subtask 1.1: Tạo component SyncProfileForm.tsx với các trường source/destination
  - [ ] Subtask 1.2: Implement FolderPickerField component sử dụng Tauri dialog API
  - [ ] Subtask 1.3: Kết nối form với Tauri command `create_profile`
  - [ ] Subtask 1.4: Validate form - yêu cầu cả hai thư mục được chọn
  - [ ] Subtask 1.5: Hiển thị đường dẫn đã chọn rõ trên form
- [ ] Task 2: Implement backend command create_profile (AC: #2)
  - [ ] Subtask 2.1: Tạo Tauri command `create_profile` trong `src-tauri/src/commands/profiles.rs`
  - [ ] Subtask 2.2: Validate source_path và destination_path không được trống
  - [ ] Subtask 2.3: Lưu profile vào SQLite với trạng thái pending
  - [ ] Subtask 2.4: Trả về profile_id cho frontend

## Dev Notes

### Architecture Patterns
- **Frontend:** React + TypeScript với Fluent UI components
- **Native:** Rust Tauri commands với SQLite persistence
- **Dialog API:** `@tauri-apps/api/dialog` cho native folder picker
- **Validation:** 2-tier validation (frontend UX + backend source of truth)

### Source Tree Components to Touch
```
src/
├── features/sync-config/
│   ├── components/
│   │   ├── SyncProfileForm.tsx
│   │   └── FolderPickerField.tsx
│   └── services/
│       └── tauri/commands.ts
src-tauri/
├── src/commands/
│   └── profiles.rs
└── src/repositories/
    └── profiles_repo.rs
```

### Testing Standards
- Frontend: Vitest - test form validation, folder picker interaction
- Native: `cargo test` - test create_profile command validation
- Integration: E2E test folder picker flow

### References
- [Source: architecture.md#Command Design]
- [Source: architecture.md#API Naming Conventions]
- [Source: ux-design-specification.md#Pair Setup Card]
- [Source: epics.md#Story 1.2]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Frontend: SyncProfileForm component already existed with form validation
- Frontend: FolderPickerField updated to use Tauri dialog API (@tauri-apps/plugin-dialog)
- Frontend: ScheduleSelect component already existed with 4 schedule options (30, 60, 90, 1440 minutes)
- Backend: create_profile command implemented in commands/profiles.rs
- Backend: Validates source_path and destination_path are not empty
- Backend: Returns profile_id for frontend after creation
- Integration: Folder picker now opens native Windows folder picker dialog

### File List
- src-tauri/src/commands/profiles.rs (create_profile function)
- src/features/sync-config/components/FolderPickerField.tsx (updated to use Tauri dialog)
- src/features/sync-config/components/SyncProfileForm.tsx (already existed, now properly integrated)
- src/features/sync-config/components/ScheduleSelect.tsx (already existed)
