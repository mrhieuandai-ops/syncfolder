// Log entry types for sync jobs (NFR11, NFR12)
export interface SyncEventLog {
  id: string;
  jobId: string;
  profileId: string;
  timestamp: string;
  eventType: 'job_started' | 'job_progress' | 'job_completed' | 'job_failed' | 'error';
  message: string;
  details: string | null;
  errorCode: string | null;
}

export interface JobLogSummary {
  jobId: string;
  profileId: string;
  startedAt: string;
  completedAt: string | null;
  status: string;
  itemsProcessed: number | null;
  itemsFailed: number | null;
  errorCode: string | null;
  errorMessage: string | null;
}

// Error classification (NFR12)
export type ErrorGroup = 'availability' | 'access' | 'lock' | 'validation';

export interface ErrorClassification {
  group: ErrorGroup;
  code: string;
  message: string;
  retryable: boolean;
  userMessage: string; // Vietnamese localized
  suggestedAction: string;
}

// Log detail view entry
export interface LogEntry {
  id: string;
  timestamp: string;
  level: 'info' | 'warning' | 'error' | 'success';
  message: string;
  details: string | null;
  errorCode: string | null;
}
