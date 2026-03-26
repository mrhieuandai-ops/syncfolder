// Job status enum matching Rust backend
export type JobStatus = 
  | 'scheduled'
  | 'queued'
  | 'running'
  | 'succeeded'
  | 'failed'
  | 'skipped';

export interface SyncJob {
  id: string;
  profileId: string;
  status: JobStatus;
  startedAt: string | null;
  completedAt: string | null;
  itemsProcessed: number | null;
  itemsFailed: number | null;
  errorCode: string | null;
  errorMessage: string | null;
}

export interface JobProgress {
  jobId: string;
  profileId: string;
  phase: 'scanning' | 'comparing' | 'copying' | 'finalizing';
  processedItems: number;
  totalItems: number | null;
  currentPath: string | null;
}

export interface LastJobInfo {
  profileId: string;
  jobId: string;
  status: JobStatus;
  completedAt: string;
  itemsProcessed: number | null;
}

// Status display text (not color-only - NFR10)
export const STATUS_DISPLAY: Record<JobStatus, { text: string; icon: string }> = {
  scheduled: { text: 'Đã lên lịch', icon: 'Clock' },
  queued: { text: 'Đang chờ', icon: 'ArrowSync' },
  running: { text: 'Đang chạy', icon: 'Sync' },
  succeeded: { text: 'Thành công', icon: 'CheckmarkCircle' },
  failed: { text: 'Thất bại', icon: 'ErrorCircle' },
  skipped: { text: 'Đã bỏ qua', icon: 'Cancel' },
};

export function getStatusDisplay(status: JobStatus): { text: string; icon: string } {
  return STATUS_DISPLAY[status] ?? { text: status, icon: 'Question' };
}
