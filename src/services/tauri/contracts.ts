// Tauri command contracts - TypeScript interfaces matching Rust structs
import type { Profile, ProfileSummary } from '../../types/profile';
import type { SyncJob, JobProgress, LastJobInfo, JobStatus } from '../../types/job';
import type { SyncEventLog, JobLogSummary } from '../../types/log';
import type { AppError } from '../../types/error';

// Command response wrapper
export interface CommandResult<T> {
  data: T | null;
  error: AppError | null;
}

export function isSuccess<T>(result: CommandResult<T>): result is CommandResult<T> & { data: T } {
  return result.error === null && result.data !== null;
}

// Profile commands
export interface CreateProfileRequest {
  name: string;
  sourcePath: string;
  destinationPath: string;
  scheduleMinutes: number | null;
  isEnabled: boolean;
}

export interface UpdateProfileRequest {
  id: string;
  name?: string;
  sourcePath?: string;
  destinationPath?: string;
  scheduleMinutes?: number | null;
  isEnabled?: boolean;
}

// Sync commands
export interface RunSyncResult {
  jobId: string;
  profileId: string;
}

// Status commands
export interface GetLastJobStatusResult {
  profileId: string;
  job: SyncJob | null;
}

// Log commands
export interface GetJobLogsResult {
  jobId: string;
  logs: SyncEventLog[];
}

export interface ListJobsResult {
  jobs: JobLogSummary[];
}

// Tauri command names (snake_case as per architecture)
export const TAURI_COMMANDS = {
  // Profile commands
  create_profile: 'create_profile',
  update_profile: 'update_profile',
  delete_profile: 'delete_profile',
  get_profile: 'get_profile',
  list_profiles: 'list_profiles',
  
  // Sync commands
  run_sync_now: 'run_sync_now',
  
  // Status commands
  get_last_job_status: 'get_last_job_status',
  get_profile_status: 'get_profile_status',
  
  // Log commands
  get_job_logs: 'get_job_logs',
  list_jobs: 'list_jobs',
  
  // Settings commands
  get_app_settings: 'get_app_settings',
  update_app_settings: 'update_app_settings',
} as const;
