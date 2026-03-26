---
stepsCompleted: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
inputDocuments:
  - /mnt/d/ai_project/syncfolder/_bmad-output/planning-artifacts/prd.md
  - /mnt/d/ai_project/syncfolder/_bmad-output/planning-artifacts/prd-validation-report.md
  - /mnt/d/ai_project/syncfolder/_bmad-output/planning-artifacts/product-brief-syncfolder-2026-03-25.md
  - /mnt/d/ai_project/syncfolder/_bmad-output/planning-artifacts/architecture.md
lastStep: 14
status: complete
completedAt: '2026-03-25'
---

# UX Design Specification syncfolder

**Author:** hieu
**Date:** 2026-03-25

---

## Executive Summary

### Project Vision

SyncFolder là một ứng dụng desktop cho Windows 11, cung cấp giải pháp đồng bộ hai chiều tự động giữa hai thư mục. Mục tiêu chính là giảm bớt gánh nặng thao tác thủ công, giảm lỗi người dùng và xây dựng niềm tin rằng dữ liệu luôn nhất quán giữa các vị trí. Sản phẩm này định vị mình là một công cụ đơn giản, đáng tin cậy, ưu tiên sự rõ ràng và an toàn dữ liệu hơn là sự phức tạp về tính năng.

### Target Users

*   **Người dùng cá nhân nâng cao trên Windows 11**: Có nhu cầu đồng bộ giữa thư mục làm việc và bản sao trên ổ cứng ngoài/ổ đĩa khác.
*   **Freelancer / nhân sự văn phòng / kế toán / studio nhỏ**: Những người làm việc với các tệp thường xuyên cập nhật và cần một giải pháp "thiết lập một lần, chạy đáng tin cậy".
*   **Người dùng có nhiều thiết bị hoặc nhiều vị trí lưu trữ cục bộ**: Cần dữ liệu nhất quán giữa hai nơi mà không nhất thiết cần cộng tác trên đám mây.
*   **Mức độ am hiểu công nghệ**: Trung bình, họ ưu tiên sự đơn giản và độ tin cậy hơn là cấu hình kỹ thuật sâu.
*   **Thiết bị sử dụng chính**: Máy tính để bàn/laptop chạy Windows 11, có thể kèm theo ổ cứng ngoài hoặc thư mục chia sẻ nội bộ.
*   **Thời điểm/địa điểm sử dụng**: Hàng ngày, trong luồng công việc với các tài liệu, ảnh, video, tệp dự án cá nhân, chủ yếu trong môi trường cục bộ/offline.

### Key Design Challenges

*   **Cân bằng đơn giản và mạnh mẽ**: Làm thế nào để cung cấp chức năng đồng bộ hai chiều mạnh mẽ nhưng vẫn giữ được sự đơn giản tối đa trong trải nghiệm người dùng, tránh gây quá tải thông tin.
*   **Xây dựng lòng tin và an toàn dữ liệu**: Đảm bảo người dùng tin tưởng rằng dữ liệu của họ an toàn, đặc biệt trong các thao tác quan trọng như xóa hoặc ghi đè tệp. Việc xử lý sai có thể dẫn đến mất lòng tin ngay lập tức.
*   **Minh bạch hóa các quy trình nền**: Vì là ứng dụng chạy nền tự động, cần đảm bảo người dùng hiểu được điều gì đang xảy ra, khi nào, và nếu có vấn đề thì phải rõ ràng mà không gây phiền nhiễu.
*   **Thông báo lỗi rõ ràng**: Cung cấp thông báo lỗi thân thiện, dễ hiểu và có thể hành động được cho các sự cố phổ biến (ví dụ: ổ đĩa bị ngắt kết nối, thiếu quyền truy cập, tệp bị khóa).

### Design Opportunities

*   **Onboarding trực quan**: Thiết kế quy trình thiết lập lần đầu cho một cặp thư mục đồng bộ cực kỳ mượt mà và nhanh chóng (dưới 3 phút).
*   **Bảng điều khiển trạng thái rõ ràng**: Xây dựng bảng điều khiển trực quan, dễ hiểu ngay lập tức để hiển thị trạng thái đồng bộ hiện tại, lịch trình và lịch sử gần đây.
*   **Phản hồi/cảnh báo chủ động**: Đưa ra các cảnh báo nhẹ nhàng, không gây hoang mang cho các vấn đề tiềm ẩn hoặc thay đổi đáng kể (ví dụ: xóa hàng loạt, mặc dù là tính năng hậu MVP, nhưng mẫu UI có thể được thiết kế sớm).
*   **Niềm tin "Thiết lập và quên đi"**: Các yếu tố thiết kế cần củng cố độ tin cậy và cho phép người dùng tin tưởng ứng dụng chạy nền mà không cần giám sát liên tục.
*   **Tích hợp Windows 11**: Tận dụng các yếu tố UI và mẫu tương tác gốc của Windows 11 để tạo ra trải nghiệm đồng nhất.

## Core User Experience

### Defining Experience

SyncFolder will focus on providing two-way file and sub-directory synchronization between two locations on Windows 11, automatically, accurately, and securely. The most frequent user action will be "set it and forget it" – configure once and let the application automatically maintain consistency. The absolute imperative is to ensure data integrity, preventing loss or incorrect overwrites. The user experience must feel effortless and trustworthy, minimizing manual intervention and data anxiety.

### Platform Strategy

SyncFolder will be a desktop application focused on Windows 11. Primary interaction will be via mouse and keyboard. Platform requirements include deep integration with the Windows file system (folder selection, access permissions), the ability to schedule periodic background synchronization, and persistent local configuration storage. Offline functionality is core; all main use cases must work without internet connection or online accounts. We will leverage native Windows folder pickers and system task scheduling mechanisms.

### Effortless Interactions

The following interactions should feel completely natural and require no thought:
*   Selecting two folders for synchronization.
*   Toggling automatic synchronization schedules on/off.
*   Instantly understanding the "completed" or "failed" status of a sync job.
*   The application automatically resuming synchronization after system or application restarts.
*   Perceiving that files have been synchronized without manual verification.

We will eliminate complex configuration steps and minimize unnecessary confirmation dialogs for safe actions, focusing on creating satisfaction when users see their files accurately synchronized without needing intervention.

### Critical Success Moments

Key success moments include:
*   The user completing the configuration of their first synchronized folder pair within minutes.
*   When the user creates or modifies a file in one folder and sees it accurately appear in the other without an manual action. This is when they realize "this is better."
*   The application safely handling and clearly notifying when a folder is unavailable (e.g., disconnected drive).
*   The user feeling successful when they realize they no longer have to think about copying files, and the application becomes a reliable background tool.

### Experience Principles

Based on our discussion, the core experience principles for SyncFolder are:
*   **Simplify Core Actions**: The process of setting up and maintaining two-way synchronization must be intuitive and minimize cognitive load.
*   **Transparent Trustworthiness**: Users must always understand what is happening, the system's status, and trust that their data is safe.
*   **Native Platform Integration**: Leverage Windows 11 elements and behaviors for a familiar and smooth experience.
*   **Proactive & Safe Error Feedback**: The application must provide clear, understandable feedback for all actions and handle errors safely, with guidance for resolution.

## Desired Emotional Response

### Primary Emotional Goals

SyncFolder users should feel **Reliable** and **Serene**. They need to trust that their data is safe, will not be lost or incorrectly overwritten, and the application will perform consistently as expected. The feeling of serenity comes from reducing the mental burden of manual file copying. Additionally, they should feel **Efficient** and **In Control** of automated processes.

### Emotional Journey Mapping

*   **Product Discovery**: Curiosity and hope that this is the solution to their manual file synchronization problem.
*   **Core Experience**: Assurance, efficiency, control (when checking status), and satisfaction upon successful data synchronization.
*   **Post-Task Completion**: Relief, confidence, and success, no longer worrying about file copying.
*   **When Issues Arise**: Clearly informed, feeling in control, and able to resolve the problem rather than panicking or experiencing loss.
*   **Returning Usage**: Trust, comfort, and expectation that the application will continue to operate smoothly.

### Micro-Emotions

Key micro-emotions include:
*   **Confidence** (vs. Confusion)
*   **Trust** (vs. Skepticism)
*   **Accomplishment** (vs. Frustration)
*   **Satisfaction** (vs. Mere Acceptance)

### Design Implications

*   To foster feelings of **Trust and Security**: The interface needs to be clear, unambiguous; sync status notifications transparent; error handling safe without data loss; detailed but readable logs.
*   To create **Serenity and Reduced Anxiety**: Onboarding process simple, non-intrusive; application operates in the background without bothering; alerts only when truly necessary.
*   To deliver **Moments of Satisfaction** when files are synchronized: Provide subtle visual feedback (e.g., a small icon, unobtrusive notification) and optimize sync speed.
*   Avoid negative interactions that cause **Confusion, Skepticism, Anxiety, Frustration, Isolation, Loss**.

### Emotional Design Principles

1.  **Design for Trust:** Every interface element and interaction must reinforce confidence in data integrity and security.
2.  **Minimize Cognitive Load:** Ensure simplicity and intuitiveness for users to feel serene.
3.  **Minh bạch trong im lặng:** Cung cấp thông tin cần thiết về hoạt động nền mà không làm phiền người dùng, chỉ thông báo khi thực sự quan trọng.
4.  **Hỗ trợ kiên nhẫn khi gặp lỗi:** Khi có sự cố, cung cấp thông báo rõ ràng, hướng dẫn khắc phục để người dùng cảm thấy được hỗ trợ và kiểm soát.

## UX Pattern Analysis & Inspiration

### Inspiring Products Analysis

We have reviewed applications that target users might love and use regularly to learn from their UX:

*   **Windows File Explorer:**
    *   **Problem Solved:** Intuitive, easy-to-understand file management.
    *   **Onboarding:** No complex onboarding required, users are already familiar.
    *   **Navigation/Information Hierarchy:** Clear hierarchy, easy folder browsing.
    *   **Unique Interactions:** Direct manipulation (drag-and-drop, copy/paste) feels very natural.
    *   **Visual Design:** Clear, functional, integrated with the operating system.
    *   **Error Handling:** Clear file system error messages, such as "drive not ready."

*   **Microsoft PowerToys (e.g., FancyZones, Always On Top):**
    *   **Problem Solved:** Enhancing productivity by adding compact utilities, seamlessly integrated with Windows.
    *   **Onboarding:** Simple configuration, easy to toggle features on/off.
    *   **Navigation/Information Hierarchy:** Separate modules, each with clear configuration.
    *   **Unique Interactions:** Provides "magical" features that simplify daily tasks.
    *   **Visual Design:** Adheres to Windows design language, clean, efficient.
    *   **Error Handling:** Typically these utilities are simple so fewer errors, focus on stability.

*   **Minimalist Cloud Sync Clients (focused on local interaction):**
    *   **Problem Solved:** Automatic background file synchronization.
    *   **Onboarding:** Often a one-time setup for folders.
    *   **Navigation/Information Hierarchy:** Often a compact dashboard showing status and history.
    *   **Unique Interactions:** "Set it and forget it" reliability, files appear automatically.
    *   **Visual Design:** Minimalist, functional focus.
    *   **Error Handling:** Clear status icons (green check, red X) to indicate errors or status.

### Transferable UX Patterns

From these applications, we can derive UX patterns applicable to SyncFolder:

*   **Navigation Patterns:**
    *   **Tab/Panel-based Navigation**: Suitable for organizing main application sections (Configuration, Status, History, Settings) within a single window, providing clarity without excessive complexity.
    *   **Clear Headings, Concise Descriptions**: Helps users know where they are and what they are doing.

*   **Interaction Patterns:**
    *   **Direct Folder Selection using Native Windows Dialog**: Leverages user familiarity with the operating system.
    *   **Toggle Switch for Scheduling**: Provides immediate visual feedback for important options.
    *   **Subtle, Non-Intrusive Status Indicators**: Small icons, brief notifications when synchronization succeeds or needs attention.
    *   **Clear "Run Sync Now" Button**: Allows users manual control when needed.

*   **Visual Patterns:**
    *   **Windows 11 Fluent Design Principles**: Ensures a modern look, consistent with the operating system, including rounded corners, subtle shadows, and a consistent icon system.
    *   **Minimalist Data Presentation for Active Sync Tasks**: Focus on key information such as "last sync," "status," "next run."

### Anti-Patterns to Avoid

We also need to identify and avoid common anti-patterns:

*   **Overly Complex Settings Dialogs**: With too many hidden or confusing options, causing confusion and anxiety.
*   **Ambiguous Error Messages**: Generic messages like "sync failed" without providing context or resolution suggestions.
*   **Constant, Intrusive Notifications for Routine Operations**: Annoying and diminishes the value of important alerts.
*   **Non-Native or Inconsistent Windows 11 UI Elements**: Creates a disjointed and unprofessional experience.
*   **Lack of Visual Feedback in Long Background Operations**: Leaves users wondering if the application is working.

### Design Inspiration Strategy

The strategy for using design inspiration for SyncFolder will be:

*   **What to Apply:**
    *   **Native Windows 11 UI/UX elements and aesthetics**: To create familiarity, consistency, and a polished product feel. This supports the "Native Platform Integration" principle.
    *   **Clear, concise status indicators**: To convey information transparently. This supports the "Transparent Trustworthiness" principle.
    *   **Simple, linear onboarding flow**: For easy setup. This supports the "Simplify Core Actions" principle.

*   **What to Adapt:**
    *   **PowerToys' approach to "powerful but simple" utilities**: Adapt the concept of enhancing core OS functionality seamlessly, but specifically for file synchronization.
    *   **The "set it and forget it" reliability of cloud sync clients**: Adapt the feeling of background automation and trustworthiness, focusing on local files.
    *   **File Explorer's direct manipulation philosophy**: Simplify file management metaphors for synchronization operations.

*   **What to Avoid:**
    *   **Complex configuration screens with multiple tabs/options**: Conflicts with the "Minimize Cognitive Load" and "Simplify Core Actions" principles.
        *   **Ambiguous technical error messages**: Conflicts with "Minh bạch đáng tin cậy" and "Hỗ trợ kiên nhẫn khi gặp lỗi".
    *   **Any UI/UX that feels outdated or non-native on Windows 11**: Conflicts with "Tích hợp nền tảng gốc".

## Design System Foundation

### 1.1 Design System Choice

Based on project requirements analysis and inspiration strategy, we propose using **Fluent UI React** as the design system foundation for SyncFolder.

### Rationale for Selection

The choice of Fluent UI React is based on the following reasons:

*   **Native Windows 11 Aesthetics**: Fluent UI is Microsoft's design system, providing UI components and patterns that naturally align with the look and feel of Windows 11. This directly supports the "Native Platform Integration" principle and creates a familiar, consistent experience for users.
*   **Development Speed**: Fluent UI offers a rich collection of pre-built, accessibility-tested React components. This will significantly accelerate the MVP development process, allowing us to focus on core logic rather than building UI components from scratch.
*   **Consistency**: Ensures a synchronized and professional user interface throughout the application, aligning with the goals of "Transparent Trustworthiness" and "Minimize Cognitive Load."
*   **Accessibility Support**: Built with accessibility from the ground up, helping to meet NFRs related to users with special needs.

### Implementation Approach

We will integrate Fluent UI React components into the Tauri application's React/TypeScript frontend. Prioritize using standard Fluent UI components whenever possible to leverage the library's advantages.

### Customization Strategy

Our customization strategy will focus on:

*   **Using Fluent UI's theming features**: To adjust colors, typography, and spacing to perfectly match the specific nuances of the Windows 11 design language (e.g., subtle changes to component states, specific shadow depths).
*   **Minimizing custom component creation**: Only build custom components when Fluent UI does not offer a suitable solution for truly unique interactions or UI elements not covered by Fluent.

## 2. Core User Experience

### 2.1 Defining Experience
Trải nghiệm cốt lõi của SyncFolder là sự **"Đồng nhất tức thì và An tâm tuyệt đối"**. Người dùng chỉ cần thực hiện một tương tác duy nhất: Chọn hai thư mục và kích hoạt. Sau đó, mọi thứ diễn ra âm thầm nhưng minh bạch. Điểm đặc biệt là sự tích hợp sâu vào Windows 11, khiến người dùng cảm thấy đây là một tính năng có sẵn của hệ điều hành chứ không phải phần mềm bên thứ ba.

### 2.2 User Mental Model
Người dùng coi hai thư mục được đồng bộ như hai mặt của một tấm gương. Họ mang tâm thế "Thiết lập một lần, tin tưởng mãi mãi". Họ không muốn quan tâm đến thuật toán so sánh file, họ chỉ quan tâm đến kết quả: Tệp tin luôn sẵn sàng ở bất cứ đâu họ cần.

### 2.3 Success Criteria
- **Zero Configuration**: Mặc định các tùy chọn an toàn nhất để người dùng không phải cấu hình sâu.
- **Visual Confirmation**: Chỉ báo trạng thái đồng bộ rõ ràng (Syncing, Up-to-date, Error) ngay tại giao diện chính và khay hệ thống.
- **Safety First**: Mọi thao tác xóa hoặc ghi đè quan trọng đều có xác nhận trực quan, dễ hiểu.
- **Resilience**: Tự động tiếp tục khi ổ cứng ngoài được cắm lại hoặc máy tính khởi động lại.

### 2.4 Novel UX Patterns
Chúng ta sẽ kết hợp các mẫu quen thuộc của Windows 11 với một chút cải tiến:
- **Guided Pair Setup**: Sử dụng luồng thiết lập dạng Card thay vì Wizard truyền thống, tạo cảm giác hiện đại và bớt gò bó.
- **Conflict Resolver trực quan**: Thay vì thông báo lỗi chữ, chúng ta hiển thị hai bản file cạnh nhau với thông tin ngày giờ, kích thước để người dùng chọn bản muốn giữ lại.

### 2.5 Experience Mechanics
1. **Khởi tạo**: Người dùng kéo thả thư mục A vào app, sau đó chọn thư mục B.
2. **Tương tác**: Một nút gạt (Toggle) lớn để "Bật đồng bộ tự động".
3. **Phản hồi**: Thanh trạng thái chạy mượt mà kèm theo danh sách các file vừa được xử lý (Real-time log).
4. **Hoàn thành**: Một thông báo "Tất cả đã đồng nhất" với icon xanh dịu mắt.

## Visual Design Foundation

### Color System

Hệ thống màu sắc sẽ tuân thủ chặt chẽ các nguyên tắc của Fluent Design để tạo sự quen thuộc và đáng tin cậy.

-   **Màu chủ đạo (Primary Accent Color):** `#0078D4` (Microsoft's Azure Blue). Màu này sẽ được sử dụng cho các nút hành động chính, trạng thái active, và các yếu tố nhấn mạnh. Nó thể hiện sự ổn định và tin cậy.
-   **Màu nền (Backgrounds):** Sử dụng các sắc thái trung tính, nhẹ nhàng. Chúng ta sẽ ưu tiên hiệu ứng **Mica** của Windows 11 (translucent, động) cho các khu vực chính của ứng dụng để tạo cảm giác sâu và hiện đại, hoặc màu xám nhẹ `#F3F3F3` cho các panel/card cụ thể.
-   **Màu văn bản (Text Colors):**
    -   `#1F1F1F` cho văn bản chính (black text on light background)
    -   `#616161` cho văn bản phụ, mô tả.
-   **Màu trạng thái (Semantic Colors):**
    -   **Success:** `#4CAF50` (Green) - Cho các thông báo hoàn thành, trạng thái đồng bộ thành công.
    -   **Warning:** `#FFC107` (Amber) - Cho các cảnh báo nhẹ, cần chú ý.
    -   **Error:** `#D32F2F` (Red) - Cho các lỗi nghiêm trọng, cần hành động ngay.
-   **Accessibility:** Đảm bảo tỷ lệ tương phản màu sắc đạt chuẩn WCAG 2.1 AA cho tất cả các cặp màu văn bản và nền để tối ưu khả năng tiếp cận.

### Typography System

Chúng ta sẽ sử dụng font chữ mặc định của Windows 11 để tạo sự liền mạch với hệ điều hành và đảm bảo tính dễ đọc.

-   **Font chữ chính:** **Segoe UI Variable**. Đây là font chữ được tối ưu cho các màn hình hiện đại, có khả năng điều chỉnh linh hoạt theo kích thước và độ phân giải, mang lại trải nghiệm đọc mượt mà.
-   **Hệ thống cỡ chữ (Type Scale):**
    -   **Display (H1):** 28px - Dùng cho tiêu đề lớn, nổi bật nhất (ví dụ: tên ứng dụng, tiêu đề chính của màn hình).
    -   **Heading (H2):** 24px - Tiêu đề các phần lớn.
    -   **Subheading (H3):** 20px - Tiêu đề các phần con.
    -   **Body Large:** 15px - Văn bản chính, dễ đọc cho nội dung dài.
    -   **Body Standard:** 13px - Văn bản thông thường, mô tả.
    -   **Caption:** 11px - Chú thích, metadata nhỏ.
-   **Cân nặng (Font Weight):** Sử dụng `Semi-Bold` cho tiêu đề và `Regular` cho văn bản nội dung để tạo sự phân cấp rõ ràng.
-   **Line-height:** Tối ưu hóa line-height để tăng cường khả năng đọc, thường là 1.5 lần kích thước font.

### Spacing & Layout Foundation

Để tạo sự thoáng đãng và chuyên nghiệp, chúng ta sẽ áp dụng hệ thống spacing và layout dựa trên grid 8px.

-   **Hệ thống Grid (8px Base):** Tất cả khoảng cách và kích thước sẽ là bội số của 8px (ví dụ: 8px, 16px, 24px, 32px...). Điều này đảm bảo sự nhất quán và dễ dàng bảo trì.
-   **Bố cục (Layout):**
    -   **Thoáng đãng (Airy & Spacious):** Ưu tiên không gian trắng giữa các thành phần để giảm gánh nặng nhận thức và làm nổi bật nội dung chính.
    -   **Cấu trúc Card-based:** Các nhóm thông tin liên quan sẽ được đặt trong các "Card" với góc bo tròn nhẹ (border-radius: 8px), tạo cảm giác hiện đại và dễ tiếp cận.
    -   **Căn chỉnh:** Các thành phần sẽ được căn chỉnh theo một đường thẳng ảo để tạo sự gọn gàng, trật tự.
-   **Khoảng cách thành phần:**
    -   **Small (8px):** Giữa các item trong một danh sách, icon và text.
    -   **Medium (16px):** Giữa các label và input, các nút cạnh nhau.
    -   **Large (24-32px):** Giữa các phần lớn của giao diện (ví dụ: giữa tiêu đề và nội dung chính, giữa các card).

### Accessibility Considerations

-   Đảm bảo tỷ lệ tương phản màu sắc cho văn bản và các thành phần tương tác theo tiêu chuẩn WCAG 2.1 AA (tối thiểu 4.5:1 cho văn bản thường, 3:1 cho văn bản lớn và thành phần UI).
-   Cung cấp các trạng thái focus rõ ràng cho các thành phần tương tác để hỗ trợ người dùng điều hướng bằng bàn phím.
-   Kích thước font chữ tối thiểu 11px cho các chú thích nhỏ, nhưng ưu tiên 13-15px cho nội dung chính.

## Design Direction Decision

### Design Directions Explored

Trong quá trình này, chúng ta đã khám phá các hướng thiết kế tiềm năng dựa trên nguyên tắc Fluent Design của Windows 11, tập trung vào sự đơn giản, tin cậy và tích hợp nền tảng. Các biến thể đã xem xét xoay quanh việc ứng dụng hệ thống màu sắc, kiểu chữ và khoảng cách đã định để tạo ra giao diện thoáng đãng, dễ sử dụng, phù hợp với trải nghiệm "thiết lập và quên đi" của SyncFolder.

### Chosen Direction

Hướng thiết kế được chọn là một giao diện trực quan, lấy cảm hứng từ các ứng dụng Windows 11 gốc như File Explorer và PowerToys, nhưng tối giản hóa để tập trung vào chức năng đồng bộ.
-   **Bố cục:** Thoáng đãng, sử dụng cấu trúc "Card-based" để nhóm các thông tin liên quan (ví dụ: một card cho mỗi cặp thư mục đồng bộ).
-   **Màu sắc:** Ưu tiên màu `#0078D4` (Azure Blue) làm màu nhấn chính, kết hợp với hiệu ứng Mica cho nền và các sắc thái xám trung tính để tạo sự hiện đại, tin cậy.
-   **Kiểu chữ:** Segoe UI Variable với hệ thống phân cấp rõ ràng để dễ đọc và phù hợp với hệ điều hành.
-   **Tương tác:** Sử dụng các thành phần Fluent UI React như Toggle Switch cho các tùy chọn quan trọng, nút hành động chính rõ ràng.

### Design Rationale

Hướng thiết kế này được lựa chọn vì nó trực tiếp hỗ trợ các nguyên tắc UX cốt lõi của SyncFolder:
-   **Đơn giản hóa hành động cốt lõi:** Bố cục và tương tác đơn giản giúp người dùng dễ dàng thiết lập và quản lý.
-   **Minh bạch đáng tin cậy:** Màu sắc và chỉ báo trạng thái rõ ràng giúp người dùng luôn hiểu điều gì đang xảy ra.
-   **Tích hợp nền tảng gốc:** Sử dụng Fluent Design và các yếu tố Windows 11 tạo ra trải nghiệm quen thuộc, mượt mà và chuyên nghiệp.
-   **Phản hồi chủ động và an toàn khi lỗi:** Các thành phần trạng thái và cảnh báo được thiết kế để truyền đạt thông tin hiệu quả mà không gây hoang mang.

### Implementation Approach

Chúng ta sẽ sử dụng Fluent UI React components và thư viện Tauri để hiện thực hóa hướng thiết kế này, đảm bảo tính nhất quán và hiệu suất cao. Ưu tiên các thành phần Fluent UI chuẩn và chỉ tùy chỉnh khi cần thiết để phù hợp với các sắc thái của Windows 11.

## User Journey Flows

### Journey 1 - Người dùng chính thiết lập thành công lần đầu

**Mô tả:** Hieu là một freelancer dùng desktop Windows 11 để xử lý tài liệu khách hàng và thường sao chép thủ công sang ổ cứng ngoài vào cuối ngày. Anh cài SyncFolder vì muốn bỏ bước copy lặp lại nhưng vẫn cần cảm giác an toàn. Khi mở ứng dụng, anh thấy giao diện gọn: chọn thư mục A, chọn thư mục B, chọn chu kỳ đồng bộ. Anh lưu cấu hình, bật lịch 60 phút và chờ lần chạy đầu tiên. Khoảnh khắc giá trị đến khi anh chỉnh sửa một file ở thư mục làm việc và sau đó thấy file tương ứng ở thư mục còn lại đã được cập nhật đúng. Từ đây, SyncFolder trở thành công cụ nền mà anh ít phải nghĩ tới nhưng tin tưởng mỗi ngày.

```mermaid
graph TD
    A[Bắt đầu: Mở ứng dụng lần đầu] --> B{Chào mừng & Onboarding};
    B --> C[Chọn Thư mục Nguồn];
    C --> D[Chọn Thư mục Đích];
    D --> E[Chọn Lịch Đồng bộ (30p/60p/90p/1d)];
    E --> F[Lưu & Bắt đầu Đồng bộ];
    F --> G{Trạng thái: "Đang đồng bộ..."};
    G --> H[Hoàn thành Sync đầu tiên];
    H --> I{Trạng thái: "Đã đồng bộ"};
    I --> J[Người dùng sửa/tạo file];
    J --> K[Ứng dụng tự động phát hiện & đồng bộ];
    K --> L[Người dùng xác nhận file đã cập nhật];
    L --> M[Kết thúc: Tin tưởng & chạy nền];

    C -- Lỗi chọn thư mục --> C;
    D -- Lỗi chọn thư mục --> D;
    F -- Lỗi Sync ban đầu --> N[Hiển thị lỗi & Gợi ý xem Log];
```

### Journey 2 - Người dùng chính gặp edge case khi thư mục tạm thời không khả dụng

**Mô tả:** Lan dùng laptop và một ổ SSD ngoài. Cô đã cấu hình đồng bộ tự động nhưng một lần rút ổ ngoài ra mà quên cắm lại. Đến chu kỳ chạy tiếp theo, ứng dụng không thể đồng bộ. Thay vì im lặng hoặc tạo trạng thái mơ hồ, hệ thống phải hiển thị rõ lần sync thất bại, nguyên nhân thư mục đích không khả dụng và giữ nguyên dữ liệu thay vì cố xử lý rủi ro. Khi cắm lại ổ, Lan chạy lại hoặc đợi chu kỳ kế tiếp và thấy job hoạt động bình thường. Journey này chứng minh sản phẩm không chỉ tốt khi mọi thứ lý tưởng mà còn phải an toàn khi môi trường thực tế không ổn định.

```mermaid
graph TD
    A[Bắt đầu: Lịch đồng bộ chạy nền] --> B{Kiểm tra Thư mục Nguồn (OK)};
    B --> C{Kiểm tra Thư mục Đích (Không khả dụng)};
    C --> D[Ứng dụng phát hiện & hủy sync];
    D --> E[Hiển thị thông báo: "Thư mục đích không khả dụng"];
    E --> F[Cập nhật trạng thái Sync cuối: "Thất bại"];
    F --> G[Người dùng cắm lại ổ đĩa/khắc phục];
    G --> H{Người dùng chạy lại thủ công HOẶC đợi chu kỳ kế tiếp};
    H --> I[Sync job chạy thành công];
    I --> J[Kết thúc: Dữ liệu an toàn & phục hồi];
```

### Journey 3 - Người dùng cấu hình/operations quản lý hồ sơ sync

**Mô tả:** Minh là IT support cho một văn phòng nhỏ. Anh cần thiết lập một cấu hình đơn giản cho người dùng không kỹ thuật. Anh mở app, chọn hai thư mục cần đồng bộ, đặt lịch 1 ngày và xác nhận trạng thái lưu thành công. Khi người dùng hỏi “app có chạy chưa?”, Minh cần nhìn thấy ngay thông tin lần chạy gần nhất, trạng thái thành công/thất bại và log cơ bản. Giá trị với Minh không nằm ở tính năng cao cấp mà ở khả năng triển khai nhanh, giải thích dễ và giảm số lần bị gọi hỗ trợ.

```mermaid
graph TD
    A[Bắt đầu: Mở ứng dụng] --> B[Hiển thị Dashboard với các Hồ sơ Sync];
    B --> C{Xem & Chỉnh sửa Hồ sơ hiện có};
    C --> D[Chọn Hồ sơ A];
    D --> E[Hiển thị Chi tiết Hồ sơ A];
    E --> F[Bấm "Chỉnh sửa"];
    F --> G[Thay đổi lịch đồng bộ];
    G --> H[Bấm "Lưu Thay đổi"];
    H --> I[Cập nhật Hồ sơ A trên Dashboard];
    C --> J{Tạo Hồ sơ Sync Mới};
    J --> K[Bấm "Tạo Hồ sơ Mới"];
    K --> L[Đi qua luồng thiết lập (như Journey 1)];
    L --> M[Lưu & Kích hoạt Hồ sơ Mới];
    M --> N[Kết thúc: Quản lý thành công hồ sơ];

    G -- Lỗi lưu --> G;
    L -- Lỗi thiết lập --> K;
```

### Journey 4 - Người dùng support/tự khắc phục sự cố bằng log

**Mô tả:** Sau một tuần sử dụng, một người dùng thấy một vài file chưa xuất hiện ở thư mục đích. Họ mở ứng dụng và xem log để biết job nào đã chạy, file nào lỗi và lỗi thuộc loại gì: quyền truy cập, file đang bị khóa hay thư mục không tồn tại. Nếu ứng dụng chỉ báo “sync failed”, người dùng sẽ mất niềm tin rất nhanh; nhưng nếu log đủ rõ để khoanh vùng vấn đề, họ vẫn tin công cụ đang kiểm soát được tình huống. Journey này tạo ra yêu cầu rõ ràng về quan sát hệ thống, khả năng chẩn đoán và thông điệp lỗi có thể hành động.

```mermaid
graph TD
    A[Bắt đầu: Nhận thông báo "Sync Thất bại"] --> B[Mở ứng dụng & Thấy trạng thái "Sync Thất bại"];
    B --> C[Bấm "Xem chi tiết" / "Xem Log"];
    C --> D[Hiển thị Log chi tiết, dễ đọc];
    D --> E{Log chỉ ra lỗi rõ ràng (ví dụ: "Thư mục không tồn tại")};
    E --> F[Người dùng xác định nguyên nhân];
    F --> G[Người dùng khắc phục sự cố];
    G --> H[Người dùng chạy đồng bộ thủ công];
    H --> I[Sync job chạy thành công];
    I --> J[Kết thúc: Khắc phục sự cố thành công];
```

### Journey Patterns

Across these flows, I'm seeing some common patterns we can standardize:

-   **Setup Wizard (Guided Pair Setup):** For initial configuration, a linear, step-by-step process.
-   **Dashboard Overview:** Main screen for quick status checks and access to profiles.
-   **Detail View/Editor:** For managing individual sync profiles.
-   **Notification & Resolution Loop:** For handling errors, informing user, and allowing recovery.

### Flow Optimization Principles

For each journey, let's ensure we're:

-   **Zero-Click Confidence**: Minimize clicks for routine tasks; auto-sync should feel invisible but trustworthy.
-   **Contextual Feedback**: Provide feedback at the right time and place (e.g., system tray notifications, in-app status banners).
-   **Actionable Errors**: Error messages are clear, concise, and guide the user to a solution, not just state a problem.
-   **Progressive Disclosure**: Only show complex options (like log details) when needed.

## Component Strategy

### Design System Components

Với Fluent UI React, chúng ta có sẵn một thư viện phong phú các thành phần UI, bao gồm:

-   **Navigation:** `NavigationView`, `Pivot` (Tabs), `CommandBar`.
-   **Inputs:** `TextField`, `Dropdown`, `Checkbox`, `ToggleSwitch`, `Button`, `DatePicker`.
-   **Feedback:** `MessageBar`, `Spinner`, `ProgressRing`, `ToolTip`, `Dialog`.
-   **Layout:** `Stack`, `Grid`, `Card`.
-   **Data Display:** `DetailsList` (cho Log), `Table`, `Icon`.

Các thành phần này sẽ là xương sống cho giao diện của SyncFolder, đảm bảo tính nhất quán với Windows 11 và hiệu quả phát triển.

### Custom Components

Dựa trên hành trình người dùng và yêu cầu riêng của SyncFolder, có một số thành phần có thể cần được tùy chỉnh hoặc xây dựng mới:

1.  **Thành phần "Pair Setup Card" (Thẻ Cấu hình Cặp Thư mục):**
    *   **Mục đích:** Hướng dẫn người dùng chọn thư mục nguồn và đích trong luồng onboarding.
    *   **Nội dung:** Hiển thị 2 vùng chọn thư mục (kèm icon và đường dẫn), nút "Browse", nút "Set as Source/Destination".
    *   **Hành động:** Mở File Picker của Windows, hiển thị đường dẫn, xác nhận lựa chọn.
    *   **Trạng thái:** Default, Invalid (nếu chưa chọn đủ 2 thư mục), Selected.
    *   **Khả năng tiếp cận:** Sử dụng `aria-label` cho các vùng chọn và nút Browse.
    *   **Lý do tùy chỉnh:** Fluent UI có `TextField` nhưng không có một thành phần tích hợp sẵn cho việc chọn cặp thư mục với luồng trải nghiệm cụ thể như này.

2.  **Thành phần "Sync Profile Card" (Thẻ Hồ sơ Đồng bộ):**
    *   **Mục đích:** Hiển thị tóm tắt một hồ sơ đồng bộ đang hoạt động hoặc đã lưu.
    *   **Nội dung:** Tên hồ sơ, đường dẫn nguồn/đích, trạng thái sync cuối cùng (icon + text), thời gian sync cuối, lịch chạy, nút Toggle bật/tắt sync.
    *   **Hành động:** Kích hoạt/vô hiệu hóa sync, click để xem chi tiết.
    *   **Trạng thái:** Active (Running/Up-to-date), Inactive (Paused), Error, Disabled (nếu thư mục không khả dụng).
    *   **Khả năng tiếp cận:** Các nút Toggle và trạng thái có `aria-live` để cập nhật cho screen reader.
    *   **Lý do tùy chỉnh:** Đây là một thành phần tổng hợp nhiều thông tin và tương tác riêng biệt cho từng hồ sơ sync, cần bố cục cụ thể không có sẵn trong Fluent UI.

3.  **Thành phần "Conflict Resolver Modal" (Hộp thoại Giải quyết Xung đột - Post-MVP):**
    *   **Mục đích:** Hiển thị khi có xung đột (cùng một file thay đổi ở cả hai bên) và cho phép người dùng chọn phiên bản muốn giữ.
    *   **Nội dung:** Tên file, thời gian sửa đổi gần nhất của mỗi phiên bản, kích thước, nút "Keep Source", "Keep Destination", "Keep Both".
    *   **Hành động:** Chọn phiên bản, đóng hộp thoại.
    *   **Trạng thái:** Default, Selected.
    *   **Khả năng tiếp cận:** Đảm bảo điều hướng bàn phím rõ ràng giữa các tùy chọn.
    *   **Lý do tùy chỉnh:** Đây là một luồng tương tác rất cụ thể và quan trọng về mặt an toàn dữ liệu, cần một giao diện chuyên biệt.

### Component Implementation Strategy

-   **Ưu tiên Fluent UI React:** Luôn bắt đầu bằng cách tìm kiếm thành phần phù hợp trong Fluent UI.
-   **Tùy chỉnh có kiểm soát:** Khi cần thành phần tùy chỉnh, xây dựng chúng bằng cách sử dụng các `Design Tokens` (màu sắc, khoảng cách, kiểu chữ) từ Fluent UI để đảm bảo tính nhất quán với hệ thống.
-   **Đóng gói (Encapsulation):** Mỗi thành phần tùy chỉnh sẽ được thiết kế độc lập, có các `props` rõ ràng và `state` được quản lý tốt.
-   **Testability:** Đảm bảo các thành phần dễ dàng kiểm thử.
-   **Khả năng tiếp cận (Accessibility):** Tích hợp các thuộc tính ARIA, hỗ trợ điều hướng bàn phím ngay từ đầu.

### Implementation Roadmap

Dựa trên tầm quan trọng trong hành trình người dùng và yêu cầu MVP:

-   **Giai đoạn 1 - Thành phần cốt lõi (MVP):**
    -   `Pair Setup Card` (quan trọng nhất cho Onboarding)
    -   `Sync Profile Card` (cho Dashboard và quản lý hồ sơ)
    -   Các thành phần cơ bản của Fluent UI: Button, ToggleSwitch, TextField, MessageBar, Icon.
-   **Giai đoạn 2 - Thành phần hỗ trợ (Post-MVP):**
    -   `Conflict Resolver Modal` (cho tính năng xử lý xung đột)
    -   `DetailsList` (cho màn hình Log chi tiết).
    -   Các thành phần phức tạp hơn của Fluent UI cho Filter, Search.
-   **Giai đoạn 3 - Thành phần nâng cao:**
    -   Bất kỳ thành phần nào cho các tính năng tầm nhìn xa (Versioning, NAS).

## UX Consistency Patterns

### Button Hierarchy

-   **Mục đích:** Đảm bảo các nút có vai trò rõ ràng và dễ nhận diện, hướng dẫn người dùng thực hiện các hành động quan trọng.
-   **Khi nào sử dụng:**
    -   **Primary Button (Nút chính):** Cho hành động quan trọng nhất trên màn hình (ví dụ: "Lưu Cấu hình", "Bắt đầu Đồng bộ"). Thường là màu nhấn (Azure Blue).
    -   **Secondary Button (Nút phụ):** Cho các hành động thứ cấp, bổ sung (ví dụ: "Hủy", "Xem Log"). Thường là màu nền trắng/xám với viền.
    -   **Tertiary Button (Nút văn bản/Icon):** Cho các hành động ít quan trọng hơn, thường là điều hướng hoặc các tùy chọn ẩn (ví dụ: nút icon "Settings", "More options").
-   **Thiết kế trực quan:**
    -   Fluent UI `PrimaryButton` và `DefaultButton` sẽ được sử dụng.
    -   Kích thước, khoảng cách và màu sắc tuân thủ chặt chẽ Fluent Design.
-   **Hành vi:**
    -   Trạng thái `hover`, `active`, `disabled` rõ ràng.
    -   Các hành động chính luôn được đặt ở vị trí dễ thấy.

### Feedback Patterns

-   **Mục đích:** Cung cấp thông tin phản hồi rõ ràng, kịp thời và có thể hành động được cho người dùng về trạng thái của ứng dụng hoặc kết quả hành động của họ.
-   **Các loại phản hồi:**
    -   **Success (Thành công):** Thông báo nhẹ nhàng, không xâm phạm (ví dụ: `MessageBar` nhỏ ở dưới cùng màn hình, icon xanh lá cây). "Đồng bộ hoàn tất."
    -   **Error (Lỗi):** Thông báo rõ ràng, có hướng dẫn khắc phục (ví dụ: `MessageBar` màu đỏ nổi bật với icon cảnh báo, hoặc `Dialog` modal cho lỗi nghiêm trọng cần tương tác). "Thư mục đích không khả dụng, vui lòng kiểm tra kết nối ổ đĩa."
    -   **Warning (Cảnh báo):** Thông báo cần chú ý nhưng không chặn tương tác (ví dụ: `MessageBar` màu hổ phách). "Có một số file không thể đồng bộ."
    -   **Info (Thông tin):** Thông báo cung cấp ngữ cảnh hoặc cập nhật trạng thái (ví dụ: `MessageBar` màu xanh dương nhạt). "Đang quét các thay đổi..."
-   **Thiết kế trực quan:** Sử dụng `MessageBar`, `Dialog`, `ProgressRing` của Fluent UI. Các icon trạng thái (check, X, tam giác) sẽ được sử dụng nhất quán.
-   **Hành vi:**
    -   Thông báo ngắn gọn, dễ hiểu, tránh thuật ngữ kỹ thuật.
    -   Các lỗi nghiêm trọng sẽ được đưa ra dưới dạng `Dialog` yêu cầu người dùng tương tác.
    -   Thông báo success/info có thể tự động biến mất sau vài giây hoặc có nút đóng.

### Form Patterns

-   **Mục đích:** Đảm bảo các biểu mẫu nhập liệu (như chọn thư mục, cài đặt lịch) dễ hiểu, dễ điền và giảm thiểu lỗi.
-   **Khi nào sử dụng:** Luồng onboarding, chỉnh sửa hồ sơ sync, cài đặt ứng dụng.
-   **Thiết kế trực quan:**
    -   Sử dụng `TextField`, `Dropdown`, `ToggleSwitch` của Fluent UI.
    -   Label rõ ràng, placeholder hữu ích.
    -   Các trường nhập liệu liên quan được nhóm lại.
-   **Hành vi:**
    -   Validation (xác thực) theo thời gian thực (real-time validation) hoặc khi người dùng rời khỏi trường nhập.
    -   Thông báo lỗi hiển thị ngay dưới trường nhập liệu bị lỗi.
    -   Các nút hành động (Lưu, Hủy) được đặt nhất quán.

### Navigation Patterns

-   **Mục đích:** Cung cấp cấu trúc rõ ràng để người dùng dễ dàng tìm kiếm thông tin và chuyển đổi giữa các phần chính của ứng dụng.
-   **Khi nào sử dụng:** Các phần chính của ứng dụng (Dashboard, Log, Settings).
-   **Thiết kế trực quan:**
    -   **Left-aligned Navigation (Sidebar):** Đối với các ứng dụng desktop, một sidebar bên trái với các mục điều hướng rõ ràng (Dashboard, Profiles, Activity Log, Settings) là hiệu quả.
    -   **Tab Navigation (Pivot):** Có thể sử dụng bên trong một màn hình để chuyển đổi giữa các chế độ xem khác nhau của cùng một đối tượng (ví dụ: trong một Profile, có thể có các tab "Overview", "Log", "Settings").
-   **Hành vi:**
    -   Mục đang hoạt động (active) được tô sáng rõ ràng.
    -   Điều hướng nhanh chóng, không có độ trễ.
    -   Tooltip cho các icon điều hướng (nếu có).

### Additional Patterns

-   **Empty States (Trạng thái rỗng):**
    -   **Mục đích:** Hướng dẫn người dùng khi không có dữ liệu (ví dụ: chưa có hồ sơ sync nào).
    -   **Thiết kế:** Hiển thị một thông báo thân thiện, icon minh họa, và một nút hành động rõ ràng để bắt đầu (ví dụ: "Tạo hồ sơ đồng bộ đầu tiên của bạn").
-   **Loading States (Trạng thái tải):**
    -   **Mục đích:** Thông báo cho người dùng rằng hệ thống đang xử lý và tránh hiểu lầm là ứng dụng bị treo.
    -   **Thiết kế:** Sử dụng `Spinner` hoặc `ProgressRing` của Fluent UI cho các thao tác ngắn, hoặc overlay với spinner cho các thao tác lâu hơn (ví dụ: "Đang phân tích thư mục...").
-   **Confirmation Dialogs (Hộp thoại xác nhận):**
    -   **Mục đích:** Ngăn người dùng thực hiện các hành động không mong muốn hoặc không thể hoàn tác (ví dụ: xóa hồ sơ sync).
    -   **Thiết kế:** Sử dụng `Dialog` modal của Fluent UI với tiêu đề rõ ràng, mô tả hậu quả, và các nút "Confirm" / "Cancel".

## Responsive Design & Accessibility

### Responsive Strategy

Mặc dù SyncFolder tập trung vào trải nghiệm desktop trên Windows 11, chúng ta vẫn cần đảm bảo ứng dụng linh hoạt trên các kích thước cửa sổ và có thể hoạt động tốt trên các thiết bị cảm ứng (tablet chạy Windows) nếu có:

-   **Chiến lược Desktop:**
    -   Tận dụng không gian màn hình lớn: Sử dụng layout nhiều cột (ví dụ: Sidebar điều hướng bên trái, khu vực nội dung chính ở giữa, và có thể một panel thông tin phụ bên phải cho log/chi tiết profile).
    -   Điều hướng bên cạnh (side navigation) là lựa chọn ưu tiên.
    -   Mật độ thông tin vừa phải, không quá dày đặc để giữ sự thoáng đãng của Fluent Design.
    -   Có thể bao gồm các tính năng desktop-specific như kéo thả (drag-and-drop) cho việc chọn thư mục.
-   **Chiến lược Tablet (nếu áp dụng):**
    -   Bố cục có thể đơn giản hóa thành một cột hoặc hai cột cơ bản.
    -   Các nút và vùng chạm (touch target) cần lớn hơn để dễ tương tác bằng ngón tay.
    -   Ưu tiên các tương tác chạm và cử chỉ (gestures) thay vì chỉ click chuột.
    -   Mật độ thông tin cần được điều chỉnh để không gây quá tải trên màn hình nhỏ hơn.
-   **Chiến lược Mobile:** (Không phải trọng tâm cho MVP nhưng cần lưu ý cho tầm nhìn dài hạn)
    -   Nếu có phiên bản mobile, điều hướng dưới cùng (bottom navigation) hoặc menu hamburger là các lựa chọn phổ biến.
    -   Bố cục sẽ thu gọn thành một cột duy nhất.
    -   Chỉ hiển thị thông tin quan trọng nhất.

### Breakpoint Strategy

Chúng ta sẽ sử dụng các breakpoint linh hoạt để điều chỉnh bố cục ứng dụng:

-   **Desktop Large:** 1440px+ (Tối ưu hóa cho màn hình lớn, hiển thị nhiều thông tin nhất có thể trong giới hạn thoáng đãng).
-   **Desktop Standard:** 1024px - 1439px (Bố cục tiêu chuẩn, thường là 2 cột chính).
-   **Tablet / Desktop Compact:** 768px - 1023px (Bố cục đơn giản hơn, có thể chuyển sidebar sang dạng menu ẩn hoặc các tab lớn).
-   **Minimum Width:** Dưới 768px (Thường là một cột, tập trung vào thông tin thiết yếu).

Chúng ta sẽ ưu tiên thiết kế theo hướng **desktop-first** (từ lớn đến nhỏ) nhưng đảm bảo tính linh hoạt khi cửa sổ ứng dụng bị thu nhỏ.

### Accessibility Strategy

Khả năng tiếp cận là yếu tố không thể thiếu để đảm bảo SyncFolder phục vụ được mọi người dùng. Chúng ta sẽ hướng tới tuân thủ **WCAG 2.1 Level AA (Recommended)**.

**Các cân nhắc chính:**

-   **Tỷ lệ tương phản màu sắc:** Tất cả văn bản và thành phần UI tương tác phải đạt tỷ lệ tương phản tối thiểu 4.5:1 (cho văn bản thường) và 3:1 (cho văn bản lớn) theo WCAG.
-   **Hỗ trợ điều hướng bằng bàn phím:**
    -   Người dùng phải có thể truy cập và tương tác với tất cả các thành phần UI bằng bàn phím (Tab, Shift+Tab, Enter, Spacebar, Arrow keys).
    -   Trạng thái focus (`:focus-visible`) phải rõ ràng để người dùng biết họ đang ở đâu.
    -   Sử dụng các phím tắt hợp lý để tăng năng suất.
-   **Tương thích với trình đọc màn hình (Screen Reader):**
    -   Sử dụng cấu trúc HTML/JSX ngữ nghĩa (semantic HTML/JSX).
    -   Cung cấp các thuộc tính ARIA (Accessible Rich Internet Applications) cần thiết cho các thành phần UI phức tạp (ví dụ: `aria-label`, `role`, `aria-live`).
    -   Mọi icon quan trọng đều phải có mô tả văn bản thay thế.
-   **Kích thước vùng chạm:** Đảm bảo các thành phần tương tác có kích thước tối thiểu 44x44px để dễ dàng chạm vào trên các thiết bị cảm ứng.
-   **Không phụ thuộc màu sắc:** Thông tin quan trọng (trạng thái thành công/thất bại, cảnh báo) phải được truyền tải không chỉ bằng màu sắc mà còn bằng icon, văn bản hoặc các yếu tố trực quan khác.
-   **Hỗ trợ chế độ tương phản cao:** Đảm bảo giao diện vẫn dễ sử dụng khi người dùng kích hoạt chế độ tương phản cao của hệ điều hành.

### Testing Strategy

Để đảm bảo thiết kế responsive và khả năng tiếp cận, chúng ta sẽ áp dụng chiến lược kiểm thử toàn diện:

-   **Kiểm thử Responsive:**
    -   **Kiểm thử thủ công:** Thay đổi kích thước cửa sổ ứng dụng trên Windows 11.
    -   **Kiểm thử thiết bị:** Nếu có các thiết bị tablet chạy Windows, thực hiện kiểm thử trực tiếp trên đó.
    -   **Kiểm thử trên các màn hình khác nhau:** Đảm bảo bố cục hiển thị tốt trên các độ phân giải phổ biến.
-   **Kiểm thử Khả năng tiếp cận:**
    -   **Công cụ tự động:** Sử dụng các công cụ như `Axe DevTools`, `Lighthouse` (nếu tích hợp được với Tauri/WebView) để quét các lỗi cơ bản.
    -   **Trình đọc màn hình:** Thực hiện kiểm thử với các trình đọc màn hình phổ biến như NVDA hoặc JAWS (trên Windows).
    -   **Điều hướng bằng bàn phím:** Kiểm tra toàn bộ ứng dụng chỉ bằng bàn phím.
    -   **Mô phỏng mù màu:** Sử dụng các công cụ mô phỏng để kiểm tra trải nghiệm của người dùng mù màu.
-   **Kiểm thử người dùng:** Bao gồm người dùng có nhu cầu đặc biệt trong các phiên kiểm thử người dùng cuối để nhận phản hồi trực tiếp.

### Implementation Guidelines

-   **Phát triển Responsive:**
    -   Sử dụng CSS Flexbox và Grid để xây dựng bố cục linh hoạt.
    -   Ưu tiên các đơn vị tương đối (rem, em, %) thay vì pixel cố định.
    -   Thiết kế các component có khả năng co giãn và thích ứng.
-   **Phát triển Khả năng tiếp cận:**
    -   Luôn sử dụng các thành phần Fluent UI React có hỗ trợ A11y.
    -   Viết HTML ngữ nghĩa, đảm bảo cấu trúc heading (H1, H2) và các landmark region (main, nav, aside) chính xác.
    -   Cung cấp `alt text` cho tất cả các hình ảnh có ý nghĩa.
    -   Đảm bảo các thành phần tương tác có `tabIndex` và `role` phù hợp.
    -   Xử lý focus management khi các modal hoặc dialog được mở/đóng.
