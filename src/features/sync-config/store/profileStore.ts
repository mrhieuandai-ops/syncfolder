// Profile store for managing sync profile state
import { create } from 'zustand';
import type { Profile, ProfileSummary } from '../../../types/profile';
import * as tauriCommands from '../../../services/tauri/commands';
import { isSuccess } from '../../../services/tauri/contracts';

// Cache configuration
const CACHE_TTL_MS = 30 * 1000; // 30 seconds

interface CacheEntry<T> {
  data: T;
  timestamp: number;
}

interface ProfileState {
  profiles: Profile[];
  selectedProfile: Profile | null;
  isLoading: boolean;
  error: string | null;
  cache: CacheEntry<Profile[]> | null;
  
  // Actions
  loadProfiles: () => Promise<void>;
  selectProfile: (profile: Profile | null) => void;
  createProfile: (data: CreateProfileData) => Promise<Profile>;
  updateProfile: (data: UpdateProfileData) => Promise<Profile>;
  deleteProfile: (id: string) => Promise<void>;
  clearError: () => void;
  invalidateCache: () => void;
}

function isCacheValid<T>(entry: CacheEntry<T> | null): boolean {
  if (!entry) return false;
  return Date.now() - entry.timestamp < CACHE_TTL_MS;
}

interface CreateProfileData {
  name: string;
  sourcePath: string;
  destinationPath: string;
  scheduleMinutes: number | null;
  isEnabled: boolean;
}

interface UpdateProfileData {
  id: string;
  name?: string;
  sourcePath?: string;
  destinationPath?: string;
  scheduleMinutes?: number | null;
  isEnabled?: boolean;
}

export const useProfileStore = create<ProfileState>((set, get) => ({
  profiles: [],
  selectedProfile: null,
  isLoading: false,
  error: null,
  cache: null,

  loadProfiles: async () => {
    const state = get();
    const cache = state.cache;
    
    // Return cached data if valid
    if (cache && isCacheValid(cache)) {
      set({ profiles: cache.data, isLoading: false });
      return;
    }

    set({ isLoading: true, error: null });
    try {
      const result = await tauriCommands.listProfiles();
      if (isSuccess(result)) {
        const profiles = result.data as Profile[];
        set({ 
          profiles, 
          cache: { data: profiles, timestamp: Date.now() },
          isLoading: false 
        });
      } else {
        set({ error: result.error?.message || 'Failed to load profiles', isLoading: false });
      }
    } catch (err) {
      set({ error: (err as Error).message, isLoading: false });
    }
  },

  selectProfile: (profile) => {
    set({ selectedProfile: profile });
  },

  createProfile: async (data) => {
    set({ isLoading: true, error: null });
    try {
      const result = await tauriCommands.createProfile({
        name: data.name,
        sourcePath: data.sourcePath,
        destinationPath: data.destinationPath,
        scheduleMinutes: data.scheduleMinutes,
        isEnabled: data.isEnabled,
      });
      
      if (isSuccess(result)) {
        const newProfile = result.data as Profile;
        set(state => ({
          profiles: [...state.profiles, newProfile],
          cache: { data: [...state.profiles, newProfile], timestamp: Date.now() },
          isLoading: false,
        }));
        return newProfile;
      } else {
        set({ error: result.error?.message || 'Failed to create profile', isLoading: false });
        throw new Error(result.error?.message || 'Failed to create profile');
      }
    } catch (err) {
      set({ error: (err as Error).message, isLoading: false });
      throw err;
    }
  },

  updateProfile: async (data) => {
    set({ isLoading: true, error: null });
    try {
      const result = await tauriCommands.updateProfile({
        id: data.id,
        name: data.name,
        sourcePath: data.sourcePath,
        destinationPath: data.destinationPath,
        scheduleMinutes: data.scheduleMinutes,
        isEnabled: data.isEnabled,
      });
      
      if (isSuccess(result)) {
        const updatedProfile = result.data as Profile;
        set(state => {
          const newProfiles = state.profiles.map(p => p.id === data.id ? updatedProfile : p);
          return {
            profiles: newProfiles,
            selectedProfile: state.selectedProfile?.id === data.id ? updatedProfile : state.selectedProfile,
            cache: { data: newProfiles, timestamp: Date.now() },
            isLoading: false,
          };
        });
        return updatedProfile;
      } else {
        set({ error: result.error?.message || 'Failed to update profile', isLoading: false });
        throw new Error(result.error?.message || 'Failed to update profile');
      }
    } catch (err) {
      set({ error: (err as Error).message, isLoading: false });
      throw err;
    }
  },

  deleteProfile: async (id) => {
    set({ isLoading: true, error: null });
    try {
      const result = await tauriCommands.deleteProfile(id);
      if (isSuccess(result)) {
        set(state => {
          const newProfiles = state.profiles.filter(p => p.id !== id);
          return {
            profiles: newProfiles,
            selectedProfile: state.selectedProfile?.id === id ? null : state.selectedProfile,
            cache: { data: newProfiles, timestamp: Date.now() },
            isLoading: false,
          };
        });
      } else {
        set({ error: result.error?.message || 'Failed to delete profile', isLoading: false });
        throw new Error(result.error?.message || 'Failed to delete profile');
      }
    } catch (err) {
      set({ error: (err as Error).message, isLoading: false });
      throw err;
    }
  },

  clearError: () => {
    set({ error: null });
  },

  invalidateCache: () => {
    set({ cache: null });
  },
}));
