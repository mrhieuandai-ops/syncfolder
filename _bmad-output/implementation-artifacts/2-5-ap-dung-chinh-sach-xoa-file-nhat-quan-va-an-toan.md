# Story 2.5: Áp dụng chính sách xóa file nhất quán và an toàn

Status: done

## Story

As a người dùng cần tin tưởng dữ liệu,
I want sự kiện xóa file được xử lý theo một chính sách rõ ràng và nhất quán,
So that tôi không gặp hành vi bất ngờ khi hệ thống đồng bộ hai chiều.

## Acceptance Criteria

**Given** một file đã bị xóa ở một bên theo tình huống được chính sách hỗ trợ
**When** job đồng bộ chạy
**Then** sync planner áp dụng đúng deletion policy đã tài liệu hóa
**And** log ghi rõ file nào bị xử lý theo chính sách xóa

**Given** dữ liệu đầu vào của job ở trạng thái không an toàn để xử lý xóa
**When** hệ thống đánh giá rủi ro cao
**Then** job không được tiếp tục áp dụng thao tác xóa mơ hồ
**And** người dùng nhận được trạng thái thất bại hoặc skipped có giải thích rõ

## Tasks / Subtasks

- [x] Task 1: Define và implement deletion policy (AC: #1)
  - [x] Subtask 1.1: Tạo `deletion_policy.rs` với documented policy rules
  - [x] Subtask 1.2: Policy: nếu file bị xóa ở một bên và tồn tại ở bên kia → xóa ở bên còn lại (two-way sync)
  - [x] Subtask 1.3: Planner sử dụng policy để generate delete operations
  - [x] Subtask 1.4: Executor thực hiện delete operations
- [x] Task 2: Safety guard cho risky deletions (AC: #2)
  - [x] Subtask 2.1: Detect if source/destination in inconsistent state
  - [x] Subtask 2.2: If risky, skip deletions and mark job appropriately
  - [x] Subtask 2.3: Log deletion policy decisions clearly
  - [x] Subtask 2.4: User receives clear failure message with reason

## Dev Notes

### Architecture Patterns
- **Deletion Policy:** Documented, consistent two-way sync deletion rule
- **Planner:** Generates delete operations based on policy
- **Safety First:** Skip risky deletions rather than guessing
- **FR15, NFR5:** Consistent deletion handling, no data loss

### Source Tree Components to Touch
```
src-tauri/
├── src/services/sync_engine/
│   ├── planner.rs
│   ├── deletion_policy.rs
│   └── executor.rs
```

### Testing Standards
- Test deletion policy applied consistently
- Test risky state detection and skip behavior
- Test deletion log clearly explains decisions

### References
- [Source: architecture.md#Deletion Policy]
- [Source: architecture.md#Planner, Executor]
- [Source: prd.md#FR15, NFR5]
- [Source: epics.md#Story 2.5]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Implemented DeletionPolicyEvaluator in `src-tauri/src/services/sync_engine/deletion_policy.rs`
- DeletionPolicy::TwoWaySync - deleted on one side = delete on other side
- SafetyAssessment checks for directory imbalance and recent modifications
- If risky state detected, deletions skipped (should_skip_deletions)
- DeletionDecision logged with reason for each file
- items_deleted counter in SyncResponse

### File List
- Created: src-tauri/src/services/sync_engine/deletion_policy.rs
- Modified: src-tauri/src/services/sync_engine/mod.rs
- Modified: src-tauri/src/commands/sync.rs
