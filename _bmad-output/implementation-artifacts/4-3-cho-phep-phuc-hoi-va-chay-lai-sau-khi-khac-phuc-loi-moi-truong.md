# Story 4.3: Cho phép phục hồi và chạy lại sau khi khắc phục lỗi môi trường

Status: done

## Story

As a người dùng sau khi đã sửa sự cố,
I want chạy lại job bị lỗi mà không cần cấu hình lại hồ sơ,
So that tôi có thể phục hồi nhanh và tiếp tục tin tưởng ứng dụng.

## Acceptance Criteria

**Given** một hồ sơ đã từng thất bại do lỗi môi trường
**When** người dùng khắc phục nguyên nhân và bấm chạy lại
**Then** job mới có thể được tạo và thực thi bình thường
**And** hồ sơ cấu hình trước đó vẫn được giữ nguyên

**Given** job thất bại bất ngờ hoặc ứng dụng đóng đột ngột
**When** người dùng mở lại ứng dụng
**Then** cấu hình đồng bộ đã lưu vẫn còn nguyên
**And** trạng thái lịch sử vẫn cho phép người dùng hiểu lần thất bại gần nhất

## Tasks / Subtasks

- [ ] Task 1: Implement retry mechanism (AC: #1)
  - [ ] Subtask 1.1: `run_sync_now` command works on previously failed profiles
  - [ ] Subtask 1.2: New job created, old job status remains in history
  - [ ] Subtask 1.3: Profile config never modified by failed jobs
- [ ] Task 2: Profile persistence after failures (AC: #2 - NFR6)
  - [ ] Subtask 2.1: Profile data in SQLite, survives app restart
  - [ ] Subtask 2.2: Job history preserved for diagnosis
  - [ ] Subtask 2.3: App state recovery on startup

## Dev Notes

### Architecture Patterns
- **Retry Safe:** Failed jobs don't corrupt profile configuration
- **NFR6:** Cấu hình được giữ nguyên sau khi app đóng đột ngột hoặc job thất bại
- **History Preservation:** Old jobs remain in log for diagnosis
- **FR23, FR24:** Recovery without reconfiguration

### Source Tree Components to Touch
```
src-tauri/
├── src/commands/
│   └── sync.rs
├── src/repositories/
│   └── profiles_repo.rs
```

### Testing Standards
- Test retry creates new job without modifying profile
- Test profile survives app crash/restart
- Test job history preserved

### References
- [Source: architecture.md#Persistence]
- [Source: prd.md#FR23, FR24, NFR6]
- [Source: epics.md#Story 4.3]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Implemented retry mechanism: run_sync_now creates new job for profile
- Old job remains in history for diagnosis (FR23 compliance)
- Profile config never modified by failed jobs (NFR6 compliance)
- update_last_run only updates last_run_at timestamp, not config
- JobsRepository::get_last_for_profile retrieves job history
- Anti-concurrent execution via running_jobs Mutex guard
- Events emitted for all state transitions

### File List
- src-tauri/src/commands/sync.rs (created)
- src-tauri/src/repositories/mod.rs (modified - uses existing ProfilesRepository)
