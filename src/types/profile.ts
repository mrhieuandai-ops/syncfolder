// Profile represents a sync profile configuration
export interface Profile {
  id: string;
  name: string;
  sourcePath: string;
  destinationPath: string;
  scheduleMinutes: number | null; // null means manual only
  isEnabled: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface ProfileSummary {
  id: string;
  name: string;
  sourcePath: string;
  destinationPath: string;
  scheduleMinutes: number | null;
  isEnabled: boolean;
}

export type ScheduleOption = 30 | 60 | 90 | 1440; // minutes

export const SCHEDULE_OPTIONS: { value: ScheduleOption; label: string }[] = [
  { value: 30, label: '30 phút' },
  { value: 60, label: '60 phút' },
  { value: 90, label: '90 phút' },
  { value: 1440, label: '1 ngày' },
];

export function formatSchedule(minutes: number | null): string {
  if (minutes === null) return 'Thủ công';
  const option = SCHEDULE_OPTIONS.find(o => o.value === minutes);
  return option?.label ?? `${minutes} phút`;
}
