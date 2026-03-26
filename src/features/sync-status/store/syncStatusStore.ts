// Zustand store for sync status dashboard (Story 3.1)
import { create } from 'zustand';
import type { UnlistenFn } from '@tauri-apps/api/event';
import type { ProfileSummary } from '../../../types/profile';
import type { SyncJob, JobStatus } from '../../../types/job';
import { listenToSyncEvents, type SyncCompletedPayload, type SyncFailedPayload, type SyncProgressPayload, type SyncStartedPayload, type SyncQueuedPayload } from '../../../services/tauri/events';
import { getProfileStatus } from '../../../services/tauri/commands';

export interface ProfileWithStatus extends ProfileSummary {
  lastJob: SyncJob | null;
  currentJob: SyncJob | null; // Currently running job
}

interface DashboardState {
  // State
  profiles: ProfileWithStatus[];
  isLoading: boolean;
  error: string | null;
  isInitialized: boolean;
  unlistenFns: UnlistenFn[]; // Store unlisten functions for cleanup
  
  // Actions
  initialize: () => Promise<void>;
  refreshProfiles: () => Promise<void>;
  setProfileStatus: (profileId: string, job: SyncJob | null) => void;
  updateJobProgress: (profileId: string, progress: SyncProgressPayload) => void;
  handleJobStarted: (payload: SyncStartedPayload) => void;
  handleJobQueued: (payload: SyncQueuedPayload) => void;
  handleJobCompleted: (payload: SyncCompletedPayload) => void;
  handleJobFailed: (payload: SyncFailedPayload) => void;
  cleanup: () => Promise<void>;
}

export const useSyncStatusStore = create<DashboardState>((set, get) => ({
  profiles: [],
  isLoading: false,
  error: null,
  isInitialized: false,
  unlistenFns: [],

  initialize: async () => {
    if (get().isInitialized) return;
    
    set({ isLoading: true, error: null });
    
    try {
      // Initial load of profiles with status
      await get().refreshProfiles();
      
      // Set up event listeners (NFR3: UI updates within 5 seconds)
      // Store unlisten functions for cleanup
      const unlisteners = await listenToSyncEvents({
        onQueued: (payload) => get().handleJobQueued(payload),
        onStarted: (payload) => get().handleJobStarted(payload),
        onProgress: (payload) => get().updateJobProgress(payload.profileId, payload),
        onCompleted: (payload) => get().handleJobCompleted(payload),
        onFailed: (payload) => get().handleJobFailed(payload),
      });
      
      set({ isInitialized: true, unlistenFns: unlisteners });
    } catch (error) {
      console.error('[Dashboard] Initialize error:', error);
      set({ error: 'Không thể khởi tạo dashboard' });
    } finally {
      set({ isLoading: false });
    }
  },

  refreshProfiles: async () => {
    try {
      // Import dynamically to avoid circular deps
      const { listProfiles } = await import('../../../services/tauri/commands');
      const result = await listProfiles();
      
      if (result.error || !result.data) {
        throw new Error(result.error?.message ?? 'Failed to load profiles');
      }
      
      // Load last job status for each profile
      const profilesWithStatus: ProfileWithStatus[] = await Promise.all(
        result.data.map(async (profile) => {
          const statusResult = await getProfileStatus(profile.id);
          return {
            ...profile,
            lastJob: statusResult.data?.lastJob ?? null,
            currentJob: null,
          };
        })
      );
      
      set({ profiles: profilesWithStatus });
    } catch (error) {
      console.error('[Dashboard] Refresh error:', error);
      set({ error: 'Không thể tải danh sách hồ sơ' });
    }
  },

  setProfileStatus: (profileId: string, job: SyncJob | null) => {
    set((state) => ({
      profiles: state.profiles.map((p) =>
        p.id === profileId ? { ...p, lastJob: job, currentJob: null } : p
      ),
    }));
  },

  updateJobProgress: (profileId: string, progress: SyncProgressPayload) => {
    set((state) => ({
      profiles: state.profiles.map((p) => {
        if (p.id !== profileId) return p;
        
        const currentJob: SyncJob = {
          id: progress.jobId,
          profileId: progress.profileId,
          status: 'running',
          startedAt: progress.timestamp,
          completedAt: null,
          itemsProcessed: progress.processedItems,
          itemsFailed: null,
          errorCode: null,
          errorMessage: null,
        };
        
        return { ...p, currentJob };
      }),
    }));
  },

  handleJobQueued: (payload: SyncQueuedPayload) => {
    set((state) => ({
      profiles: state.profiles.map((p) => {
        if (p.id !== payload.profileId) return p;
        
        const queuedJob: SyncJob = {
          id: payload.jobId,
          profileId: payload.profileId,
          status: 'queued',
          startedAt: payload.timestamp,
          completedAt: null,
          itemsProcessed: null,
          itemsFailed: null,
          errorCode: null,
          errorMessage: null,
        };
        
        return { ...p, currentJob: queuedJob };
      }),
    }));
  },

  handleJobStarted: (payload: SyncStartedPayload) => {
    set((state) => ({
      profiles: state.profiles.map((p) => {
        if (p.id !== payload.profileId) return p;
        
        const startedJob: SyncJob = {
          id: payload.jobId,
          profileId: payload.profileId,
          status: 'running',
          startedAt: payload.timestamp,
          completedAt: null,
          itemsProcessed: null,
          itemsFailed: null,
          errorCode: null,
          errorMessage: null,
        };
        
        return { ...p, currentJob: startedJob };
      }),
    }));
  },

  handleJobCompleted: (payload: SyncCompletedPayload) => {
    const { profileId, jobId, timestamp, status, itemsProcessed, itemsFailed, errorCode, errorMessage } = payload;
    
    set((state) => ({
      profiles: state.profiles.map((p) => {
        if (p.id !== profileId) return p;
        
        const completedJob: SyncJob = {
          id: jobId,
          profileId,
          status: status as JobStatus,
          startedAt: p.currentJob?.startedAt ?? null,
          completedAt: timestamp,
          itemsProcessed: itemsProcessed ?? null,
          itemsFailed: itemsFailed ?? null,
          errorCode: errorCode ?? null,
          errorMessage: errorMessage ?? null,
        };
        
        return { ...p, lastJob: completedJob, currentJob: null };
      }),
    }));
  },

  handleJobFailed: (payload: SyncFailedPayload) => {
    const { profileId, jobId, timestamp, errorCode, errorMessage } = payload;
    
    set((state) => ({
      profiles: state.profiles.map((p) => {
        if (p.id !== profileId) return p;
        
        const failedJob: SyncJob = {
          id: jobId,
          profileId,
          status: 'failed',
          startedAt: p.currentJob?.startedAt ?? null,
          completedAt: timestamp,
          itemsProcessed: p.currentJob?.itemsProcessed ?? null,
          itemsFailed: null,
          errorCode,
          errorMessage,
        };
        
        return { ...p, lastJob: failedJob, currentJob: null };
      }),
    }));
  },

  cleanup: async () => {
    // Clean up event listeners to prevent memory leaks
    const { unlistenFns } = get();
    unlistenFns.forEach((unlisten) => unlisten());
    set({ isInitialized: false, unlistenFns: [] });
  },
}));
