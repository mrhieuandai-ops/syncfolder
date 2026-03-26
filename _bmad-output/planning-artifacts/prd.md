---
stepsCompleted:
  - step-01-init
  - step-02-discovery
  - step-02b-vision
  - step-02c-executive-summary
  - step-03-success
  - step-04-journeys
  - step-05-domain
  - step-06-innovation
  - step-07-project-type
  - step-08-scoping
  - step-09-functional
  - step-10-nonfunctional
  - step-11-polish
  - step-12-complete
inputDocuments:
  - /mnt/d/ai_project/syncfolder/_bmad-output/planning-artifacts/product-brief-syncfolder-2026-03-25.md
workflowType: 'prd'
documentCounts:
  briefCount: 1
  researchCount: 0
  brainstormingCount: 0
  projectDocsCount: 0
classification:
  projectType: desktop_app
  domain: general
  complexity: low
  projectContext: greenfield
date: '2026-03-25'
---

# Product Requirements Document - syncfolder

**Author:** hieu
**Date:** 2026-03-25

## Executive Summary

SyncFolder là ứng dụng desktop native cho Windows 11, được xây dựng để giải quyết một nhu cầu rất cụ thể nhưng lặp lại hàng ngày: giữ hai thư mục luôn đồng bộ hai chiều một cách tự động, dễ hiểu và đáng tin cậy. Sản phẩm phục vụ người dùng cá nhân nâng cao, freelancer và SMB nhỏ thường xuyên làm việc giữa máy chính, ổ cứng ngoài hoặc thư mục cục bộ/chia sẻ nội bộ. Giá trị cốt lõi không nằm ở việc cung cấp mọi chế độ đồng bộ phức tạp, mà ở việc giảm thao tác thủ công, giảm lỗi người dùng và tăng niềm tin rằng dữ liệu ở cả hai vị trí luôn nhất quán.

### Core Problem Statement

Người dùng Windows hiện phải copy file thủ công hoặc dựa vào công cụ sync khó dùng để giữ hai thư mục luôn nhất quán. Hệ quả là mất thời gian, dễ quên đồng bộ, khó phát hiện sai lệch và tăng rủi ro ghi đè hoặc mất dữ liệu khi thao tác sai.

### What Makes This Special

Khác với nhiều công cụ sync hiện có vốn mạnh nhưng khó tiếp cận, SyncFolder được định vị là giải pháp Windows 11 local-first với GUI tối giản, quy trình thiết lập ngắn và lịch đồng bộ rõ ràng. Điểm khác biệt chính là tối ưu cho bài toán “chọn 2 thư mục và tin tưởng app chạy ổn định” thay vì buộc người dùng phải hiểu một ma trận tùy chọn kỹ thuật. Insight trung tâm là người dùng trong phân khúc này sẵn sàng đánh đổi bớt chiều sâu tính năng để lấy sự rõ ràng, an toàn và dễ dùng.

## Project Classification

- **Project Type:** desktop_app
- **Domain:** general
- **Complexity:** low
- **Project Context:** greenfield

## Success Criteria

### User Success

- Người dùng cấu hình thành công một cặp thư mục đồng bộ đầu tiên trong dưới 3 phút mà không cần tài liệu hỗ trợ.
- Người dùng hiểu ngay trạng thái hệ thống: thư mục nào đang được đồng bộ, lịch hiện tại là gì, lần chạy gần nhất thành công hay thất bại.
- Người dùng cảm nhận giá trị rõ ràng khi file tạo mới/chỉnh sửa ở một bên xuất hiện chính xác ở bên còn lại mà không cần thao tác thủ công.
- Người dùng đủ tin tưởng để bật chế độ tự động và duy trì sử dụng thường xuyên trong workflow hàng ngày.

### Business Success

- Xác thực nhu cầu thực cho phân khúc Windows local-first sync GUI thông qua nhóm người dùng đầu tiên có hành vi sử dụng lặp lại.
- Chứng minh rằng một sản phẩm đơn giản hơn FreeFileSync/GoodSync/SyncBack vẫn có thể tạo khác biệt bằng UX và độ tin cậy.
- Tạo nền tảng sản phẩm đủ tốt để quyết định hướng phát triển tiếp theo: giữ bản free, mở rộng Pro, hoặc bổ sung tính năng safety nâng cao.

### Technical Success

- Hoàn thành đồng bộ 2 chiều ổn định cho các tình huống cơ bản: file mới, file sửa đổi, thư mục con mới, xóa file.
- Sync job thành công với tỷ lệ mục tiêu >= 95% trong môi trường sử dụng chuẩn.
- Ứng dụng lưu và nạp lại cấu hình ổn định giữa các lần khởi động.
- Hệ thống có log đủ chi tiết để chẩn đoán lỗi người dùng phổ biến như mất quyền truy cập, đường dẫn không tồn tại, thư mục đích không khả dụng.

### Measurable Outcomes

- Tỷ lệ hoàn tất onboarding lần đầu >= 80%.
- Tỷ lệ sync job thành công >= 95% trong nhóm beta mục tiêu.
- Tối thiểu 60% người dùng thử nghiệm quay lại dùng profile đã tạo trong vòng 7 ngày.
- Tỷ lệ lỗi nghiêm trọng gây mất niềm tin (xử lý sai xóa/ghi đè) phải tiệm cận 0 trước phát hành rộng hơn.

## Product Scope

### MVP - Minimum Viable Product

- Ứng dụng desktop GUI cho Windows 11.
- Chọn thư mục nguồn và thư mục đích bằng giao diện trực quan.
- Đồng bộ 2 chiều toàn bộ file và thư mục con giữa hai vị trí đã chọn.
- Lên lịch tự động với 4 chu kỳ cố định: 30 phút, 60 phút, 90 phút, 1 ngày.
- Hiển thị trạng thái đồng bộ gần nhất: thời gian, kết quả, lỗi cơ bản nếu có.
- Ghi log cơ bản và lưu cấu hình sync.

### Growth Features (Post-MVP)

- Preview thay đổi trước khi sync.
- Conflict handling rõ ràng khi cùng một file bị thay đổi ở cả hai bên.
- Exclusion patterns cho file/thư mục.
- Nhiều profile sync trong một ứng dụng.
- Cảnh báo xóa hàng loạt và recycle-bin safety.

### Vision (Future)

- Hỗ trợ network share/NAS mạnh hơn.
- Versioning/restore cho các phiên bản file.
- Bộ sản phẩm local-first sync/backup cho cá nhân và SMB nhỏ trên Windows.

## User Journeys

### Journey 1 - Người dùng chính thiết lập thành công lần đầu

Hieu là một freelancer dùng desktop Windows 11 để xử lý tài liệu khách hàng và thường sao chép thủ công sang ổ cứng ngoài vào cuối ngày. Anh cài SyncFolder vì muốn bỏ bước copy lặp lại nhưng vẫn cần cảm giác an toàn. Khi mở ứng dụng, anh thấy giao diện gọn: chọn thư mục A, chọn thư mục B, chọn chu kỳ đồng bộ. Anh lưu cấu hình, bật lịch 60 phút và chờ lần chạy đầu tiên. Khoảnh khắc giá trị đến khi anh chỉnh sửa một file ở thư mục làm việc và sau đó thấy file tương ứng ở thư mục còn lại đã được cập nhật đúng. Từ đây, SyncFolder trở thành công cụ nền mà anh ít phải nghĩ tới nhưng tin tưởng mỗi ngày.

### Journey 2 - Người dùng chính gặp edge case khi thư mục tạm thời không khả dụng

Lan dùng laptop và một ổ SSD ngoài. Cô đã cấu hình đồng bộ tự động nhưng một lần rút ổ ngoài ra mà quên cắm lại. Đến chu kỳ chạy tiếp theo, ứng dụng không thể đồng bộ. Thay vì im lặng hoặc tạo trạng thái mơ hồ, hệ thống phải hiển thị rõ lần sync thất bại, nguyên nhân thư mục đích không khả dụng và giữ nguyên dữ liệu thay vì cố xử lý rủi ro. Khi cắm lại ổ, Lan chạy lại hoặc đợi chu kỳ kế tiếp và thấy job hoạt động bình thường. Journey này chứng minh sản phẩm không chỉ tốt khi mọi thứ lý tưởng mà còn phải an toàn khi môi trường thực tế không ổn định.

### Journey 3 - Người dùng cấu hình/operations quản lý hồ sơ sync

Minh là IT support cho một văn phòng nhỏ. Anh cần thiết lập một cấu hình đơn giản cho người dùng không kỹ thuật. Anh mở app, chọn hai thư mục cần đồng bộ, đặt lịch 1 ngày và xác nhận trạng thái lưu thành công. Khi người dùng hỏi “app có chạy chưa?”, Minh cần nhìn thấy ngay thông tin lần chạy gần nhất, trạng thái thành công/thất bại và log cơ bản. Giá trị với Minh không nằm ở tính năng cao cấp mà ở khả năng triển khai nhanh, giải thích dễ và giảm số lần bị gọi hỗ trợ.

### Journey 4 - Người dùng support/tự khắc phục sự cố bằng log

Sau một tuần sử dụng, một người dùng thấy một vài file chưa xuất hiện ở thư mục đích. Họ mở ứng dụng và xem log để biết job nào đã chạy, file nào lỗi và lỗi thuộc loại gì: quyền truy cập, file đang bị khóa hay thư mục không tồn tại. Nếu ứng dụng chỉ báo “sync failed”, người dùng sẽ mất niềm tin rất nhanh; nhưng nếu log đủ rõ để khoanh vùng vấn đề, họ vẫn tin công cụ đang kiểm soát được tình huống. Journey này tạo ra yêu cầu rõ ràng về quan sát hệ thống, khả năng chẩn đoán và thông điệp lỗi có thể hành động.

### Journey Requirements Summary

- Cần onboarding cực ngắn cho cấu hình cặp thư mục đầu tiên.
- Cần cơ chế lưu profile và lịch chạy rõ ràng.
- Cần hiển thị trạng thái lần đồng bộ gần nhất, bao gồm thành công/thất bại và thời điểm chạy.
- Cần xử lý lỗi an toàn khi thư mục, ổ đĩa hoặc quyền truy cập không khả dụng.
- Cần log cơ bản nhưng dễ đọc để người dùng và người hỗ trợ chẩn đoán vấn đề.
- Cần trạng thái hệ thống minh bạch để xây dựng niềm tin dài hạn.
- Nhu cầu chỉnh sửa cấu hình và xem lại cặp thư mục đang chạy xuất phát từ Journey 1 và Journey 3.
- Nhu cầu tiếp tục sử dụng app sau khi khởi động lại xuất phát từ Journey 1 và mục tiêu vận hành ổn định hằng ngày.

## Desktop App Specific Requirements

### Project-Type Overview

SyncFolder là ứng dụng desktop native tập trung cho Windows 11, ưu tiên hoạt động ổn định trong môi trường offline/local-first. Đây không phải web app bọc shell; trải nghiệm yêu cầu tích hợp tốt với hệ thống file của Windows, folder picker, lịch chạy nền và trạng thái ứng dụng rõ ràng.

### Technical Architecture Considerations

- Ứng dụng phải tương tác an toàn với file system của Windows cho thao tác quét thư mục, so sánh thay đổi và sao chép hai chiều.
- Cần cơ chế scheduler phù hợp cho các chu kỳ 30/60/90 phút và 1 ngày.
- Cần lưu cấu hình cục bộ bền vững giữa các lần khởi động.
- Cần kiến trúc đủ rõ để tách biệt UI, engine đồng bộ và lớp ghi log/trạng thái.

### Platform Support

- Nền tảng mục tiêu cho MVP: Windows 11.
- Không yêu cầu cross-platform ở giai đoạn đầu.
- Cần hỗ trợ đường dẫn thư mục cục bộ và nên thiết kế để có thể mở rộng cho external drive hoặc network share ở giai đoạn sau.

### System Integration

- Tích hợp với folder picker của hệ điều hành.
- Hỗ trợ chạy lịch đồng bộ định kỳ mà không buộc người dùng thao tác tay mỗi lần.
- Cần phản hồi tốt khi thư mục hoặc ổ đĩa bên ngoài tạm thời không sẵn sàng.

### Update Strategy

- MVP có thể phát hành với chiến lược cập nhật thủ công hoặc bán tự động đơn giản.
- Cơ chế cập nhật tự động hoàn chỉnh không phải yêu cầu bắt buộc của MVP nhưng kiến trúc phát hành không nên chặn khả năng bổ sung sau này.

### Offline Capabilities

- Toàn bộ use case cốt lõi phải hoạt động không cần tài khoản và không cần Internet.
- Ứng dụng vẫn phải dùng được đầy đủ khi máy offline vì sản phẩm giải quyết bài toán đồng bộ cục bộ.

### Implementation Considerations

- Ưu tiên độ tin cậy của sync engine hơn số lượng tính năng phụ.
- Cần quy tắc rõ ràng cho phát hiện thay đổi, xử lý xung đột và xử lý lỗi I/O.
- UI phải làm rõ trạng thái hệ thống để giảm lo âu mất dữ liệu.

## Project Scoping & Phased Development

### MVP Strategy & Philosophy

**MVP Approach:** Problem-solving MVP tập trung vào một use case hẹp nhưng có nhu cầu rõ ràng: đồng bộ 2 chiều giữa hai thư mục trên Windows 11 bằng GUI đơn giản, đáng tin và có lịch tự động.

**Resource Requirements:** Đội tối thiểu gồm 1 desktop engineer có kinh nghiệm file system/sync engine, 1 product/QA tổng quát, và năng lực test trên môi trường Windows với ổ cục bộ + ổ ngoài.

### MVP Feature Set (Phase 1)

**Core User Journeys Supported:**
- Thiết lập cặp thư mục đầu tiên và bật lịch tự động.
- Theo dõi trạng thái lần sync gần nhất.
- Phát hiện lỗi cơ bản khi thư mục không khả dụng.
- Kiểm tra log để hiểu nguyên nhân sự cố phổ biến.

**Must-Have Capabilities:**
- GUI cho Windows 11.
- Chọn thư mục nguồn/đích.
- Đồng bộ 2 chiều toàn bộ file/thư mục con.
- Lịch chạy 30/60/90 phút và 1 ngày.
- Lưu cấu hình sync.
- Trạng thái đồng bộ gần nhất.
- Log cơ bản có thể đọc được.

### Post-MVP Features

**Phase 2 (Post-MVP):**
- Preview trước sync.
- Conflict handling có giải thích rõ.
- Exclusion rules.
- Multi-profile.
- Cảnh báo xóa hàng loạt và recycle-bin safety.

**Phase 3 (Expansion):**
- Hỗ trợ NAS/network share tốt hơn.
- Versioning và restore.
- Tăng cường quản trị cho người dùng SMB nhỏ.

### Risk Mitigation Strategy

**Technical Risks:** Sync engine xử lý sai thay đổi/xóa file; giảm rủi ro bằng cách giới hạn scope, kiểm thử kỹ các case cốt lõi và bổ sung logging/trạng thái minh bạch.

**Market Risks:** Người dùng thấy sản phẩm chưa đủ khác biệt với công cụ hiện có; giảm rủi ro bằng cách định vị mạnh vào sự đơn giản, Windows 11-first và time-to-value ngắn.

**Resource Risks:** Nếu nguồn lực hạn chế, trì hoãn preview, exclusions và multi-profile sang giai đoạn sau để bảo vệ độ ổn định của lõi đồng bộ.

## Functional Requirements

### Sync Configuration Management

- FR1: Người dùng có thể tạo một cấu hình đồng bộ mới bằng cách chọn thư mục nguồn và thư mục đích.
- FR2: Người dùng có thể chỉnh sửa cấu hình đồng bộ đã lưu để cập nhật thư mục hoặc lịch chạy của một profile hiện có.
- FR3: Người dùng có thể lưu cấu hình đồng bộ để tái sử dụng ở các lần mở ứng dụng sau.
- FR4: Người dùng có thể xem thông tin cặp thư mục và lịch chạy đang được cấu hình cho profile hiện tại.
- FR5: Hệ thống có thể ngăn người dùng lưu cấu hình không hợp lệ khi thiếu thư mục nguồn hoặc đích.

### Scheduling & Execution

- FR6: Người dùng có thể chọn chu kỳ đồng bộ tự động từ các tùy chọn 30 phút, 60 phút, 90 phút hoặc 1 ngày.
- FR7: Hệ thống có thể thực thi job đồng bộ theo lịch đã cấu hình.
- FR8: Người dùng có thể kích hoạt hoặc vô hiệu hóa lịch đồng bộ của cấu hình hiện tại.
- FR9: Người dùng có thể khởi chạy đồng bộ thủ công ngoài lịch định kỳ.

### Two-Way Synchronization

- FR10: Hệ thống có thể đồng bộ hai chiều toàn bộ file giữa hai thư mục đã chọn.
- FR11: Hệ thống có thể đồng bộ hai chiều toàn bộ thư mục con giữa hai thư mục đã chọn.
- FR12: Hệ thống có thể phát hiện file mới xuất hiện ở một trong hai thư mục và sao chép sang thư mục còn lại.
- FR13: Hệ thống có thể phát hiện file đã thay đổi ở một trong hai thư mục và cập nhật sang thư mục còn lại.
- FR14: Hệ thống có thể phát hiện thư mục con mới và đồng bộ cấu trúc sang phía còn lại.
- FR15: Hệ thống có thể xử lý sự kiện xóa file theo một chính sách đồng bộ hai chiều nhất quán, được tài liệu hóa và áp dụng giống nhau cho mọi job sync.

### Status, Visibility & Logging

- FR16: Người dùng có thể xem thời điểm chạy gần nhất của job đồng bộ.
- FR17: Người dùng có thể xem kết quả của lần đồng bộ gần nhất theo trạng thái thành công hoặc thất bại.
- FR18: Người dùng có thể xem thông báo lỗi cơ bản khi một lần đồng bộ không hoàn thành.
- FR19: Hệ thống có thể ghi log cho mỗi lần đồng bộ.
- FR20: Người dùng có thể xem log đủ để xác định nguyên nhân lỗi phổ biến như đường dẫn không tồn tại, mất quyền truy cập hoặc thư mục không khả dụng.

### Error Handling & Recovery

- FR21: Hệ thống có thể phát hiện khi thư mục nguồn hoặc đích không còn khả dụng tại thời điểm chạy job.
- FR22: Hệ thống có thể dừng hoặc bỏ qua một job đồng bộ khi phát hiện thư mục không khả dụng, lỗi quyền truy cập hoặc trạng thái dữ liệu không xác định thay vì tiếp tục xử lý rủi ro.
- FR23: Người dùng có thể thực hiện lại job đồng bộ sau khi lỗi môi trường đã được khắc phục.
- FR24: Hệ thống có thể bảo toàn cấu hình hiện có ngay cả khi một lần đồng bộ thất bại.

### Application Operation

- FR25: Người dùng có thể sử dụng toàn bộ chức năng cốt lõi của ứng dụng trên Windows 11 mà không cần tài khoản trực tuyến hoặc kết nối Internet.
- FR26: Người dùng có thể mở lại ứng dụng sau khi khởi động lại máy hoặc khởi động lại ứng dụng và tiếp tục dùng các cấu hình đã lưu mà không cần thiết lập lại.

## Non-Functional Requirements

### Performance

- NFR1: Ứng dụng phải mở và hiển thị trạng thái cấu hình hiện có trong vòng 3 giây trên môi trường Windows 11 mục tiêu với cấu hình người dùng thông thường.
- NFR2: Ứng dụng phải cho phép người dùng hoàn tất luồng tạo cấu hình đầu tiên trong dưới 3 phút nếu đã biết hai thư mục cần chọn.
- NFR3: Hệ thống phải ghi nhận và hiển thị kết quả lần đồng bộ gần nhất trong vòng 5 giây sau khi job kết thúc.

### Reliability

- NFR4: Hệ thống phải đạt tỷ lệ job đồng bộ thành công tối thiểu 95% trong nhóm thử nghiệm mục tiêu với các use case chuẩn đã xác định.
- NFR5: Hệ thống không được xóa hoặc ghi đè dữ liệu ngoài chính sách đồng bộ đã xác định ngay cả khi job thất bại giữa chừng.
- NFR6: Hệ thống phải giữ nguyên cấu hình đã lưu sau khi ứng dụng đóng đột ngột hoặc job đồng bộ thất bại.

### Security

- NFR7: Dữ liệu cấu hình cục bộ phải chỉ đọc được bởi tài khoản Windows đã tạo cấu hình, được kiểm chứng bằng cách xác minh một tài khoản người dùng Windows khác trên cùng máy không thể mở trực tiếp file cấu hình đó.
- NFR8: Ứng dụng chỉ được đọc/ghi trong các thư mục mà người dùng đã chủ động chọn trong cấu hình đồng bộ, được kiểm chứng bằng test chức năng xác nhận không có thao tác file nào xảy ra ngoài hai đường dẫn đã cấu hình.

### Accessibility

- NFR9: Giao diện phải hỗ trợ điều hướng rõ ràng bằng bàn phím cho các thao tác chính như chọn thư mục, chọn lịch và chạy đồng bộ thủ công.
- NFR10: Trạng thái thành công, thất bại và cảnh báo phải được thể hiện bằng văn bản rõ ràng, không phụ thuộc duy nhất vào màu sắc.

### Observability & Supportability

- NFR11: Mỗi job đồng bộ phải tạo bản ghi log có thời gian chạy, kết quả và nguyên nhân lỗi nếu có.
- NFR12: Log và trạng thái lỗi phải cho phép người dùng hoặc người hỗ trợ phân biệt chính xác ít nhất 4 nhóm lỗi — thư mục không khả dụng, lỗi quyền truy cập, file đang bị khóa, cấu hình không hợp lệ — và được kiểm chứng bằng bộ test lỗi mẫu cho cả 4 trường hợp.
