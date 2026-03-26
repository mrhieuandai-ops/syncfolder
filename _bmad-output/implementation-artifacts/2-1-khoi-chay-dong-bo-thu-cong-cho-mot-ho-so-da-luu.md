# Story 2.1: Khởi chạy đồng bộ thủ công cho một hồ sơ đã lưu

Status: done

## Story

As a người dùng muốn kiểm tra ngay,
I want chạy đồng bộ thủ công ngoài lịch định kỳ,
So that tôi có thể chủ động đồng bộ khi vừa tạo hoặc sửa file quan trọng.

## Acceptance Criteria

**Given** hồ sơ đồng bộ hợp lệ đã tồn tại
**When** người dùng bấm "Run sync now"
**Then** hệ thống tạo một sync job mới cho hồ sơ đó
**And** giao diện cập nhật trạng thái là job đang được xử lý

**Given** một sync job thủ công hoàn thành
**When** kết quả được ghi nhận
**Then** thời điểm chạy gần nhất được cập nhật trong giao diện
**And** người dùng có thể phân biệt lần chạy mới nhất với các lần trước

## Tasks / Subtasks

- [x] Task 1: Implement manual sync trigger (AC: #1, #2)
  - [x] Subtask 1.1: Tạo Tauri command `run_sync_now(profile_id)`
  - [x] Subtask 1.2: Implement job creation trong sync_jobs table
  - [x] Subtask 1.3: Emit event `sync:started` với job details
  - [x] Subtask 1.4: Frontend button "Run sync now" gọi command
- [x] Task 2: Update last_run timestamp và UI feedback (AC: #2)
  - [x] Subtask 2.1: Sau khi job completes, update profile.last_run_at
  - [x] Subtask 2.2: Emit event `sync:completed` hoặc `sync:failed`
  - [x] Subtask 2.3: Frontend nhận event và cập nhật UI
  - [x] Subtask 2.4: Dashboard hiển thị last run time rõ ràng

## Dev Notes

### Architecture Patterns
- **Event System:** `sync:queued`, `sync:started`, `sync:completed`, `sync:failed`
- **Job Model:** `sync_jobs` table với status: `scheduled | queued | running | succeeded | failed | skipped`
- **Manual vs Scheduled:** Job có source="manual" hoặc source="scheduled"

### Source Tree Components to Touch
```
src/
├── features/sync-status/
│   ├── components/JobStatusIndicator.tsx
│   └── store/syncStatusStore.ts
src-tauri/
├── src/commands/
│   └── sync.rs
├── src/services/
│   └── sync_engine/
│       └── executor.rs
└── src/models/
    └── job.rs
```

### Testing Standards
- Test `run_sync_now` command creates job with correct status
- Test event emission flow
- Test UI update upon job completion

### References
- [Source: architecture.md#Event System Patterns]
- [Source: architecture.md#Job Model]
- [Source: prd.md#FR9, FR16]
- [Source: epics.md#Story 2.1]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Implemented `run_sync_now` command in `src-tauri/src/commands/sync.rs`
- Integrated sync engine components (Scanner, Comparer, Planner, Executor)
- Added event emission for sync:started, sync:progress, sync:completed, sync:failed
- Updated profile.last_run_at after successful sync
- Frontend already has event listeners in `src/services/tauri/events.ts`

### File List
- Modified: src-tauri/src/commands/sync.rs
- Modified: src-tauri/src/main.rs
- Created: src-tauri/src/services/sync_engine/scanner.rs
- Created: src-tauri/src/services/sync_engine/comparer.rs
- Created: src-tauri/src/services/sync_engine/planner.rs
- Created: src-tauri/src/services/sync_engine/executor.rs
- Created: src-tauri/src/services/sync_engine/deletion_policy.rs
- Created: src-tauri/src/services/sync_engine/mod.rs
