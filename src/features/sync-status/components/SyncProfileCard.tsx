// SyncProfileCard component (Story 3.1 AC#1-3)
// Displays profile status with text status (not color-only - NFR10)
import React from 'react';
import { Card, Text, Button, Tooltip, ProgressBar } from '@fluentui/react-components';
import { CheckmarkCircle24Regular, ErrorCircle24Regular, ArrowSync24Regular, Clock24Regular, Play24Regular } from '@fluentui/react-icons';
import type { ProfileWithStatus } from '../store/syncStatusStore';
import { formatSchedule } from '../../../types/profile';
import { getStatusDisplay } from '../../../types/job';

interface SyncProfileCardProps {
  profile: ProfileWithStatus;
}

export const SyncProfileCard: React.FC<SyncProfileCardProps> = ({ profile }) => {
  const isRunning = profile.currentJob?.status === 'running';
  const status = profile.currentJob?.status ?? profile.lastJob?.status ?? 'scheduled';
  const statusDisplay = getStatusDisplay(status);
  
  const lastRunText = profile.lastJob?.completedAt
    ? formatLastRun(profile.lastJob.completedAt)
    : 'Chưa chạy';

  const StatusIcon = isRunning 
    ? ArrowSync24Regular 
    : status === 'succeeded' 
      ? CheckmarkCircle24Regular 
      : status === 'failed' 
        ? ErrorCircle24Regular 
        : Clock24Regular;

  return (
    <Card className="sync-profile-card" style={{ padding: '16px', marginBottom: '12px' }}>
      {/* Header with profile name and status */}
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', marginBottom: '12px' }}>
        <div>
          <Text weight="semibold" size={300}>{profile.name}</Text>
          <Text size={200} style={{ display: 'block', color: '#616161' }}>
            {profile.sourcePath}
          </Text>
          <Text size={200} style={{ display: 'block', color: '#616161' }}>
            {profile.destinationPath}
          </Text>
        </div>
        
        {/* Status badge with text (not color-only) */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <StatusIcon />
          <Text size={200} weight="medium">{statusDisplay.text}</Text>
        </div>
      </div>
      
      {/* Schedule info */}
      <div style={{ marginBottom: '12px' }}>
        <Text size={200} style={{ color: '#616161' }}>
          Lịch: {formatSchedule(profile.scheduleMinutes)}
        </Text>
      </div>
      
      {/* Last run info */}
      <div style={{ marginBottom: '12px' }}>
        <Text size={200} style={{ color: '#616161' }}>
          Lần chạy gần nhất: {lastRunText}
        </Text>
        {profile.lastJob?.itemsProcessed !== null && profile.lastJob?.itemsProcessed !== undefined && (
          <Text size={200} style={{ display: 'block', color: '#616161' }}>
            Đã xử lý: {profile.lastJob.itemsProcessed} file
          </Text>
        )}
      </div>
      
      {/* Running progress indicator */}
      {isRunning && profile.currentJob && (
        <div style={{ marginBottom: '12px' }}>
          <Text size={200} style={{ color: '#616161' }}>Đang đồng bộ...</Text>
          <ProgressBar 
            value={profile.currentJob.itemsProcessed !== null ? 0.5 : 0} 
          />
        </div>
      )}
      
      {/* Error message if failed (NFR10 - text, not color-only) */}
      {status === 'failed' && profile.lastJob?.errorMessage && (
        <div style={{ 
          marginBottom: '12px', 
          padding: '8px', 
          backgroundColor: '#FEF2F2', 
          borderRadius: '4px',
          border: '1px solid #D32F2F'
        }}>
          <Text size={200} style={{ color: '#D32F2F' }}>
            <ErrorCircle24Regular style={{ marginRight: '4px' }} />
            {profile.lastJob.errorMessage}
          </Text>
        </div>
      )}
      
      {/* Actions */}
      <div style={{ display: 'flex', gap: '8px' }}>
        <Button 
          appearance="primary" 
          size="small"
          icon={<Play24Regular />}
          disabled={isRunning || !profile.isEnabled}
        >
          Chạy ngay
        </Button>
        <Tooltip content={profile.isEnabled ? 'Tạm dừng' : 'Bật'} relationship="label">
          <Button 
            size="small"
            appearance={profile.isEnabled ? 'secondary' : 'primary'}
          >
            {profile.isEnabled ? 'Tạm dừng' : 'Bật'}
          </Button>
        </Tooltip>
      </div>
    </Card>
  );
};

function formatLastRun(timestamp: string): string {
  const date = new Date(timestamp);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  
  if (diffMins < 1) return 'Vừa xong';
  if (diffMins < 60) return `${diffMins} phút trước`;
  
  const diffHours = Math.floor(diffMins / 60);
  if (diffHours < 24) return `${diffHours} giờ trước`;
  
  const diffDays = Math.floor(diffHours / 24);
  if (diffDays < 7) return `${diffDays} ngày trước`;
  
  return date.toLocaleDateString('vi-VN');
}

export default SyncProfileCard;
