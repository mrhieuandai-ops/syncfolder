---
stepsCompleted: [1, 2, 3, 4, 5]
inputDocuments: []
date: 2026-03-25
author: hieu
---

# Product Brief: syncfolder

> Ghi chú: Tài liệu này được khởi tạo theo workflow Product Brief và đã được **tự động điền các phần còn thiếu** dựa trên yêu cầu ban đầu của bạn + phân tích xu hướng thị trường desktop sync 2024–2026. Các nội dung dưới đây nên được xem là **đề xuất bản v1 để review/xác nhận**.

## Executive Summary

SyncFolder là ứng dụng desktop cho Windows 11 với giao diện GUI, giúp người dùng đồng bộ 2 chiều tự động giữa hai thư mục một cách đơn giản, an toàn và dễ hiểu. Sản phẩm tập trung vào nhu cầu rất thực tế: giữ cho dữ liệu ở hai vị trí luôn nhất quán mà không cần thao tác thủ công lặp lại, đặc biệt khi làm việc giữa máy chính, ổ cứng ngoài, thư mục nội bộ hoặc thư mục chia sẻ trong môi trường Windows.

Thị trường hiện đã có các công cụ mạnh như FreeFileSync, GoodSync và SyncBack, nhưng phần lớn hoặc quá nhiều tính năng với người dùng phổ thông, hoặc gây áp lực tâm lý vì rủi ro mất dữ liệu khi cấu hình sai. Cơ hội của SyncFolder là trở thành một “SyncToy hiện đại cho Windows 11”: GUI gọn, thiết lập trong vài phút, đồng bộ định kỳ rõ ràng, và ưu tiên độ tin cậy hơn sự phức tạp.

---

## Core Vision

### Problem Statement

Người dùng Windows thường phải duy trì cùng một bộ file ở hai thư mục khác nhau như: Documents trên máy và ổ cứng ngoài, thư mục làm việc và bản sao lưu cục bộ, hoặc hai thư mục giữa desktop/laptop trong cùng môi trường. Cách làm hiện tại thường là copy thủ công, ghi đè thủ công, hoặc dùng công cụ có giao diện khó hiểu. Điều này dẫn tới lỗi thao tác, quên đồng bộ, phát sinh file xung đột, và mất niềm tin vào công cụ.

### Problem Impact

- Mất thời gian lặp đi lặp lại cho thao tác copy/kiểm tra file.
- Rủi ro bỏ sót file mới, ghi đè nhầm hoặc xóa nhầm dữ liệu.
- Khó đảm bảo hai thư mục luôn cùng trạng thái sau nhiều lần chỉnh sửa.
- Người dùng phổ thông ngại dùng công cụ chuyên sâu vì sợ cấu hình sai.

### Why Existing Solutions Fall Short

- Nhiều đối thủ rất mạnh nhưng UI và workflow thiên về người dùng kỹ thuật.
- Một số công cụ dồn quá nhiều chế độ và tùy chọn ngay từ đầu, tăng tải nhận thức.
- Niềm tin sản phẩm thấp nếu không có preview rõ ràng, log dễ đọc và cảnh báo hành động nguy hiểm.
- Người dùng chỉ cần bài toán “2 thư mục luôn giống nhau” nhưng phải học một bộ tính năng quá rộng.

### Proposed Solution

Xây dựng một ứng dụng Windows 11 có GUI cho phép người dùng:

- Chọn thư mục nguồn và thư mục đích bằng trình chọn thư mục trực quan.
- Thiết lập đồng bộ tự động định kỳ theo các mốc: 30 phút, 60 phút, 90 phút, 1 ngày.
- Đồng bộ 2 chiều toàn bộ file và thư mục giữa hai thư mục đã chọn.
- Chạy đồng bộ ổn định, dễ hiểu, có trạng thái rõ ràng và giảm tối đa thao tác kỹ thuật.

### Key Differentiators

- Chỉ tập trung vào **Windows 11 + local folder sync GUI**, không ôm đồm cloud ngay ở MVP.
- Trải nghiệm thiết lập cực ngắn: chọn 2 thư mục → chọn lịch → bật tự động sync.
- Ưu tiên niềm tin người dùng: dễ hiểu, trạng thái minh bạch, log rõ, cảnh báo khi có rủi ro.
- Định vị là công cụ “đủ mạnh để dùng thật, đủ đơn giản để không sợ dùng”.

## Target Users

### Primary Users

1. **Người dùng cá nhân nâng cao trên Windows 11**
   - Có nhu cầu giữ đồng bộ giữa thư mục làm việc và bản sao trên ổ cứng ngoài/ổ khác.
   - Thường xử lý tài liệu, ảnh, video, file dự án cá nhân.
   - Muốn tự động hóa nhưng không muốn học công cụ backup/sync quá phức tạp.

2. **Freelancer / nhân sự văn phòng / kế toán / studio nhỏ**
   - Làm việc với file cập nhật thường xuyên.
   - Cần một giải pháp “set once, run reliably”.
   - Giá trị lớn nhất là tránh sai sót và tiết kiệm thời gian tác vụ lặp.

3. **Người dùng có nhiều thiết bị hoặc nhiều vị trí lưu trữ cục bộ**
   - Có laptop/desktop/ổ cứng ngoài/NAS share nội bộ.
   - Không nhất thiết cần cloud collaboration, chỉ cần dữ liệu nhất quán giữa hai nơi.

### Secondary Users

- IT support cho doanh nghiệp nhỏ cần thiết lập công cụ đơn giản cho nhân viên.
- Người ra quyết định mua công cụ trong nhóm SMB nhỏ, ưu tiên ổn định và dễ đào tạo.
- Power users muốn một công cụ phụ nhẹ hơn các giải pháp đồng bộ chuyên sâu đang có.

### User Journey

1. **Discovery**: Người dùng nhận ra mình đang copy file thủ công quá nhiều hoặc từng gặp lỗi thiếu file/ghi đè nhầm.
2. **Onboarding**: Mở app, chọn 2 thư mục, chọn chu kỳ sync, xem trạng thái cấu hình.
3. **Core Usage**: Ứng dụng chạy đồng bộ định kỳ, cập nhật thay đổi hai chiều giữa hai thư mục.
4. **Success Moment**: Người dùng thấy file mới/chỉnh sửa xuất hiện đúng ở cả hai nơi mà không cần can thiệp tay.
5. **Long-term Habit**: App trở thành công cụ nền tin cậy cho workflow sao chép và duy trì dữ liệu hàng ngày.

## Success Metrics

### User Success Metrics

- Người dùng hoàn tất cấu hình cặp thư mục đầu tiên trong vòng dưới 3 phút.
- Tỷ lệ sync job chạy thành công không lỗi ở mức >= 95% trong môi trường sử dụng chuẩn.
- Người dùng quay lại sử dụng profile sync đã tạo ít nhất 3 lần/tuần trong nhóm active users.
- Giảm nhu cầu copy file thủ công trong use case mục tiêu.

### Business Objectives

- Xác thực có nhu cầu thật cho một công cụ local-first, Windows-only, GUI đơn giản.
- Đạt nhóm người dùng đầu tiên có mức sử dụng lặp lại và phản hồi tích cực về độ đơn giản/độ tin cậy.
- Tìm ra bộ tính năng tối thiểu đủ cạnh tranh trước khi mở rộng sang versioning nâng cao, network sync hoặc bản trả phí.

### Key Performance Indicators

- Số lượng profile sync được tạo mỗi người dùng.
- Tỷ lệ hoàn tất onboarding lần đầu.
- Tỷ lệ job đồng bộ thành công/thất bại.
- Tỷ lệ người dùng active sau 7 ngày và 30 ngày.
- Số lỗi xung đột hoặc lỗi quyền truy cập trên mỗi 100 job sync.
- Mức độ hài lòng định tính xoay quanh 3 trục: dễ dùng, đáng tin, tiết kiệm thời gian.

## MVP Scope

### Core Features

1. Ứng dụng desktop chạy trên Windows 11 với GUI.
2. Cho phép chọn **thư mục nguồn** và **thư mục đích** bằng folder picker.
3. Đồng bộ **2 chiều** toàn bộ file và thư mục con giữa hai vị trí đã chọn.
4. Lên lịch tự động theo 4 tùy chọn cố định:
   - 30 phút
   - 60 phút
   - 90 phút
   - 1 ngày
5. Hiển thị trạng thái đồng bộ gần nhất: thời gian chạy, kết quả thành công/thất bại.
6. Ghi log cơ bản để người dùng biết app đã làm gì.
7. Lưu cấu hình sync để không phải chọn lại mỗi lần mở app.

### Out of Scope for MVP

- Đồng bộ cloud drive (Google Drive, OneDrive, Dropbox).
- Đồng bộ nhiều cặp thư mục phức tạp trong cùng dashboard doanh nghiệp.
- Realtime sync theo file system event ở mức nâng cao.
- Version history nâng cao, rollback nhiều phiên bản.
- Script automation, CLI nâng cao, rule engine phức tạp.
- Quản trị tập trung cho nhiều máy.

### MVP Success Criteria

- Người dùng mục tiêu có thể thiết lập và chạy được tác vụ đồng bộ đầu tiên mà không cần tài liệu kỹ thuật.
- Ứng dụng xử lý ổn định các tình huống cơ bản: tạo file mới, sửa file, thêm thư mục con, xóa file.
- Người dùng đánh giá công cụ dễ dùng hơn các lựa chọn hiện có trong phân khúc quá phức tạp.
- App tạo được niềm tin đủ mạnh để người dùng bật lịch tự động trong công việc hằng ngày.

### Future Vision

- Thêm preview trước khi sync và cảnh báo khi có thay đổi/xóa số lượng lớn.
- Bổ sung conflict handling rõ ràng hơn khi cùng một file bị sửa ở cả hai nơi.
- Hỗ trợ exclusion patterns, bộ lọc loại file/thư mục.
- Mở rộng hỗ trợ nhiều profile sync.
- Hỗ trợ network share/NAS tốt hơn.
- Bổ sung versioning hoặc recycle-bin safety để tăng độ an tâm.
- Xa hơn có thể phát triển thành bộ công cụ sync/backup local-first cho Windows SMB.

## Phân tích xu hướng thị trường & hàm ý cho sản phẩm

### Xu hướng chính

- Dù cloud phổ biến, nhu cầu **local/offline sync** vẫn mạnh vì tốc độ, quyền riêng tư và độ chủ động.
- Người dùng cá nhân và SMB nhỏ vẫn cần giải pháp giữ dữ liệu nhất quán giữa máy chính và thiết bị lưu trữ phụ.
- Thị trường ưu tiên **độ tin cậy và an toàn dữ liệu** hơn là số lượng tính năng.

### Chuẩn thị trường hiện nay

- Có compare/analyze trước khi sync.
- Có conflict handling, exclusions, scheduler, log.
- Có cơ chế safety khi xóa hàng loạt hoặc ghi đè file.
- Có khả năng phục hồi/khôi phục ở một mức nào đó.

### Cơ hội cho SyncFolder

- Định vị rõ: **Windows 11 only, GUI đơn giản, local-first, 2-folder sync**.
- Không cạnh tranh bằng số tính năng; cạnh tranh bằng **độ dễ dùng + cảm giác an toàn**.
- Có thể trở thành lựa chọn tốt cho nhóm người dùng từng thấy FreeFileSync/GoodSync/SyncBack quá “nặng đầu”.

### Rủi ro sản phẩm

- Chỉ cần một lỗi xóa nhầm hoặc ghi đè sai là mất niềm tin nghiêm trọng.
- Edge cases kỹ thuật nhiều: file đang mở, quyền truy cập, ổ ngoài đổi ký tự ổ đĩa, path dài, clock skew.
- Nếu mở rộng scope quá sớm, sản phẩm sẽ mất lợi thế đơn giản.

## Các đề xuất còn thiếu để bạn review/xác nhận

1. **Tên sản phẩm hiển thị ra thị trường**
   - Đề xuất tạm: `SyncFolder`

2. **Định vị chính xác ở MVP**
   - Đề xuất: “Ứng dụng đồng bộ 2 chiều thư mục cục bộ trên Windows 11, đơn giản và đáng tin cậy cho người dùng cá nhân và SMB nhỏ.”

3. **Mức an toàn mặc định**
   - Đề xuất nên thêm ngay sau MVP sớm: preview thay đổi, recycle-bin safety, conflict list.

4. **Mô hình phát hành**
   - Đề xuất ban đầu: free bản cơ bản để validate nhu cầu; cân nhắc Pro nếu sau này có multi-profile, filters, versioning, NAS/network nâng cao.

5. **Ưu tiên kỹ thuật cần xác nhận sớm**
   - Cơ chế phát hiện xung đột file.
   - Chính sách khi cùng một file thay đổi ở cả hai nơi.
   - Cách xử lý xóa file trong đồng bộ 2 chiều.
   - Hành vi khi thư mục đích hoặc nguồn tạm thời không khả dụng.
