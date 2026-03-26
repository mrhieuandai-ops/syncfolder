# Story 1.4: Chỉnh sửa hồ sơ và cấu hình lịch đồng bộ

Status: ready-for-dev

## Story

As a người dùng quản trị cấu hình,
I want chỉnh sửa thư mục hoặc lịch chạy của hồ sơ hiện có,
So that tôi có thể cập nhật cách đồng bộ khi nhu cầu thay đổi.

## Acceptance Criteria

**Given** người dùng đang xem một hồ sơ đã lưu
**When** họ thay đổi thư mục hoặc chọn một trong bốn chu kỳ 30/60/90 phút hoặc 1 ngày
**Then** ứng dụng cho phép lưu thay đổi hợp lệ
**And** lịch mới được phản ánh ở giao diện hồ sơ

**Given** hồ sơ đã có lịch chạy
**When** người dùng bật hoặc tắt auto sync
**Then** trạng thái kích hoạt của lịch được lưu nhất quán
**And** ứng dụng hiển thị rõ hồ sơ đang active hay paused

## Tasks / Subtasks

- [ ] Task 1: Xây dựng form chỉnh sửa profile (AC: #1, #2)
  - [ ] Subtask 1.1: Tạo component ProfileEditForm pre-populated với existing data
  - [ ] Subtask 1.2: Implement schedule selector với 4 options: 30 phút, 60 phút, 90 phút, 1 ngày
  - [ ] Subtask 1.3: Toggle switch cho enable/disable auto sync
  - [ ] Subtask 1.4: Kết nối form với Tauri command `update_profile`
- [ ] Task 2: Implement backend update_profile command (AC: #1, #2)
  - [ ] Subtask 2.1: Tạo Tauri command `update_profile` trong profiles.rs
  - [ ] Subtask 2.2: Validate schedule enum values (30, 60, 90, 1440 minutes)
  - [ ] Subtask 2.3: Update profile trong SQLite
  - [ ] Subtask 2.4: Return updated profile cho frontend

## Dev Notes

### Architecture Patterns
- **Tauri Command:** `update_profile` với payload `{profile_id, source_path?, destination_path?, schedule?, auto_sync_enabled?}`
- **Schedule Enum:** `ThirtyMinutes`, `SixtyMinutes`, `NinetyMinutes`, `OneDay`
- **Repository:** `update_profile()` method trong profiles_repo

### Source Tree Components to Touch
```
src/
├── features/sync-config/
│   ├── components/
│   │   ├── ProfileEditForm.tsx
│   │   └── ScheduleSelect.tsx
│   └── services/tauri/commands.ts
src-tauri/
├── src/commands/
│   └── profiles.rs
└── src/repositories/
    └── profiles_repo.rs
```

### Testing Standards
- Frontend: Test schedule options rendering, toggle state
- Backend: Test update with valid/invalid schedule values
- Integration: Edit profile, verify changes persisted

### References
- [Source: architecture.md#Command Design]
- [Source: prd.md#FR6, FR8]
- [Source: ux-design-specification.md#Sync Profile Card]
- [Source: epics.md#Story 1.4]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Backend: update_profile command implemented with full validation
- Backend: Validates schedule enum values (30, 60, 90, 1440 minutes)
- Backend: Validates all fields are provided correctly
- Frontend: SyncProfileForm already supports initialData for edit mode
- Frontend: ScheduleSelect component already existed with 4 schedule options
- Frontend: profileStore has updateProfile action
- Frontend: ProfileEditForm functionality available via SyncProfileForm with initialData

### File List
- src-tauri/src/commands/profiles.rs (update_profile function)
- src/features/sync-config/components/SyncProfileForm.tsx (supports edit via initialData)
- src/features/sync-config/components/ScheduleSelect.tsx (already existed)
- src/features/sync-config/store/profileStore.ts (updateProfile action)
