// Tauri event listeners - frontend subscribes to native events
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { JobProgress, JobStatus } from '../../types/job';
import type { SyncEventLog } from '../../types/log';

// Event names matching Rust backend (kebab-case as per architecture)
export const TAURI_EVENTS = {
  SYNC_QUEUED: 'sync:queued',
  SYNC_STARTED: 'sync:started',
  SYNC_PROGRESS: 'sync:progress',
  SYNC_COMPLETED: 'sync:completed',
  SYNC_FAILED: 'sync:failed',
} as const;

// Event payloads
export interface SyncQueuedPayload {
  profileId: string;
  jobId: string;
  timestamp: string;
}

export interface SyncStartedPayload {
  profileId: string;
  jobId: string;
  timestamp: string;
}

export interface SyncProgressPayload {
  profileId: string;
  jobId: string;
  timestamp: string;
  phase: 'scanning' | 'comparing' | 'copying' | 'finalizing';
  processedItems: number;
  totalItems: number | null;
  currentPath: string | null;
}

export interface SyncCompletedPayload {
  profileId: string;
  jobId: string;
  timestamp: string;
  status: 'succeeded' | 'failed' | 'skipped';
  itemsProcessed: number | null;
  itemsFailed: number | null;
  errorCode: string | null;
  errorMessage: string | null;
}

export interface SyncFailedPayload {
  profileId: string;
  jobId: string;
  timestamp: string;
  errorCode: string;
  errorMessage: string;
}

export interface SyncSkippedPayload {
  profileId: string;
  jobId: string;
  reason: string;
  skippedAt: string;
}

export type SyncEventPayload = 
  | SyncQueuedPayload 
  | SyncStartedPayload 
  | SyncProgressPayload 
  | SyncCompletedPayload 
  | SyncFailedPayload
  | SyncSkippedPayload;

// Event listener wrapper with error handling (Story 3.2 AC#2)
export async function listenToSyncEvents(
  handlers: {
    onQueued?: (payload: SyncQueuedPayload) => void;
    onStarted?: (payload: SyncStartedPayload) => void;
    onProgress?: (payload: SyncProgressPayload) => void;
    onCompleted?: (payload: SyncCompletedPayload) => void;
    onFailed?: (payload: SyncFailedPayload) => void;
  }
): Promise<UnlistenFn[]> {
  const unlisteners: UnlistenFn[] = [];

  try {
    if (handlers.onQueued) {
      const unlisten = await listen<SyncQueuedPayload>(TAURI_EVENTS.SYNC_QUEUED, (event) => {
        handlers.onQueued!(event.payload);
      });
      unlisteners.push(unlisten);
    }

    if (handlers.onStarted) {
      const unlisten = await listen<SyncStartedPayload>(TAURI_EVENTS.SYNC_STARTED, (event) => {
        handlers.onStarted!(event.payload);
      });
      unlisteners.push(unlisten);
    }

    if (handlers.onProgress) {
      const unlisten = await listen<SyncProgressPayload>(TAURI_EVENTS.SYNC_PROGRESS, (event) => {
        handlers.onProgress!(event.payload);
      });
      unlisteners.push(unlisten);
    }

    if (handlers.onCompleted) {
      const unlisten = await listen<SyncCompletedPayload>(TAURI_EVENTS.SYNC_COMPLETED, (event) => {
        handlers.onCompleted!(event.payload);
      });
      unlisteners.push(unlisten);
    }

    if (handlers.onFailed) {
      const unlisten = await listen<SyncFailedPayload>(TAURI_EVENTS.SYNC_FAILED, (event) => {
        handlers.onFailed!(event.payload);
      });
      unlisteners.push(unlisten);
    }
  } catch (error) {
    console.error('[Events] Failed to setup event listeners:', error);
    // Clean up any listeners that were set up
    unlisteners.forEach(unlisten => unlisten());
    throw error;
  }

  return unlisteners;
}

// Refetch mechanism if state desync detected (Story 3.2 AC#2)
export async function refetchJobStatus(profileId: string): Promise<void> {
  // Import here to avoid circular dependency
  const { getLastJobStatus } = await import('./commands');
  try {
    const result = await getLastJobStatus(profileId);
    if (result.error) {
      console.warn('[Events] Refetch failed:', result.error);
    }
  } catch (error) {
    console.error('[Events] Refetch error:', error);
  }
}
