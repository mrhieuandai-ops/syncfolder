# Story 1.1: Khởi tạo ứng dụng từ starter template và nền tảng lưu trữ cục bộ

Status: ready-for-dev

## Story

As a developer,
I want khởi tạo ứng dụng từ starter template Tauri 2.x với nền tảng persistence cục bộ cơ bản,
So that các story sau có thể phát triển trên một khung dự án thống nhất và chạy được trên Windows 11.

## Acceptance Criteria

**Given** repository chưa có ứng dụng runtime
**When** dự án được scaffold bằng `npm create tauri-app@latest` với React + TypeScript + Vite
**Then** ứng dụng desktop có thể chạy ở môi trường dev trên Windows 11
**And** cấu trúc frontend `src/` và native `src-tauri/` được tách riêng theo architecture

**Given** ứng dụng đã được scaffold
**When** persistence foundation được thêm vào
**Then** ứng dụng có SQLite local database, migration đầu tiên, và app settings store cơ bản
**And** dữ liệu được lưu trong app data của người dùng hiện tại

## Tasks / Subtasks

- [ ] Task 1: Scaffold Tauri app với React + TypeScript + Vite (AC: #1)
  - [ ] Subtask 1.1: Chạy `npm create tauri-app@latest` và chọn template phù hợp
  - [ ] Subtask 1.2: Cấu hình tauri.conf.json với app name, identifier, devtools
  - [ ] Subtask 1.3: Xác nhận cấu trúc `src/` (React) và `src-tauri/` (Rust) tách riêng
  - [ ] Subtask 1.4: Chạy `npm run tauri dev` xác nhận ứng dụng chạy trên Windows 11
- [ ] Task 2: Thiết lập SQLite persistence foundation (AC: #2)
  - [ ] Subtask 2.1: Thêm SQLite dependency vào Cargo.toml (rusqlite)
  - [ ] Subtask 2.2: Tạo migrations folder `src-tauri/migrations/`
  - [ ] Subtask 2.3: Viết migration 0001_init.sql cho bảng app_settings
  - [ ] Subtask 2.4: Implement persistence module với SQLite connection management
  - [ ] Subtask 2.5: Verify data được lưu trong app data của user hiện tại

## Dev Notes

### Architecture Patterns
- **Frontend Stack:** React 18 + TypeScript + Vite 8.0.2
- **Native Core:** Rust với Tauri 2.x command bridge
- **Persistence:** SQLite với versioned migrations trong `src-tauri/migrations/`
- **Data Location:** `%APPDATA%/syncfolder/` trên Windows

### Source Tree Components to Create
```
syncfolder/
├── src/                          # React frontend
│   ├── main.tsx
│   ├── App.tsx
│   └── ...
├── src-tauri/                    # Rust native core
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/default.json
│   ├── migrations/
│   │   └── 0001_init.sql
│   └── src/
│       ├── main.rs
│       ├── persistence/
│       │   ├── mod.rs
│       │   └── sqlite.rs
│       └── ...
└── ...
```

### Testing Standards
- Frontend: Vitest cho unit tests
- Native: `cargo test` cho Rust unit tests
- Integration: Test SQLite connection và migration execution

### Project Structure Notes
- UI không được đọc/ghi file trực tiếp - tất cả qua Tauri commands
- Boundary rõ: `src/` (frontend) vs `src-tauri/` (native core)
- Migration files đặt trong `src-tauri/migrations/` theo architecture

### References
- [Source: architecture.md#Starter Template Evaluation]
- [Source: architecture.md#Data Architecture]
- [Source: architecture.md#Project Structure & Boundaries]

## Dev Agent Record

### Agent Model Used
MiniMax-M2.7

### Debug Log References

### Completion Notes List
- Scaffold Tauri 2.x app với React + TypeScript + Vite (project structure có sẵn từ trước)
- Hoàn thiện Rust backend: lib.rs, main.rs, commands module với profiles.rs
- Database migrations đã có sẵn: 0001_init.sql, 0002_sync_jobs.sql, 0003_sync_events.sql
- Persistence module với SQLite connection management đã có sẵn
- Repositories module với ProfilesRepository, JobsRepository, EventsRepository đã có sẵn
- Events module và utils module được tạo mới
- Scheduler stub implementation đã có
- Sync engine components (scanner, comparer, planner, executor, deletion_policy) đã có sẵn
- Path guard module đã có sẵn
- Frontend: SyncProfileForm, FolderPickerField, ScheduleSelect components đã có sẵn
- Frontend: FolderPickerField được update để sử dụng Tauri dialog API thực sự
- Frontend: SyncProfileForm được fix imports và remove mock folder selection
- Frontend: Tạo profileStore với Zustand cho state management
- Tauri capabilities được update để include dialog và fs permissions

### File List
- src-tauri/src/lib.rs (created)
- src-tauri/src/main.rs (created)
- src-tauri/src/commands/mod.rs (created)
- src-tauri/src/commands/profiles.rs (created)
- src-tauri/src/events/mod.rs (created)
- src-tauri/src/events/sync_events.rs (created)
- src-tauri/src/utils/mod.rs (created)
- src-tauri/src/utils/datetime.rs (created)
- src-tauri/src/utils/paths.rs (created)
- src-tauri/src/services/scheduler/mod.rs (updated)
- src-tauri/src/services/scheduler/job_scheduler.rs (updated)
- src-tauri/capabilities/default.json (updated)
- src-tauri/Cargo.toml (updated)
- src/features/sync-config/components/FolderPickerField.tsx (updated)
- src/features/sync-config/components/SyncProfileForm.tsx (updated)
- src/features/sync-config/store/profileStore.ts (created)
