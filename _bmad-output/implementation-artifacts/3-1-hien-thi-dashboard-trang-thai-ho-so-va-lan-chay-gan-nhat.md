# Story 3.1: Hiển thị dashboard trạng thái hồ sơ và lần chạy gần nhất

Status: done

## Story

As a người dùng muốn kiểm tra nhanh,
I want nhìn thấy trạng thái hồ sơ, lần chạy gần nhất và lịch hiện tại ngay trên dashboard,
So that tôi biết ứng dụng có đang hoạt động bình thường hay không.

## Acceptance Criteria

**Given** ứng dụng mở với ít nhất một hồ sơ đã lưu
**When** dashboard được hiển thị
**Then** mỗi hồ sơ hiển thị trạng thái hiện tại, thời điểm chạy gần nhất và lịch chạy
**And** trạng thái thành công/thất bại được thể hiện bằng văn bản rõ ràng

**Given** một job vừa hoàn tất
**When** kết quả được phát sự kiện từ native layer
**Then** giao diện cập nhật trong giới hạn thời gian của NFR3
**And** người dùng không cần khởi động lại app để thấy trạng thái mới

## Tasks / Subtasks

- [ ] Task 1: Xây dashboard UI với profile status cards (AC: #1)
  - [ ] Subtask 1.1: Tạo Dashboard component với profile overview
  - [ ] Subtask 1.2: SyncProfileCard hiển thị: status, last run, schedule
  - [ ] Subtask 1.3: Status text: "Thành công", "Thất bại", "Đang chạy" (không chỉ màu)
  - [ ] Subtask 1.4: Real-time update khi event nhận được
- [ ] Task 2: Implement real-time event binding (AC: #2 - NFR3)
  - [ ] Subtask 2.1: Frontend lắng nghe `sync:completed` và `sync:failed` events
  - [ ] Subtask 2.2: Cập nhật Zustand store khi event nhận được
  - [ ] Subtask 2.3: Verify UI update trong < 5 giây (NFR3)

## Dev Notes

### Architecture Patterns
- **Zustand Store:** `syncStatusStore` cho dashboard state
- **Event Listening:** `src/services/tauri/events.ts` wrapper cho event listeners
- **NFR1:** Dashboard hiển thị trong 3 giây khi app khởi động
- **NFR3:** Kết quả sync hiển thị trong 5 giây sau khi job kết thúc

### Source Tree Components to Touch
```
src/
├── features/sync-status/
│   ├── components/
│   │   ├── Dashboard.tsx
│   │   └── SyncProfileCard.tsx
│   └── store/
│       └── syncStatusStore.ts
├── services/tauri/
│   ├── events.ts
│   └── commands.ts
```

### Testing Standards
- Performance test: Dashboard loads < 3 seconds (NFR1)
- Event binding test: UI updates within 5 seconds of job completion (NFR3)
- Status text verification (not color-only)

### References
- [Source: architecture.md#Frontend Architecture]
- [Source: architecture.md#State Management Patterns]
- [Source: prd.md#FR16, FR17, NFR1, NFR3]
- [Source: ux-design-specification.md#Dashboard]
- [Source: epics.md#Story 3.1]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Implemented Dashboard component with profile overview
- Implemented SyncProfileCard with status text display (not color-only) per NFR10
- Implemented Zustand store for sync status with real-time event binding
- Event listeners for sync:completed, sync:failed, sync:progress events
- Status display: "Thành công", "Thất bại", "Đang chạy" with appropriate icons
- Profile cards show: status, last run time, schedule, items processed

### File List
- src/types/profile.ts (created)
- src/types/job.ts (created)
- src/services/tauri/events.ts (created)
- src/services/tauri/commands.ts (created)
- src/services/tauri/contracts.ts (created)
- src/features/sync-status/store/syncStatusStore.ts (created)
- src/features/sync-status/components/Dashboard.tsx (created)
- src/features/sync-status/components/SyncProfileCard.tsx (created)
