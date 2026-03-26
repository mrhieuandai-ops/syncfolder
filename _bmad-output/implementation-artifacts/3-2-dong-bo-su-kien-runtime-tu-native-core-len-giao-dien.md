# Story 3.2: Đồng bộ sự kiện runtime từ native core lên giao diện

Status: done

## Story

As a người dùng đang theo dõi tiến trình,
I want giao diện nhận trạng thái job trực tiếp từ native core,
So that tôi thấy tiến trình và kết quả nhất quán với thực tế hệ thống.

## Acceptance Criteria

**Given** một sync job thay đổi trạng thái trong native layer
**When** các event `sync:queued`, `sync:started`, `sync:progress`, `sync:completed` hoặc `sync:failed` được phát
**Then** frontend nhận và ánh xạ đúng payload sang trạng thái UI
**And** không có thao tác suy diễn trạng thái mâu thuẫn từ nhiều nguồn khác nhau

**Given** event payload thiếu dữ liệu bắt buộc
**When** frontend xử lý event
**Then** lỗi được ghi nhận an toàn thay vì làm hỏng giao diện
**And** trạng thái cuối cùng vẫn có thể được refetch từ source of truth

## Tasks / Subtasks

- [ ] Task 1: Implement event system in native layer (AC: #1)
  - [ ] Subtask 1.1: Tạo event module `src-tauri/src/events/sync_events.rs`
  - [ ] Subtask 1.2: Define payload schema cho mỗi event type
  - [ ] Subtask 1.3: Emit events at correct sync lifecycle points
  - [ ] Subtask 1.4: Event payload: `{profileId, jobId, timestamp, ...}`
- [ ] Task 2: Frontend event listener và state sync (AC: #1, #2)
  - [ ] Subtask 2.1: Create `events.ts` service wrapper cho Tauri event listeners
  - [ ] Subtask 2.2: Map events to Zustand store actions
  - [ ] Subtask 2.3: Handle missing payload data gracefully
  - [ ] Subtask 2.4: Provide refetch mechanism if state desync detected

## Dev Notes

### Architecture Patterns
- **Event Lifecycle:** `sync:queued` → `sync:started` → `sync:progress` → `sync:completed`/`sync:failed`
- **Event Payload:** Always includes `profileId`, `jobId`, `timestamp`
- **Single Source of Truth:** Native layer is SOT; UI syncs from events + explicit refetch
- **FR17, FR19:** Event system for runtime state sync

### Source Tree Components to Touch
```
src/
├── services/tauri/
│   ├── events.ts
│   └── contracts.ts
src-tauri/
├── src/events/
│   └── sync_events.rs
```

### Testing Standards
- Test event emission at each lifecycle point
- Test frontend correctly maps events to state
- Test graceful handling of malformed payloads

### References
- [Source: architecture.md#Event System Patterns]
- [Source: architecture.md#Communication Patterns]
- [Source: prd.md#FR17, FR19]
- [Source: epics.md#Story 3.2]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Implemented event system in native layer (src-tauri/src/events/sync_events.rs)
- Event payload schemas: SyncProgressPayload, SyncCompletedPayload, SyncFailedPayload
- Event lifecycle: sync:queued -> sync:started -> sync:progress -> sync:completed/sync:failed
- Frontend events.ts wrapper with listenToSyncEvents function
- Graceful error handling for malformed payloads with try-catch
- Refetch mechanism via refetchJobStatus function when desync detected

### File List
- src-tauri/src/events/mod.rs (updated)
- src-tauri/src/events/sync_events.rs (updated - event emission functions)
- src/services/tauri/events.ts (created - frontend event listeners)
