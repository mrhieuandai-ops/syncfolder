# Story 3.3: Ghi log có cấu trúc cho mọi sync job

Status: done

## Story

As a người dùng hoặc người hỗ trợ,
I want mỗi lần đồng bộ đều có log thời gian chạy, kết quả và nguyên nhân lỗi,
So that tôi có đủ dữ liệu để kiểm tra điều gì đã xảy ra.

## Acceptance Criteria

**Given** một sync job bắt đầu và kết thúc
**When** hệ thống ghi log
**Then** log bao gồm tối thiểu profileId hoặc jobId, timestamp, trạng thái kết thúc và summary kết quả
**And** khi có lỗi thì log chứa code và message phù hợp

**Given** nhiều job đã chạy qua thời gian
**When** người dùng truy xuất lịch sử
**Then** hệ thống có thể truy xuất log theo từng job
**And** dữ liệu log không phụ thuộc vào việc giao diện đang mở hay đóng

## Tasks / Subtasks

- [ ] Task 1: Implement structured logging for sync jobs (AC: #1)
  - [ ] Subtask 1.1: Tạo `job_logger.rs` module với structured log format
  - [ ] Subtask 1.2: Log entries: job_id, profile_id, timestamp, status, summary
  - [ ] Subtask 1.3: Error logs include code và message
  - [ ] Subtask 1.4: Store logs in SQLite `sync_events` table
- [ ] Task 2: Implement log retrieval API (AC: #2)
  - [ ] Subtask 2.1: Tạo Tauri command `get_job_logs(job_id)`
  - [ ] Subtask 2.2: Tạo Tauri command `list_jobs(profile_id, limit)`
  - [ ] Subtask 2.3: Frontend hiển thị log history
  - [ ] Subtask 2.4: Logs persisted independent of UI state

## Dev Notes

### Architecture Patterns
- **Structured Logs:** JSON format với consistent fields
- **Log Storage:** SQLite `sync_events` table (not filesystem logs only)
- **Log Schema:** `{job_id, profile_id, timestamp, status, error_code?, error_message?, items_processed?}`
- **NFR11:** Every job creates log with runtime, result, error if any

### Source Tree Components to Touch
```
src-tauri/
├── src/services/logging/
│   ├── mod.rs
│   └── job_logger.rs
├── src/repositories/
│   └── events_repo.rs
└── src/commands/
    └── logs.rs
```

### Testing Standards
- Test log entries created for each job
- Test log retrieval by job_id
- Test error logs include code and message

### References
- [Source: architecture.md#Logging]
- [Source: prd.md#FR19, NFR11]
- [Source: epics.md#Story 3.3]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Implemented structured logging via job_logger.rs with JobLogEntry and JobEventType
- Log entries include: job_id, profile_id, timestamp, event_type, message, details, error_code
- Log storage in SQLite sync_events table via EventsRepository
- Implemented Tauri commands: get_job_logs, list_jobs, get_last_job_status
- Added get_all method to JobsRepository for job history retrieval
- Frontend TypeScript types for log entries defined in src/types/log.ts

### File List
- src/types/log.ts (created)
- src-tauri/src/services/logging/mod.rs (existing)
- src-tauri/src/services/logging/job_logger.rs (existing)
- src-tauri/src/repositories/mod.rs (updated - added get_all method)
- src-tauri/src/commands/logs.rs (updated - full implementation)
- src-tauri/src/commands/status.rs (updated - imports)
