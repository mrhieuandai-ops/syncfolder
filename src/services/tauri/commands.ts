// Tauri command wrappers - bridge between frontend and Rust backend
import { invoke } from '@tauri-apps/api/core';
import { TAURI_COMMANDS, type CommandResult } from './contracts';
import type { Profile } from '../../types/profile';
import type { SyncJob, LastJobInfo } from '../../types/job';
import type { SyncEventLog, JobLogSummary } from '../../types/log';
import type { CreateProfileRequest, UpdateProfileRequest } from './contracts';

// Profile commands
export async function createProfile(request: CreateProfileRequest): Promise<CommandResult<Profile>> {
  return invoke(TAURI_COMMANDS.create_profile, { request });
}

export async function updateProfile(request: UpdateProfileRequest): Promise<CommandResult<Profile>> {
  return invoke(TAURI_COMMANDS.update_profile, { request });
}

export async function deleteProfile(id: string): Promise<CommandResult<void>> {
  return invoke(TAURI_COMMANDS.delete_profile, { id });
}

export async function getProfile(id: string): Promise<CommandResult<Profile>> {
  return invoke(TAURI_COMMANDS.get_profile, { id });
}

export async function listProfiles(): Promise<CommandResult<Profile[]>> {
  return invoke(TAURI_COMMANDS.list_profiles);
}

// Sync commands
export async function runSyncNow(profileId: string): Promise<CommandResult<{ jobId: string; profileId: string }>> {
  return invoke(TAURI_COMMANDS.run_sync_now, { profileId });
}

// Status commands
export async function getLastJobStatus(profileId: string): Promise<CommandResult<LastJobInfo | null>> {
  return invoke(TAURI_COMMANDS.get_last_job_status, { profileId });
}

export async function getProfileStatus(profileId: string): Promise<CommandResult<{
  profile: Profile;
  lastJob: SyncJob | null;
}>> {
  return invoke(TAURI_COMMANDS.get_profile_status, { profileId });
}

// Log commands
export async function getJobLogs(jobId: string): Promise<CommandResult<SyncEventLog[]>> {
  return invoke(TAURI_COMMANDS.get_job_logs, { jobId });
}

export async function listJobs(profileId: string, limit: number = 50): Promise<CommandResult<JobLogSummary[]>> {
  return invoke(TAURI_COMMANDS.list_jobs, { profileId, limit });
}

// App settings (stub for now)
export async function getAppSettings(): Promise<CommandResult<Record<string, unknown>>> {
  return invoke(TAURI_COMMANDS.get_app_settings);
}

export async function updateAppSettings(settings: Record<string, unknown>): Promise<CommandResult<void>> {
  return invoke(TAURI_COMMANDS.update_app_settings, { settings });
}
