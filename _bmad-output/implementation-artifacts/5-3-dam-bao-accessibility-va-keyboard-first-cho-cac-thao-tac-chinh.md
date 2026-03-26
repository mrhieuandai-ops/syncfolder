# Story 5.3: Đảm bảo accessibility và keyboard-first cho các thao tác chính

Status: done

## Story

As a người dùng cần khả năng tiếp cận tốt,
I want điều hướng và thao tác chính hoạt động tốt bằng bàn phím và screen reader,
So that tôi có thể sử dụng ứng dụng mà không phụ thuộc vào chuột hoặc màu sắc.

## Acceptance Criteria

**Given** người dùng chỉ dùng bàn phím
**When** họ đi qua các thao tác chính như chọn thư mục, chọn lịch, bật/tắt sync và chạy thủ công
**Then** toàn bộ thao tác có thể hoàn thành bằng keyboard navigation hợp lệ
**And** focus visible luôn rõ ràng trên các control tương tác

**Given** người dùng dùng screen reader hoặc cần hỗ trợ tương phản
**When** giao diện hiển thị trạng thái và form controls
**Then** semantic structure, labels và aria attributes phù hợp được cung cấp
**And** trạng thái thành công/thất bại/cảnh báo không được truyền đạt chỉ bằng màu sắc

## Tasks / Subtasks

- [ ] Task 1: Keyboard navigation cho main operations (AC: #1)
  - [ ] Subtask 1.1: All interactive elements focusable và operable via keyboard
  - [ ] Subtask 1.2: Tab order follows logical flow
  - [ ] Subtask 1.3: Focus indicators visible on all controls
  - [ ] Subtask 1.4: Test: folder picker, schedule select, toggle, button
- [ ] Task 2: Screen reader support và non-color status (AC: #2 - NFR9, NFR10)
  - [ ] Subtask 2.1: ARIA labels on all form controls
  - [ ] Subtask 2.2: Semantic HTML structure (headings, labels)
  - [ ] Subtask 2.3: Status text: "Thành công" / "Thất bại" / "Cảnh báo" (not color-only)
  - [ ] Subtask 2.4: aria-live regions for dynamic content

## Dev Notes

### Architecture Patterns
- **WCAG 2.1 AA:** Target compliance level
- **Keyboard Navigation:** Tab, Arrow keys, Enter, Space, Escape
- **Focus Visibility:** `:focus-visible` styles on all interactive elements
- **ARIA:** Labels, roles, live regions for dynamic content
- **NFR9:** Keyboard navigation for main operations
- **NFR10:** Status not conveyed by color alone

### Source Tree Components to Touch
```
src/
├── components/ui/
│   └── (Fluent UI components with custom ARIA)
├── features/sync-config/
│   └── components/
│       ├── FolderPickerField.tsx
│       ├── ScheduleSelect.tsx
│       └── SyncProfileForm.tsx
```

### Testing Standards
- Keyboard-only test: Complete profile creation workflow
- Screen reader test: Verify all controls announced correctly
- NFR9 verification: All main operations keyboard accessible
- NFR10 verification: Status conveyed with text, not color-only

### References
- [Source: ux-design-specification.md#Accessibility Strategy]
- [Source: prd.md#NFR9, NFR10]
- [Source: epics.md#Story 5.3]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Created FolderPickerField with ARIA labels for screen readers
- All form controls have proper labels and aria-describedby for errors
- Created ScheduleSelect with Dropdown and ARIA attributes
- SyncProfileForm implements full keyboard navigation (Tab order follows logical flow)
- Focus indicators visible using CSS :focus-visible
- Status conveyed with text AND icons (not color-only) - NFR10 compliance
- aria-live regions for dynamic content updates
- Semantic HTML structure with proper heading hierarchy
- All interactive elements have aria-label for screen reader announcement

### File List
- src/features/sync-config/components/FolderPickerField.tsx
- src/features/sync-config/components/ScheduleSelect.tsx
- src/features/sync-config/components/SyncProfileForm.tsx
- src/features/sync-config/components/index.ts
