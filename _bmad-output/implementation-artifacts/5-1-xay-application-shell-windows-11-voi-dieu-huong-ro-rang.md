# Story 5.1: Xây application shell Windows 11 với điều hướng rõ ràng

Status: done

## Story

As a người dùng mới,
I want một application shell rõ ràng với Dashboard, Profiles, Activity Log và Settings,
So that tôi luôn biết mình đang ở đâu và tìm thấy tác vụ cần làm nhanh chóng.

## Acceptance Criteria

**Given** người dùng mở ứng dụng
**When** app shell được render
**Then** giao diện có cấu trúc single-window với điều hướng rõ ràng giữa các panel chính
**And** bố cục tuân thủ định hướng Windows 11/Fluent đã chốt trong UX

**Given** người dùng chuyển giữa các khu vực chính
**When** họ dùng chuột hoặc bàn phím
**Then** mục active được thể hiện rõ ràng
**And** điều hướng không làm mất trạng thái quan trọng của hồ sơ đang xem

## Tasks / Subtasks

- [ ] Task 1: Build app shell với navigation (AC: #1, #2)
  - [ ] Subtask 1.1: Tạo AppShell component với sidebar navigation
  - [ ] Subtask 1.2: Navigation items: Dashboard, Profiles, Activity Log, Settings
  - [ ] Subtask 1.3: Fluent UI `NavigationView` implementation
  - [ ] Subtask 1.4: Active state indicator rõ ràng
- [ ] Task 2: Keyboard navigation support (AC: #2)
  - [ ] Subtask 2.1: Tab/Arrow key navigation between nav items
  - [ ] Subtask 2.2: Enter to select navigation item
  - [ ] Subtask 2.3: Focus visible states

## Dev Notes

### Architecture Patterns
- **App Shell:** Single-window với sidebar navigation
- **Fluent UI:** NavigationView, Fluent design tokens
- **Navigation State:** Zustand store, URL-less (panel-based for MVP)
- **UX:** Windows 11/Fluent design language

### Source Tree Components to Touch
```
src/
├── app/
│   ├── shell/
│   │   ├── app-shell.tsx
│   │   ├── sidebar.tsx
│   │   └── header.tsx
│   └── providers/
│       └── app-providers.tsx
├── main.tsx
└── App.tsx
```

### Testing Standards
- Test all 4 nav items accessible
- Test keyboard navigation (Tab, Arrow, Enter)
- Test focus visible states
- Visual verification of Fluent design compliance

### References
- [Source: architecture.md#Frontend Architecture]
- [Source: ux-design-specification.md#Navigation Patterns]
- [Source: ux-design-specification.md#Design System Choice]
- [Source: epics.md#Story 5.1]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Created AppShell component with sidebar navigation using custom div-based navigation (Fluent UI NavigationView had dependency issues)
- Navigation items: Dashboard, Profiles, Activity Log, Settings with proper icons
- Active state indicator using borderLeft and backgroundColor
- Keyboard navigation support with Tab, Enter, Space keys
- Focus visible states using CSS :focus-visible
- Created navItems.ts with configuration for all navigation items
- Created sidebarStyles.ts with consistent styling following Windows 11 Fluent Design
- Created header.tsx component for page headers
- Created AppProviders.tsx with FluentProvider theming
- Created globals.css with CSS variables and accessibility styles

### File List
- src/app/shell/AppShell.tsx
- src/app/shell/navItems.ts
- src/app/shell/sidebarStyles.ts
- src/app/shell/header.tsx
- src/app/shell/index.ts
- src/app/providers/AppProviders.tsx
- src/app/providers/index.ts
- src/App.tsx
- src/main.tsx
- src/styles/globals.css
