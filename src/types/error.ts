// Error types matching Rust backend error codes
export interface AppError {
  code: string;
  message: string;
  details: string | null;
  retryable: boolean;
}

export interface ErrorCodeInfo {
  code: string;
  group: 'availability' | 'access' | 'lock' | 'validation';
  message: string;
  retryable: boolean;
}

// Predefined error codes (NFR12)
export const ERROR_CODES = {
  // Availability errors
  SOURCE_NOT_FOUND: 'SOURCE_NOT_FOUND',
  DESTINATION_NOT_FOUND: 'DESTINATION_NOT_FOUND',
  SOURCE_UNAVAILABLE: 'SOURCE_UNAVAILABLE',
  DESTINATION_UNAVAILABLE: 'DESTINATION_UNAVAILABLE',
  
  // Access errors
  ACCESS_DENIED_SOURCE: 'ACCESS_DENIED_SOURCE',
  ACCESS_DENIED_DESTINATION: 'ACCESS_DENIED_DESTINATION',
  
  // Lock errors
  FILE_LOCKED: 'FILE_LOCKED',
  
  // Validation errors
  INVALID_CONFIG: 'INVALID_CONFIG',
  PROFILE_NOT_FOUND: 'PROFILE_NOT_FOUND',
  JOB_RUNNING: 'JOB_RUNNING',
} as const;

export type ErrorCode = typeof ERROR_CODES[keyof typeof ERROR_CODES];
