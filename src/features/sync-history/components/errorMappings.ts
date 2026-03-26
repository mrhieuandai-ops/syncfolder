// Error to user message mapping (Story 3.4 AC#2)
// Maps error codes to Vietnamese user-friendly messages with suggestions
import type { ErrorClassification } from '../../../types/log';
import { ERROR_CODES } from '../../../types/error';

// Error classification mappings (NFR12 - 4 error groups)
export function mapErrorToUserMessage(errorCode: string): string {
  switch (errorCode) {
    // Availability errors
    case ERROR_CODES.SOURCE_NOT_FOUND:
      return 'Thư mục nguồn không tồn tại';
    case ERROR_CODES.DESTINATION_NOT_FOUND:
      return 'Thư mục đích không tồn tại';
    case ERROR_CODES.SOURCE_UNAVAILABLE:
      return 'Thư mục nguồn hiện không khả dụng';
    case ERROR_CODES.DESTINATION_UNAVAILABLE:
      return 'Thư mục đích hiện không khả dụng';
    
    // Access errors
    case ERROR_CODES.ACCESS_DENIED_SOURCE:
      return 'Không có quyền truy cập thư mục nguồn';
    case ERROR_CODES.ACCESS_DENIED_DESTINATION:
      return 'Không có quyền truy cập thư mục đích';
    
    // Lock errors
    case ERROR_CODES.FILE_LOCKED:
      return 'File đang bị khóa bởi ứng dụng khác';
    
    // Validation errors
    case ERROR_CODES.INVALID_CONFIG:
      return 'Cấu hình hồ sơ không hợp lệ';
    case ERROR_CODES.PROFILE_NOT_FOUND:
      return 'Không tìm thấy hồ sơ đồng bộ';
    case ERROR_CODES.JOB_RUNNING:
      return 'Đang có job đồng bộ khác đang chạy';
    
    default:
      return `Lỗi không xác định: ${errorCode}`;
  }
}

// Get error classification group (NFR12)
export function getErrorGroup(errorCode: string): 'availability' | 'access' | 'lock' | 'validation' | null {
  switch (errorCode) {
    case ERROR_CODES.SOURCE_NOT_FOUND:
    case ERROR_CODES.DESTINATION_NOT_FOUND:
    case ERROR_CODES.SOURCE_UNAVAILABLE:
    case ERROR_CODES.DESTINATION_UNAVAILABLE:
      return 'availability';
    
    case ERROR_CODES.ACCESS_DENIED_SOURCE:
    case ERROR_CODES.ACCESS_DENIED_DESTINATION:
      return 'access';
    
    case ERROR_CODES.FILE_LOCKED:
      return 'lock';
    
    case ERROR_CODES.INVALID_CONFIG:
    case ERROR_CODES.PROFILE_NOT_FOUND:
    case ERROR_CODES.JOB_RUNNING:
      return 'validation';
    
    default:
      return null;
  }
}

// Get full error classification info
export function getErrorClassification(errorCode: string): ErrorClassification | null {
  const group = getErrorGroup(errorCode);
  if (!group) return null;
  
  const userMessage = mapErrorToUserMessage(errorCode);
  const retryable = ['SOURCE_UNAVAILABLE', 'DESTINATION_UNAVAILABLE', 'FILE_LOCKED'].includes(errorCode);
  
  return {
    group,
    code: errorCode,
    message: userMessage,
    retryable,
    userMessage,
    suggestedAction: getSuggestion(errorCode),
  };
}

function getSuggestion(errorCode: string): string {
  switch (errorCode) {
    case ERROR_CODES.SOURCE_NOT_FOUND:
    case ERROR_CODES.DESTINATION_NOT_FOUND:
      return 'Kiểm tra lại đường dẫn thư mục trong cấu hình hồ sơ.';
    
    case ERROR_CODES.SOURCE_UNAVAILABLE:
    case ERROR_CODES.DESTINATION_UNAVAILABLE:
      return 'Đảm bảo ổ đĩa đã được kết nối và thử chạy lại.';
    
    case ERROR_CODES.ACCESS_DENIED_SOURCE:
    case ERROR_CODES.ACCESS_DENIED_DESTINATION:
      return 'Chạy SyncFolder với quyền Administrator hoặc kiểm tra quyền NTFS.';
    
    case ERROR_CODES.FILE_LOCKED:
      return 'Đóng các ứng dụng đang sử dụng file (Word, Excel,...) và thử chạy lại.';
    
    case ERROR_CODES.INVALID_CONFIG:
      return 'Mở cấu hình hồ sơ và kiểm tra lại thông tin.';
    
    case ERROR_CODES.PROFILE_NOT_FOUND:
      return 'Hồ sơ có thể đã bị xóa. Vui lòng tạo hồ sơ mới.';
    
    case ERROR_CODES.JOB_RUNNING:
      return 'Đợi job hiện tại hoàn thành hoặc khởi động lại ứng dụng.';
    
    default:
      return 'Liên hệ bộ phận hỗ trợ nếu lỗi tiếp tục xảy ra.';
  }
}

// Get suggestion for error (exported for use in LogDetailView)
export function getSuggestionForError(errorCode: string): string | null {
  switch (errorCode) {
    case ERROR_CODES.SOURCE_NOT_FOUND:
    case ERROR_CODES.DESTINATION_NOT_FOUND:
      return 'Kiểm tra lại đường dẫn thư mục và đảm bảo ổ đĩa đã được kết nối.';
    case ERROR_CODES.SOURCE_UNAVAILABLE:
    case ERROR_CODES.DESTINATION_UNAVAILABLE:
      return 'Đảm bảo ổ đĩa ngoài đã được cắm và thử chạy lại.';
    case ERROR_CODES.ACCESS_DENIED_SOURCE:
    case ERROR_CODES.ACCESS_DENIED_DESTINATION:
      return 'Kiểm tra quyền truy cập thư mục trong Windows Explorer.';
    case ERROR_CODES.FILE_LOCKED:
      return 'Đóng các ứng dụng đang sử dụng file và thử chạy lại.';
    case ERROR_CODES.INVALID_CONFIG:
      return 'Vui lòng kiểm tra lại cấu hình hồ sơ.';
    default:
      return null;
  }
}
