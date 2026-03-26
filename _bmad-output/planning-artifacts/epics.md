---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - /mnt/d/ai_project/syncfolder/_bmad-output/planning-artifacts/prd.md
  - /mnt/d/ai_project/syncfolder/_bmad-output/planning-artifacts/architecture.md
  - /mnt/d/ai_project/syncfolder/_bmad-output/planning-artifacts/ux-design-specification.md
---

# syncfolder - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for syncfolder, decomposing the requirements from the PRD, UX Design if it exists, and Architecture requirements into implementable stories.

## Requirements Inventory

### Functional Requirements

FR1: Người dùng có thể tạo một cấu hình đồng bộ mới bằng cách chọn thư mục nguồn và thư mục đích.
FR2: Người dùng có thể chỉnh sửa cấu hình đồng bộ đã lưu để cập nhật thư mục hoặc lịch chạy của một profile hiện có.
FR3: Người dùng có thể lưu cấu hình đồng bộ để tái sử dụng ở các lần mở ứng dụng sau.
FR4: Người dùng có thể xem thông tin cặp thư mục và lịch chạy đang được cấu hình cho profile hiện tại.
FR5: Hệ thống có thể ngăn người dùng lưu cấu hình không hợp lệ khi thiếu thư mục nguồn hoặc đích.
FR6: Người dùng có thể chọn chu kỳ đồng bộ tự động từ các tùy chọn 30 phút, 60 phút, 90 phút hoặc 1 ngày.
FR7: Hệ thống có thể thực thi job đồng bộ theo lịch đã cấu hình.
FR8: Người dùng có thể kích hoạt hoặc vô hiệu hóa lịch đồng bộ của cấu hình hiện tại.
FR9: Người dùng có thể khởi chạy đồng bộ thủ công ngoài lịch định kỳ.
FR10: Hệ thống có thể đồng bộ hai chiều toàn bộ file giữa hai thư mục đã chọn.
FR11: Hệ thống có thể đồng bộ hai chiều toàn bộ thư mục con giữa hai thư mục đã chọn.
FR12: Hệ thống có thể phát hiện file mới xuất hiện ở một trong hai thư mục và sao chép sang thư mục còn lại.
FR13: Hệ thống có thể phát hiện file đã thay đổi ở một trong hai thư mục và cập nhật sang thư mục còn lại.
FR14: Hệ thống có thể phát hiện thư mục con mới và đồng bộ cấu trúc sang phía còn lại.
FR15: Hệ thống có thể xử lý sự kiện xóa file theo một chính sách đồng bộ hai chiều nhất quán, được tài liệu hóa và áp dụng giống nhau cho mọi job sync.
FR16: Người dùng có thể xem thời điểm chạy gần nhất của job đồng bộ.
FR17: Người dùng có thể xem kết quả của lần đồng bộ gần nhất theo trạng thái thành công hoặc thất bại.
FR18: Người dùng có thể xem thông báo lỗi cơ bản khi một lần đồng bộ không hoàn thành.
FR19: Hệ thống có thể ghi log cho mỗi lần đồng bộ.
FR20: Người dùng có thể xem log đủ để xác định nguyên nhân lỗi phổ biến như đường dẫn không tồn tại, mất quyền truy cập hoặc thư mục không khả dụng.
FR21: Hệ thống có thể phát hiện khi thư mục nguồn hoặc đích không còn khả dụng tại thời điểm chạy job.
FR22: Hệ thống có thể dừng hoặc bỏ qua một job đồng bộ khi phát hiện thư mục không khả dụng, lỗi quyền truy cập hoặc trạng thái dữ liệu không xác định thay vì tiếp tục xử lý rủi ro.
FR23: Người dùng có thể thực hiện lại job đồng bộ sau khi lỗi môi trường đã được khắc phục.
FR24: Hệ thống có thể bảo toàn cấu hình hiện có ngay cả khi một lần đồng bộ thất bại.
FR25: Người dùng có thể sử dụng toàn bộ chức năng cốt lõi của ứng dụng trên Windows 11 mà không cần tài khoản trực tuyến hoặc kết nối Internet.
FR26: Người dùng có thể mở lại ứng dụng sau khi khởi động lại máy hoặc khởi động lại ứng dụng và tiếp tục dùng các cấu hình đã lưu mà không cần thiết lập lại.

### NonFunctional Requirements

NFR1: Ứng dụng phải mở và hiển thị trạng thái cấu hình hiện có trong vòng 3 giây trên môi trường Windows 11 mục tiêu với cấu hình người dùng thông thường.
NFR2: Ứng dụng phải cho phép người dùng hoàn tất luồng tạo cấu hình đầu tiên trong dưới 3 phút nếu đã biết hai thư mục cần chọn.
NFR3: Hệ thống phải ghi nhận và hiển thị kết quả lần đồng bộ gần nhất trong vòng 5 giây sau khi job kết thúc.
NFR4: Hệ thống phải đạt tỷ lệ job đồng bộ thành công tối thiểu 95% trong nhóm thử nghiệm mục tiêu với các use case chuẩn đã xác định.
NFR5: Hệ thống không được xóa hoặc ghi đè dữ liệu ngoài chính sách đồng bộ đã xác định ngay cả khi job thất bại giữa chừng.
NFR6: Hệ thống phải giữ nguyên cấu hình đã lưu sau khi ứng dụng đóng đột ngột hoặc job đồng bộ thất bại.
NFR7: Dữ liệu cấu hình cục bộ phải chỉ đọc được bởi tài khoản Windows đã tạo cấu hình, được kiểm chứng bằng cách xác minh một tài khoản người dùng Windows khác trên cùng máy không thể mở trực tiếp file cấu hình đó.
NFR8: Ứng dụng chỉ được đọc/ghi trong các thư mục mà người dùng đã chủ động chọn trong cấu hình đồng bộ, được kiểm chứng bằng test chức năng xác nhận không có thao tác file nào xảy ra ngoài hai đường dẫn đã cấu hình.
NFR9: Giao diện phải hỗ trợ điều hướng rõ ràng bằng bàn phím cho các thao tác chính như chọn thư mục, chọn lịch và chạy đồng bộ thủ công.
NFR10: Trạng thái thành công, thất bại và cảnh báo phải được thể hiện bằng văn bản rõ ràng, không phụ thuộc duy nhất vào màu sắc.
NFR11: Mỗi job đồng bộ phải tạo bản ghi log có thời gian chạy, kết quả và nguyên nhân lỗi nếu có.
NFR12: Log và trạng thái lỗi phải cho phép người dùng hoặc người hỗ trợ phân biệt chính xác ít nhất 4 nhóm lỗi — thư mục không khả dụng, lỗi quyền truy cập, file đang bị khóa, cấu hình không hợp lệ — và được kiểm chứng bằng bộ test lỗi mẫu cho cả 4 trường hợp.

### Additional Requirements

- Architecture chỉ ra starter template greenfield: Epic 1 Story 1 phải bắt đầu bằng scaffold `npm create tauri-app@latest` với lựa chọn Tauri 2.x + React + TypeScript + Vite.
- Kiến trúc phải tách rõ frontend React/TypeScript và native core Rust thông qua Tauri command bridge; UI không được đọc/ghi file trực tiếp.
- Phải có persistence cục bộ bằng SQLite và migrations versioned cho profiles, jobs, events, settings.
- Phải có sync engine tách lớp scanner/comparer/planner/executor cùng deletion policy rõ ràng.
- Phải có scheduler riêng cho các chu kỳ 30/60/90 phút và 1 ngày, tránh chạy chồng job trên cùng profile.
- Phải có structured logging, error code chuẩn hóa, event model `sync:queued|started|progress|completed|failed` và in-app log viewer.
- Phải có path allowlist/capability boundary để đảm bảo ứng dụng chỉ thao tác trong các thư mục đã cấu hình.
- Phải hỗ trợ packaging Windows installer, CI/build path, và chuẩn bị đường cho auto-update/code-signing sau MVP.
- UX yêu cầu giao diện single-window với panel/navigation rõ ràng cho Dashboard, Profiles, Activity Log, Settings.
- UX yêu cầu Guided Pair Setup, Sync Profile Card, Feedback patterns, Empty states, Loading states, Confirmation dialogs và Log detail view.
- UX yêu cầu tuân thủ Fluent UI/Windows 11 design language, card-based layout, Segoe UI Variable, spacing grid 8px, semantic status colors.
- UX yêu cầu accessibility theo WCAG 2.1 AA: keyboard navigation, visible focus, screen reader support, semantic structure, không truyền đạt trạng thái chỉ bằng màu.
- UX yêu cầu responsive cho desktop-first với các breakpoint 1440+, 1024-1439, 768-1023, <768 và touch target tối thiểu 44x44px.
- UX yêu cầu thông báo lỗi rõ ràng, có thể hành động, đặc biệt cho mất ổ đĩa, thiếu quyền truy cập, file bị khóa và cấu hình không hợp lệ.
- Theo chỉ đạo bổ sung của stakeholder: các epic phải nhóm theo miền triển khai backend, frontend, mobile và có epic tích hợp giữa frontend/backend/mobile/3rd parties.

### FR Coverage Map

FR1: Epic 1 - Tạo hồ sơ đồng bộ mới
FR2: Epic 1 - Chỉnh sửa hồ sơ đồng bộ đã lưu
FR3: Epic 1 - Lưu cấu hình để tái sử dụng
FR4: Epic 1 - Xem cấu hình profile hiện tại
FR5: Epic 1 - Ngăn lưu cấu hình không hợp lệ
FR6: Epic 1 - Chọn chu kỳ đồng bộ tự động
FR7: Epic 2 - Thực thi job đồng bộ theo lịch
FR8: Epic 1 - Kích hoạt hoặc vô hiệu hóa lịch đồng bộ
FR9: Epic 2 - Khởi chạy đồng bộ thủ công
FR10: Epic 2 - Đồng bộ hai chiều toàn bộ file
FR11: Epic 2 - Đồng bộ hai chiều toàn bộ thư mục con
FR12: Epic 2 - Phát hiện và đồng bộ file mới
FR13: Epic 2 - Phát hiện và đồng bộ file đã thay đổi
FR14: Epic 2 - Phát hiện và đồng bộ thư mục con mới
FR15: Epic 2 - Xử lý sự kiện xóa file theo chính sách nhất quán
FR16: Epic 3 - Hiển thị thời điểm chạy gần nhất
FR17: Epic 3 - Hiển thị kết quả lần đồng bộ gần nhất
FR18: Epic 3 - Hiển thị lỗi cơ bản khi đồng bộ thất bại
FR19: Epic 3 - Ghi log cho mỗi lần đồng bộ
FR20: Epic 3 - Cung cấp log đủ để chẩn đoán lỗi phổ biến
FR21: Epic 4 - Phát hiện thư mục nguồn hoặc đích không khả dụng
FR22: Epic 4 - Dừng hoặc bỏ qua job rủi ro an toàn
FR23: Epic 4 - Cho phép chạy lại sau khi khắc phục lỗi
FR24: Epic 4 - Bảo toàn cấu hình sau thất bại
FR25: Epic 5 - Cung cấp toàn bộ chức năng cốt lõi ở chế độ offline
FR26: Epic 1 - Tiếp tục dùng cấu hình đã lưu sau khi khởi động lại

## Epic List

### Epic 1: Thiết lập hồ sơ đồng bộ đầu tiên và sẵn sàng sử dụng
Người dùng có thể tạo, chỉnh sửa, lưu và quản lý một hồ sơ đồng bộ hợp lệ để bắt đầu dùng ứng dụng nhanh và an toàn.
**FRs covered:** FR1, FR2, FR3, FR4, FR5, FR6, FR8, FR26

### Epic 2: Đồng bộ hai chiều và chạy theo lịch đáng tin cậy
Người dùng có thể bật đồng bộ tự động hoặc chạy thủ công để giữ hai thư mục luôn nhất quán theo lịch đã chọn.
**FRs covered:** FR7, FR9, FR10, FR11, FR12, FR13, FR14, FR15

### Epic 3: Minh bạch trạng thái, kết quả và lịch sử đồng bộ
Người dùng có thể biết ngay điều gì đã xảy ra, lần chạy gần nhất ra sao, và đọc log đủ rõ để tin tưởng hệ thống.
**FRs covered:** FR16, FR17, FR18, FR19, FR20

### Epic 4: Xử lý lỗi an toàn và phục hồi sau sự cố môi trường
Người dùng được bảo vệ khi thư mục không khả dụng, lỗi quyền truy cập, file bị khóa hoặc job thất bại, và có thể phục hồi mà không mất cấu hình.
**FRs covered:** FR21, FR22, FR23, FR24

### Epic 5: Trải nghiệm desktop Windows 11 hoàn chỉnh, accessible, và sẵn sàng phát hành
Người dùng có được trải nghiệm desktop nhất quán, dễ dùng, hỗ trợ accessibility, và hệ thống sẵn sàng để team triển khai, kiểm thử và phát hành.
**FRs covered:** FR25

## Epic 1: Thiết lập hồ sơ đồng bộ đầu tiên và sẵn sàng sử dụng

Người dùng có thể tạo, chỉnh sửa, lưu và quản lý một hồ sơ đồng bộ hợp lệ để bắt đầu dùng ứng dụng nhanh và an toàn.

### Story 1.1: Khởi tạo ứng dụng từ starter template và nền tảng lưu trữ cục bộ

As a developer,
I want khởi tạo ứng dụng từ starter template Tauri 2.x với nền tảng persistence cục bộ cơ bản,
So that các story sau có thể phát triển trên một khung dự án thống nhất và chạy được trên Windows 11.

**Implements:** Additional architecture requirement (starter template), FR25, FR26

**Acceptance Criteria:**

**Given** repository chưa có ứng dụng runtime
**When** dự án được scaffold bằng `npm create tauri-app@latest` với React + TypeScript + Vite
**Then** ứng dụng desktop có thể chạy ở môi trường dev trên Windows 11
**And** cấu trúc frontend `src/` và native `src-tauri/` được tách riêng theo architecture

**Given** ứng dụng đã được scaffold
**When** persistence foundation được thêm vào
**Then** ứng dụng có SQLite local database, migration đầu tiên, và app settings store cơ bản
**And** dữ liệu được lưu trong app data của người dùng hiện tại

### Story 1.2: Tạo hồ sơ đồng bộ mới bằng chọn thư mục nguồn và đích

As a người dùng Windows 11,
I want chọn thư mục nguồn và thư mục đích bằng folder picker native,
So that tôi có thể thiết lập hồ sơ đồng bộ đầu tiên một cách nhanh và rõ ràng.

**Implements:** FR1, FR5

**Acceptance Criteria:**

**Given** người dùng đang ở màn hình tạo hồ sơ mới
**When** họ bấm chọn thư mục nguồn hoặc thư mục đích
**Then** ứng dụng mở folder picker native của Windows
**And** đường dẫn đã chọn được hiển thị rõ trên form

**Given** người dùng chưa chọn đủ cả hai thư mục
**When** họ cố lưu hồ sơ
**Then** hệ thống chặn thao tác lưu
**And** hiển thị lỗi dễ hiểu chỉ ra trường còn thiếu hoặc không hợp lệ

### Story 1.3: Lưu và khôi phục hồ sơ đồng bộ đã cấu hình

As a người dùng thường xuyên,
I want lưu hồ sơ đồng bộ và mở lại được ở lần dùng sau,
So that tôi không phải thiết lập lại mỗi khi khởi động ứng dụng.

**Implements:** FR3, FR4, FR26, NFR6

**Acceptance Criteria:**

**Given** người dùng đã nhập cấu hình hợp lệ cho một hồ sơ
**When** họ lưu cấu hình
**Then** hồ sơ được ghi xuống local persistence thành công
**And** hồ sơ xuất hiện lại sau khi đóng và mở lại ứng dụng

**Given** ứng dụng khởi động với dữ liệu đã lưu
**When** màn hình chính được tải
**Then** người dùng nhìn thấy thông tin cặp thư mục và lịch chạy hiện tại
**And** thời gian mở ứng dụng vẫn đáp ứng mục tiêu hiển thị trạng thái nhanh của NFR1

### Story 1.4: Chỉnh sửa hồ sơ và cấu hình lịch đồng bộ

As a người dùng quản trị cấu hình,
I want chỉnh sửa thư mục hoặc lịch chạy của hồ sơ hiện có,
So that tôi có thể cập nhật cách đồng bộ khi nhu cầu thay đổi.

**Implements:** FR2, FR6, FR8

**Acceptance Criteria:**

**Given** người dùng đang xem một hồ sơ đã lưu
**When** họ thay đổi thư mục hoặc chọn một trong bốn chu kỳ 30/60/90 phút hoặc 1 ngày
**Then** ứng dụng cho phép lưu thay đổi hợp lệ
**And** lịch mới được phản ánh ở giao diện hồ sơ

**Given** hồ sơ đã có lịch chạy
**When** người dùng bật hoặc tắt auto sync
**Then** trạng thái kích hoạt của lịch được lưu nhất quán
**And** ứng dụng hiển thị rõ hồ sơ đang active hay paused

## Epic 2: Đồng bộ hai chiều và chạy theo lịch đáng tin cậy

Người dùng có thể bật đồng bộ tự động hoặc chạy thủ công để giữ hai thư mục luôn nhất quán theo lịch đã chọn.

### Story 2.1: Khởi chạy đồng bộ thủ công cho một hồ sơ đã lưu

As a người dùng muốn kiểm tra ngay,
I want chạy đồng bộ thủ công ngoài lịch định kỳ,
So that tôi có thể chủ động đồng bộ khi vừa tạo hoặc sửa file quan trọng.

**Implements:** FR9, FR16

**Acceptance Criteria:**

**Given** hồ sơ đồng bộ hợp lệ đã tồn tại
**When** người dùng bấm “Run sync now”
**Then** hệ thống tạo một sync job mới cho hồ sơ đó
**And** giao diện cập nhật trạng thái là job đang được xử lý

**Given** một sync job thủ công hoàn thành
**When** kết quả được ghi nhận
**Then** thời điểm chạy gần nhất được cập nhật trong giao diện
**And** người dùng có thể phân biệt lần chạy mới nhất với các lần trước

### Story 2.2: Thực thi đồng bộ theo lịch với cơ chế chống chạy chồng

As a người dùng muốn “set it and forget it”,
I want hệ thống tự chạy đồng bộ theo lịch đã cấu hình,
So that hai thư mục luôn được giữ nhất quán mà không cần thao tác tay liên tục.

**Implements:** FR7, FR8, NFR4

**Acceptance Criteria:**

**Given** hồ sơ được bật auto sync với một chu kỳ hợp lệ
**When** đến thời điểm chạy lịch
**Then** scheduler khởi tạo job tự động cho hồ sơ
**And** job được đánh dấu là thuộc lần chạy định kỳ

**Given** một job của cùng hồ sơ đang chạy
**When** lịch kế tiếp đến hạn
**Then** hệ thống không tạo job chồng lấn gây rủi ro trạng thái
**And** sự kiện skip hoặc defer được ghi nhận vào log/job state

### Story 2.3: Đồng bộ file và thư mục con mới giữa hai phía

As a người dùng làm việc giữa hai vị trí lưu trữ,
I want file mới và thư mục con mới xuất hiện ở một bên được sao chép sang bên còn lại,
So that hai thư mục luôn có cùng cấu trúc và dữ liệu mới.

**Implements:** FR10, FR11, FR12, FR14

**Acceptance Criteria:**

**Given** một file mới hoặc thư mục con mới xuất hiện ở một phía của hồ sơ
**When** job đồng bộ chạy
**Then** hệ thống phát hiện phần tử mới và sao chép sang phía còn lại
**And** cấu trúc thư mục được tạo đúng trước khi sao chép file con

**Given** quá trình sao chép thành công
**When** job hoàn tất
**Then** cả hai phía đều chứa cùng file hoặc thư mục mới
**And** log lưu số lượng item đã được thêm mới

### Story 2.4: Đồng bộ thay đổi nội dung file hai chiều

As a người dùng thường xuyên chỉnh sửa tài liệu,
I want các file đã thay đổi ở một bên được cập nhật sang bên còn lại,
So that dữ liệu luôn phản ánh phiên bản mới nhất theo chính sách đồng bộ đã định.

**Implements:** FR10, FR13, NFR5

**Acceptance Criteria:**

**Given** một file đã tồn tại ở cả hai bên nhưng chỉ một phía thay đổi hợp lệ
**When** job đồng bộ chạy
**Then** hệ thống phát hiện file thay đổi và cập nhật sang phía còn lại
**And** metadata của job ghi nhận file được cập nhật thay vì tạo mới

**Given** việc cập nhật gặp lỗi giữa chừng
**When** job kết thúc thất bại
**Then** hệ thống không để lại trạng thái ghi đè ngoài chính sách đồng bộ đã định
**And** lỗi được phân loại để hỗ trợ chẩn đoán

### Story 2.5: Áp dụng chính sách xóa file nhất quán và an toàn

As a người dùng cần tin tưởng dữ liệu,
I want sự kiện xóa file được xử lý theo một chính sách rõ ràng và nhất quán,
So that tôi không gặp hành vi bất ngờ khi hệ thống đồng bộ hai chiều.

**Implements:** FR15, NFR5

**Acceptance Criteria:**

**Given** một file đã bị xóa ở một bên theo tình huống được chính sách hỗ trợ
**When** job đồng bộ chạy
**Then** sync planner áp dụng đúng deletion policy đã tài liệu hóa
**And** log ghi rõ file nào bị xử lý theo chính sách xóa

**Given** dữ liệu đầu vào của job ở trạng thái không an toàn để xử lý xóa
**When** hệ thống đánh giá rủi ro cao
**Then** job không được tiếp tục áp dụng thao tác xóa mơ hồ
**And** người dùng nhận được trạng thái thất bại hoặc skipped có giải thích rõ

## Epic 3: Minh bạch trạng thái, kết quả và lịch sử đồng bộ

Người dùng có thể biết ngay điều gì đã xảy ra, lần chạy gần nhất ra sao, và đọc log đủ rõ để tin tưởng hệ thống.

### Story 3.1: Hiển thị dashboard trạng thái hồ sơ và lần chạy gần nhất

As a người dùng muốn kiểm tra nhanh,
I want nhìn thấy trạng thái hồ sơ, lần chạy gần nhất và lịch hiện tại ngay trên dashboard,
So that tôi biết ứng dụng có đang hoạt động bình thường hay không.

**Implements:** FR16, FR17, NFR1, NFR3

**Acceptance Criteria:**

**Given** ứng dụng mở với ít nhất một hồ sơ đã lưu
**When** dashboard được hiển thị
**Then** mỗi hồ sơ hiển thị trạng thái hiện tại, thời điểm chạy gần nhất và lịch chạy
**And** trạng thái thành công/thất bại được thể hiện bằng văn bản rõ ràng

**Given** một job vừa hoàn tất
**When** kết quả được phát sự kiện từ native layer
**Then** giao diện cập nhật trong giới hạn thời gian của NFR3
**And** người dùng không cần khởi động lại app để thấy trạng thái mới

### Story 3.2: Đồng bộ sự kiện runtime từ native core lên giao diện

As a người dùng đang theo dõi tiến trình,
I want giao diện nhận trạng thái job trực tiếp từ native core,
So that tôi thấy tiến trình và kết quả nhất quán với thực tế hệ thống.

**Implements:** FR17, FR19

**Acceptance Criteria:**

**Given** một sync job thay đổi trạng thái trong native layer
**When** các event `sync:queued`, `sync:started`, `sync:progress`, `sync:completed` hoặc `sync:failed` được phát
**Then** frontend nhận và ánh xạ đúng payload sang trạng thái UI
**And** không có thao tác suy diễn trạng thái mâu thuẫn từ nhiều nguồn khác nhau

**Given** event payload thiếu dữ liệu bắt buộc
**When** frontend xử lý event
**Then** lỗi được ghi nhận an toàn thay vì làm hỏng giao diện
**And** trạng thái cuối cùng vẫn có thể được refetch từ source of truth

### Story 3.3: Ghi log có cấu trúc cho mọi sync job

As a người dùng hoặc người hỗ trợ,
I want mỗi lần đồng bộ đều có log thời gian chạy, kết quả và nguyên nhân lỗi,
So that tôi có đủ dữ liệu để kiểm tra điều gì đã xảy ra.

**Implements:** FR19, NFR11

**Acceptance Criteria:**

**Given** một sync job bắt đầu và kết thúc
**When** hệ thống ghi log
**Then** log bao gồm tối thiểu profileId hoặc jobId, timestamp, trạng thái kết thúc và summary kết quả
**And** khi có lỗi thì log chứa code và message phù hợp

**Given** nhiều job đã chạy qua thời gian
**When** người dùng truy cập lịch sử
**Then** hệ thống có thể truy xuất log theo từng job
**And** dữ liệu log không phụ thuộc vào việc giao diện đang mở hay đóng

### Story 3.4: Hiển thị lịch sử và chi tiết log dễ đọc để chẩn đoán

As a người dùng gặp sự cố,
I want xem lịch sử job và log chi tiết theo ngôn ngữ dễ hiểu,
So that tôi có thể tự xác định nguyên nhân lỗi phổ biến mà không cần đoán.

**Implements:** FR18, FR20, NFR12

**Acceptance Criteria:**

**Given** người dùng mở màn hình Activity Log hoặc chi tiết hồ sơ
**When** họ chọn một job đã chạy
**Then** ứng dụng hiển thị log chi tiết theo từng job với lỗi và ngữ cảnh dễ hiểu
**And** người dùng có thể phân biệt ít nhất các nhóm lỗi: thư mục không khả dụng, lỗi quyền truy cập, file bị khóa, cấu hình không hợp lệ

**Given** trạng thái job là failed
**When** giao diện hiển thị thông tin lỗi
**Then** thông báo lỗi có gợi ý hành động kế tiếp phù hợp
**And** thông tin quan trọng không được biểu diễn chỉ bằng màu sắc

## Epic 4: Xử lý lỗi an toàn và phục hồi sau sự cố môi trường

Người dùng được bảo vệ khi thư mục không khả dụng, lỗi quyền truy cập, file bị khóa hoặc job thất bại, và có thể phục hồi mà không mất cấu hình.

### Story 4.1: Phát hiện sớm thư mục không khả dụng trước khi thực thi job

As a người dùng sử dụng ổ ngoài hoặc thư mục có thể gián đoạn,
I want hệ thống kiểm tra tính sẵn sàng của thư mục trước khi đồng bộ,
So that ứng dụng không thực hiện các thao tác rủi ro khi nguồn hoặc đích không tồn tại.

**Implements:** FR21, FR22

**Acceptance Criteria:**

**Given** một job sắp chạy cho hồ sơ đã lưu
**When** hệ thống kiểm tra source và destination path trước khi sync
**Then** job bị dừng hoặc skipped nếu một trong hai thư mục không khả dụng
**And** trạng thái lỗi nêu rõ thư mục nào gặp vấn đề

**Given** ổ đĩa ngoài bị tháo hoặc đổi trạng thái giữa các lần chạy
**When** đến chu kỳ đồng bộ tiếp theo
**Then** ứng dụng không được cố đọc hoặc ghi dữ liệu mơ hồ
**And** log ghi lại loại lỗi availability tương ứng

### Story 4.2: Phân loại lỗi truy cập, khóa file và cấu hình không hợp lệ

As a người dùng cần được hướng dẫn rõ,
I want lỗi được phân loại thành các nhóm có thể hành động,
So that tôi biết mình cần xử lý quyền truy cập, file đang mở hay cấu hình sai.

**Implements:** FR18, FR20, FR22, NFR12

**Acceptance Criteria:**

**Given** job đồng bộ gặp lỗi runtime
**When** native layer chuẩn hóa lỗi
**Then** lỗi được ánh xạ về ít nhất các nhóm access, availability, lock và validation
**And** mỗi lỗi có code, message và cờ retryable khi phù hợp

**Given** frontend nhận lỗi đã phân loại
**When** hiển thị thông báo cho người dùng
**Then** UI dùng ngôn ngữ dễ hiểu thay cho thuật ngữ kỹ thuật thuần túy
**And** log chi tiết vẫn giữ được diagnostic raw cần thiết cho support

### Story 4.3: Cho phép phục hồi và chạy lại sau khi khắc phục lỗi môi trường

As a người dùng sau khi đã sửa sự cố,
I want chạy lại job bị lỗi mà không cần cấu hình lại hồ sơ,
So that tôi có thể phục hồi nhanh và tiếp tục tin tưởng ứng dụng.

**Implements:** FR23, FR24

**Acceptance Criteria:**

**Given** một hồ sơ đã từng thất bại do lỗi môi trường
**When** người dùng khắc phục nguyên nhân và bấm chạy lại
**Then** job mới có thể được tạo và thực thi bình thường
**And** hồ sơ cấu hình trước đó vẫn được giữ nguyên

**Given** job thất bại bất ngờ hoặc ứng dụng đóng đột ngột
**When** người dùng mở lại ứng dụng
**Then** cấu hình đồng bộ đã lưu vẫn còn nguyên
**And** trạng thái lịch sử vẫn cho phép người dùng hiểu lần thất bại gần nhất

### Story 4.4: Bảo vệ boundary truy cập file theo path allowlist

As a người dùng quan tâm an toàn dữ liệu,
I want ứng dụng chỉ đọc và ghi trong các thư mục tôi đã cấu hình,
So that hệ thống không tác động ngoài phạm vi tôi cho phép.

**Implements:** NFR7, NFR8

**Acceptance Criteria:**

**Given** một command hoặc sync operation yêu cầu truy cập file system
**When** native layer kiểm tra đường dẫn mục tiêu
**Then** thao tác chỉ được phép tiếp tục nếu đường dẫn nằm trong allowlist của hồ sơ
**And** mọi truy cập ngoài phạm vi bị chặn và ghi log như lỗi bảo mật hoặc validation

**Given** dữ liệu local persistence được lưu trên máy
**When** ứng dụng ghi cấu hình xuống đĩa
**Then** file dữ liệu được lưu trong vùng app data của người dùng hiện tại
**And** thiết kế lưu trữ hỗ trợ mục tiêu chỉ tài khoản tạo cấu hình mới đọc trực tiếp được dữ liệu đó

## Epic 5: Trải nghiệm desktop Windows 11 hoàn chỉnh, accessible, và sẵn sàng phát hành

Người dùng có được trải nghiệm desktop nhất quán, dễ dùng, hỗ trợ accessibility, và hệ thống sẵn sàng để team triển khai, kiểm thử và phát hành.

### Story 5.1: Xây application shell Windows 11 với điều hướng rõ ràng

As a người dùng mới,
I want một application shell rõ ràng với Dashboard, Profiles, Activity Log và Settings,
So that tôi luôn biết mình đang ở đâu và tìm thấy tác vụ cần làm nhanh chóng.

**Implements:** FR25, UX navigation requirements

**Acceptance Criteria:**

**Given** người dùng mở ứng dụng
**When** app shell được render
**Then** giao diện có cấu trúc single-window với điều hướng rõ ràng giữa các panel chính
**And** bố cục tuân thủ định hướng Windows 11/Fluent đã chốt trong UX

**Given** người dùng chuyển giữa các khu vực chính
**When** họ dùng chuột hoặc bàn phím
**Then** mục active được thể hiện rõ ràng
**And** điều hướng không làm mất trạng thái quan trọng của hồ sơ đang xem

### Story 5.2: Hoàn thiện luồng onboarding và các trạng thái giao diện cốt lõi

As a người dùng lần đầu,
I want onboarding đơn giản cùng empty state, loading state và feedback state rõ ràng,
So that tôi có thể hiểu ứng dụng ngay cả trước khi có dữ liệu hoặc khi hệ thống đang xử lý.

**Implements:** NFR2, UX Pair Setup/Empty State/Loading State patterns

**Acceptance Criteria:**

**Given** ứng dụng chưa có hồ sơ nào
**When** người dùng mở app lần đầu
**Then** giao diện hiển thị empty state thân thiện với CTA rõ ràng để tạo hồ sơ đầu tiên
**And** luồng onboarding hỗ trợ hoàn tất trong mục tiêu dưới 3 phút nếu người dùng đã biết hai thư mục

**Given** ứng dụng đang tải dữ liệu hoặc thực hiện thao tác dài
**When** người dùng chờ kết quả
**Then** loading state hiển thị rõ mà không gây cảm giác app bị treo
**And** success, warning, error feedback tuân thủ hierarchy và copywriting đã chốt trong UX

### Story 5.3: Đảm bảo accessibility và keyboard-first cho các thao tác chính

As a người dùng cần khả năng tiếp cận tốt,
I want điều hướng và thao tác chính hoạt động tốt bằng bàn phím và screen reader,
So that tôi có thể sử dụng ứng dụng mà không phụ thuộc vào chuột hoặc màu sắc.

**Implements:** NFR9, NFR10, UX accessibility requirements

**Acceptance Criteria:**

**Given** người dùng chỉ dùng bàn phím
**When** họ đi qua các thao tác chính như chọn thư mục, chọn lịch, bật/tắt sync và chạy thủ công
**Then** toàn bộ thao tác có thể hoàn thành bằng keyboard navigation hợp lệ
**And** focus visible luôn rõ ràng trên các control tương tác

**Given** người dùng dùng screen reader hoặc cần hỗ trợ tương phản
**When** giao diện hiển thị trạng thái và form controls
**Then** semantic structure, labels và aria attributes phù hợp được cung cấp
**And** trạng thái thành công/thất bại/cảnh báo không được truyền đạt chỉ bằng màu sắc

### Story 5.4: Tích hợp build, packaging và smoke-readiness cho phát hành Windows

As a team triển khai,
I want pipeline build và packaging cơ bản cho Windows hoạt động ổn định,
So that sản phẩm có thể được kiểm thử và phát hành nhất quán.

**Implements:** Additional architecture requirement for packaging/release readiness

**Acceptance Criteria:**

**Given** codebase đã có frontend và native core chạy được trong dev
**When** team chạy build production
**Then** frontend build và Tauri build tạo được artifact Windows hợp lệ
**And** cấu hình dự án không vi phạm boundary giữa UI, command bridge và native services

**Given** artifact build được tạo ra
**When** team thực hiện smoke test cài đặt và mở ứng dụng
**Then** ứng dụng khởi chạy được, tải được dữ liệu local cơ bản và hiển thị app shell đúng
**And** đường cho CI, signing hoặc auto-update sau MVP không bị chặn bởi quyết định hiện tại
