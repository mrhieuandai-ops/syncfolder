# Story 5.2: Hoàn thiện luồng onboarding và các trạng thái giao diện cốt lõi

Status: done

## Story

As a người dùng lần đầu,
I want onboarding đơn giản cùng empty state, loading state và feedback state rõ ràng,
So that tôi có thể hiểu ứng dụng ngay cả trước khi có dữ liệu hoặc khi hệ thống đang xử lý.

## Acceptance Criteria

**Given** ứng dụng chưa có hồ sơ nào
**When** người dùng mở app lần đầu
**Then** giao diện hiển thị empty state thân thiện với CTA rõ ràng để tạo hồ sơ đầu tiên
**And** luồng onboarding hỗ trợ hoàn tất trong mục tiêu dưới 3 phút nếu người dùng đã biết hai thư mục

**Given** ứng dụng đang tải dữ liệu hoặc thực hiện thao tác dài
**When** người dùng chờ kết quả
**Then** loading state hiển thị rõ mà không gây cảm giác app bị treo
**And** success, warning, error feedback tuân thủ hierarchy và copywriting đã chốt trong UX

## Tasks / Subtasks

- [ ] Task 1: Implement empty state UI (AC: #1)
  - [ ] Subtask 1.1: Empty state component với icon, message, CTA
  - [ ] Subtask 1.2: Message: "Chưa có hồ sơ đồng bộ nào"
  - [ ] Subtask 1.3: Primary button: "Tạo hồ sơ đầu tiên"
  - [ ] Subtask 1.4: Verify onboarding flow completes < 3 minutes
- [ ] Task 2: Implement loading và feedback states (AC: #2)
  - [ ] Subtask 2.1: LoadingSpinner/ProgressRing for async operations
  - [ ] Subtask 2.2: MessageBar for success/warning/error feedback
  - [ ] Subtask 2.3: Copywriting guidelines per UX spec
  - [ ] Subtask 2.4: Non-blocking feedback (does not freeze UI)

## Dev Notes

### Architecture Patterns
- **Empty State:** Icon + message + CTA pattern
- **Loading State:** Fluent UI Spinner/ProgressRing
- **Feedback:** MessageBar với semantic colors (success/warning/error)
- **NFR2:** Onboarding hoàn tất trong < 3 phút

### Source Tree Components to Touch
```
src/
├── components/ui/
│   ├── EmptyState.tsx
│   ├── LoadingSpinner.tsx
│   └── FeedbackMessage.tsx
├── features/sync-config/
│   └── components/
│       └── SyncProfileForm.tsx
```

### Testing Standards
- Test empty state displays when no profiles
- Test loading state during async operations
- Test feedback messages for success/warning/error
- Test NFR2: Onboarding timing

### References
- [Source: ux-design-specification.md#Empty States]
- [Source: ux-design-specification.md#Loading States]
- [Source: ux-design-specification.md#Feedback Patterns]
- [Source: prd.md#NFR2]
- [Source: epics.md#Story 5.2]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Created EmptyState component with icon, message, and CTA button
- Empty state message: "Chưa có hồ sơ đồng bộ nào" with primary button "Tạo hồ sơ đầu tiên"
- Created LoadingSpinner component using Fluent UI Spinner with size options
- Created FeedbackMessage component with success/warning/error/info types
- Each feedback type has appropriate icon (CheckmarkCircle, Warning, DismissCircle, Info)
- aria-live regions configured appropriately (assertive for errors, polite for others)
- All states follow Windows 11 Fluent Design with proper colors and spacing

### File List
- src/components/ui/EmptyState.tsx
- src/components/ui/LoadingSpinner.tsx
- src/components/ui/FeedbackMessage.tsx
- src/components/ui/index.ts
