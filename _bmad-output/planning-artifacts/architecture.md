---
stepsCompleted: [1, 2, 3, 4, 5, 6, 7, 8]
inputDocuments:
  - /mnt/d/ai_project/syncfolder/_bmad-output/planning-artifacts/prd.md
  - /mnt/d/ai_project/syncfolder/_bmad-output/planning-artifacts/product-brief-syncfolder-2026-03-25.md
workflowType: 'architecture'
project_name: 'syncfolder'
user_name: 'hieu'
date: '2026-03-25'
lastStep: 8
status: 'complete'
completedAt: '2026-03-25'
---

# Architecture Decision Document

_This document builds collaboratively through step-by-step discovery. Sections are appended as we work through each architectural decision together._

## Project Context Analysis

### Requirements Overview

**Functional Requirements:**

PRD hiện có 26 FR, xoay quanh 5 cụm năng lực chính: quản lý cấu hình đồng bộ, lập lịch chạy tự động, sync 2 chiều file/thư mục, hiển thị trạng thái và log, và xử lý lỗi/phục hồi. Về mặt kiến trúc, đây là một desktop app local-first với một sync engine trung tâm, một lớp scheduler độc lập, một lớp persistence cục bộ, và một GUI điều khiển/trình bày trạng thái.

**Non-Functional Requirements:**

Các NFR định hình kiến trúc rõ nhất là:
- khởi động nhanh và time-to-config ngắn
- độ tin cậy sync job >= 95% trong use case chuẩn
- không được thao tác ngoài hai đường dẫn được cấu hình
- log phải đủ chi tiết để phân loại ít nhất 4 nhóm lỗi
- toàn bộ chức năng cốt lõi hoạt động offline, không cần tài khoản

**Scale & Complexity:**

Đây không phải hệ phân tán, nhưng cũng không phải app CRUD đơn giản. Độ phức tạp kiến trúc thực tế là **medium** vì logic đồng bộ file có nhiều edge case: file đang bị khóa, path dài, external drive không sẵn sàng, xóa file 2 chiều, timestamp drift, và đảm bảo an toàn dữ liệu.

- Primary domain: desktop_app local-first file synchronization
- Complexity level: medium
- Estimated architectural components: 7

### Technical Constraints & Dependencies

- Chỉ hỗ trợ **Windows 11** ở MVP
- Không có backend/cloud trong MVP
- GUI là yêu cầu bắt buộc
- Đồng bộ chạy định kỳ theo 4 mốc cố định: 30/60/90 phút/1 ngày
- Phải hỗ trợ full sync file + thư mục con giữa 2 thư mục cục bộ/chia sẻ nội bộ
- Phải lưu cấu hình và lịch chạy cục bộ
- Kiến trúc cần chừa đường cho các tính năng hậu MVP: preview, conflict handling, exclusions, multi-profile, recycle-bin safety, NAS/network share

### Cross-Cutting Concerns Identified

- sync correctness và deletion safety
- filesystem access control và path allowlist
- scheduler reliability
- structured logging + error classification
- local persistence cho profiles, history, settings
- đóng gói Windows installer, update path, code signing readiness

## Starter Template Evaluation

### Primary Technology Domain

Desktop application based on project requirements analysis.

### Starter Options Considered

1. **Tauri 2.x + React + TypeScript + Vite**
   - Ưu điểm: binary nhỏ, dùng native WebView, nền Rust mạnh cho bài toán filesystem, mô hình capability/security tốt, phù hợp local desktop utility.
   - Nhược điểm: cần làm việc ở cả Rust và frontend stack.

2. **Electron 41.0.4 + React + Vite**
   - Ưu điểm: ecosystem lớn, dễ tiếp cận hơn nếu chỉ dùng JS/TS.
   - Nhược điểm: footprint lớn hơn, bundle nặng hơn, không tối ưu cho tiện ích desktop Windows-only cần hiệu năng và cảm giác “native nhẹ”.

3. **Tự dựng desktop shell không starter rõ ràng**
   - Linh hoạt hơn nhưng tăng số quyết định nền tảng và tăng rủi ro agent implementation divergence.

### Selected Starter: Tauri 2.x + React + TypeScript + Vite

**Rationale for Selection:**

Dựa trên xu hướng thị trường cho desktop sync utility 2024–2026, người dùng đánh giá cao 3 yếu tố: nhẹ, đáng tin, ít ngốn tài nguyên. Vì sản phẩm là Windows 11 local-first sync tool chứ không phải collaboration platform, lựa chọn tối ưu là **Tauri** thay vì Electron. Tauri tận dụng native webview nên footprint nhỏ hơn; Rust phù hợp để viết sync engine an toàn hơn cho thao tác file system; mô hình permission/capability giảm rủi ro truy cập ngoài phạm vi cấu hình. React + TypeScript + Vite vẫn giữ được tốc độ phát triển GUI và DX tốt.

**Initialization Command:**

```bash
npm create tauri-app@latest
```

> Trong quá trình scaffold, chọn: **React + TypeScript + Vite**.

**Architectural Decisions Provided by Starter:**

**Language & Runtime:**
- Rust backend core cho native logic
- TypeScript frontend cho GUI
- Tauri 2.x bridge giữa UI và native commands

**Styling Solution:**
- CSS/utility-first friendly frontend base qua Vite stack
- Không khóa vào design system nặng ở MVP

**Build Tooling:**
- Vite **8.0.2** cho frontend build/dev
- Tauri CLI **2.10.x** cho dev/build/bundle
- Rust toolchain hiện tại **1.94.0**

**Testing Framework:**
- Frontend unit testing dễ tích hợp từ Vite ecosystem
- Native/service tests ở Rust cargo test
- E2E desktop smoke qua Playwright/WebDriver for Tauri nếu cần

**Code Organization:**
- Frontend và native core tách lớp rõ: `src/` và `src-tauri/`
- Phù hợp cho boundary rõ giữa GUI, commands, services, persistence

**Development Experience:**
- Hot reload cho UI
- cargo-based test/build cho native core
- packaging Windows installer từ Tauri bundler

**Note:** Project initialization using this command should be the first implementation story.

## Core Architectural Decisions

### Decision Priority Analysis

**Critical Decisions (Block Implementation):**
- Chọn desktop shell: **Tauri 2.x**
- Chọn frontend stack: **React + TypeScript + Vite**
- Chọn sync core model: **Rust service layer** tách khỏi UI
- Chọn persistence cục bộ: **SQLite + local config files**
- Chọn communication model: **Tauri commands + app events**, không dùng HTTP API nội bộ

**Important Decisions (Shape Architecture):**
- Dùng **single-window application shell** với feature panels thay vì multi-window phức tạp
- Dùng **Zustand** cho UI state local, không cần global enterprise state layer
- Dùng **JSON structured logs + human-readable summary view**
- Dùng **job-oriented sync execution** với planner/executor tách riêng

**Deferred Decisions (Post-MVP):**
- realtime filesystem watch sync thay polling/scheduled only
- versioning/rollback
- advanced conflict resolution UI
- auto-update production rollout

### Data Architecture

- **Primary local store:** SQLite
- **Rationale:** app là local-first desktop utility; dữ liệu có cấu trúc vừa phải (profiles, schedules, jobs, logs index, conflict markers, settings); SQLite đủ mạnh, không cần local server.
- **Data modeling approach:**
  - `sync_profiles`: định nghĩa cặp thư mục, schedule, flags
  - `sync_jobs`: từng lần chạy sync, trạng thái, timestamps
  - `sync_events`: event/log index cho từng job
  - `app_settings`: cấu hình app cấp hệ thống
- **Metadata strategy:** file content không lưu DB; DB chỉ lưu metadata, job history, cấu hình, dấu vết lỗi.
- **Validation strategy:** validate 2 tầng
  - frontend form validation cho UX nhanh
  - backend command/service validation là nguồn sự thật cuối
- **Migration approach:** SQL migration versioned trong `src-tauri/migrations/`
- **Caching strategy:** in-memory cache nhẹ cho profile hiện hành và trạng thái job gần nhất; không dùng distributed cache.

### Authentication & Security

- **Authentication:** không có đăng nhập người dùng trong MVP
- **Authorization boundary:** app chỉ được thao tác trong các thư mục do người dùng chọn trong profile
- **Security model:**
  - path allowlist ở native layer
  - mọi thao tác file đi qua sync engine, không cho UI gọi filesystem tùy ý
  - capability/permission model của Tauri 2 cho command exposure
- **Local data protection:** file config/DB nằm trong app data của người dùng hiện tại; ưu tiên ACL theo tài khoản Windows.
- **Encryption approach:** chưa cần encryption at rest cho MVP vì chưa lưu secret; nếu sau này có cloud credential thì bổ sung secret store riêng.

### API & Communication Patterns

- **No external backend API in MVP**
- **Internal app communication:**
  - UI gọi native qua `invoke` commands
  - native emit app events cho tiến trình sync, progress, completion, failure
- **Command design:** command-oriented, ví dụ:
  - `create_profile`
  - `update_profile`
  - `run_sync_now`
  - `get_last_job_status`
  - `list_job_logs`
- **Error handling standard:** mọi command trả lỗi chuẩn hóa gồm `code`, `message`, `details?`, `retryable`
- **Rate limiting:** không cần network rate limiting; thay vào đó dùng single-flight / per-profile execution guard để tránh chạy chồng job.

### Frontend Architecture

- **UI framework:** React + TypeScript
- **State management:** Zustand cho app state, selected profile, last job status, settings UI state
- **Routing strategy:** không cần React Router phức tạp cho MVP; dùng single-shell với panel-based navigation
- **Component architecture:** feature-first
  - sync configuration
  - schedule settings
  - run status
  - logs/history
  - app settings
- **Form strategy:** controlled forms hoặc React Hook Form nếu cần mở rộng; ưu tiên đơn giản ở MVP
- **Performance optimization:** lazy load log/history panel nếu cần; phần sync core không chạy ở UI thread

### Infrastructure & Deployment

- **Target deployment:** Windows 11 desktop installer
- **Packaging:** Tauri Windows bundling/MSI-capable distribution path
- **CI/CD:** GitHub Actions cho lint/test/build/package
- **Environment config:** dev/test/prod config tách rõ ở frontend và `tauri.conf`/env native
- **Monitoring/logging:** log file cục bộ + in-app log viewer; remote telemetry để ngoài MVP
- **Scaling strategy:** không scale theo server; scale theo thư mục/file volume bằng incremental scan, bounded concurrency, retry rules và planner/executor tách lớp.

### Decision Impact Analysis

**Implementation Sequence:**
1. Scaffold Tauri + React + TypeScript + Vite
2. Thiết lập persistence + migrations
3. Xây sync engine core
4. Xây scheduler
5. Xây command bridge
6. Xây GUI cho profiles/schedule/status/logs
7. Hoàn thiện packaging, logging, tests

**Cross-Component Dependencies:**
- UI phụ thuộc command contracts
- scheduler và manual run cùng phụ thuộc sync engine
- log viewer phụ thuộc sync_events và event emission
- deletion/conflict policy phải được chia sẻ nhất quán giữa planner, executor, UI messages và logs

## Implementation Patterns & Consistency Rules

### Pattern Categories Defined

**Critical Conflict Points Identified:** 5 areas where AI agents could make different choices.

### Naming Patterns

**Database Naming Conventions:**
- tables: plural snake_case (`sync_profiles`, `sync_jobs`, `sync_events`)
- columns: snake_case (`source_path`, `destination_path`, `last_run_at`)
- foreign keys: `<entity>_id`
- indexes: `idx_<table>_<column>`

**API Naming Conventions:**
- Tauri commands: snake_case verbs (`run_sync_now`, `get_profile_by_id`)
- frontend service wrappers: camelCase (`runSyncNow`, `getProfileById`)
- event names: namespaced kebab/colon form (`sync:started`, `sync:progress`, `sync:completed`, `sync:failed`)

**Code Naming Conventions:**
- React components: PascalCase (`SyncProfileForm.tsx`)
- TS files except components: kebab-case (`sync-history-store.ts`)
- Rust modules/functions: snake_case
- types/interfaces in TS: PascalCase

### Structure Patterns

**Project Organization:**
- feature-first trong frontend
- service-first trong native core
- tests co-located cho unit nhỏ; integration/e2e ở thư mục test riêng
- shared schemas/types ở `src/types` và `src/lib/validation`

**File Structure Patterns:**
- config frontend trong `src/config`
- tauri/native config trong `src-tauri/tauri.conf.json` và `src-tauri/capabilities/`
- migrations riêng trong `src-tauri/migrations/`
- logs runtime không commit vào repo

### Format Patterns

**API Response Formats:**
- success: direct typed payload nếu đơn giản
- error: `{ code, message, details?, retryable }`
- progress event: `{ profileId, jobId, phase, processedItems, totalItems?, currentPath? }`

**Data Exchange Formats:**
- frontend JSON fields: camelCase
- Rust internal/db fields: snake_case
- datetime: ISO 8601 UTC strings khi qua bridge/UI
- booleans luôn `true/false`
- nullable fields dùng `null`, không dùng sentinel values

### Communication Patterns

**Event System Patterns:**
- lifecycle events chuẩn: `sync:queued`, `sync:started`, `sync:progress`, `sync:completed`, `sync:failed`
- event payload luôn có `profileId`, `jobId`, `timestamp`
- UI không tự suy diễn state từ nhiều nguồn; state sync dựa trên event + explicit refetch khi cần

**State Management Patterns:**
- server-state analogue không tồn tại vì app local-only; dùng Zustand store theo feature
- optimistic update chỉ dùng cho chỉnh config đơn giản; sync execution state luôn lấy từ native events/source of truth
- một store không quản nhiều responsibility không liên quan

### Process Patterns

**Error Handling Patterns:**
- native layer phân loại lỗi thành validation, access, availability, lock, unknown
- UI hiển thị message thân thiện nhưng vẫn giữ raw diagnostic trong log
- không nuốt lỗi silently

**Loading State Patterns:**
- `idle | loading | success | error` cho UI actions
- sync job state riêng: `scheduled | queued | running | succeeded | failed | skipped`
- disable action gây xung đột khi profile đang chạy

### Enforcement Guidelines

**All AI Agents MUST:**
- giữ nguyên boundary: UI không đọc/ghi file trực tiếp
- mọi thao tác sync phải đi qua sync engine service
- tuân thủ naming/event/error patterns đã chốt

**Pattern Enforcement:**
- kiểm tra bằng lint, typecheck, unit tests và review diff theo architecture doc
- vi phạm pattern phải sửa ở code hoặc cập nhật architecture doc trước khi lan rộng
- mọi contract mới qua command/event phải được thêm vào shared type/schema trước

### Pattern Examples

**Good Examples:**
- `src/features/sync-config/components/SyncProfileForm.tsx`
- `src-tauri/src/commands/run_sync_now.rs`
- event `sync:failed` với payload có `code`, `message`, `jobId`

**Anti-Patterns:**
- component UI gọi filesystem plugin trực tiếp để bypass service layer
- đặt tên lẫn lộn `runSyncNow` ở Rust command hoặc `run_sync_now` ở React component
- response lỗi lúc thì string, lúc thì object

## Project Structure & Boundaries

### Complete Project Directory Structure

```text
syncfolder/
├── README.md
├── package.json
├── tsconfig.json
├── vite.config.ts
├── eslint.config.js
├── .gitignore
├── .env.example
├── .github/
│   └── workflows/
│       ├── ci.yml
│       └── release-windows.yml
├── src/
│   ├── main.tsx
│   ├── App.tsx
│   ├── app/
│   │   ├── shell/
│   │   │   ├── app-shell.tsx
│   │   │   ├── sidebar.tsx
│   │   │   └── header.tsx
│   │   └── providers/
│   │       └── app-providers.tsx
│   ├── features/
│   │   ├── sync-config/
│   │   │   ├── components/
│   │   │   │   ├── SyncProfileForm.tsx
│   │   │   │   ├── FolderPickerField.tsx
│   │   │   │   └── ScheduleSelect.tsx
│   │   │   ├── hooks/
│   │   │   ├── services/
│   │   │   └── store/
│   │   ├── sync-status/
│   │   │   ├── components/
│   │   │   ├── services/
│   │   │   └── store/
│   │   ├── sync-history/
│   │   │   ├── components/
│   │   │   ├── services/
│   │   │   └── store/
│   │   ├── settings/
│   │   │   ├── components/
│   │   │   └── store/
│   │   └── notifications/
│   │       └── components/
│   ├── components/
│   │   └── ui/
│   ├── hooks/
│   ├── lib/
│   │   ├── validation/
│   │   ├── formatters/
│   │   └── errors/
│   ├── services/
│   │   └── tauri/
│   │       ├── commands.ts
│   │       ├── events.ts
│   │       └── contracts.ts
│   ├── types/
│   │   ├── profile.ts
│   │   ├── job.ts
│   │   ├── log.ts
│   │   └── error.ts
│   └── styles/
│       └── globals.css
├── src-tauri/
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── capabilities/
│   │   └── default.json
│   ├── icons/
│   ├── migrations/
│   │   ├── 0001_init.sql
│   │   ├── 0002_sync_jobs.sql
│   │   └── 0003_sync_events.sql
│   └── src/
│       ├── main.rs
│       ├── app_state.rs
│       ├── commands/
│       │   ├── mod.rs
│       │   ├── profiles.rs
│       │   ├── sync.rs
│       │   ├── status.rs
│       │   ├── logs.rs
│       │   └── settings.rs
│       ├── models/
│       │   ├── mod.rs
│       │   ├── profile.rs
│       │   ├── job.rs
│       │   ├── sync_event.rs
│       │   └── app_setting.rs
│       ├── repositories/
│       │   ├── mod.rs
│       │   ├── profiles_repo.rs
│       │   ├── jobs_repo.rs
│       │   ├── events_repo.rs
│       │   └── settings_repo.rs
│       ├── services/
│       │   ├── mod.rs
│       │   ├── scheduler/
│       │   │   ├── mod.rs
│       │   │   └── job_scheduler.rs
│       │   ├── sync_engine/
│       │   │   ├── mod.rs
│       │   │   ├── scanner.rs
│       │   │   ├── comparer.rs
│       │   │   ├── planner.rs
│       │   │   ├── executor.rs
│       │   │   ├── conflict.rs
│       │   │   └── deletion_policy.rs
│       │   ├── logging/
│       │   │   ├── mod.rs
│       │   │   └── job_logger.rs
│       │   ├── persistence/
│       │   │   ├── mod.rs
│       │   │   ├── sqlite.rs
│       │   │   └── migrations.rs
│       │   └── path_guard/
│       │       ├── mod.rs
│       │       └── allowlist.rs
│       ├── events/
│       │   ├── mod.rs
│       │   └── sync_events.rs
│       ├── errors/
│       │   ├── mod.rs
│       │   ├── app_error.rs
│       │   └── error_codes.rs
│       └── utils/
│           ├── mod.rs
│           ├── datetime.rs
│           └── paths.rs
├── tests/
│   ├── frontend/
│   ├── integration/
│   ├── native/
│   ├── e2e/
│   └── fixtures/
└── docs/
    └── architecture/
```

### Architectural Boundaries

**API Boundaries:**
- Không có public web API trong MVP
- Boundary chính là Tauri command layer giữa React UI và Rust core
- Chỉ `commands/*` được export ra UI; service/repository nội bộ không expose trực tiếp

**Component Boundaries:**
- feature UI chỉ dùng service wrapper trong `src/services/tauri/`
- shared UI component không chứa business sync logic
- stores chỉ quản UI/app state, không chứa filesystem logic

**Service Boundaries:**
- `sync_engine` chịu trách nhiệm scan/compare/plan/execute
- `scheduler` chỉ quyết định khi nào chạy job, không biết chi tiết sync internals
- `logging` chuẩn hóa audit trail
- `persistence` cô lập SQLite access

**Data Boundaries:**
- SQLite chỉ lưu metadata/config/job history
- filesystem source/destination là data source thực
- logs có thể được index DB nhưng raw file log tách riêng nếu cần

### Requirements to Structure Mapping

**Feature/Epic Mapping:**
- Sync Configuration (FR1-FR5) → `src/features/sync-config/` + `src-tauri/src/commands/profiles.rs`
- Scheduling & Execution (FR6-FR9) → `src/features/sync-config/`, `scheduler/`, `commands/sync.rs`
- Two-Way Sync (FR10-FR15) → `services/sync_engine/`
- Status & Logging (FR16-FR20) → `src/features/sync-status/`, `src/features/sync-history/`, `logging/`, `events/`
- Error Handling & Recovery (FR21-FR24) → `errors/`, `path_guard/`, `sync_engine/`, `commands/status.rs`
- Application Operation (FR25-FR26) → `settings/`, `persistence/`, startup state bootstrapping

**Cross-Cutting Concerns:**
- NFR security → `capabilities/default.json`, `path_guard/`, config storage location
- NFR observability → `logging/`, `events/`, `sync-history/`
- NFR accessibility → frontend `components/ui/`, shell/layout semantics

### Integration Points

**Internal Communication:**
- React UI → Tauri command bridge → Rust services
- Rust services → event emitter → React stores/UI
- scheduler → sync engine → repositories/logging

**External Integrations:**
- Windows folder picker / filesystem
- Windows installer/bundler pipeline
- tương lai: updater/signing, NAS/network paths

**Data Flow:**
- User configures profile in UI
- command validates + persists profile
- scheduler/manual trigger starts job
- sync engine scans both folders, builds plan, executes operations
- logger + event emitter publish results
- UI updates status/history from events and queries

### File Organization Patterns

**Configuration Files:**
- repo-level JS/TS tooling ở root
- tauri/native config trong `src-tauri/`
- env examples chỉ chứa non-secret defaults

**Source Organization:**
- frontend feature-first
- native service-first with explicit commands/repositories/events/errors split

**Test Organization:**
- frontend component/unit tests trong `tests/frontend`
- native/service tests trong `tests/native`
- integration job flow tests trong `tests/integration`
- desktop smoke tests trong `tests/e2e`

**Asset Organization:**
- frontend static assets trong `src/styles` hoặc public-equivalent nếu cần
- app icons và bundle assets trong `src-tauri/icons`

### Development Workflow Integration

**Development Server Structure:**
- Vite phục vụ frontend dev
- Tauri CLI chạy shell desktop và native commands

**Build Process Structure:**
- `vite build` → frontend dist
- `tauri build` → Windows bundle/installer

**Deployment Structure:**
- GitHub Actions build/test/package
- artifact output tập trung cho Windows release

## Architecture Validation Results

### Coherence Validation ✅

**Decision Compatibility:**
Tauri 2.x, React/TS/Vite, Rust core và SQLite tạo thành một stack nhất quán cho desktop sync utility local-first. Không có xung đột kiến trúc chính.

**Pattern Consistency:**
Naming, command/event contracts, error format, state boundaries và project structure hỗ trợ trực tiếp cho stack đã chọn.

**Structure Alignment:**
Project structure hỗ trợ đầy đủ việc tách GUI, native commands, sync engine, persistence và logging; phù hợp để nhiều AI agents làm việc song song mà ít đụng nhau.

### Requirements Coverage Validation ✅

**Epic/Feature Coverage:**
Tất cả feature từ PRD đã được ánh xạ vào module hoặc service cụ thể.

**Functional Requirements Coverage:**
- FR1-FR9 được hỗ trợ bởi profile management + scheduler + command bridge
- FR10-FR15 được hỗ trợ bởi sync engine + deletion policy
- FR16-FR20 được hỗ trợ bởi status/history/events/logging
- FR21-FR24 được hỗ trợ bởi path guard + app_error + retry-safe job model
- FR25-FR26 được hỗ trợ bởi offline-first stack + local persistence bootstrapping

**Non-Functional Requirements Coverage:**
- Performance: Vite + native shell nhẹ; sync core off UI thread
- Reliability: job model, planner/executor split, guard rails
- Security: per-user local storage + path allowlist + capability boundary
- Accessibility: semantic UI/text-first status messages
- Supportability: structured logs + explicit error codes

### Implementation Readiness Validation ✅

**Decision Completeness:**
Các quyết định block implementation đã được chốt đủ rõ ở mức architecture.

**Structure Completeness:**
Project tree đủ cụ thể để triển khai theo story và tách việc cho AI agents.

**Pattern Completeness:**
Các conflict point chính giữa nhiều agents đã được khóa: naming, command contracts, event model, error shape, directory ownership.

### Gap Analysis Results

**Critical Gaps:** none

**Important Gaps:**
- Chính sách conflict resolution hậu MVP có thể cần tài liệu phụ khi mở rộng preview/conflict UI.
- Auto-update/code-signing production rollout có thể thêm trong release architecture supplement.

**Nice-to-Have Gaps:**
- telemetry/crash reporting nếu sau này có beta external
- network share compatibility matrix cho phase mở rộng

### Validation Issues Addressed

- Chọn Tauri thay Electron để phù hợp trend “lightweight + trustworthy desktop utility”
- Khóa rõ local-only/no-auth architecture để tránh overengineering backend
- Tách sync engine thành scanner/comparer/planner/executor để giảm mơ hồ implementation

### Architecture Completeness Checklist

**✅ Requirements Analysis**

- [x] Project context thoroughly analyzed
- [x] Scale and complexity assessed
- [x] Technical constraints identified
- [x] Cross-cutting concerns mapped

**✅ Architectural Decisions**

- [x] Critical decisions documented with versions
- [x] Technology stack fully specified
- [x] Integration patterns defined
- [x] Performance considerations addressed

**✅ Implementation Patterns**

- [x] Naming conventions established
- [x] Structure patterns defined
- [x] Communication patterns specified
- [x] Process patterns documented

**✅ Project Structure**

- [x] Complete directory structure defined
- [x] Component boundaries established
- [x] Integration points mapped
- [x] Requirements to structure mapping complete

### Architecture Readiness Assessment

**Overall Status:** READY FOR IMPLEMENTATION

**Confidence Level:** high

**Key Strengths:**
- chọn đúng stack cho Windows local sync use case
- boundary giữa UI và sync core rất rõ
- đủ rule để nhiều AI agents triển khai nhất quán
- giữ MVP đơn giản nhưng không chặn mở rộng hậu MVP

**Areas for Future Enhancement:**
- preview/conflict handling supplement
- release/update/signing supplement
- NAS/network-share testing supplement

### Implementation Handoff

**AI Agent Guidelines:**

- Follow all architectural decisions exactly as documented
- Use implementation patterns consistently across all components
- Respect project structure and boundaries
- Refer to this document for all architectural questions

**First Implementation Priority:**

```bash
npm create tauri-app@latest
```

Sau đó scaffold với lựa chọn **React + TypeScript + Vite**, rồi triển khai story đầu tiên: project bootstrap + command bridge + persistence foundation.
