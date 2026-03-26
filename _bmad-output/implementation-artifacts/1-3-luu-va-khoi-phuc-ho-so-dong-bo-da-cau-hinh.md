# Story 1.3: Lưu và khôi phục hồ sơ đồng bộ đã cấu hình

Status: ready-for-dev

## Story

As a người dùng thường xuyên,
I want lưu hồ sơ đồng bộ và mở lại được ở lần dùng sau,
So that tôi không phải thiết lập lại mỗi khi khởi động ứng dụng.

## Acceptance Criteria

**Given** người dùng đã nhập cấu hình hợp lệ cho một hồ sơ
**When** họ lưu cấu hình
**Then** hồ sơ được ghi xuống local persistence thành công
**And** hồ sơ xuất hiện lại sau khi đóng và mở lại ứng dụng

**Given** ứng dụng khởi động với dữ liệu đã lưu
**When** màn hình chính được tải
**Then** người dùng nhìn thấy thông tin cặp thư mục và lịch chạy hiện tại
**And** thời gian mở ứng dụng vẫn đáp ứng mục tiêu hiển thị trạng thái nhanh của NFR1

## Tasks / Subtasks

- [ ] Task 1: Implement profile persistence và retrieval (AC: #1, #2)
  - [ ] Subtask 1.1: Hoàn thiện bảng `sync_profiles` schema trong migrations
  - [ ] Subtask 1.2: Implement `profiles_repo.rs` với CRUD operations
  - [ ] Subtask 1.3: Tạo Tauri command `get_all_profiles` và `get_profile_by_id`
  - [ ] Subtask 1.4: Frontend gọi API lấy profiles khi app khởi động
  - [ ] Subtask 1.5: Hiển thị profiles đã lưu trên Dashboard
- [ ] Task 2: Đảm bảo startup performance (AC: #2 - NFR1)
  - [ ] Subtask 2.1: Profile data được cache in-memory sau khi load
  - [ ] Subtask 2.2: Verify app hiển thị trạng thái trong < 3 giây
  - [ ] Subtask 2.3: Lazy load profiles không active

## Dev Notes

### Architecture Patterns
- **Repository Pattern:** `profiles_repo.rs` cho database operations
- **State Management:** Zustand store cho frontend state
- **Caching:** In-memory cache cho profile hiện hành
- **NFR1:** App phải mở và hiển thị trạng thái trong vòng 3 giây

### Source Tree Components to Touch
```
src/
├── features/sync-config/
│   ├── store/profileStore.ts
│   └── services/tauri/commands.ts
src-tauri/
├── migrations/
│   └── 0002_sync_profiles.sql
├── src/repositories/
│   └── profiles_repo.rs
└── src/commands/
    └── profiles.rs
```

### Testing Standards
- Performance test: Verify startup < 3 seconds
- Persistence test: Save profile, restart app, verify profile restored
- Unit test: profiles_repo CRUD operations

### References
- [Source: architecture.md#Data Architecture]
- [Source: architecture.md#Repository Pattern]
- [Source: prd.md#NFR1]
- [Source: epics.md#Story 1.3]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Backend: ProfilesRepository already has create, get_all, get_by_id methods
- Backend: list_profiles command implemented returning all profiles
- Backend: get_profile command implemented returning single profile by ID
- Frontend: profileStore created with Zustand for state management
- Frontend: loadProfiles action fetches all profiles on app startup
- Frontend: createProfile action creates new profile and updates store
- Frontend: Profile data structure matches TypeScript types from contracts

### File List
- src-tauri/src/repositories/mod.rs (ProfilesRepository - already existed)
- src-tauri/src/commands/profiles.rs (list_profiles, get_profile - already existed)
- src/features/sync-config/store/profileStore.ts (created for state management)
- src/services/tauri/commands.ts (already existed with listProfiles, getProfile)
- src/types/profile.ts (already existed)
