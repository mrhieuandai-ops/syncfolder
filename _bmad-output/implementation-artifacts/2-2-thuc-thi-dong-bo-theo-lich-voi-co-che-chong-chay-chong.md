# Story 2.2: Thực thi đồng bộ theo lịch với cơ chế chống chạy chồng

Status: done

## Story

As a người dùng muốn "set it and forget it",
I want hệ thống tự chạy đồng bộ theo lịch đã cấu hình,
So that hai thư mục luôn được giữ nhất quán mà không cần thao tác tay liên tục.

## Acceptance Criteria

**Given** hồ sơ được bật auto sync với một chu kỳ hợp lệ
**When** đến thời điểm chạy lịch
**Then** scheduler khởi tạo job tự động cho hồ sơ
**And** job được đánh dấu là thuộc lần chạy định kỳ

**Given** một job của cùng hồ sơ đang chạy
**When** lịch kế tiếp đến hạn
**Then** hệ thống không tạo job chồng lấn gây rủi ro trạng thái
**And** sự kiện skip hoặc defer được ghi nhận vào log/job state

## Tasks / Subtasks

- [x] Task 1: Implement scheduler service (AC: #1)
  - [x] Subtask 1.1: Tạo scheduler module trong `src-tauri/src/services/scheduler/`
  - [x] Subtask 1.2: Implement job queue với schedule intervals: 30, 60, 90, 1440 minutes
  - [x] Subtask 1.3: Scheduler check profiles có auto_sync_enabled = true
  - [x] Subtask 1.4: Trigger job creation khi đến schedule time
- [x] Task 2: Implement anti-overlap mechanism (AC: #2)
  - [x] Subtask 2.1: Job execution guard - check if profile has running job
  - [x] Subtask 2.2: Nếu job đang chạy, skip/defer và ghi log
  - [x] Subtask 2.3: Job status `skipped` với reason "overlap_prevented"

## Dev Notes

### Architecture Patterns
- **Scheduler:** Rust-based scheduler sử dụng tokio runtime
- **Anti-overlap:** Per-profile execution guard (single-flight pattern)
- **Schedule Intervals:** 30, 60, 90 phút và 1440 phút (1 ngày)
- **NFR4:** Hệ thống phải đạt tỷ lệ job thành công >= 95%

### Source Tree Components to Touch
```
src-tauri/
├── src/services/
│   ├── scheduler/
│   │   ├── mod.rs
│   │   └── job_scheduler.rs
│   └── sync_engine/
│       └── mod.rs
└── src/commands/
    └── sync.rs
```

### Testing Standards
- Test scheduler fires at correct intervals
- Test overlap prevention - concurrent jobs skipped
- Integration test: full scheduled job flow

### References
- [Source: architecture.md#Scheduler Service]
- [Source: architecture.md#Decision Priority Analysis]
- [Source: prd.md#FR7, NFR4]
- [Source: epics.md#Story 2.2]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Implemented JobScheduler in `src-tauri/src/services/scheduler/job_scheduler.rs`
- Scheduler uses tokio runtime for async scheduling
- Anti-overlap implemented via `running_jobs` Mutex in SyncState
- Job source marked as "scheduled" vs "manual"
- Skipped jobs get reason "overlap_prevented"
- Schedule intervals: 30, 60, 90, 1440 minutes

### File List
- Created: src-tauri/src/services/scheduler/job_scheduler.rs
- Created: src-tauri/src/services/scheduler/mod.rs
- Modified: src-tauri/src/main.rs
- Modified: src-tauri/src/commands/sync.rs
