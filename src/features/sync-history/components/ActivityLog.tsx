// ActivityLog component (Story 3.4 AC#1)
// Displays job history list with human-readable messages
import React, { useState, useEffect } from 'react';
import { Text, Spinner, MessageBar, MessageBarBody, List, ListItem } from '@fluentui/react-components';
import { CheckmarkCircle24Regular, ErrorCircle24Regular, ArrowSync24Regular, Clock24Regular } from '@fluentui/react-icons';
import { listJobs } from '../../../services/tauri/commands';
import type { JobLogSummary } from '../../../types/log';
import { getStatusDisplay, type JobStatus } from '../../../types/job';
import { LogDetailView } from './LogDetailView';

export const ActivityLog: React.FC = () => {
  const [jobs, setJobs] = useState<JobLogSummary[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedJobId, setSelectedJobId] = useState<string | null>(null);

  useEffect(() => {
    loadJobHistory();
  }, []);

  const loadJobHistory = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      // For now, we need a profile ID - in real app this would come from route/context
      // Using empty string to get all jobs
      const result = await listJobs('', 100);
      
      if (result.error || !result.data) {
        throw new Error(result.error?.message ?? 'Failed to load job history');
      }
      
      setJobs(result.data);
    } catch (err) {
      console.error('[ActivityLog] Load error:', err);
      setError('Không thể tải lịch sử đồng bộ');
    } finally {
      setIsLoading(false);
    }
  };

  // Loading state
  if (isLoading) {
    return (
      <div style={{ 
        display: 'flex', 
        flexDirection: 'column', 
        alignItems: 'center', 
        justifyContent: 'center',
        height: '100%',
        gap: '16px'
      }}>
        <Spinner size="large" label="Đang tải lịch sử..." />
      </div>
    );
  }

  // Error state
  if (error) {
    return (
      <MessageBar intent="error">
        <MessageBarBody>{error}</MessageBarBody>
      </MessageBar>
    );
  }

  // Empty state
  if (jobs.length === 0) {
    return (
      <div style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        height: '100%',
        gap: '16px',
        padding: '32px'
      }}>
        <Text size={300} weight="semibold">Chưa có lịch sử đồng bộ</Text>
        <Text size={200} style={{ color: '#616161', textAlign: 'center' }}>
          Lịch sử các lần chạy sẽ hiển thị tại đây
        </Text>
      </div>
    );
  }

  // Job list with detail view
  return (
    <div style={{ display: 'flex', height: '100%' }}>
      {/* Job list */}
      <div style={{ 
        width: '400px', 
        borderRight: '1px solid #E0E0E0',
        overflow: 'auto',
        padding: '16px'
      }}>
        <Text size={400} weight="semibold" style={{ marginBottom: '16px', display: 'block' }}>
          Lịch sử đồng bộ
        </Text>
        
        <List>
          {jobs.map((job) => {
            const statusDisplay = getStatusDisplay(job.status as JobStatus);
            const StatusIcon = job.status === 'succeeded' 
              ? CheckmarkCircle24Regular 
              : job.status === 'failed' 
                ? ErrorCircle24Regular 
                : job.status === 'running'
                  ? ArrowSync24Regular
                  : Clock24Regular;
            
            return (
              <ListItem
                key={job.jobId}
                onClick={() => setSelectedJobId(job.jobId)}
                style={{
                  padding: '12px',
                  marginBottom: '8px',
                  cursor: 'pointer',
                  backgroundColor: selectedJobId === job.jobId ? '#F3F3F3' : 'transparent',
                  borderRadius: '4px',
                  border: selectedJobId === job.jobId ? '1px solid #0078D4' : '1px solid transparent'
                }}
              >
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '4px' }}>
                  <StatusIcon />
                  <Text size={200} weight="medium">{statusDisplay.text}</Text>
                </div>
                <Text size={200} style={{ display: 'block', color: '#616161' }}>
                  {formatDateTime(job.completedAt ?? job.startedAt)}
                </Text>
                {job.itemsProcessed !== null && (
                  <Text size={200} style={{ display: 'block', color: '#616161' }}>
                    Đã xử lý: {job.itemsProcessed} file
                  </Text>
                )}
              </ListItem>
            );
          })}
        </List>
      </div>
      
      {/* Log detail view */}
      <div style={{ flex: 1, overflow: 'auto', padding: '16px' }}>
        {selectedJobId ? (
          <LogDetailView jobId={selectedJobId} />
        ) : (
          <div style={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            height: '100%'
          }}>
            <Text size={200} style={{ color: '#616161' }}>
              Chọn một mục để xem chi tiết log
            </Text>
          </div>
        )}
      </div>
    </div>
  );
};

function formatDateTime(timestamp: string): string {
  const date = new Date(timestamp);
  return date.toLocaleString('vi-VN', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

export default ActivityLog;
